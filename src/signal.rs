// src/signal.rs — Signal/callback bridging infrastructure
//
// ## Memory model
//
// Closures are double-boxed and leaked onto the heap. The outer Box
// provides a stable heap address; the inner Box provides type erasure.
// The raw pointer is cast to `u64` and stored in the C++ lambda capture.
//
// ## Safety invariant
//
// On Drop with `has_parent = false`: reclaim closure, then delete C++ widget.
//   → Safe: no more signals can fire after widget deletion.
// On Drop with `has_parent = true`:  leak closure intentionally.
//   → Safe: prevents use-after-free (C++ widget may outlive Rust wrapper).

use crate::ffi;
use std::sync::Once;

static INIT: Once = Once::new();

pub(crate) fn ensure_trampolines_registered() {
    INIT.call_once(|| unsafe {
        ffi::qtrs_setVoidTrampoline(trampoline_void);
        ffi::qtrs_setBoolTrampoline(trampoline_bool);
        ffi::qtrs_setIntTrampoline(trampoline_int);
        ffi::qtrs_setStringTrampoline(trampoline_string);
    });
}

// ============================================================
// Signal handle — tagged token so Drop can reclaim correctly
// ============================================================

#[derive(Clone, Copy)]
pub(crate) enum SignalKind {
    Void,
    Bool,
    Int,
    String,
}

pub(crate) struct SignalHandle {
    pub token: u64,
    pub kind: SignalKind,
}

impl SignalHandle {
    pub(crate) unsafe fn reclaim(self) {
        if self.token == 0 { return; }
        match self.kind {
            SignalKind::Void => {
                let _ = Box::from_raw(self.token as *mut Box<dyn Fn()>);
            }
            SignalKind::Bool => {
                let _ = Box::from_raw(self.token as *mut Box<dyn Fn(bool)>);
            }
            SignalKind::Int => {
                let _ = Box::from_raw(self.token as *mut Box<dyn Fn(i32)>);
            }
            SignalKind::String => {
                let _ = Box::from_raw(self.token as *mut Box<dyn Fn(String)>);
            }
        }
    }
}

// ============================================================
// Leak helpers
// ============================================================

pub(crate) fn leak_void<F: Fn() + 'static>(f: F) -> SignalHandle {
    ensure_trampolines_registered();
    let boxed: Box<dyn Fn()> = Box::new(f);
    SignalHandle {
        token: Box::into_raw(Box::new(boxed)) as u64,
        kind: SignalKind::Void,
    }
}

pub(crate) fn leak_bool<F: Fn(bool) + 'static>(f: F) -> SignalHandle {
    ensure_trampolines_registered();
    let boxed: Box<dyn Fn(bool)> = Box::new(f);
    SignalHandle {
        token: Box::into_raw(Box::new(boxed)) as u64,
        kind: SignalKind::Bool,
    }
}

pub(crate) fn leak_int<F: Fn(i32) + 'static>(f: F) -> SignalHandle {
    ensure_trampolines_registered();
    let boxed: Box<dyn Fn(i32)> = Box::new(f);
    SignalHandle {
        token: Box::into_raw(Box::new(boxed)) as u64,
        kind: SignalKind::Int,
    }
}

pub(crate) fn leak_string<F: Fn(String) + 'static>(f: F) -> SignalHandle {
    ensure_trampolines_registered();
    let boxed: Box<dyn Fn(String)> = Box::new(f);
    SignalHandle {
        token: Box::into_raw(Box::new(boxed)) as u64,
        kind: SignalKind::String,
    }
}

// ============================================================
// Trampolines
// ============================================================

pub(crate) fn trampoline_void(ctx: u64) {
    debug_assert_ne!(ctx, 0, "trampoline_void called with null ctx");
    let cb: &Box<dyn Fn()> = unsafe { &*(ctx as *const Box<dyn Fn()>) };
    cb();
}

pub(crate) fn trampoline_bool(ctx: u64, value: bool) {
    debug_assert_ne!(ctx, 0, "trampoline_bool called with null ctx");
    let cb: &Box<dyn Fn(bool)> = unsafe { &*(ctx as *const Box<dyn Fn(bool)>) };
    cb(value);
}

pub(crate) fn trampoline_int(ctx: u64, value: i32) {
    debug_assert_ne!(ctx, 0, "trampoline_int called with null ctx");
    let cb: &Box<dyn Fn(i32)> = unsafe { &*(ctx as *const Box<dyn Fn(i32)>) };
    cb(value);
}

pub(crate) fn trampoline_string(ctx: u64, value: String) {
    debug_assert_ne!(ctx, 0, "trampoline_string called with null ctx");
    let cb: &Box<dyn Fn(String)> = unsafe { &*(ctx as *const Box<dyn Fn(String)>) };
    cb(value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_void_callback() {
        let called = Arc::new(AtomicBool::new(false));
        let c = called.clone();
        let h = leak_void(move || { c.store(true, Ordering::SeqCst); });
        trampoline_void(h.token);
        assert!(called.load(Ordering::SeqCst));
        unsafe { h.reclaim(); }
    }

    #[test]
    fn test_bool_callback() {
        let val = Arc::new(AtomicBool::new(false));
        let v = val.clone();
        let h = leak_bool(move |b| { v.store(b, Ordering::SeqCst); });
        trampoline_bool(h.token, true);
        assert!(val.load(Ordering::SeqCst));
        unsafe { h.reclaim(); }
    }

    #[test]
    fn test_int_callback() {
        let val = Arc::new(AtomicI32::new(0));
        let v = val.clone();
        let h = leak_int(move |i| { v.store(i, Ordering::SeqCst); });
        trampoline_int(h.token, 42);
        assert_eq!(val.load(Ordering::SeqCst), 42);
        unsafe { h.reclaim(); }
    }
}
