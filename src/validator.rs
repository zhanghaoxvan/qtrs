//! Input validators for line edits.
//! Wraps [`QIntValidator`](https://doc.qt.io/qt-6/qintvalidator.html)
//! and [`QDoubleValidator`](https://doc.qt.io/qt-6/qdoublevalidator.html).

use crate::ffi;

pub struct IntValidator { ptr: *mut ffi::QIntValidator }
impl IntValidator {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QIntValidator_new(std::ptr::null_mut()) };
        assert!(!ptr.is_null()); Self { ptr }
    }
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { ffi::QIntValidator_setRange(self.ptr, min, max); }
    }
    pub fn set_bottom(&self, bottom: i32) { unsafe { ffi::QIntValidator_setBottom(self.ptr, bottom); } }
    pub fn set_top(&self, top: i32) { unsafe { ffi::QIntValidator_setTop(self.ptr, top); } }
    pub fn raw_ptr(&self) -> *mut ffi::QIntValidator { self.ptr }
}
impl Drop for IntValidator {
    fn drop(&mut self) {
        if !self.ptr.is_null() { unsafe { ffi::QIntValidator_delete(self.ptr); } }
        self.ptr = std::ptr::null_mut();
    }
}

pub struct DoubleValidator { ptr: *mut ffi::QDoubleValidator }
impl DoubleValidator {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QDoubleValidator_new(std::ptr::null_mut()) };
        assert!(!ptr.is_null()); Self { ptr }
    }
    pub fn set_range(&self, min: f64, max: f64, decimals: i32) {
        unsafe { ffi::QDoubleValidator_setRange(self.ptr, min, max, decimals); }
    }
    pub fn raw_ptr(&self) -> *mut ffi::QDoubleValidator { self.ptr }
}
impl Drop for DoubleValidator {
    fn drop(&mut self) {
        if !self.ptr.is_null() { unsafe { ffi::QDoubleValidator_delete(self.ptr); } }
        self.ptr = std::ptr::null_mut();
    }
}
