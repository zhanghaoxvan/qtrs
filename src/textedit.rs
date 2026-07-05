//! Multi-line text editor widget.
//!
//! Wraps [`QTextEdit`](https://doc.qt.io/qt-6/qtextedit.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A multi-line rich-text / plain-text editor.
///
/// Use a **builder pattern**: [`TextEdit::new`] returns a [`Builder`].
///
/// # Signals
///
/// | Method | Qt signal | Note |
/// |---|---|---|
/// | [`Builder::on_text_changed`] | `QTextEdit::textChanged` | Void — call `plain_text()` to read value |
///
/// # Example
///
/// ```no_run
/// let editor = TextEdit::new()
///     .placeholder("Type here...")
///     .on_text_changed(|| println!("text changed"))
///     .build();
/// ```
pub struct TextEdit {
    ptr: *mut ffi::QTextEdit,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl TextEdit {
    pub fn new() -> Builder { Builder::new() }

    /// Get the plain-text content.
    pub fn plain_text(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTextEdit_toPlainText(self.ptr) }
    }

    /// Replace the content with plain text.
    pub fn set_plain_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QTextEdit_setPlainText(self.ptr, &c_text); }
    }
}

impl AsWidget for TextEdit {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QTextEdit(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for TextEdit {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.signal_handles.clear();
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QTextEdit_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

pub struct Builder {
    placeholder: Option<String>,
    on_text_changed: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self { Self { placeholder: None, on_text_changed: None, parent: None } }

    /// Set placeholder text shown when the editor is empty.
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into()); self
    }

    /// Called when the content changes (void — call `plain_text()` to read).
    pub fn on_text_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_text_changed = Some(Box::new(f)); self
    }

    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr()); self
    }

    pub fn build(self) -> TextEdit {
        let ptr = unsafe {
            ffi::QTextEdit_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut te = TextEdit { ptr, has_parent: self.parent.is_some(), signal_handles: Vec::new() };
        if let Some(ref ph) = self.placeholder {
            let_cxx_string!(c_ph = ph);
            unsafe { ffi::QTextEdit_setPlainText(ptr, &c_ph); }
        }
        if let Some(f) = self.on_text_changed {
            let h = signal::leak_void(f);
            unsafe { ffi::QTextEdit_onTextChanged(ptr, h.token); }
            te.signal_handles.push(h);
        }
        te
    }
}
