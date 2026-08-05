unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QApplication ---
    unsafe fn QApplication_new() -> *mut QApplication;
    unsafe fn QApplication_exec(app: *mut QApplication) -> i32;
    unsafe fn QApplication_setWindowIcon(app: *mut QApplication, icon_path: &CxxString);
    unsafe fn QApplication_setDesktopFileName(app: *mut QApplication, name: &CxxString);
}
