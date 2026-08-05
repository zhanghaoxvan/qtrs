unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QGroupBox ---
    unsafe fn QGroupBox_new(title: &CxxString, parent: *mut QWidget) -> *mut QGroupBox;
    unsafe fn QGroupBox_setTitle(gb: *mut QGroupBox, title: &CxxString);
    unsafe fn QGroupBox_delete(gb: *mut QGroupBox);
    }
