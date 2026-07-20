//! Keyboard shortcut for triggering actions.
//!
//! Wraps [`QShortcut`](https://doc.qt.io/qt-6/qshortcut.html).
//!
//! # Note
//!
//! `QShortcut` is a `QObject` subclass, **not** a `QWidget`. It does **not**
//! implement [`AsWidget`](crate::widget::AsWidget). Signal closures are always
//! reclaimed on [`Drop`]; the C++ object is always deleted.

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};

/// A keyboard shortcut that triggers a callback when pressed.
///
/// `Shortcut` uses a **builder pattern**: call [`Shortcut::new`] to obtain
/// a [`Builder`], chain configuration, then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_activated`] / [`Shortcut::connect_activated`] | `QShortcut::activated` | `()` |
///
/// # Example
///
/// ```no_run
/// use qtrs::Shortcut;
///
/// let sc = Shortcut::new("Ctrl+O")
///     .on_activated(|| println!("opened!"))
///     .build();
/// ```
pub struct Shortcut {
    ptr: *mut ffi::QShortcut,
    signal_handles: Vec<SignalHandle>,
}

impl Shortcut {
    /// Start building a new shortcut with the given key sequence.
    pub fn new(key: impl Into<String>) -> Builder {
        Builder::new(key.into())
    }

    // --- Properties ---

    /// Set the keyboard shortcut key.
    pub fn set_key(&self, key: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_key = key);
        unsafe { ffi::QShortcut_setKey(self.ptr, &c_key); }
    }

    /// Enable or disable the shortcut.
    pub fn set_enabled(&self, enabled: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QShortcut_setEnabled(self.ptr, enabled); }
    }

    /// Set whether the shortcut auto-repeats when held.
    pub fn set_auto_repeat(&self, repeat: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QShortcut_setAutoRepeat(self.ptr, repeat); }
    }

    // --- Runtime signal connections ---

    /// Connect a callback when the shortcut is activated.
    pub fn connect_activated<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QShortcut_onActivated(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }
}

impl Drop for Shortcut {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        for h in self.signal_handles.drain(..) {
            unsafe { h.reclaim(); }
        }
        unsafe { ffi::QShortcut_delete(self.ptr); }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`Shortcut`].
pub struct Builder {
    key: String,
    enabled: Option<bool>,
    auto_repeat: Option<bool>,
    on_activated: Option<Box<dyn Fn()>>,
}

impl Builder {
    fn new(key: String) -> Self {
        Self {
            key,
            enabled: None,
            auto_repeat: None,
            on_activated: None,
        }
    }

    /// Enable or disable the shortcut.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Set whether the shortcut auto-repeats when held.
    pub fn auto_repeat(mut self, repeat: bool) -> Self {
        self.auto_repeat = Some(repeat);
        self
    }

    /// Called when the shortcut is activated.
    pub fn on_activated<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_activated = Some(Box::new(f));
        self
    }

    /// Create the C++ `QShortcut` and return the Rust wrapper.
    pub fn build(self) -> Shortcut {
        let_cxx_string!(c_key = &self.key);
        let ptr = unsafe {
            ffi::QShortcut_new(&c_key, std::ptr::null_mut())
        };
        debug_assert!(!ptr.is_null());
        let mut sc = Shortcut { ptr, signal_handles: Vec::new() };

        if let Some(enabled) = self.enabled {
            unsafe { ffi::QShortcut_setEnabled(ptr, enabled); }
        }
        if let Some(repeat) = self.auto_repeat {
            unsafe { ffi::QShortcut_setAutoRepeat(ptr, repeat); }
        }

        if let Some(f) = self.on_activated {
            let h = signal::leak_void(f);
            unsafe { ffi::QShortcut_onActivated(ptr, h.token); }
            sc.signal_handles.push(h);
        }

        sc
    }
}
