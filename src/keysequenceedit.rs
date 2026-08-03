//! Key sequence editor for shortcut configuration.
//! Wraps [`QKeySequenceEdit`](https://doc.qt.io/qt-6/qkeysequenceedit.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

pub struct KeySequenceEdit {
    ptr: *mut ffi::QKeySequenceEdit, has_parent: bool, signal_handles: Vec<SignalHandle>,
}
impl KeySequenceEdit {
    pub fn new() -> Builder { Builder::new() }
    pub fn clear(&self) { unsafe { ffi::QKeySequenceEdit_clear(self.ptr); } }
    pub fn connect_editing_finished<F: Fn()>(&mut self, f: F) {
        let h = signal::leak_void(f);
        unsafe { ffi::QKeySequenceEdit_onEditingFinished(self.ptr, h.token); }
        self.signal_handles.push(h);
    }
}
impl AsWidget for KeySequenceEdit {
    fn widget_ptr(&self) -> *mut ffi::QWidget { unsafe { ffi::toQWidget_QKeySequenceEdit(self.ptr) } }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}
impl Drop for KeySequenceEdit {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QKeySequenceEdit_delete(self.ptr); }
        }
        self.ptr = std::ptr::null_mut();
    }
}
pub struct Builder { on_editing_finished: Option<Box<dyn Fn()>>, parent: Option<*mut ffi::QWidget> }
impl Builder {
    fn new() -> Self { Self { on_editing_finished: None, parent: None } }
    pub fn on_editing_finished<F: Fn() + 'static>(mut self, f: F) -> Self { self.on_editing_finished = Some(Box::new(f)); self }
    pub fn parent(mut self, p: &dyn AsWidget) -> Self { self.parent = Some(p.widget_ptr()); self }
    pub fn build(self) -> KeySequenceEdit {
        let ptr = unsafe { ffi::QKeySequenceEdit_new(self.parent.unwrap_or(std::ptr::null_mut())) };
        assert!(!ptr.is_null());
        let mut e = KeySequenceEdit { ptr, has_parent: self.parent.is_some(), signal_handles: Vec::new() };
        if let Some(f) = self.on_editing_finished {
            let h = signal::leak_void(f);
            unsafe { ffi::QKeySequenceEdit_onEditingFinished(ptr, h.token); }
            e.signal_handles.push(h);
        }
        e
    }
}
