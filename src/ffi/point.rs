unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QPoint ---
    unsafe fn QPoint_new(x: i32, y: i32) -> *mut QPoint;
    unsafe fn QPoint_delete(point: *mut QPoint);

    // ============================================================
    // ListWidget
    // ============================================================
        

    unsafe fn QListWidget_new(parent: *mut QWidget) -> *mut QListWidget;
    unsafe fn QListWidget_delete(w: *mut QListWidget);
    unsafe fn QListWidget_addItem(w: *mut QListWidget, text: &CxxString);
    unsafe fn QListWidget_addItems(w: *mut QListWidget, items: &CxxVector<CxxString>);
    unsafe fn QListWidget_insertItem(w: *mut QListWidget, row: i32, text: &CxxString);
    unsafe fn QListWidget_clear(w: *mut QListWidget);
    unsafe fn QListWidget_removeItem(w: *mut QListWidget, row: i32);
    unsafe fn QListWidget_count(w: *mut QListWidget) -> i32;
    unsafe fn QListWidget_itemText(w: *mut QListWidget, row: i32) -> String;
    unsafe fn QListWidget_currentRow(w: *mut QListWidget) -> i32;
    unsafe fn QListWidget_currentText(w: *mut QListWidget) -> String;
    unsafe fn QListWidget_setCurrentRow(w: *mut QListWidget, row: i32);
    unsafe fn QListWidget_setSelectionMode(w: *mut QListWidget, mode: i32);
    unsafe fn QListWidget_onItemClicked(w: *mut QListWidget, ctx: u64);
    unsafe fn QListWidget_onItemDoubleClicked(w: *mut QListWidget, ctx: u64);
    unsafe fn QListWidget_onCurrentItemChanged(w: *mut QListWidget, ctx: u64);
    unsafe fn toQWidget_QListWidget(w: *mut QListWidget) -> *mut QWidget;
    unsafe fn toQWidget_QMainWindow(w: *mut QMainWindow) -> *mut QWidget;
    unsafe fn toQWidget_QToolBar(toolbar: *mut QToolBar) -> *mut QWidget;
    unsafe fn toQWidget_QStatusBar(bar: *mut QStatusBar) -> *mut QWidget;
    unsafe fn toQWidget_QMessageBox(w: *mut QMessageBox) -> *mut QWidget;
    unsafe fn toQWidget_QProgressDialog(w: *mut QProgressDialog) -> *mut QWidget;
    unsafe fn toQWidget_QTableWidget(w: *mut QTableWidget) -> *mut QWidget;
    unsafe fn toQWidget_QTreeWidget(w: *mut QTreeWidget) -> *mut QWidget;
    unsafe fn toQWidget_QScrollArea(w: *mut QScrollArea) -> *mut QWidget;
    unsafe fn toQWidget_QStackedWidget(w: *mut QStackedWidget) -> *mut QWidget;
    unsafe fn toQWidget_QSplitter(w: *mut QSplitter) -> *mut QWidget;
    unsafe fn toQWidget_QDateEdit(w: *mut QDateEdit) -> *mut QWidget;
    unsafe fn toQWidget_QTimeEdit(w: *mut QTimeEdit) -> *mut QWidget;
    unsafe fn toQWidget_QDateTimeEdit(w: *mut QDateTimeEdit) -> *mut QWidget;
    unsafe fn toQWidget_QPlainTextEdit(w: *mut QPlainTextEdit) -> *mut QWidget;
    unsafe fn toQWidget_QTextBrowser(w: *mut QTextBrowser) -> *mut QWidget;
    unsafe fn toQWidget_QFrame(w: *mut QFrame) -> *mut QWidget;
    unsafe fn toQWidget_QToolButton(btn: *mut QToolButton) -> *mut QWidget;
    unsafe fn toQWidget_QCalendarWidget(cal: *mut QCalendarWidget) -> *mut QWidget;
    }
