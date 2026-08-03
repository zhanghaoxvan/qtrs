//! LCD number display widget.
//!
//! Wraps [`QLCDNumber`](https://doc.qt.io/qt-6/qlcdnumber.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// Mode constants for [`LcdNumber`].
pub const HEX: i32 = 0;
pub const DEC: i32 = 1;
pub const OCT: i32 = 2;
pub const BIN: i32 = 3;

/// Segment style constants.
pub const OUTLINE: i32 = 0;
pub const FILLED: i32 = 1;
pub const FLAT: i32 = 2;

/// An LCD-style number display.
///
/// `LcdNumber` uses a builder pattern: call [`LcdNumber::new`] to obtain
/// a [`Builder`], chain configuration, then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | When |
/// |---|---|---|
/// | [`Builder::on_overflow`] | `QLCDNumber::overflow` | Value exceeds display capacity |
pub struct LcdNumber {
    ptr: *mut ffi::QLCDNumber,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl LcdNumber {
    /// Start building a new LCD number display.
    pub fn new() -> Builder {
        Builder::new()
    }

    /// Display an integer value.
    pub fn display(&self, value: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QLCDNumber_displayInt(self.ptr, value); }
    }

    /// Display a string (e.g. "3.14" or "HEX").
    pub fn display_str(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QLCDNumber_displayStr(self.ptr, &c_text); }
    }

    /// Set the number of digits to display.
    pub fn set_digit_count(&self, num_digits: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QLCDNumber_setDigitCount(self.ptr, num_digits); }
    }

    /// Get the number of digits displayed.
    pub fn digit_count(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QLCDNumber_digitCount(self.ptr) }
    }

    /// Set the display mode (Hex, Dec, Oct, Bin).
    pub fn set_mode(&self, mode: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QLCDNumber_setMode(self.ptr, mode); }
    }

    /// Set the segment style (Outline, Filled, Flat).
    pub fn set_segment_style(&self, style: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QLCDNumber_setSegmentStyle(self.ptr, style); }
    }

    /// Set whether the decimal point is drawn small.
    pub fn set_small_decimal_point(&self, enabled: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QLCDNumber_setSmallDecimalPoint(self.ptr, enabled); }
    }

    /// Check if displaying the given value would overflow.
    pub fn check_overflow(&self, value: i32) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QLCDNumber_checkOverflow(self.ptr, value) }
    }

    /// Connect an overflow callback.
    pub fn connect_overflow<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QLCDNumber_onOverflow(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QLCDNumber) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for LcdNumber {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QLCDNumber(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for LcdNumber {
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
            unsafe { ffi::QLCDNumber_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`LcdNumber`].
pub struct Builder {
    digit_count: Option<i32>,
    mode: Option<i32>,
    segment_style: Option<i32>,
    on_overflow: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            digit_count: None,
            mode: None,
            segment_style: None,
            on_overflow: None,
            parent: None,
        }
    }

    /// Set the number of digits to display.
    pub fn digit_count(mut self, n: i32) -> Self {
        self.digit_count = Some(n);
        self
    }

    /// Set the display mode ([`HEX`], [`DEC`], [`OCT`], [`BIN`]).
    pub fn mode(mut self, mode: i32) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Set the segment style ([`OUTLINE`], [`FILLED`], [`FLAT`]).
    pub fn segment_style(mut self, style: i32) -> Self {
        self.segment_style = Some(style);
        self
    }

    /// Called when the displayed value overflows.
    pub fn on_overflow<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_overflow = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QLCDNumber` and return the Rust wrapper.
    pub fn build(self) -> LcdNumber {
        let ptr = unsafe {
            ffi::QLCDNumber_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut lcd = LcdNumber {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        unsafe {
            if let Some(n) = self.digit_count {
                ffi::QLCDNumber_setDigitCount(ptr, n);
            }
            if let Some(m) = self.mode {
                ffi::QLCDNumber_setMode(ptr, m);
            }
            if let Some(s) = self.segment_style {
                ffi::QLCDNumber_setSegmentStyle(ptr, s);
            }
        }
        if let Some(f) = self.on_overflow {
            let h = signal::leak_void(f);
            unsafe { ffi::QLCDNumber_onOverflow(ptr, h.token); }
            lcd.signal_handles.push(h);
        }
        lcd
    }

    /// Build and immediately show.
    pub fn show(self) -> LcdNumber {
        let lcd = self.build();
        unsafe { ffi::QWidget_show(ffi::toQWidget_QLCDNumber(lcd.ptr)); }
        lcd
    }
}
