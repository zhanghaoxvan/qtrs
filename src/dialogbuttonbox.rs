//! Dialog button box widget (OK/Cancel/Apply etc.).
//!
//! Wraps [`QDialogButtonBox`](https://doc.qt.io/qt-6/qdialogbuttonbox.html).

use crate::ffi;
use crate::widget::AsWidget;

/// Standard button constants for [`DialogButtonBox`].
pub const OK_BUTTON: i32 = 0x00000400;
pub const CANCEL_BUTTON: i32 = 0x00400000;
pub const YES_BUTTON: i32 = 0x00004000;
pub const NO_BUTTON: i32 = 0x00010000;
pub const CLOSE_BUTTON: i32 = 0x00200000;
pub const SAVE_BUTTON: i32 = 0x00000800;
pub const DISCARD_BUTTON: i32 = 0x00800000;
pub const APPLY_BUTTON: i32 = 0x02000000;
pub const RESET_BUTTON: i32 = 0x04000000;
pub const HELP_BUTTON: i32 = 0x01000000;
pub const SAVE_ALL_BUTTON: i32 = 0x00001000;
pub const YES_TO_ALL_BUTTON: i32 = 0x00008000;
pub const NO_TO_ALL_BUTTON: i32 = 0x00020000;
pub const ABORT_BUTTON: i32 = 0x00040000;
pub const RETRY_BUTTON: i32 = 0x00080000;
pub const IGNORE_BUTTON: i32 = 0x00100000;
pub const NO_BUTTON_VAL: i32 = 0x00000000;

/// A dialog button box with standard buttons (OK, Cancel, Apply, etc.).
///
/// Use a **builder pattern**: [`DialogButtonBox::new`] returns a [`Builder`].
///
/// # Example
///
/// ```no_run
/// # use qtrs::prelude::*;
/// # use qtrs::dialogbuttonbox::{OK_BUTTON, CANCEL_BUTTON};
/// let bbox = DialogButtonBox::new()
///     .standard_buttons(OK_BUTTON | CANCEL_BUTTON)
///     .build();
/// ```
pub struct DialogButtonBox {
    ptr: *mut ffi::QDialogButtonBox,
    has_parent: bool,
}

impl DialogButtonBox {
    /// Start building a new button box.
    pub fn new() -> Builder { Builder::new() }

    /// Set the standard buttons to display.
    pub fn set_standard_buttons(&self, buttons: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDialogButtonBox_setStandardButtons(self.ptr, buttons); }
    }

    /// Get a pointer to the concrete `QPushButton` for a standard button.
    #[doc(hidden)]
    pub fn button_ptr(&self, button: i32) -> *mut ffi::QPushButton {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDialogButtonBox_button(self.ptr, button) }
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QDialogButtonBox) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true }
    }
}

impl AsWidget for DialogButtonBox {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        self.ptr as *mut ffi::QWidget
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for DialogButtonBox {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if !self.has_parent {
            unsafe { ffi::QDialogButtonBox_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`DialogButtonBox`].
pub struct Builder {
    standard_buttons: Option<i32>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self { standard_buttons: None, parent: None }
    }

    /// Set the standard buttons (bitwise OR of [`OK_BUTTON`], [`CANCEL_BUTTON`], etc.).
    pub fn standard_buttons(mut self, buttons: i32) -> Self {
        self.standard_buttons = Some(buttons);
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the `QDialogButtonBox` and return the Rust wrapper.
    pub fn build(self) -> DialogButtonBox {
        let ptr = unsafe {
            ffi::QDialogButtonBox_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        assert!(!ptr.is_null(), "QDialogButtonBox_new returned null");
        let bbox = DialogButtonBox {
            ptr,
            has_parent: self.parent.is_some(),
        };
        if let Some(buttons) = self.standard_buttons {
            bbox.set_standard_buttons(buttons);
        }
        bbox
    }
}
