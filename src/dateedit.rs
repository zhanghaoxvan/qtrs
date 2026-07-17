//! Date editing widget.
//!
//! Wraps [`QDateEdit`](https://doc.qt.io/qt-6/qdateedit.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A date editing widget.
///
/// Use a **builder pattern**: [`DateEdit::new`] returns a [`Builder`].
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_date_changed`] | `QDateEdit::dateChanged` | `String` (ISO date) |
pub struct DateEdit {
    ptr: *mut ffi::QDateEdit,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl DateEdit {
    /// Start building a new date editor.
    pub fn new() -> Builder { Builder::new() }

    /// Set the date using an ISO 8601 string (e.g. "2024-03-15").
    pub fn set_date(&self, date: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_date = date);
        unsafe { ffi::QDateEdit_setDate(self.ptr, &c_date); }
    }

    /// Get the current date as an ISO 8601 string.
    pub fn date(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDateEdit_date(self.ptr) }
    }

    /// Set the minimum allowed date.
    pub fn set_minimum_date(&self, date: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_date = date);
        unsafe { ffi::QDateEdit_setMinimumDate(self.ptr, &c_date); }
    }

    /// Set the maximum allowed date.
    pub fn set_maximum_date(&self, date: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_date = date);
        unsafe { ffi::QDateEdit_setMaximumDate(self.ptr, &c_date); }
    }

    /// Clear the minimum date constraint.
    pub fn clear_minimum_date(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDateEdit_clearMinimumDate(self.ptr); }
    }

    /// Clear the maximum date constraint.
    pub fn clear_maximum_date(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDateEdit_clearMaximumDate(self.ptr); }
    }

    /// Set the display format (e.g. "yyyy-MM-dd").
    pub fn set_display_format(&self, format: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_format = format);
        unsafe { ffi::QDateEdit_setDisplayFormat(self.ptr, &c_format); }
    }

    /// Enable or disable the calendar popup.
    pub fn set_calendar_popup(&self, enabled: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDateEdit_setCalendarPopup(self.ptr, enabled); }
    }

    /// Connect a callback that fires when the date changes. Receives the ISO date string.
    pub fn connect_date_changed<F: Fn(String) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QDateEdit_onDateChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QDateEdit, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for DateEdit {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QDateEdit(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for DateEdit {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QDateEdit_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`DateEdit`].
pub struct Builder {
    on_date_changed: Option<Box<dyn Fn(String)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self { on_date_changed: None, parent: None }
    }

    /// Called when the date changes. Receives the ISO date string.
    pub fn on_date_changed<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_date_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QDateEdit` and return the Rust wrapper.
    pub fn build(self) -> DateEdit {
        let ptr = unsafe {
            ffi::QDateEdit_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut de = DateEdit {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(f) = self.on_date_changed {
            let h = signal::leak_string(f);
            unsafe { ffi::QDateEdit_onDateChanged(ptr, h.token); }
            de.signal_handles.push(h);
        }
        de
    }
}
