unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QScrollArea ---
    unsafe fn QScrollArea_new(parent: *mut QWidget) -> *mut QScrollArea;
    unsafe fn QScrollArea_delete(w: *mut QScrollArea);
    unsafe fn QScrollArea_setWidget(w: *mut QScrollArea, widget: *mut QWidget);
    unsafe fn QScrollArea_takeWidget(w: *mut QScrollArea) -> *mut QWidget;
    unsafe fn QScrollArea_setWidgetResizable(w: *mut QScrollArea, resizable: bool);
    unsafe fn QScrollArea_setHorizontalScrollBarPolicy(w: *mut QScrollArea, policy: i32);
    unsafe fn QScrollArea_setVerticalScrollBarPolicy(w: *mut QScrollArea, policy: i32);
    unsafe fn QScrollArea_ensureVisible(w: *mut QScrollArea, x: i32, y: i32);
    unsafe fn QScrollArea_ensureWidgetVisible(w: *mut QScrollArea, widget: *mut QWidget);
}
