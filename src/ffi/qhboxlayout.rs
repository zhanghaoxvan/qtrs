unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
// --- QHBoxLayout ---
        unsafe fn QHBoxLayout_new(parent: *mut QWidget) -> *mut QHBoxLayout;
        unsafe fn QHBoxLayout_addWidget(layout: *mut QHBoxLayout, widget: *mut QWidget);
        unsafe fn QHBoxLayout_delete(layout: *mut QHBoxLayout);
        unsafe fn QHBoxLayout_setSpacing(layout: *mut QHBoxLayout, spacing: i32);
        unsafe fn QHBoxLayout_setContentsMargins(
            layout: *mut QHBoxLayout,
            left: i32,
            top: i32,
            right: i32,
            bottom: i32,
        );
    }
