//! Action widget for menus and toolbars.
//!
//! Wraps [`QAction`](https://doc.qt.io/qt-6/qaction.html).
//!
//! # Note
//!
//! `QAction` is **not** a `QWidget`, so [`Action`] does **not** implement
//! [`AsWidget`](crate::widget::AsWidget).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};

/// A user interface action that can be added to menus, toolbars, etc.
///
/// `Action` uses a **builder pattern**: call [`Action::new`] to obtain
/// a [`Builder`], chain configuration, then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_triggered`] / [`Action::connect_triggered`] | `QAction::triggered` | `bool` (checked state) |
/// | [`Builder::on_toggled`] / [`Action::connect_toggled`] | `QAction::toggled` | `bool` (checked state) |
///
/// # Memory safety
///
/// `QAction` is a `QObject` subclass, not a `QWidget`. It does not
/// participate in the widget parent-child tree. Signal closures are
/// always reclaimed on [`Drop`]; the C++ object is always deleted.
///
/// # Example
///
/// ```no_run
/// use qtrs::Action;
///
/// let action = Action::new("Open")
///     .on_triggered(|checked| println!("Open triggered, checked={}", checked))
///     .build();
/// ```
pub struct Action {
    ptr: *mut ffi::QAction,
    signal_handles: Vec<SignalHandle>,
}

impl Action {
    /// Start building a new `QAction`.
    pub fn new(text: impl Into<String>) -> Builder {
        Builder::new(text.into())
    }

    // --- Properties ---

    /// Set the action text.
    pub fn set_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QAction_setText(self.ptr, &c_text); }
    }

    /// Get the action text.
    pub fn text(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QAction_text(self.ptr) }
    }

    /// Set the action icon from a file path.
    pub fn set_icon(&self, icon_path: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_path = icon_path);
        unsafe { ffi::QAction_setIcon(self.ptr, &c_path); }
    }

    /// Make this action checkable.
    pub fn set_checkable(&self, checkable: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QAction_setCheckable(self.ptr, checkable); }
    }

    /// Set the checked state.
    pub fn set_checked(&self, checked: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QAction_setChecked(self.ptr, checked); }
    }

    /// Get the checked state.
    pub fn is_checked(&self) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QAction_isChecked(self.ptr) }
    }

    /// Set a keyboard shortcut.
    pub fn set_shortcut(&self, key: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_key = key);
        unsafe { ffi::QAction_setShortcut(self.ptr, &c_key); }
    }

    /// Enable or disable the action.
    pub fn set_enabled(&self, enabled: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QAction_setEnabled(self.ptr, enabled); }
    }

    /// Get whether the action is enabled.
    pub fn is_enabled(&self) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QAction_isEnabled(self.ptr) }
    }

    /// Set a tool-tip for this action.
    pub fn set_tool_tip(&self, tip: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_tip = tip);
        unsafe { ffi::QAction_setToolTip(self.ptr, &c_tip); }
    }

    // --- Runtime signal connections ---

    /// Connect a callback when the action is triggered. Receives the checked state.
    pub fn connect_triggered<F: Fn(bool)>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_bool(f);
        unsafe { ffi::QAction_onTriggered(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback when the action toggles. Receives the checked state.
    pub fn connect_toggled<F: Fn(bool)>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_bool(f);
        unsafe { ffi::QAction_onToggled(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }
}

impl Drop for Action {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        for h in self.signal_handles.drain(..) {
            unsafe { h.reclaim(); }
        }
        unsafe { ffi::QAction_delete(self.ptr); }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`Action`].
pub struct Builder {
    text: String,
    on_triggered: Option<Box<dyn Fn(bool)>>,
    on_toggled: Option<Box<dyn Fn(bool)>>,
}

impl Builder {
    fn new(text: String) -> Self {
        Self {
            text,
            on_triggered: None,
            on_toggled: None,
        }
    }

    /// Called when the action is triggered. Receives the checked state.
    pub fn on_triggered<F: Fn(bool) + 'static>(mut self, f: F) -> Self {
        self.on_triggered = Some(Box::new(f));
        self
    }

    /// Called when the action toggles. Receives the checked state.
    pub fn on_toggled<F: Fn(bool) + 'static>(mut self, f: F) -> Self {
        self.on_toggled = Some(Box::new(f));
        self
    }

    /// Create the C++ `QAction` and return the Rust wrapper.
    pub fn build(self) -> Action {
        let_cxx_string!(c_text = &self.text);
        let ptr = unsafe {
            ffi::QAction_new(&c_text, std::ptr::null_mut())
        };
        debug_assert!(!ptr.is_null());

        let mut signal_handles = Vec::new();

        if let Some(cb) = self.on_triggered {
            let handle = signal::leak_bool(cb);
            unsafe { ffi::QAction_onTriggered(ptr, handle.token); }
            signal_handles.push(handle);
        }
        if let Some(cb) = self.on_toggled {
            let handle = signal::leak_bool(cb);
            unsafe { ffi::QAction_onToggled(ptr, handle.token); }
            signal_handles.push(handle);
        }

        Action { ptr, signal_handles }
    }
}
