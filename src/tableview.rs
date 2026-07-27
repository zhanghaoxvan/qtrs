//! Table view for model-based tabular data display.
//!
//! Wraps [`QTableView`](https://doc.qt.io/qt-6/qtableview.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A table view that displays data from a [`StandardItemModel`](crate::StandardItemModel).
///
/// Unlike [`TableWidget`](crate::TableWidget) (which manages its own items),
/// `TableView` requires an external model. This gives you full control over
/// data storage, sorting, and filtering.
///
/// # Signals
///
/// | Method | Qt signal |
/// |---|---|
/// | [`Builder::on_clicked`] / [`connect_clicked`](Self::connect_clicked) | `clicked` |
/// | [`Builder::on_double_clicked`] / [`connect_double_clicked`](Self::connect_double_clicked) | `doubleClicked` |
///
/// # Example
///
/// ```no_run
/// # use qtrs::prelude::*;
/// let model = StandardItemModel::new()
///     .rows(5)
///     .cols(3)
///     .build();
///
/// let table = TableView::new()
///     .model(&model)
///     .build();
/// ```
pub struct TableView {
    ptr: *mut ffi::QTableView,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl TableView {
    /// Start building a new table view.
    pub fn new() -> Builder { Builder::new() }

    /// Set the model for this view.
    ///
    /// The model must outlive the view — Qt does not take ownership.
    pub fn set_model(&self, model: &crate::StandardItemModel) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableView_setModel(self.ptr, model.raw_ptr()); }
    }

    /// Get a pointer to the current model, or null.
    #[doc(hidden)]
    pub fn model_ptr(&self) -> *mut ffi::QStandardItemModel {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableView_model(self.ptr) }
    }

    /// Set the selection mode (see `SINGLE_SELECTION`, `MULTI_SELECTION`, etc.).
    pub fn set_selection_mode(&self, mode: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableView_setSelectionMode(self.ptr, mode); }
    }

    /// Set the selection behavior (see `SELECT_ITEMS`, `SELECT_ROWS`, `SELECT_COLUMNS`).
    pub fn set_selection_behavior(&self, behavior: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableView_setSelectionBehavior(self.ptr, behavior); }
    }

    /// Show or hide the grid lines between cells.
    pub fn set_show_grid(&self, show: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableView_setShowGrid(self.ptr, show); }
    }

    /// Enable alternating row colors for readability.
    pub fn set_alternating_row_colors(&self, enable: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableView_setAlternatingRowColors(self.ptr, enable); }
    }

    /// Enable click-to-sort on the header.
    pub fn set_sorting_enabled(&self, enabled: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableView_setSortingEnabled(self.ptr, enabled); }
    }

    /// Resize all columns to fit their content.
    pub fn resize_columns_to_contents(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableView_resizeColumnsToContents(self.ptr); }
    }

    /// Resize all rows to fit their content.
    pub fn resize_rows_to_contents(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableView_resizeRowsToContents(self.ptr); }
    }

    /// Select a single row and its model index.
    pub fn select_row(&self, row: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableView_selectRow(self.ptr, row); }
    }

    /// Clear all selections.
    pub fn clear_selection(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableView_clearSelection(self.ptr); }
    }

    // --- Signal connections ---

    /// Connect a callback that fires when a cell is clicked.
    pub fn connect_clicked<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QTableView_onClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when a cell is double-clicked.
    pub fn connect_double_clicked<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QTableView_onDoubleClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QTableView) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for TableView {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QTableView(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for TableView {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QTableView_delete(self.ptr); }
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`TableView`].
pub struct Builder {
    model: Option<*mut ffi::QStandardItemModel>,
    on_clicked: Option<Box<dyn Fn()>>,
    on_double_clicked: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            model: None,
            on_clicked: None,
            on_double_clicked: None,
            parent: None,
        }
    }

    /// Set the data model.
    pub fn model(mut self, model: &crate::StandardItemModel) -> Self {
        self.model = Some(model.raw_ptr());
        self
    }

    /// Called when a cell is clicked.
    pub fn on_clicked<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_clicked = Some(Box::new(f));
        self
    }

    /// Called when a cell is double-clicked.
    pub fn on_double_clicked<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_double_clicked = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the `QTableView` and return the Rust wrapper.
    pub fn build(self) -> TableView {
        let ptr = unsafe {
            ffi::QTableView_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        assert!(!ptr.is_null(), "QTableView_new returned null");
        let mut view = TableView {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(model_ptr) = self.model {
            unsafe { ffi::QTableView_setModel(ptr, model_ptr); }
        }
        if let Some(f) = self.on_clicked {
            let h = signal::leak_void(f);
            unsafe { ffi::QTableView_onClicked(ptr, h.token); }
            view.signal_handles.push(h);
        }
        if let Some(f) = self.on_double_clicked {
            let h = signal::leak_void(f);
            unsafe { ffi::QTableView_onDoubleClicked(ptr, h.token); }
            view.signal_handles.push(h);
        }
        view
    }
}
