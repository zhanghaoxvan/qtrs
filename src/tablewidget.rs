//! Table widget for displaying and editing tabular data.
//!
//! Wraps [`QTableWidget`](https://doc.qt.io/qt-6/qtablewidget.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// Selection mode constants matching Qt's `QAbstractItemView::SelectionMode`.
pub const SINGLE_SELECTION: i32 = 0;
pub const MULTI_SELECTION: i32 = 1;
pub const EXTENDED_SELECTION: i32 = 2;
pub const CONTIGUOUS_SELECTION: i32 = 3;
pub const NO_SELECTION: i32 = 4;

/// Selection behavior constants matching Qt's `QAbstractItemView::SelectionBehavior`.
pub const SELECT_ITEMS: i32 = 0;
pub const SELECT_ROWS: i32 = 1;
pub const SELECT_COLUMNS: i32 = 2;

/// A table widget for displaying and editing tabular data.
///
/// Use a **builder pattern**: [`TableWidget::new`] returns a [`Builder`].
///
/// # Signals
///
/// | Method | Qt signal | Note |
/// |---|---|---|
/// | [`Builder::on_cell_clicked`] | `QTableWidget::cellClicked` | Void — use runtime connect for cell args |
/// | [`Builder::on_cell_double_clicked`] | `QTableWidget::cellDoubleClicked` | Void |
/// | [`Builder::on_current_cell_changed`] | `QTableWidget::currentCellChanged` | Void |
///
/// # Example
///
/// ```no_run
/// use qtrs::TableWidget;
///
/// let table = TableWidget::new(3, 2)
///     .on_cell_clicked(|| println!("cell clicked"))
///     .build();
/// table.set_item(0, 0, "Hello");
/// assert_eq!(table.item_text(0, 0), "Hello");
/// ```
pub struct TableWidget {
    ptr: *mut ffi::QTableWidget,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl TableWidget {
    /// Start building a new table widget with the given row and column count.
    pub fn new(rows: i32, cols: i32) -> Builder { Builder::new(rows, cols) }

    // --- Row/Column count ---

    /// Set the number of rows.
    pub fn set_row_count(&self, rows: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_setRowCount(self.ptr, rows); }
    }

    /// Set the number of columns.
    pub fn set_column_count(&self, cols: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_setColumnCount(self.ptr, cols); }
    }

    // --- Item management ---

    /// Set the text of the cell at (row, col).
    pub fn set_item(&self, row: i32, col: i32, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QTableWidget_setItem(self.ptr, row, col, &c_text); }
    }

    /// Get the text of the cell at (row, col). Returns empty string if no item.
    pub fn item_text(&self, row: i32, col: i32) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_itemText(self.ptr, row, col) }
    }

    // --- Header labels ---

    /// Set the horizontal header labels from a slice of strings.
    pub fn set_horizontal_header_labels(&self, labels: &[&str]) {
        debug_assert!(!self.ptr.is_null());
        let vec: Vec<String> = labels.iter().map(|s| s.to_string()).collect();
        unsafe { ffi::QTableWidget_setHorizontalHeaderLabels(self.ptr, vec); }
    }

    /// Set the vertical header labels from a slice of strings.
    pub fn set_vertical_header_labels(&self, labels: &[&str]) {
        debug_assert!(!self.ptr.is_null());
        let vec: Vec<String> = labels.iter().map(|s| s.to_string()).collect();
        unsafe { ffi::QTableWidget_setVerticalHeaderLabels(self.ptr, vec); }
    }

    // --- Current cell ---

    /// Set the current cell.
    pub fn set_current_cell(&self, row: i32, col: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_setCurrentCell(self.ptr, row, col); }
    }

    /// Get the current row index, or -1 if no current item.
    pub fn current_row(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_currentRow(self.ptr) }
    }

    /// Get the current column index, or -1 if no current item.
    pub fn current_column(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_currentColumn(self.ptr) }
    }

    // --- Selected rows ---

    /// Get the list of selected row indices.
    pub fn selected_rows(&self) -> Vec<i32> {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_selectedRows(self.ptr) }
    }

    // --- Clear ---

    /// Clear the entire table (all items, selections, and headers).
    pub fn clear(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_clear(self.ptr); }
    }

    /// Clear the table contents but keep the headers.
    pub fn clear_contents(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_clearContents(self.ptr); }
    }

    // --- Selection ---

    /// Set the selection mode (see `SINGLE_SELECTION`, `MULTI_SELECTION`, etc.).
    pub fn set_selection_mode(&self, mode: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_setSelectionMode(self.ptr, mode); }
    }

    /// Set the selection behavior (see `SELECT_ITEMS`, `SELECT_ROWS`, `SELECT_COLUMNS`).
    pub fn set_selection_behavior(&self, behavior: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_setSelectionBehavior(self.ptr, behavior); }
    }

    // --- Row operations ---

    /// Remove the row at the given index.
    pub fn remove_row(&self, row: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_removeRow(self.ptr, row); }
    }

    /// Insert a new empty row at the given index.
    pub fn insert_row(&self, row: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_insertRow(self.ptr, row); }
    }

    // --- Column/row sizing ---

    /// Set the width of a column in pixels.
    pub fn set_column_width(&self, col: i32, width: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_setColumnWidth(self.ptr, col, width); }
    }

    /// Set the height of a row in pixels.
    pub fn set_row_height(&self, row: i32, height: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTableWidget_setRowHeight(self.ptr, row, height); }
    }

    // --- Signal connections (runtime) ---

    /// Connect a callback that fires when a cell is clicked (void callback).
    pub fn connect_cell_clicked<F: Fn() + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QTableWidget_onCellClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when a cell is double-clicked (void callback).
    pub fn connect_cell_double_clicked<F: Fn() + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QTableWidget_onCellDoubleClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when the current cell changes (void callback).
    pub fn connect_current_cell_changed<F: Fn() + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QTableWidget_onCurrentCellChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QTableWidget) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for TableWidget {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QTableWidget(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for TableWidget {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QTableWidget_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`TableWidget`].
pub struct Builder {
    rows: i32,
    cols: i32,
    on_cell_clicked: Option<Box<dyn Fn()>>,
    on_cell_double_clicked: Option<Box<dyn Fn()>>,
    on_current_cell_changed: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new(rows: i32, cols: i32) -> Self {
        Self {
            rows,
            cols,
            on_cell_clicked: None,
            on_cell_double_clicked: None,
            on_current_cell_changed: None,
            parent: None,
        }
    }

    /// Called when a cell is clicked (void callback).
    pub fn on_cell_clicked<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_cell_clicked = Some(Box::new(f));
        self
    }

    /// Called when a cell is double-clicked (void callback).
    pub fn on_cell_double_clicked<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_cell_double_clicked = Some(Box::new(f));
        self
    }

    /// Called when the current cell changes (void callback).
    pub fn on_current_cell_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_current_cell_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QTableWidget` and return the Rust wrapper.
    pub fn build(self) -> TableWidget {
        let ptr = unsafe {
            ffi::QTableWidget_new(
                self.rows,
                self.cols,
                self.parent.unwrap_or(std::ptr::null_mut()),
            )
        };
        debug_assert!(!ptr.is_null());
        let mut tw = TableWidget {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(f) = self.on_cell_clicked {
            let h = signal::leak_void(f);
            unsafe { ffi::QTableWidget_onCellClicked(ptr, h.token); }
            tw.signal_handles.push(h);
        }
        if let Some(f) = self.on_cell_double_clicked {
            let h = signal::leak_void(f);
            unsafe { ffi::QTableWidget_onCellDoubleClicked(ptr, h.token); }
            tw.signal_handles.push(h);
        }
        if let Some(f) = self.on_current_cell_changed {
            let h = signal::leak_void(f);
            unsafe { ffi::QTableWidget_onCurrentCellChanged(ptr, h.token); }
            tw.signal_handles.push(h);
        }
        tw
    }
}
