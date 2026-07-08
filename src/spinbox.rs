//! Spin box widget for integer input.
//!
//! Wraps [`QSpinBox`](https://doc.qt.io/qt-6/qspinbox.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// An integer input widget with up/down buttons.
///
/// `SpinBox` uses a builder pattern: call [`SpinBox::new`] to obtain
/// a [`Builder`], chain configuration methods, then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_value_changed`] | `QSpinBox::valueChanged` | `i32` (new value) |
///
/// # Example
///
/// ```no_run
/// use qtrs::SpinBox;
///
/// let sb = SpinBox::new()
///     .range(0, 100)
///     .suffix(" cm")
///     .on_value_changed(|v| println!("value: {}", v))
///     .build();
/// ```
pub struct SpinBox {
    ptr: *mut ffi::QSpinBox,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl SpinBox {
    /// Start building a new spin box.
    pub fn new() -> Builder {
        Builder::new()
    }

    /// Set the current value.
    pub fn set_value(&self, value: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSpinBox_setValue(self.ptr, value); }
    }

    /// Get the current value.
    pub fn value(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSpinBox_value(self.ptr) }
    }

    /// Set the value range. Default is `0` to `99`.
    pub fn set_range(&self, min: i32, max: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSpinBox_setRange(self.ptr, min, max); }
    }

    /// Set the suffix text (e.g. " cm", " kg").
    pub fn set_suffix(&self, suffix: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_suffix = suffix);
        unsafe { ffi::QSpinBox_setSuffix(self.ptr, &c_suffix); }
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QSpinBox) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for SpinBox {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QSpinBox(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for SpinBox {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
        } else {
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
            unsafe { ffi::QSpinBox_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`SpinBox`].
pub struct Builder {
    min: i32,
    max: i32,
    value: i32,
    suffix: Option<String>,
    on_value_changed: Option<Box<dyn Fn(i32)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            min: 0,
            max: 99,
            value: 0,
            suffix: None,
            on_value_changed: None,
            parent: None,
        }
    }

    /// Set the value range. Default is `0` to `99`.
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

    /// Set the suffix text.
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Called when the value changes.
    ///
    /// The callback receives the new value (`i32`).
    pub fn on_value_changed<F: Fn(i32) + 'static>(mut self, f: F) -> Self {
        self.on_value_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QSpinBox` and return the Rust wrapper.
    pub fn build(self) -> SpinBox {
        let ptr = unsafe {
            ffi::QSpinBox_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut sb = SpinBox {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        unsafe {
            ffi::QSpinBox_setRange(ptr, self.min, self.max);
            ffi::QSpinBox_setValue(ptr, self.value);
            if let Some(ref s) = self.suffix {
                let_cxx_string!(c_suffix = s);
                ffi::QSpinBox_setSuffix(ptr, &c_suffix);
            }
        }
        if let Some(f) = self.on_value_changed {
            let h = signal::leak_int(f);
            unsafe { ffi::QSpinBox_onValueChanged(ptr, h.token); }
            sb.signal_handles.push(h);
        }
        sb
    }
}
