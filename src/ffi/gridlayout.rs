unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QGridLayout ---
    unsafe fn QGridLayout_new(parent: *mut QWidget) -> *mut QGridLayout;
    unsafe fn QGridLayout_addWidget(
        layout: *mut QGridLayout,
        widget: *mut QWidget,
        row: i32,
        col: i32,
        rowSpan: i32,
        colSpan: i32,
    );
    unsafe fn QGridLayout_delete(layout: *mut QGridLayout);
}
