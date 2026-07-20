//! Time editing widget.
//!
//! Wraps [`QTimeEdit`](https://doc.qt.io/qt-6/qtimeedit.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A time editing widget.
///
/// Use a **builder pattern**: [`TimeEdit::new`] returns a [`Builder`].
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_time_changed`] | `QTimeEdit::timeChanged` | `String` (ISO time) |
pub struct TimeEdit {
    ptr: *mut ffi::QTimeEdit,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl TimeEdit {
    /// Start building a new time editor.
    pub fn new() -> Builder { Builder::new() }

    /// Set the time using an ISO 8601 string (e.g. "14:30:00").
    pub fn set_time(&self, time: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_time = time);
        unsafe { ffi::QTimeEdit_setTime(self.ptr, &c_time); }
    }

    /// Get the current time as an ISO 8601 string.
    pub fn time(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTimeEdit_time(self.ptr) }
    }

    /// Set the display format (e.g. "HH:mm:ss").
    pub fn set_display_format(&self, format: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_format = format);
        unsafe { ffi::QTimeEdit_setDisplayFormat(self.ptr, &c_format); }
    }

    /// Connect a callback that fires when the time changes. Receives the ISO time string.
    pub fn connect_time_changed<F: Fn(String)>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QTimeEdit_onTimeChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QTimeEdit, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for TimeEdit {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QTimeEdit(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for TimeEdit {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QTimeEdit_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`TimeEdit`].
pub struct Builder {
    on_time_changed: Option<Box<dyn Fn(String)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self { on_time_changed: None, parent: None }
    }

    /// Called when the time changes. Receives the ISO time string.
    pub fn on_time_changed<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_time_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QTimeEdit` and return the Rust wrapper.
    pub fn build(self) -> TimeEdit {
        let ptr = unsafe {
            ffi::QTimeEdit_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut te = TimeEdit {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(f) = self.on_time_changed {
            let h = signal::leak_string(f);
            unsafe { ffi::QTimeEdit_onTimeChanged(ptr, h.token); }
            te.signal_handles.push(h);
        }
        te
    }
}
