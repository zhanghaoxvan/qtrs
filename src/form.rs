//! Form layout for label-field pairs.
//!
//! Wraps [`QFormLayout`](https://doc.qt.io/qt-6/qformlayout.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::widget::AsWidget;
use crate::layout::AsLayout;

/// A form layout arranges label-field pairs in a two-column grid.
///
/// # Example
///
/// ```no_run
/// # use qtrs::{FormLayout, Label, LineEdit};
/// let mut layout = FormLayout::new();
/// layout.add_row("Username:", LineEdit::new("").build());
/// layout.add_row("Password:", LineEdit::new("").build());
/// ```
pub struct FormLayout {
    ptr: *mut ffi::QFormLayout,
    children: Vec<Box<dyn AsWidget>>,
}

impl FormLayout {
    /// Create a new form layout.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QFormLayout_new(std::ptr::null_mut()) };
        assert!(!ptr.is_null(), "QFormLayout_new returned null");
        Self {
            ptr,
            children: Vec::new(),
        }
    }

    /// Create a form layout and attach it to a parent widget.
    pub fn with_parent(parent: &dyn AsWidget) -> Self {
        let ptr = unsafe { ffi::QFormLayout_new(parent.widget_ptr()) };
        assert!(!ptr.is_null(), "QFormLayout_new returned null");
        Self {
            ptr,
            children: Vec::new(),
        }
    }

    /// Add a label-field row to the form.
    ///
    /// The label text is shown in the left column, and the widget
    /// is placed in the right column.
    pub fn add_row<T: AsWidget + 'static>(&mut self, label: impl Into<String>, widget: T) {
        debug_assert!(!self.ptr.is_null(), "FormLayout::add_row on null pointer");
        let_cxx_string!(c_label = label.into());
        unsafe {
            ffi::QFormLayout_addRow(self.ptr, &c_label.to_string(), widget.widget_ptr());
        }
        // Store the widget to keep it alive
        self.children.push(Box::new(widget));
    }

    /// Add a widget that spans the full width of the form.
    pub fn add_widget<T: AsWidget + 'static>(&mut self, widget: T) {
        debug_assert!(!self.ptr.is_null(), "FormLayout::add_widget on null pointer");
        unsafe {
            ffi::QFormLayout_addRowWidget(self.ptr, widget.widget_ptr());
        }
        self.children.push(Box::new(widget));
    }

    /// Set spacing between rows.
    pub fn set_spacing(&self, spacing: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QFormLayout_setSpacing(self.ptr, spacing); }
    }

    /// Set margins around the layout.
    pub fn set_contents_margins(&self, left: i32, top: i32, right: i32, bottom: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QFormLayout_setContentsMargins(self.ptr, left, top, right, bottom); }
    }

    /// Get the raw layout pointer.
    pub fn layout_ptr(&self) -> *mut ffi::QFormLayout {
        self.ptr
    }
}

impl AsLayout for FormLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout {
        self.ptr as *mut u8 as *mut ffi::QLayout
    }
}

impl Drop for FormLayout {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        self.children.clear();
        unsafe { ffi::QFormLayout_delete(self.ptr) };
        self.ptr = std::ptr::null_mut();
    }
}
