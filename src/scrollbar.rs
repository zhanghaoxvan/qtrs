//! Scroll bar widget for scrolling content.
//!
//! Wraps [`QScrollBar`](https://doc.qt.io/qt-6/qscrollbar.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

const HORIZONTAL: i32 = 1;
const VERTICAL: i32 = 2;

/// A vertical or horizontal scroll bar control.
///
/// `ScrollBar` uses a builder pattern: call [`ScrollBar::horizontal`] or
/// [`ScrollBar::vertical`] to obtain a [`Builder`], chain configuration,
/// then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_value_changed`] | `QScrollBar::valueChanged` | `i32` (new value) |
pub struct ScrollBar {
    ptr: *mut ffi::QScrollBar,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl ScrollBar {
    /// Start building a horizontal scroll bar.
    pub fn horizontal() -> Builder {
        Builder::new(HORIZONTAL)
    }

    /// Start building a vertical scroll bar.
    pub fn vertical() -> Builder {
        Builder::new(VERTICAL)
    }

    /// Get the current value.
    pub fn value(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_value(self.ptr) }
    }

    /// Set the current value.
    pub fn set_value(&self, value: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_setValue(self.ptr, value); }
    }

    /// Set the value range.
    pub fn set_range(&self, min: i32, max: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_setRange(self.ptr, min, max); }
    }

    /// Set the single step increment.
    pub fn set_single_step(&self, step: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_setSingleStep(self.ptr, step); }
    }

    /// Set the page step increment (amount moved when clicking the groove).
    pub fn set_page_step(&self, step: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_setPageStep(self.ptr, step); }
    }

    /// Get the minimum value.
    pub fn minimum(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_minimum(self.ptr) }
    }

    /// Get the maximum value.
    pub fn maximum(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_maximum(self.ptr) }
    }

    /// Set the minimum value.
    pub fn set_minimum(&self, min: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_setMinimum(self.ptr, min); }
    }

    /// Set the maximum value.
    pub fn set_maximum(&self, max: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_setMaximum(self.ptr, max); }
    }

    /// Set the orientation (horizontal or vertical).
    pub fn set_orientation(&self, orientation: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_setOrientation(self.ptr, orientation); }
    }

    /// Set whether the scroll bar's appearance is inverted.
    pub fn set_inverted_appearance(&self, inverted: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_setInvertedAppearance(self.ptr, inverted); }
    }

    /// Set whether the scroll bar's controls are inverted.
    pub fn set_inverted_controls(&self, inverted: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_setInvertedControls(self.ptr, inverted); }
    }

    /// Set the slider position (visual, without emitting valueChanged).
    pub fn set_slider_position(&self, pos: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_setSliderPosition(self.ptr, pos); }
    }

    /// Get the current slider position.
    pub fn slider_position(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollBar_sliderPosition(self.ptr) }
    }

    /// Connect a value-changed callback.
    pub fn connect_value_changed<F: Fn(i32)>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_int(f);
        unsafe { ffi::QScrollBar_onValueChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QScrollBar) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for ScrollBar {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QScrollBar(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for ScrollBar {
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
            unsafe { ffi::QScrollBar_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`ScrollBar`].
pub struct Builder {
    orientation: i32,
    min: i32,
    max: i32,
    value: i32,
    on_value_changed: Option<Box<dyn Fn(i32)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new(orientation: i32) -> Self {
        Self {
            orientation,
            min: 0,
            max: 99,
            value: 0,
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

    /// Create the C++ `QScrollBar` and return the Rust wrapper.
    pub fn build(self) -> ScrollBar {
        let ptr = unsafe {
            ffi::QScrollBar_new(self.orientation, self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut sb = ScrollBar {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        unsafe {
            ffi::QScrollBar_setRange(ptr, self.min, self.max);
            ffi::QScrollBar_setValue(ptr, self.value);
        }
        if let Some(f) = self.on_value_changed {
            let h = signal::leak_int(f);
            unsafe { ffi::QScrollBar_onValueChanged(ptr, h.token); }
            sb.signal_handles.push(h);
        }
        sb
    }

    /// Build and immediately show.
    pub fn show(self) -> ScrollBar {
        let sb = self.build();
        unsafe { ffi::QWidget_show(ffi::toQWidget_QScrollBar(sb.ptr)); }
        sb
    }
}
