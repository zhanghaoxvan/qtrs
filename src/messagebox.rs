//! Modal message box dialog.
//!
//! Wraps [`QMessageBox`](https://doc.qt.io/qt-6/qmessagebox.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::SignalHandle;
use crate::widget::AsWidget;

// ============================================================
// Icon enum
// ============================================================

/// Icon constants matching `QMessageBox::Icon`.
pub const NO_ICON: i32 = 0;
pub const INFORMATION: i32 = 1;
pub const WARNING: i32 = 2;
pub const CRITICAL: i32 = 3;
pub const QUESTION: i32 = 4;

// ============================================================
// StandardButton enum constants
// ============================================================

/// Standard button constants matching `QMessageBox::StandardButton`.
pub const OK: i32 = 0x00000400;
pub const CANCEL: i32 = 0x00400000;
pub const YES: i32 = 0x00004000;
pub const NO: i32 = 0x00010000;
pub const CLOSE: i32 = 0x00200000;
pub const SAVE: i32 = 0x00000800;
pub const DISCARD: i32 = 0x00800000;
pub const APPLY: i32 = 0x02000000;
pub const RESET: i32 = 0x04000000;
pub const RESTORE_DEFAULTS: i32 = 0x08000000;
pub const HELP: i32 = 0x01000000;
pub const SAVE_ALL: i32 = 0x00001000;
pub const YES_TO_ALL: i32 = 0x00008000;
pub const NO_TO_ALL: i32 = 0x00020000;
pub const ABORT: i32 = 0x00040000;
pub const RETRY: i32 = 0x00080000;
pub const IGNORE: i32 = 0x00100000;
pub const NO_BUTTON: i32 = 0x00000000;

// ============================================================
// MessageBox
// ============================================================

/// A modal message box for informing the user or asking questions.
///
/// `MessageBox` uses a **builder pattern**: call [`MessageBox::new`] to
/// obtain a [`Builder`], chain configuration, then call `.build()` and
/// `.exec()`.
///
/// # Example
///
/// ```no_run
/// use qtrs::MessageBox;
///
/// let result = MessageBox::new()
///     .icon(qtrs::INFORMATION)
///     .text("File saved.")
///     .window_title("Success")
///     .set_standard_buttons(qtrs::OK)
///     .exec();
/// ```
pub struct MessageBox {
    ptr: *mut ffi::QMessageBox,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl MessageBox {
    /// Start building a new message box.
    pub fn new() -> Builder { Builder::new() }

    /// Set the icon displayed in the message box.
    pub fn set_icon(&self, icon: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMessageBox_setIcon(self.ptr, icon); }
    }

    /// Set the main text.
    pub fn set_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QMessageBox_setText(self.ptr, &c_text); }
    }

    /// Set the informative text (secondary text).
    pub fn set_informative_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QMessageBox_setInformativeText(self.ptr, &c_text); }
    }

    /// Set the window title.
    pub fn set_window_title(&self, title: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_title = title);
        unsafe { ffi::QMessageBox_setWindowTitle(self.ptr, &c_title); }
    }

    /// Set the standard buttons shown.
    pub fn set_standard_buttons(&self, buttons: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMessageBox_setStandardButtons(self.ptr, buttons); }
    }

    /// Set the default button.
    pub fn set_default_button(&self, button: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMessageBox_setDefaultButton(self.ptr, button); }
    }

    /// Set detailed text (shown in a collapsible area).
    pub fn set_detailed_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QMessageBox_setDetailedText(self.ptr, &c_text); }
    }

    /// Show the message box modally. Returns the clicked button's code.
    pub fn exec(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMessageBox_exec(self.ptr) }
    }

    // --- Static convenience ---

    /// Show an **About** dialog.
    pub fn about(parent: Option<&dyn AsWidget>, title: &str, text: &str) {
        let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
        let_cxx_string!(c_title = title);
        let_cxx_string!(c_text = text);
        unsafe { ffi::QMessageBox_about(parent_ptr, &c_title, &c_text); }
    }

    /// Wrap an existing `QMessageBox*` obtained via `findChild`.
    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QMessageBox, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for MessageBox {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QMessageBox(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for MessageBox {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.signal_handles.clear();
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QMessageBox_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`MessageBox`].
pub struct Builder {
    icon: Option<i32>,
    text: Option<String>,
    informative_text: Option<String>,
    window_title: Option<String>,
    standard_buttons: Option<i32>,
    default_button: Option<i32>,
    detailed_text: Option<String>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            icon: None,
            text: None,
            informative_text: None,
            window_title: None,
            standard_buttons: None,
            default_button: None,
            detailed_text: None,
            parent: None,
        }
    }

    /// Set the icon.
    pub fn icon(mut self, icon: i32) -> Self {
        self.icon = Some(icon); self
    }

    /// Set the main text.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into()); self
    }

    /// Set the informative text.
    pub fn informative_text(mut self, text: impl Into<String>) -> Self {
        self.informative_text = Some(text.into()); self
    }

    /// Set the window title.
    pub fn window_title(mut self, title: impl Into<String>) -> Self {
        self.window_title = Some(title.into()); self
    }

    /// Set the standard buttons.
    pub fn standard_buttons(mut self, buttons: i32) -> Self {
        self.standard_buttons = Some(buttons); self
    }

    /// Set the default button.
    pub fn default_button(mut self, button: i32) -> Self {
        self.default_button = Some(button); self
    }

    /// Set detailed text.
    pub fn detailed_text(mut self, text: impl Into<String>) -> Self {
        self.detailed_text = Some(text.into()); self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr()); self
    }

    /// Create the C++ `QMessageBox` and return the Rust wrapper.
    pub fn build(self) -> MessageBox {
        let ptr = unsafe {
            ffi::QMessageBox_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mb = MessageBox {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(icon) = self.icon { mb.set_icon(icon); }
        if let Some(text) = &self.text { mb.set_text(text); }
        if let Some(text) = &self.informative_text { mb.set_informative_text(text); }
        if let Some(title) = &self.window_title { mb.set_window_title(title); }
        if let Some(btns) = self.standard_buttons { mb.set_standard_buttons(btns); }
        if let Some(btn) = self.default_button { mb.set_default_button(btn); }
        if let Some(text) = &self.detailed_text { mb.set_detailed_text(text); }
        mb
    }
}
