//! Sort/filter proxy model for filtering and sorting source models.
//! Wraps [`QSortFilterProxyModel`](https://doc.qt.io/qt-6/qsortfilterproxymodel.html).

use cxx::let_cxx_string;
use crate::ffi;

pub struct SortFilterProxyModel { ptr: *mut ffi::QSortFilterProxyModel }
impl SortFilterProxyModel {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QSortFilterProxyModel_new(std::ptr::null_mut()) };
        assert!(!ptr.is_null()); Self { ptr }
    }
    pub fn set_source_model(&self, src: &crate::StandardItemModel) {
        unsafe { ffi::QSortFilterProxyModel_setSourceModel(self.ptr, src.raw_ptr()); }
    }
    pub fn set_filter_fixed_string(&self, text: &str) {
        let_cxx_string!(c = text);
        unsafe { ffi::QSortFilterProxyModel_setFilterFixedString(self.ptr, &c); }
    }
    pub fn set_filter_case_sensitivity(&self, cs: i32) {
        unsafe { ffi::QSortFilterProxyModel_setFilterCaseSensitivity(self.ptr, cs); }
    }
    pub fn sort(&self, col: i32, order: i32) {
        unsafe { ffi::QSortFilterProxyModel_sort(self.ptr, col, order); }
    }
    pub fn raw_ptr(&self) -> *mut ffi::QSortFilterProxyModel { self.ptr }
}
impl Drop for SortFilterProxyModel {
    fn drop(&mut self) {
        if !self.ptr.is_null() { unsafe { ffi::QSortFilterProxyModel_delete(self.ptr); } }
        self.ptr = std::ptr::null_mut();
    }
}
