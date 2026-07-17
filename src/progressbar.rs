//! Progress bar widget.
//!
//! Wraps [`QProgressBar`](https://doc.qt.io/qt-6/qprogressbar.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::widget::AsWidget;

/// A progress bar.
///
/// `ProgressBar` uses a builder pattern: call [`ProgressBar::new`] to obtain
/// a [`Builder`], chain configuration methods, then call `.build()`.
///
/// # Example
///
/// ```no_run
/// use qtrs::ProgressBar;
///
/// let bar = ProgressBar::new()
///     .range(0, 100)
///     .format("%p%")
///     .build();
///
/// bar.set_value(50); // 50%
/// ```
pub struct ProgressBar {
    ptr: *mut ffi::QProgressBar,
    has_parent: bool,
}

impl ProgressBar {
    /// Start building a new progress bar.
    pub fn new() -> Builder {
        Builder::new()
    }

    /// Set the current value.
    pub fn set_value(&self, value: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressBar_setValue(self.ptr, value); }
    }

    /// Get the current value.
    pub fn value(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressBar_value(self.ptr) }
    }

    /// Set the range (minimum and maximum).
    pub fn set_range(&self, min: i32, max: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressBar_setRange(self.ptr, min, max); }
    }

    /// Set the minimum value.
    pub fn set_minimum(&self, min: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressBar_setMinimum(self.ptr, min); }
    }

    /// Set the maximum value.
    pub fn set_maximum(&self, max: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QProgressBar_setMaximum(self.ptr, max); }
    }

    /// Set the display format.
    ///
    /// - `"%p%"` — percentage (default)
    /// - `"%v"` — current value
    /// - `"%m"` — maximum value
    /// - `"%v/%m"` — value/maximum
    pub fn set_format(&self, format: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_format = format);
        unsafe { ffi::QProgressBar_setFormat(self.ptr, &c_format); }
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QProgressBar) -> Self {
        debug_assert!(!ptr.is_null());
        Self {
            ptr,
            has_parent: true,
        }
    }
}

impl AsWidget for ProgressBar {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QProgressBar(self.ptr) }
    }

    fn set_has_parent(&mut self) {
        self.has_parent = true;
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        if !self.has_parent {
            unsafe { ffi::QProgressBar_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`ProgressBar`].
pub struct Builder {
    min: i32,
    max: i32,
    value: i32,
    format: Option<String>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            min: 0,
            max: 100,
            value: 0,
            format: None,
            parent: None,
        }
    }

    /// Set the range.
    pub fn range(mut self, min: i32, max: i32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// Set the initial value.
    pub fn value(mut self, value: i32) -> Self {
        self.value = value;
        self
    }

    /// Set the display format.
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QProgressBar` and return the Rust wrapper.
    pub fn build(self) -> ProgressBar {
        let ptr = unsafe {
            ffi::QProgressBar_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        assert!(!ptr.is_null(), "QProgressBar_new returned null");

        let bar = ProgressBar {
            ptr,
            has_parent: self.parent.is_some(),
        };

        unsafe {
            ffi::QProgressBar_setRange(ptr, self.min, self.max);
            ffi::QProgressBar_setValue(ptr, self.value);
            if let Some(ref fmt) = self.format {
                let_cxx_string!(c_fmt = fmt);
                ffi::QProgressBar_setFormat(ptr, &c_fmt);
            }
        }

        bar
    }
}