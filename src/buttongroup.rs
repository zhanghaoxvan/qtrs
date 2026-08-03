//! Button group for logical radio-button grouping.
//! Wraps [`QButtonGroup`](https://doc.qt.io/qt-6/qbuttongroup.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};

pub struct ButtonGroup { ptr: *mut ffi::QButtonGroup, signal_handles: Vec<SignalHandle> }
impl ButtonGroup {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QButtonGroup_new(std::ptr::null_mut()) };
        assert!(!ptr.is_null());
        Self { ptr, signal_handles: Vec::new() }
    }
    pub fn add_button(&self, btn: *mut ffi::QAbstractButton, id: i32) {
        unsafe { ffi::QButtonGroup_addButton(self.ptr, btn, id); }
    }
    pub fn set_exclusive(&self, exclusive: bool) {
        unsafe { ffi::QButtonGroup_setExclusive(self.ptr, exclusive); }
    }
    pub fn connect_button_clicked<F: Fn(i32)>(&mut self, f: F) {
        let h = signal::leak_int(f);
        unsafe { ffi::QButtonGroup_onButtonClicked(self.ptr, h.token); }
        self.signal_handles.push(h);
    }
}
impl Drop for ButtonGroup {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        unsafe { ffi::QButtonGroup_delete(self.ptr); }
        self.ptr = std::ptr::null_mut();
    }
}
