//! Standard item model for tree/table/list data storage.
//!
//! Wraps [`QStandardItemModel`](https://doc.qt.io/qt-6/qstandarditemmodel.html),
//! the default concrete model in Qt's Model/View framework.

use cxx::let_cxx_string;
use crate::ffi;
use crate::signal::{self, SignalHandle};

/// Orientation constants for [`set_header_data`](StandardItemModel::set_header_data).
pub const HORIZONTAL: i32 = 1;
pub const VERTICAL: i32 = 2;

/// A generic item model for storing tabular or tree-structured data.
///
/// `StandardItemModel` is a `QObject` (not a `QWidget`), so it does **not**
/// implement [`AsWidget`](crate::AsWidget). It is always deleted on [`Drop`].
///
/// # Signals
///
/// | Method | Qt signal |
/// |---|---|
/// | [`connect_model_reset`](Self::connect_model_reset) | `modelReset` |
/// | [`connect_data_changed`](Self::connect_data_changed) | `dataChanged` |
/// | [`connect_rows_inserted`](Self::connect_rows_inserted) | `rowsInserted` |
/// | [`connect_rows_removed`](Self::connect_rows_removed) | `rowsRemoved` |
///
/// # Example
///
/// ```no_run
/// # use qtrs::prelude::*;
/// let model = StandardItemModel::new()
///     .rows(3)
///     .cols(2)
///     .build();
/// model.set_data(0, 0, "Hello");
/// assert_eq!(model.data(0, 0), "Hello");
/// ```
pub struct StandardItemModel {
    ptr: *mut ffi::QStandardItemModel,
    signal_handles: Vec<SignalHandle>,
}

impl StandardItemModel {
    /// Start building a new model.
    pub fn new() -> Builder { Builder::new() }

    /// Get the number of rows.
    pub fn row_count(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStandardItemModel_rowCount(self.ptr) }
    }

    /// Get the number of columns.
    pub fn column_count(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStandardItemModel_columnCount(self.ptr) }
    }

    /// Set the number of rows.
    pub fn set_row_count(&self, rows: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStandardItemModel_setRowCount(self.ptr, rows); }
    }

    /// Set the number of columns.
    pub fn set_column_count(&self, cols: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStandardItemModel_setColumnCount(self.ptr, cols); }
    }

    /// Set the string data at `(row, col)`.
    pub fn set_data(&self, row: i32, col: i32, value: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_value = value);
        unsafe { ffi::QStandardItemModel_setData(self.ptr, row, col, &c_value); }
    }

    /// Get the string data at `(row, col)`.
    pub fn data(&self, row: i32, col: i32) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStandardItemModel_data(self.ptr, row, col) }
    }

    /// Set header data for a section.
    ///
    /// `orientation` is [`HORIZONTAL`] or [`VERTICAL`].
    pub fn set_header_data(&self, section: i32, orientation: i32, value: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_value = value);
        unsafe {
            ffi::QStandardItemModel_setHeaderData(self.ptr, section, orientation, &c_value);
        }
    }

    /// Get header data for a section.
    pub fn header_data(&self, section: i32, orientation: i32) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStandardItemModel_headerData(self.ptr, section, orientation) }
    }

    /// Insert a row at the given index.
    pub fn insert_row(&self, row: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStandardItemModel_insertRow(self.ptr, row); }
    }

    /// Remove the row at the given index.
    pub fn remove_row(&self, row: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStandardItemModel_removeRow(self.ptr, row); }
    }

    /// Insert a column at the given index.
    pub fn insert_column(&self, column: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStandardItemModel_insertColumn(self.ptr, column); }
    }

    /// Remove the column at the given index.
    pub fn remove_column(&self, column: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStandardItemModel_removeColumn(self.ptr, column); }
    }

    /// Remove all rows and columns from the model.
    pub fn clear(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStandardItemModel_clear(self.ptr); }
    }

    /// Append a row with the given text values (one per column).
    pub fn append_row(&self, texts: &[&str]) {
        debug_assert!(!self.ptr.is_null());
        let vec: Vec<String> = texts.iter().map(|s| s.to_string()).collect();
        unsafe { ffi::QStandardItemModel_appendRow(self.ptr, vec); }
    }

    // --- Signal connections ---

    /// Connect a callback that fires when the model is reset (structure change).
    pub fn connect_model_reset<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QStandardItemModel_onModelReset(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when cell data changes.
    pub fn connect_data_changed<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QStandardItemModel_onDataChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when rows are inserted.
    pub fn connect_rows_inserted<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QStandardItemModel_onRowsInserted(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when rows are removed.
    pub fn connect_rows_removed<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QStandardItemModel_onRowsRemoved(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Return the raw C++ pointer (for use by views).
    #[doc(hidden)]
    pub fn raw_ptr(&self) -> *mut ffi::QStandardItemModel { self.ptr }
}

impl Drop for StandardItemModel {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        for h in self.signal_handles.drain(..) {
            unsafe { h.reclaim(); }
        }
        unsafe { ffi::QStandardItemModel_delete(self.ptr); }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`StandardItemModel`].
pub struct Builder {
    rows: i32,
    cols: i32,
    parent: Option<*mut ffi::QObject>,
}

impl Builder {
    fn new() -> Self {
        Self { rows: 0, cols: 0, parent: None }
    }

    /// Set the initial row count.
    pub fn rows(mut self, rows: i32) -> Self { self.rows = rows; self }

    /// Set the initial column count.
    pub fn cols(mut self, cols: i32) -> Self { self.cols = cols; self }

    /// Set a QObject parent (typically a view that will display this model).
    pub fn parent(mut self, parent: *mut ffi::QObject) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Create the `QStandardItemModel` and return the Rust wrapper.
    pub fn build(self) -> StandardItemModel {
        let ptr = unsafe {
            ffi::QStandardItemModel_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        assert!(!ptr.is_null(), "QStandardItemModel_new returned null");
        let model = StandardItemModel { ptr, signal_handles: Vec::new() };
        if self.rows > 0 { model.set_row_count(self.rows); }
        if self.cols > 0 { model.set_column_count(self.cols); }
        model
    }
}
