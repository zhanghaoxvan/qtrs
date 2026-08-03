//! Undo stack for undo/redo functionality.
//! Wraps [`QUndoStack`](https://doc.qt.io/qt-6/qundostack.html).

use crate::ffi;

pub struct UndoStack { ptr: *mut ffi::QUndoStack }
impl UndoStack {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QUndoStack_new(std::ptr::null_mut()) };
        assert!(!ptr.is_null()); Self { ptr }
    }
    pub fn undo(&self) { unsafe { ffi::QUndoStack_undo(self.ptr); } }
    pub fn redo(&self) { unsafe { ffi::QUndoStack_redo(self.ptr); } }
    pub fn clear(&self) { unsafe { ffi::QUndoStack_clear(self.ptr); } }
    pub fn can_undo(&self) -> bool { unsafe { ffi::QUndoStack_canUndo(self.ptr) } }
    pub fn can_redo(&self) -> bool { unsafe { ffi::QUndoStack_canRedo(self.ptr) } }
    pub fn count(&self) -> i32 { unsafe { ffi::QUndoStack_count(self.ptr) } }
}
impl Drop for UndoStack {
    fn drop(&mut self) {
        if !self.ptr.is_null() { unsafe { ffi::QUndoStack_delete(self.ptr); } }
        self.ptr = std::ptr::null_mut();
    }
}
