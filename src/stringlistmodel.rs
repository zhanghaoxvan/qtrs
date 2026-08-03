//! Simple string list model for use with views and completers.
//! Wraps [`QStringListModel`](https://doc.qt.io/qt-6/qstringlistmodel.html).

use crate::ffi;

pub struct StringListModel { ptr: *mut ffi::QStringListModel }
impl StringListModel {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QStringListModel_new(std::ptr::null_mut()) };
        assert!(!ptr.is_null());
        Self { ptr }
    }
    pub fn set_string_list(&self, list: &[&str]) {
        let v: Vec<String> = list.iter().map(|s| s.to_string()).collect();
        unsafe { ffi::QStringListModel_setStringList(self.ptr, v); }
    }
    pub fn data(&self, row: i32) -> String { unsafe { ffi::QStringListModel_data(self.ptr, row) } }
    pub fn row_count(&self) -> i32 { unsafe { ffi::QStringListModel_rowCount(self.ptr) } }
    pub fn raw_ptr(&self) -> *mut ffi::QStringListModel { self.ptr }
}
impl Drop for StringListModel {
    fn drop(&mut self) {
        if !self.ptr.is_null() { unsafe { ffi::QStringListModel_delete(self.ptr); } }
        self.ptr = std::ptr::null_mut();
    }
}
