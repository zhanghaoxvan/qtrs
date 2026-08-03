//! Header view for table/tree views.
//! Wraps [`QHeaderView`](https://doc.qt.io/qt-6/qheaderview.html).

use crate::ffi;
use crate::widget::AsWidget;

pub struct HeaderView { ptr: *mut ffi::QHeaderView, has_parent: bool }
impl HeaderView {
    pub fn new(orientation: i32) -> Builder { Builder::new(orientation) }
    pub fn set_stretch_last_section(&self, stretch: bool) {
        unsafe { ffi::QHeaderView_setStretchLastSection(self.ptr, stretch); }
    }
    pub fn resize_section(&self, section: i32, size: i32) {
        unsafe { ffi::QHeaderView_resizeSection(self.ptr, section, size); }
    }
    pub fn hide_section(&self, section: i32) {
        unsafe { ffi::QHeaderView_hideSection(self.ptr, section); }
    }
    pub fn show_section(&self, section: i32) {
        unsafe { ffi::QHeaderView_showSection(self.ptr, section); }
    }
    pub fn set_section_resize_mode(&self, mode: i32) {
        unsafe { ffi::QHeaderView_setSectionResizeMode(self.ptr, mode); }
    }
}
impl AsWidget for HeaderView {
    fn widget_ptr(&self) -> *mut ffi::QWidget { unsafe { ffi::toQWidget_QHeaderView(self.ptr) } }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}
impl Drop for HeaderView {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if !self.has_parent { unsafe { ffi::QHeaderView_delete(self.ptr); } }
        self.ptr = std::ptr::null_mut();
    }
}
pub struct Builder { orientation: i32, stretch_last: Option<bool>, parent: Option<*mut ffi::QWidget> }
impl Builder {
    fn new(o: i32) -> Self { Self { orientation: o, stretch_last: None, parent: None } }
    pub fn stretch_last_section(mut self, s: bool) -> Self { self.stretch_last = Some(s); self }
    pub fn parent(mut self, p: &dyn AsWidget) -> Self { self.parent = Some(p.widget_ptr()); self }
    pub fn build(self) -> HeaderView {
        let ptr = unsafe { ffi::QHeaderView_new(self.orientation, self.parent.unwrap_or(std::ptr::null_mut())) };
        assert!(!ptr.is_null());
        let h = HeaderView { ptr, has_parent: self.parent.is_some() };
        if let Some(s) = self.stretch_last { unsafe { ffi::QHeaderView_setStretchLastSection(ptr, s); } }
        h
    }
}
