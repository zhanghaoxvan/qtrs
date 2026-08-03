//! File system model for browsing directories.
//! Wraps [`QFileSystemModel`](https://doc.qt.io/qt-6/qfilesystemmodel.html).

use cxx::let_cxx_string;
use crate::ffi;

pub struct FileSystemModel { ptr: *mut ffi::QFileSystemModel }
impl FileSystemModel {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QFileSystemModel_new(std::ptr::null_mut()) };
        assert!(!ptr.is_null()); Self { ptr }
    }
    pub fn set_root_path(&self, path: &str) {
        let_cxx_string!(c = path);
        unsafe { ffi::QFileSystemModel_setRootPath(self.ptr, &c); }
    }
    pub fn root_path(&self) -> String { unsafe { ffi::QFileSystemModel_rootPath(self.ptr) } }
    pub fn file_path(&self, row: i32, col: i32) -> String {
        unsafe { ffi::QFileSystemModel_filePath(self.ptr, row, col) }
    }
    pub fn is_dir(&self, row: i32, col: i32) -> bool {
        unsafe { ffi::QFileSystemModel_isDir(self.ptr, row, col) }
    }
    pub fn raw_ptr(&self) -> *mut ffi::QFileSystemModel { self.ptr }
}
impl Drop for FileSystemModel {
    fn drop(&mut self) {
        if !self.ptr.is_null() { unsafe { ffi::QFileSystemModel_delete(self.ptr); } }
        self.ptr = std::ptr::null_mut();
    }
}
