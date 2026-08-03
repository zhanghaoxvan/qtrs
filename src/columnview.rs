//! Column view for hierarchical data (macOS Finder style).
//! Wraps [`QColumnView`](https://doc.qt.io/qt-6/qcolumnview.html).

use crate::ffi;
use crate::widget::AsWidget;

pub struct ColumnView { ptr: *mut ffi::QColumnView, has_parent: bool }
impl ColumnView {
    pub fn new() -> Builder { Builder::new() }
    pub fn set_model(&self, model: &crate::StandardItemModel) {
        unsafe { ffi::QColumnView_setModel(self.ptr, model.raw_ptr()); }
    }
}
impl AsWidget for ColumnView {
    fn widget_ptr(&self) -> *mut ffi::QWidget { unsafe { ffi::toQWidget_QColumnView(self.ptr) } }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}
impl Drop for ColumnView {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if !self.has_parent { unsafe { ffi::QColumnView_delete(self.ptr); } }
        self.ptr = std::ptr::null_mut();
    }
}
pub struct Builder { parent: Option<*mut ffi::QWidget> }
impl Builder {
    fn new() -> Self { Self { parent: None } }
    pub fn parent(mut self, p: &dyn AsWidget) -> Self { self.parent = Some(p.widget_ptr()); self }
    pub fn build(self) -> ColumnView {
        let ptr = unsafe { ffi::QColumnView_new(self.parent.unwrap_or(std::ptr::null_mut())) };
        assert!(!ptr.is_null());
        ColumnView { ptr, has_parent: self.parent.is_some() }
    }
}
