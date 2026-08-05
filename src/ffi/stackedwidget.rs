unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QStackedWidget ---
    unsafe fn QStackedWidget_new(parent: *mut QWidget) -> *mut QStackedWidget;
    unsafe fn QStackedWidget_delete(w: *mut QStackedWidget);
    unsafe fn QStackedWidget_addWidget(w: *mut QStackedWidget, page: *mut QWidget) -> i32;
    unsafe fn QStackedWidget_insertWidget(w: *mut QStackedWidget, index: i32, page: *mut QWidget) -> i32;
    unsafe fn QStackedWidget_removeWidget(w: *mut QStackedWidget, page: *mut QWidget);
    unsafe fn QStackedWidget_setCurrentIndex(w: *mut QStackedWidget, index: i32);
    unsafe fn QStackedWidget_currentIndex(w: *mut QStackedWidget) -> i32;
    unsafe fn QStackedWidget_count(w: *mut QStackedWidget) -> i32;
    unsafe fn QStackedWidget_widget(w: *mut QStackedWidget, index: i32) -> *mut QWidget;
    unsafe fn QStackedWidget_onCurrentChanged(w: *mut QStackedWidget, ctx: u64);
}
