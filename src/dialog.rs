//! Dialog support.
//!
//! Provides a builder-pattern API for creating modal dialogs with
//! custom content.

use cxx::let_cxx_string;

use crate::ffi;
use crate::widget::AsWidget;
use crate::layout::VBoxLayout;

/// A custom dialog window.
///
/// Use the builder pattern: call [`Dialog::new`] to get a [`DialogBuilder`],
/// configure with methods like `.title()`, `.modal()`, `.size()`, and
/// finally `.build()` to create the dialog.
///
/// # Example
///
/// ```no_run
/// use qtrs::{Dialog, Label, PushButton, VBoxLayout};
///
/// let dialog = Dialog::new()
///     .title("Configuration")
///     .modal(true)
///     .size(400, 300)
///     .build(|layout| {
///         layout.add(Label::new("Hello, world!").build());
///     });
/// ```
pub struct Dialog {
    ptr: *mut ffi::QDialog,
    has_parent: bool,
}

impl Dialog {
    /// Start building a new dialog.
    pub fn new() -> DialogBuilder {
        DialogBuilder::new()
    }

    /// Get the raw dialog pointer.
    pub fn dialog_ptr(&self) -> *mut ffi::QDialog {
        self.ptr
    }

    /// Show the dialog (non-modal).
    pub fn show(&self) {
        unsafe { ffi::QDialog_show(self.ptr); }
    }

    /// Show the dialog as a modal (blocks until closed).
    pub fn exec(&self) {
        unsafe { ffi::QDialog_exec(self.ptr); }
    }

    /// Close the dialog with accept.
    pub fn accept(&self) {
        unsafe { ffi::QDialog_accept(self.ptr); }
    }

    /// Close the dialog with reject.
    pub fn reject(&self) {
        unsafe { ffi::QDialog_reject(self.ptr); }
    }
}

impl AsWidget for Dialog {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        self.ptr as *mut ffi::QWidget
    }

    fn set_has_parent(&mut self) {
        self.has_parent = true;
    }
}

/// Builder for [`Dialog`].
pub struct DialogBuilder {
    title: Option<String>,
    modal: bool,
    size: Option<(i32, i32)>,
    parent: Option<*mut ffi::QWidget>,
}

impl DialogBuilder {
    fn new() -> Self {
        Self {
            title: None,
            modal: false,
            size: None,
            parent: None,
        }
    }

    /// Set the dialog title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Make the dialog modal (blocks parent window).
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// Set the dialog size.
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.size = Some((width, height));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Build the dialog with a callback that populates the content.
    ///
    /// The callback receives a `&mut VBoxLayout` that you can use to
    /// add widgets to the dialog.
    pub fn build<F>(self, content: F) -> Dialog
    where
        F: FnOnce(&mut VBoxLayout),
    {
        let parent = self.parent.unwrap_or(std::ptr::null_mut());
        let ptr = unsafe { ffi::QDialog_new(parent) };
        assert!(!ptr.is_null(), "QDialog_new returned null");

        if let Some(title) = &self.title {
            let_cxx_string!(c_title = title);
            unsafe { ffi::QDialog_setWindowTitle(ptr, &c_title); }
        }

        if self.modal {
            unsafe { ffi::QDialog_setModal(ptr, true); }
        }

        if let Some((w, h)) = self.size {
            unsafe { ffi::QDialog_setMinimumSize(ptr, w, h); }
            unsafe { ffi::QDialog_resize(ptr, w, h); }
        }

        let dialog = Dialog {
            ptr,
            has_parent: self.parent.is_some(),
        };

        // Create main layout
        let mut layout = VBoxLayout::new();
        unsafe { ffi::QDialog_setLayout(ptr, layout.layout_ptr() as *mut ffi::QLayout); }

        // Call content callback
        content(&mut layout);

        dialog
    }
}

impl Drop for Dialog {
    fn drop(&mut self) {
        if self.ptr.is_null() || self.has_parent {
            return;
        }
        unsafe { ffi::QDialog_delete(self.ptr) };
        self.ptr = std::ptr::null_mut();
    }
}
