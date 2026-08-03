unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
// --- QTableWidget ---
        unsafe fn QTableWidget_new(rows: i32, cols: i32, parent: *mut QWidget) -> *mut QTableWidget;
        unsafe fn QTableWidget_delete(w: *mut QTableWidget);
        unsafe fn QTableWidget_setRowCount(w: *mut QTableWidget, rows: i32);
        unsafe fn QTableWidget_setColumnCount(w: *mut QTableWidget, cols: i32);
        unsafe fn QTableWidget_setItem(w: *mut QTableWidget, row: i32, col: i32, text: &CxxString);
        unsafe fn QTableWidget_itemText(w: *mut QTableWidget, row: i32, col: i32) -> String;
        unsafe fn QTableWidget_setHorizontalHeaderLabels(w: *mut QTableWidget, labels: Vec<String>);
        unsafe fn QTableWidget_setVerticalHeaderLabels(w: *mut QTableWidget, labels: Vec<String>);
        unsafe fn QTableWidget_setCurrentCell(w: *mut QTableWidget, row: i32, col: i32);
        unsafe fn QTableWidget_currentRow(w: *mut QTableWidget) -> i32;
        unsafe fn QTableWidget_currentColumn(w: *mut QTableWidget) -> i32;
        unsafe fn QTableWidget_selectedRows(w: *mut QTableWidget) -> Vec<i32>;
        unsafe fn QTableWidget_clear(w: *mut QTableWidget);
        unsafe fn QTableWidget_clearContents(w: *mut QTableWidget);
        unsafe fn QTableWidget_setSelectionMode(w: *mut QTableWidget, mode: i32);
        unsafe fn QTableWidget_setSelectionBehavior(w: *mut QTableWidget, behavior: i32);
        unsafe fn QTableWidget_removeRow(w: *mut QTableWidget, row: i32);
        unsafe fn QTableWidget_insertRow(w: *mut QTableWidget, row: i32);
        unsafe fn QTableWidget_setColumnWidth(w: *mut QTableWidget, col: i32, width: i32);
        unsafe fn QTableWidget_setRowHeight(w: *mut QTableWidget, row: i32, height: i32);
        unsafe fn QTableWidget_onCellClicked(w: *mut QTableWidget, ctx: u64);
        unsafe fn QTableWidget_onCellDoubleClicked(w: *mut QTableWidget, ctx: u64);
        unsafe fn QTableWidget_onCurrentCellChanged(w: *mut QTableWidget, ctx: u64);
    }
