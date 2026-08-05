unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QMenuBar ---
    unsafe fn QMenuBar_new(parent: *mut QWidget) -> *mut QMenuBar;
    unsafe fn QMenuBar_addMenu(mb: *mut QMenuBar, menu: *mut QMenu);
    unsafe fn QMenuBar_delete(mb: *mut QMenuBar);
    }
