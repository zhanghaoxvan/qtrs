//! Status bar widget.
//!
//! Wraps [`QStatusBar`](https://doc.qt.io/qt-6/qstatusbar.html).

use crate::ffi;
use crate::signal::SignalHandle;
use crate::widget::AsWidget;
use cxx::let_cxx_string;

/// A status bar for displaying status information at the bottom of a window.
///
/// `StatusBar` uses a **builder pattern**: call [`StatusBar::new`] to obtain
/// a [`Builder`], chain configuration, then call `.build()`.
///
/// # Example
///
/// ```no_run
/// use qtrs::StatusBar;
///
/// let bar = StatusBar::new().build();
/// bar.show_message("Ready", 3000);
/// ```
pub struct StatusBar {
    pub(crate) ptr: *mut ffi::QStatusBar,
    pub(crate) has_parent: bool,
    pub(crate) signal_handles: Vec<SignalHandle>,
}

impl StatusBar {
    /// Start building a new status bar.
    pub fn new() -> Builder { Builder::new() }

    /// Show a temporary message for the given number of milliseconds.
    pub fn show_message(&self, text: &str, timeout_ms: i32) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QStatusBar_showMessage(self.ptr, &c_text, timeout_ms); }
    }

    /// Clear the current message.
    pub fn clear_message(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStatusBar_clearMessage(self.ptr); }
    }

    /// Add a widget to the status bar (left side).
    pub fn add_widget(&self, w: &dyn AsWidget) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStatusBar_addWidget(self.ptr, w.widget_ptr()); }
    }

    /// Add a permanent widget to the status bar (right side).
    pub fn add_permanent_widget(&self, w: &dyn AsWidget) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStatusBar_addPermanentWidget(self.ptr, w.widget_ptr()); }
    }

    /// Remove a widget from the status bar.
    pub fn remove_widget(&self, w: &dyn AsWidget) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStatusBar_removeWidget(self.ptr, w.widget_ptr()); }
    }

    /// Set whether the size grip is enabled.
    pub fn set_size_grip_enabled(&self, enabled: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStatusBar_setSizeGripEnabled(self.ptr, enabled); }
    }

    /// Get the raw status bar pointer for use with [`MainWindow::set_status_bar`](crate::MainWindow).
    pub(crate) fn statusbar_ptr(&self) -> *mut ffi::QStatusBar {
        self.ptr
    }

    /// Wrap an existing `QStatusBar*` obtained via `findChild`.
    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QStatusBar) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for StatusBar {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QStatusBar(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for StatusBar {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QStatusBar_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`StatusBar`].
pub struct Builder {
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self { parent: None }
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QStatusBar` and return the Rust wrapper.
    pub fn build(self) -> StatusBar {
        let ptr = unsafe {
            ffi::QStatusBar_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        StatusBar {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        }
    }
}
