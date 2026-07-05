//! Grid layout for arranging widgets in rows and columns.
//!
//! Wraps [`QGridLayout`](https://doc.qt.io/qt-6/qgridlayout.html).

use crate::ffi;
use crate::layout::AsLayout;
use crate::widget::AsWidget;

/// A grid-based layout.
///
/// Widgets are placed at `(row, col)` cells, optionally spanning
/// multiple rows or columns.
///
/// # Example
///
/// ```ignore
/// let mut grid = GridLayout::new();
/// grid.add_widget(Box::new(Label::new("Name:").build()), 0, 0, 1, 1);
/// grid.add_widget(Box::new(LineEdit::new("").build()), 0, 1, 1, 1);
/// grid.add_widget(Box::new(PushButton::new("OK").build()), 1, 0, 1, 2);
/// window.set_grid(&grid);
/// ```
pub struct GridLayout {
    ptr: *mut ffi::QGridLayout,
    children: Vec<Box<dyn AsWidget>>,
}

impl GridLayout {
    /// Create a new, empty grid layout.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QGridLayout_new(std::ptr::null_mut()) };
        debug_assert!(!ptr.is_null());
        Self { ptr, children: Vec::new() }
    }

    /// Create and attach to a parent widget.
    pub fn with_parent(parent: &dyn AsWidget) -> Self {
        let ptr = unsafe { ffi::QGridLayout_new(parent.widget_ptr()) };
        debug_assert!(!ptr.is_null());
        Self { ptr, children: Vec::new() }
    }

    /// Add a widget at `(row, col)`, optionally spanning cells.
    ///
    /// `row_span` and `col_span` default to 1 (single cell).
    pub fn add_widget(
        &mut self, mut widget: Box<dyn AsWidget>,
        row: i32, col: i32, row_span: i32, col_span: i32,
    ) {
        debug_assert!(!self.ptr.is_null());
        unsafe {
            ffi::QGridLayout_addWidget(self.ptr, widget.widget_ptr(), row, col, row_span, col_span);
        }
        widget.set_has_parent();
        self.children.push(widget);
    }

    /// Get the raw layout pointer for use with [`Widget::set_grid`].
    ///
    /// [`Widget::set_grid`]: crate::Widget::set_grid
    pub fn layout_ptr(&self) -> *mut ffi::QGridLayout { self.ptr }
}

impl AsLayout for GridLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout {
        self.ptr as *mut u8 as *mut ffi::QLayout
    }
}

impl Drop for GridLayout {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        self.children.clear();
        unsafe { ffi::QGridLayout_delete(self.ptr) };
        self.ptr = std::ptr::null_mut();
    }
}
