//! Date-time editing widget.
//!
//! Wraps [`QDateTimeEdit`](https://doc.qt.io/qt-6/qdatetimeedit.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A date-time editing widget.
///
/// Use a **builder pattern**: [`DateTimeEdit::new`] returns a [`Builder`].
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_date_time_changed`] | `QDateTimeEdit::dateTimeChanged` | `String` (ISO datetime) |
pub struct DateTimeEdit {
    ptr: *mut ffi::QDateTimeEdit,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl DateTimeEdit {
    /// Start building a new date-time editor.
    pub fn new() -> Builder { Builder::new() }

    /// Set the date-time using an ISO 8601 string (e.g. "2024-03-15T14:30:00").
    pub fn set_date_time(&self, dt: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_dt = dt);
        unsafe { ffi::QDateTimeEdit_setDateTime(self.ptr, &c_dt); }
    }

    /// Get the current date-time as an ISO 8601 string.
    pub fn date_time(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDateTimeEdit_dateTime(self.ptr) }
    }

    /// Set the display format (e.g. "yyyy-MM-dd HH:mm:ss").
    pub fn set_display_format(&self, format: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_format = format);
        unsafe { ffi::QDateTimeEdit_setDisplayFormat(self.ptr, &c_format); }
    }

    /// Enable or disable the calendar popup.
    pub fn set_calendar_popup(&self, enabled: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDateTimeEdit_setCalendarPopup(self.ptr, enabled); }
    }

    /// Connect a callback that fires when the date-time changes. Receives the ISO string.
    pub fn connect_date_time_changed<F: Fn(String)>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QDateTimeEdit_onDateTimeChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QDateTimeEdit, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for DateTimeEdit {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QDateTimeEdit(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for DateTimeEdit {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QDateTimeEdit_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`DateTimeEdit`].
pub struct Builder {
    on_date_time_changed: Option<Box<dyn Fn(String)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self { on_date_time_changed: None, parent: None }
    }

    /// Called when the date-time changes. Receives the ISO datetime string.
    pub fn on_date_time_changed<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_date_time_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QDateTimeEdit` and return the Rust wrapper.
    pub fn build(self) -> DateTimeEdit {
        let ptr = unsafe {
            ffi::QDateTimeEdit_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut dte = DateTimeEdit {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(f) = self.on_date_time_changed {
            let h = signal::leak_string(f);
            unsafe { ffi::QDateTimeEdit_onDateTimeChanged(ptr, h.token); }
            dte.signal_handles.push(h);
        }
        dte
    }
}
