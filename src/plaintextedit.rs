//! Plain text editing widget.
//!
//! Wraps [`QPlainTextEdit`](https://doc.qt.io/qt-6/qplaintextedit.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// Line wrap mode constants matching `QPlainTextEdit::LineWrapMode`.
pub const NO_WRAP: i32 = 0;
pub const WIDGET_WIDTH: i32 = 1;

/// A plain text editing widget.
///
/// Use a **builder pattern**: [`PlainTextEdit::new`] returns a [`Builder`].
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_text_changed`] | `QPlainTextEdit::textChanged` | `()` (void) |
/// | [`Builder::on_cursor_position_changed`] | `QPlainTextEdit::cursorPositionChanged` | `()` (void) |
pub struct PlainTextEdit {
    ptr: *mut ffi::QPlainTextEdit,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl PlainTextEdit {
    /// Start building a new plain text editor.
    pub fn new() -> Builder { Builder::new() }

    /// Set the plain text content.
    pub fn set_plain_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QPlainTextEdit_setPlainText(self.ptr, &c_text); }
    }

    /// Get the plain text content.
    pub fn plain_text(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QPlainTextEdit_plainText(self.ptr) }
    }

    /// Set the placeholder text shown when empty.
    pub fn set_placeholder_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QPlainTextEdit_setPlaceholderText(self.ptr, &c_text); }
    }

    /// Set the read-only state.
    pub fn set_read_only(&self, read_only: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QPlainTextEdit_setReadOnly(self.ptr, read_only); }
    }

    /// Set the line wrap mode (see [`NO_WRAP`], [`WIDGET_WIDTH`]).
    pub fn set_line_wrap_mode(&self, mode: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QPlainTextEdit_setLineWrapMode(self.ptr, mode); }
    }

    /// Append text to the end.
    pub fn append_plain_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QPlainTextEdit_appendPlainText(self.ptr, &c_text); }
    }

    /// Clear all text.
    pub fn clear(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QPlainTextEdit_clear(self.ptr); }
    }

    /// Connect a callback that fires when the text changes.
    pub fn connect_text_changed<F: Fn() + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QPlainTextEdit_onTextChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when the cursor position changes.
    pub fn connect_cursor_position_changed<F: Fn() + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QPlainTextEdit_onCursorPositionChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QPlainTextEdit, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for PlainTextEdit {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QPlainTextEdit(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for PlainTextEdit {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QPlainTextEdit_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`PlainTextEdit`].
pub struct Builder {
    text: Option<String>,
    on_text_changed: Option<Box<dyn Fn()>>,
    on_cursor_position_changed: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            text: None,
            on_text_changed: None,
            on_cursor_position_changed: None,
            parent: None,
        }
    }

    /// Set the initial plain text content.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Called when the text changes (void — call `plain_text()` to read).
    pub fn on_text_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_text_changed = Some(Box::new(f));
        self
    }

    /// Called when the cursor position changes.
    pub fn on_cursor_position_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_cursor_position_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QPlainTextEdit` and return the Rust wrapper.
    pub fn build(self) -> PlainTextEdit {
        let ptr = unsafe {
            ffi::QPlainTextEdit_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut pte = PlainTextEdit {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(text) = &self.text {
            let_cxx_string!(c_text = text);
            unsafe { ffi::QPlainTextEdit_setPlainText(ptr, &c_text); }
        }
        if let Some(f) = self.on_text_changed {
            let h = signal::leak_void(f);
            unsafe { ffi::QPlainTextEdit_onTextChanged(ptr, h.token); }
            pte.signal_handles.push(h);
        }
        if let Some(f) = self.on_cursor_position_changed {
            let h = signal::leak_void(f);
            unsafe { ffi::QPlainTextEdit_onCursorPositionChanged(ptr, h.token); }
            pte.signal_handles.push(h);
        }
        pte
    }
}
