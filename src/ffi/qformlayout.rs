unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
// --- QFormLayout ---
        unsafe fn QFormLayout_new(parent: *mut QWidget) -> *mut QFormLayout;
        unsafe fn QFormLayout_addRow(layout: *mut QFormLayout, label: &String, widget: *mut QWidget);
        unsafe fn QFormLayout_addRowWidget(layout: *mut QFormLayout, widget: *mut QWidget);
        unsafe fn QFormLayout_setSpacing(layout: *mut QFormLayout, spacing: i32);
        unsafe fn QFormLayout_setContentsMargins(layout: *mut QFormLayout, left: i32, top: i32, right: i32, bottom: i32);
        unsafe fn QFormLayout_delete(layout: *mut QFormLayout);
    }
