unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QVBoxLayout ---
    unsafe fn QVBoxLayout_new(parent: *mut QWidget) -> *mut QVBoxLayout;
    unsafe fn QVBoxLayout_addWidget(layout: *mut QVBoxLayout, widget: *mut QWidget);
    unsafe fn QVBoxLayout_delete(layout: *mut QVBoxLayout);
    unsafe fn QVBoxLayout_setSpacing(layout: *mut QVBoxLayout, spacing: i32);
    unsafe fn QVBoxLayout_setContentsMargins(
    layout: *mut QVBoxLayout,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    );
    }
