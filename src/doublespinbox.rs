//! Double spin box widget for floating-point input.
//!
//! Wraps [`QDoubleSpinBox`](https://doc.qt.io/qt-6/qdoublespinbox.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A spin box for floating-point numbers with up/down buttons.
///
/// `DoubleSpinBox` uses a builder pattern: call [`DoubleSpinBox::new`] to
/// obtain a [`Builder`], chain configuration methods, then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_value_changed`] | `QDoubleSpinBox::valueChanged` | `String` (new value as string) |
///
/// # Example
///
/// ```no_run
/// use qtrs::DoubleSpinBox;
///
/// let sb = DoubleSpinBox::new()
///     .range(0.0, 100.0)
///     .decimals(2)
///     .suffix(" kg")
///     .on_value_changed(|v| println!("value: {}", v))
///     .build();
/// ```
pub struct DoubleSpinBox {
    ptr: *mut ffi::QDoubleSpinBox,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl DoubleSpinBox {
    /// Start building a new double spin box.
    pub fn new() -> Builder {
        Builder::new()
    }

    /// Set the current value.
    pub fn set_value(&self, value: f64) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_setValue(self.ptr, value); }
    }

    /// Get the current value.
    pub fn value(&self) -> f64 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_value(self.ptr) }
    }

    /// Set the value range.
    pub fn set_range(&self, min: f64, max: f64) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_setRange(self.ptr, min, max); }
    }

    /// Set the single step increment.
    pub fn set_single_step(&self, step: f64) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_setSingleStep(self.ptr, step); }
    }

    /// Get the single step increment.
    pub fn single_step(&self) -> f64 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_singleStep(self.ptr) }
    }

    /// Set the number of decimal places displayed.
    pub fn set_decimals(&self, decimals: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_setDecimals(self.ptr, decimals); }
    }

    /// Get the number of decimal places.
    pub fn decimals(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_decimals(self.ptr) }
    }

    /// Set the prefix text (e.g. "$").
    pub fn set_prefix(&self, prefix: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_prefix = prefix);
        unsafe { ffi::QDoubleSpinBox_setPrefix(self.ptr, &c_prefix); }
    }

    /// Set the suffix text (e.g. " kg").
    pub fn set_suffix(&self, suffix: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_suffix = suffix);
        unsafe { ffi::QDoubleSpinBox_setSuffix(self.ptr, &c_suffix); }
    }

    /// Set the minimum value.
    pub fn set_minimum(&self, min: f64) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_setMinimum(self.ptr, min); }
    }

    /// Get the minimum value.
    pub fn minimum(&self) -> f64 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_minimum(self.ptr) }
    }

    /// Set the maximum value.
    pub fn set_maximum(&self, max: f64) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_setMaximum(self.ptr, max); }
    }

    /// Get the maximum value.
    pub fn maximum(&self) -> f64 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_maximum(self.ptr) }
    }

    /// Set whether the spin box is read-only.
    pub fn set_read_only(&self, read_only: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_setReadOnly(self.ptr, read_only); }
    }

    /// Set whether group separators (e.g. thousands) are shown.
    pub fn set_group_separator_shown(&self, shown: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDoubleSpinBox_setGroupSeparatorShown(self.ptr, shown); }
    }

    /// Connect a value-changed callback to an already-existing spin box.
    ///
    /// The callback receives the new value as a `String` (parsed from the
    /// display text). Use `.parse::<f64>()` to convert.
    pub fn connect_value_changed<F: Fn(String)>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QDoubleSpinBox_onValueChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QDoubleSpinBox) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for DoubleSpinBox {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QDoubleSpinBox(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for DoubleSpinBox {
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
            unsafe { ffi::QDoubleSpinBox_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`DoubleSpinBox`].
pub struct Builder {
    min: f64,
    max: f64,
    value: f64,
    decimals: i32,
    suffix: Option<String>,
    prefix: Option<String>,
    on_value_changed: Option<Box<dyn Fn(String)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            min: 0.0,
            max: 99.99,
            value: 0.0,
            decimals: 2,
            suffix: None,
            prefix: None,
            on_value_changed: None,
            parent: None,
        }
    }

    /// Set the value range.
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// Set the initial value.
    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    /// Set the number of decimal places.
    pub fn decimals(mut self, decimals: i32) -> Self {
        self.decimals = decimals;
        self
    }

    /// Set the suffix text.
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set the prefix text.
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Called when the value changes.
    ///
    /// The callback receives the new value as a `String`.
    pub fn on_value_changed<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_value_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QDoubleSpinBox` and return the Rust wrapper.
    pub fn build(self) -> DoubleSpinBox {
        let ptr = unsafe {
            ffi::QDoubleSpinBox_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut sb = DoubleSpinBox {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        unsafe {
            ffi::QDoubleSpinBox_setRange(ptr, self.min, self.max);
            ffi::QDoubleSpinBox_setValue(ptr, self.value);
            ffi::QDoubleSpinBox_setDecimals(ptr, self.decimals);
            if let Some(ref s) = self.suffix {
                let_cxx_string!(c_suffix = s);
                ffi::QDoubleSpinBox_setSuffix(ptr, &c_suffix);
            }
            if let Some(ref s) = self.prefix {
                let_cxx_string!(c_prefix = s);
                ffi::QDoubleSpinBox_setPrefix(ptr, &c_prefix);
            }
        }
        if let Some(f) = self.on_value_changed {
            let h = signal::leak_string(f);
            unsafe { ffi::QDoubleSpinBox_onValueChanged(ptr, h.token); }
            sb.signal_handles.push(h);
        }
        sb
    }

    /// Build and immediately show.
    pub fn show(self) -> DoubleSpinBox {
        let sb = self.build();
        unsafe { ffi::QWidget_show(ffi::toQWidget_QDoubleSpinBox(sb.ptr)); }
        sb
    }
}
