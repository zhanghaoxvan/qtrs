//! Slider widget for selecting a value from a range.
//!
//! Wraps [`QSlider`](https://doc.qt.io/qt-6/qslider.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// Horizontal or vertical slider.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_value_changed`] | `QSlider::valueChanged` | `i32` (current value) |
///
/// # Example
///
/// ```no_run
/// let slider = Slider::horizontal()
///     .range(0, 100)
///     .on_value_changed(|v| println!("value: {}", v))
///     .build();
/// ```
pub struct Slider {
    ptr: *mut ffi::QSlider,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

/// Orientation constants matching Qt's `Qt::Orientation`.
pub const HORIZONTAL: i32 = 1;
pub const VERTICAL: i32 = 2;

impl Slider {
    /// Start building a horizontal slider.
    pub fn horizontal() -> Builder { Builder::new(HORIZONTAL) }
    /// Start building a vertical slider.
    pub fn vertical() -> Builder { Builder::new(VERTICAL) }

    pub fn value(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSlider_value(self.ptr) }
    }

    pub fn set_value(&self, value: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSlider_setValue(self.ptr, value); }
    }

    pub fn set_range(&self, min: i32, max: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSlider_setRange(self.ptr, min, max); }
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QSlider, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for Slider {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QSlider(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for Slider {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QSlider_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

pub struct Builder {
    orientation: i32,
    min: i32,
    max: i32,
    on_value_changed: Option<Box<dyn Fn(i32)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new(orientation: i32) -> Self {
        Self { orientation, min: 0, max: 100, on_value_changed: None, parent: None }
    }

    /// Set the value range (default 0–100).
    pub fn range(mut self, min: i32, max: i32) -> Self { self.min = min; self.max = max; self }

    /// Callback receives the current value whenever it changes.
    pub fn on_value_changed<F: Fn(i32) + 'static>(mut self, f: F) -> Self {
        self.on_value_changed = Some(Box::new(f)); self
    }

    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr()); self
    }

    pub fn build(self) -> Slider {
        let ptr = unsafe {
            ffi::QSlider_new(self.orientation, self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut s = Slider { ptr, has_parent: self.parent.is_some(), signal_handles: Vec::new() };
        unsafe { ffi::QSlider_setRange(ptr, self.min, self.max); }
        if let Some(f) = self.on_value_changed {
            let h = signal::leak_int(f);
            unsafe { ffi::QSlider_onValueChanged(ptr, h.token); }
            s.signal_handles.push(h);
        }
        s
    }
}
