//! Font selection combo box.
//! Wraps [`QFontComboBox`](https://doc.qt.io/qt-6/qfontcombobox.html).

use cxx::let_cxx_string;
use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

pub struct FontComboBox {
    ptr: *mut ffi::QFontComboBox, has_parent: bool, signal_handles: Vec<SignalHandle>,
}
impl FontComboBox {
    pub fn new() -> Builder { Builder::new() }
    pub fn set_current_font(&self, family: &str) {
        let_cxx_string!(c = family);
        unsafe { ffi::QFontComboBox_setCurrentFont(self.ptr, &c); }
    }
    pub fn current_font(&self) -> String {
        unsafe { ffi::QFontComboBox_currentFont(self.ptr) }
    }
    pub fn set_font_filters(&self, filters: i32) {
        unsafe { ffi::QFontComboBox_setFontFilters(self.ptr, filters); }
    }
    pub fn connect_current_font_changed<F: Fn()>(&mut self, f: F) {
        let h = signal::leak_void(f);
        unsafe { ffi::QFontComboBox_onCurrentFontChanged(self.ptr, h.token); }
        self.signal_handles.push(h);
    }
}
impl AsWidget for FontComboBox {
    fn widget_ptr(&self) -> *mut ffi::QWidget { unsafe { ffi::toQWidget_QFontComboBox(self.ptr) } }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}
impl Drop for FontComboBox {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QFontComboBox_delete(self.ptr); }
        }
        self.ptr = std::ptr::null_mut();
    }
}
pub struct Builder {
    on_current_font_changed: Option<Box<dyn Fn()>>, parent: Option<*mut ffi::QWidget>,
}
impl Builder {
    fn new() -> Self { Self { on_current_font_changed: None, parent: None } }
    pub fn on_current_font_changed<F: Fn() + 'static>(mut self, f: F) -> Self { self.on_current_font_changed = Some(Box::new(f)); self }
    pub fn parent(mut self, p: &dyn AsWidget) -> Self { self.parent = Some(p.widget_ptr()); self }
    pub fn build(self) -> FontComboBox {
        let ptr = unsafe { ffi::QFontComboBox_new(self.parent.unwrap_or(std::ptr::null_mut())) };
        assert!(!ptr.is_null());
        let mut cb = FontComboBox { ptr, has_parent: self.parent.is_some(), signal_handles: Vec::new() };
        if let Some(f) = self.on_current_font_changed {
            let h = signal::leak_void(f);
            unsafe { ffi::QFontComboBox_onCurrentFontChanged(ptr, h.token); }
            cb.signal_handles.push(h);
        }
        cb
    }
}
