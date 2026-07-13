//! Progress dialog for showing operation progress with optional cancellation.
//!
//! Wraps [`QProgressDialog`](https://doc.qt.io/qt-6/qprogressdialog.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::SignalHandle;
use crate::widget::AsWidget;

/// A progress dialog that shows a progress bar and optional cancel button.
///
/// Use a **builder pattern**: [`ProgressDialog::new`] returns a [`Builder`].
///
/// # Example
///
/// ```no_run
/// use qtrs::ProgressDialog;
///
/// let dialog = ProgressDialog::new("Processing...", "Cancel", 0, 100)
///     .auto_close(true)
///     .auto_reset(true)
///     .build();
///
/// dialog.set_value(50);
/// ```
pub struct ProgressDialog {
    ptr: *mut ffi::QProgressDialog,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl ProgressDialog {
    /// Start building a new progress dialog.
    pub fn new(label: &str, cancel_text: &str, min: i32, max: i32) -> Builder {
        Builder::new(label, cancel_text, min, max)
    }

    // --- Range ---

    /// Set the minimum value.
    pub fn set_minimum(&self, min: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_setMinimum(self.ptr, min); }
    }

    /// Set the maximum value.
    pub fn set_maximum(&self, max: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_setMaximum(self.ptr, max); }
    }

    /// Set the minimum and maximum values.
    pub fn set_range(&self, min: i32, max: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_setRange(self.ptr, min, max); }
    }

    // --- Value ---

    /// Set the current progress value.
    pub fn set_value(&self, value: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_setValue(self.ptr, value); }
    }

    /// Get the current progress value.
    pub fn value(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_value(self.ptr) }
    }

    // --- Label / button text ---

    /// Set the label text.
    pub fn set_label_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QProgressDialog_setLabelText(self.ptr, &c_text); }
    }

    /// Set the cancel button text.
    pub fn set_cancel_button_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QProgressDialog_setCancelButtonText(self.ptr, &c_text); }
    }

    // --- State ---

    /// Check whether the dialog was canceled.
    pub fn was_canceled(&self) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_wasCanceled(self.ptr) }
    }

    /// Set the minimum duration (in ms) before the dialog appears.
    pub fn set_minimum_duration(&self, ms: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_setMinimumDuration(self.ptr, ms); }
    }

    // --- Auto behavior ---

    /// Set whether the dialog auto-closes when the range is reached.
    pub fn set_auto_close(&self, close: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_setAutoClose(self.ptr, close); }
    }

    /// Set whether the dialog auto-resets when the range is reached.
    pub fn set_auto_reset(&self, reset: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_setAutoReset(self.ptr, reset); }
    }

    // --- Visibility ---

    /// Show the dialog.
    pub fn show(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_show(self.ptr); }
    }

    /// Hide the dialog.
    pub fn hide(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_hide(self.ptr); }
    }

    /// Close the dialog.
    pub fn close(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressDialog_close(self.ptr); }
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QProgressDialog) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for ProgressDialog {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QProgressDialog(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for ProgressDialog {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.signal_handles.clear();
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QProgressDialog_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`ProgressDialog`].
pub struct Builder {
    label: String,
    cancel_text: String,
    min: i32,
    max: i32,
    auto_close: Option<bool>,
    auto_reset: Option<bool>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new(label: &str, cancel_text: &str, min: i32, max: i32) -> Self {
        Self {
            label: label.to_string(),
            cancel_text: cancel_text.to_string(),
            min,
            max,
            auto_close: None,
            auto_reset: None,
            parent: None,
        }
    }

    /// Enable auto-close when the progress range is reached.
    pub fn auto_close(mut self, close: bool) -> Self {
        self.auto_close = Some(close);
        self
    }

    /// Enable auto-reset when the progress range is reached.
    pub fn auto_reset(mut self, reset: bool) -> Self {
        self.auto_reset = Some(reset);
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QProgressDialog` and return the Rust wrapper.
    pub fn build(self) -> ProgressDialog {
        let_cxx_string!(c_label = &self.label);
        let_cxx_string!(c_cancel = &self.cancel_text);
        let ptr = unsafe {
            ffi::QProgressDialog_new(
                &c_label,
                &c_cancel,
                self.min,
                self.max,
                self.parent.unwrap_or(std::ptr::null_mut()),
            )
        };
        debug_assert!(!ptr.is_null());
        let has_parent = self.parent.is_some();
        let pd = ProgressDialog {
            ptr,
            has_parent,
            signal_handles: Vec::new(),
        };
        if let Some(close) = self.auto_close {
            unsafe { ffi::QProgressDialog_setAutoClose(ptr, close); }
        }
        if let Some(reset) = self.auto_reset {
            unsafe { ffi::QProgressDialog_setAutoReset(ptr, reset); }
        }
        pd
    }
}
