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
// SignalHandle — tagged token so Drop can reclaim correctly
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
        if self.token == 0 {
            return;
        }
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
// Leak helpers — one per callback signature
// ============================================================

// `leak_fn!` generates one leak function per signal signature.
// Takes: $name (function name), $kind (SignalKind variant),
//        $bound: the Fn(…) trait, $dyn: the dyn Fn(…) trait-object type.
macro_rules! leak_fn {
    ($name:ident, $kind:ident, $bound:path, $dyn:ty) => {
        pub(crate) fn $name<F>(f: F) -> SignalHandle
        where F: $bound
        {
            ensure_trampolines_registered();
            // SAFETY: closure is reclaimed in Drop before the widget (and its
            // captured refs) is destroyed.  Qt signals are disconnected first.
            let thin: *mut F = Box::into_raw(Box::new(f));
            let fat: *mut $dyn = thin; // raw-pointer unsizing — no lifetime check
            let inner: Box<$dyn> = unsafe { std::mem::transmute(fat) };
            SignalHandle {
                token: Box::into_raw(Box::new(inner)) as u64,
                kind: SignalKind::$kind,
            }
        }
    };
}

leak_fn!(leak_void, Void, Fn(), dyn Fn());
leak_fn!(leak_bool, Bool, Fn(bool), dyn Fn(bool));
leak_fn!(leak_int, Int, Fn(i32), dyn Fn(i32));
leak_fn!(leak_string, String, Fn(String), dyn Fn(String));

/// Convenience — leaks a void callback (most common).
#[allow(dead_code)]
pub(crate) fn leak<F: Fn()>(f: F) -> SignalHandle {
    leak_void(f)
}

// ============================================================
// Trampolines — called from C++ via registered function pointers
// ============================================================

macro_rules! define_trampoline {
    (void, $name:ident, ()) => {
        pub(crate) fn $name(ctx: u64) {
            debug_assert_ne!(ctx, 0, concat!(stringify!($name), " called with null ctx"));
            let cb: &Box<dyn Fn()> = unsafe { &*(ctx as *const Box<dyn Fn()>) };
            cb();
        }
    };
    (bool, $name:ident, $arg:ident : $ty:ty) => {
        pub(crate) fn $name(ctx: u64, $arg: $ty) {
            debug_assert_ne!(ctx, 0, concat!(stringify!($name), " called with null ctx"));
            let cb: &Box<dyn Fn($ty)> = unsafe { &*(ctx as *const Box<dyn Fn($ty)>) };
            cb($arg);
        }
    };
    (int, $name:ident, $arg:ident : $ty:ty) => {
        pub(crate) fn $name(ctx: u64, $arg: $ty) {
            debug_assert_ne!(ctx, 0, concat!(stringify!($name), " called with null ctx"));
            let cb: &Box<dyn Fn($ty)> = unsafe { &*(ctx as *const Box<dyn Fn($ty)>) };
            cb($arg);
        }
    };
    (string, $name:ident, $arg:ident : $ty:ty) => {
        pub(crate) fn $name(ctx: u64, $arg: $ty) {
            debug_assert_ne!(ctx, 0, concat!(stringify!($name), " called with null ctx"));
            let cb: &Box<dyn Fn($ty)> = unsafe { &*(ctx as *const Box<dyn Fn($ty)>) };
            cb($arg);
        }
    };
}

define_trampoline!(void, trampoline_void, ());
define_trampoline!(bool, trampoline_bool, value: bool);
define_trampoline!(int, trampoline_int, value: i32);
define_trampoline!(string, trampoline_string, value: String);

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_void_callback() {
        let called = Arc::new(AtomicBool::new(false));
        let c = called.clone();
        let h = leak(move || { c.store(true, Ordering::SeqCst); });
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
