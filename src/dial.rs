//! Dial control widget (rotary input).
//!
//! Wraps [`QDial`](https://doc.qt.io/qt-6/qdial.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A rotary dial control for integer value input.
///
/// `Dial` uses a builder pattern: call [`Dial::new`] to obtain a
/// [`Builder`], chain configuration, then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_value_changed`] | `QDial::valueChanged` | `i32` (new value) |
pub struct Dial {
    ptr: *mut ffi::QDial,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl Dial {
    /// Start building a new dial.
    pub fn new() -> Builder {
        Builder::new()
    }

    /// Get the current value.
    pub fn value(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_value(self.ptr) }
    }

    /// Set the current value.
    pub fn set_value(&self, value: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_setValue(self.ptr, value); }
    }

    /// Set the value range.
    pub fn set_range(&self, min: i32, max: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_setRange(self.ptr, min, max); }
    }

    /// Set the single step increment.
    pub fn set_single_step(&self, step: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_setSingleStep(self.ptr, step); }
    }

    /// Set the page step increment.
    pub fn set_page_step(&self, step: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_setPageStep(self.ptr, step); }
    }

    /// Get the minimum value.
    pub fn minimum(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_minimum(self.ptr) }
    }

    /// Get the maximum value.
    pub fn maximum(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_maximum(self.ptr) }
    }

    /// Set the minimum value.
    pub fn set_minimum(&self, min: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_setMinimum(self.ptr, min); }
    }

    /// Set the maximum value.
    pub fn set_maximum(&self, max: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_setMaximum(self.ptr, max); }
    }

    /// Set whether notches are visible.
    pub fn set_notches_visible(&self, visible: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_setNotchesVisible(self.ptr, visible); }
    }

    /// Get whether notches are visible.
    pub fn notches_visible(&self) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_notchesVisible(self.ptr) }
    }

    /// Set whether the dial wraps around.
    pub fn set_wrapping(&self, wrapping: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_setWrapping(self.ptr, wrapping); }
    }

    /// Get whether the dial wraps around.
    pub fn wrapping(&self) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDial_wrapping(self.ptr) }
    }

    /// Connect a value-changed callback.
    pub fn connect_value_changed<F: Fn(i32)>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_int(f);
        unsafe { ffi::QDial_onValueChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QDial) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for Dial {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QDial(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for Dial {
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
            unsafe { ffi::QDial_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`Dial`].
pub struct Builder {
    min: i32,
    max: i32,
    value: i32,
    notches_visible: bool,
    wrapping: bool,
    on_value_changed: Option<Box<dyn Fn(i32)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            min: 0,
            max: 99,
            value: 0,
            notches_visible: false,
            wrapping: false,
            on_value_changed: None,
            parent: None,
        }
    }

    /// Set the value range.
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

    /// Set whether notches are visible.
    pub fn notches_visible(mut self, visible: bool) -> Self {
        self.notches_visible = visible;
        self
    }

    /// Set whether the dial wraps around.
    pub fn wrapping(mut self, wrapping: bool) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Called when the value changes.
    pub fn on_value_changed<F: Fn(i32) + 'static>(mut self, f: F) -> Self {
        self.on_value_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QDial` and return the Rust wrapper.
    pub fn build(self) -> Dial {
        let ptr = unsafe {
            ffi::QDial_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut dial = Dial {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        unsafe {
            ffi::QDial_setRange(ptr, self.min, self.max);
            ffi::QDial_setValue(ptr, self.value);
            ffi::QDial_setNotchesVisible(ptr, self.notches_visible);
            ffi::QDial_setWrapping(ptr, self.wrapping);
        }
        if let Some(f) = self.on_value_changed {
            let h = signal::leak_int(f);
            unsafe { ffi::QDial_onValueChanged(ptr, h.token); }
            dial.signal_handles.push(h);
        }
        dial
    }

    /// Build and immediately show.
    pub fn show(self) -> Dial {
        let dial = self.build();
        unsafe { ffi::QWidget_show(ffi::toQWidget_QDial(dial.ptr)); }
        dial
    }
}
