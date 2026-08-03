//! Text completer for line edits and comboboxes.
//! Wraps [`QCompleter`](https://doc.qt.io/qt-6/qcompleter.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};

pub struct Completer { ptr: *mut ffi::QCompleter, signal_handles: Vec<SignalHandle> }
impl Completer {
    pub fn new(model: &crate::StringListModel) -> Self {
        let ptr = unsafe { ffi::QCompleter_new(model.raw_ptr(), std::ptr::null_mut()) };
        assert!(!ptr.is_null()); Self { ptr, signal_handles: Vec::new() }
    }
    pub fn set_completion_mode(&self, mode: i32) {
        unsafe { ffi::QCompleter_setCompletionMode(self.ptr, mode); }
    }
    pub fn set_case_sensitivity(&self, cs: i32) {
        unsafe { ffi::QCompleter_setCaseSensitivity(self.ptr, cs); }
    }
    pub fn connect_activated<F: Fn(String)>(&mut self, f: F) {
        let h = signal::leak_string(f);
        unsafe { ffi::QCompleter_onActivated(self.ptr, h.token); }
        self.signal_handles.push(h);
    }
}
impl Drop for Completer {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        unsafe { ffi::QCompleter_delete(self.ptr); }
        self.ptr = std::ptr::null_mut();
    }
}
