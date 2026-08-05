unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QMessageBox (object-based) ---
    unsafe fn QMessageBox_new(parent: *mut QWidget) -> *mut QMessageBox;
    unsafe fn QMessageBox_delete(w: *mut QMessageBox);
    unsafe fn QMessageBox_setIcon(w: *mut QMessageBox, icon: i32);
    unsafe fn QMessageBox_setText(w: *mut QMessageBox, text: &CxxString);
    unsafe fn QMessageBox_setInformativeText(w: *mut QMessageBox, text: &CxxString);
    unsafe fn QMessageBox_setWindowTitle(w: *mut QMessageBox, title: &CxxString);
    unsafe fn QMessageBox_setStandardButtons(w: *mut QMessageBox, buttons: i32);
    unsafe fn QMessageBox_setDefaultButton(w: *mut QMessageBox, button: i32);
    unsafe fn QMessageBox_setDetailedText(w: *mut QMessageBox, text: &CxxString);
    unsafe fn QMessageBox_exec(w: *mut QMessageBox) -> i32;
    unsafe fn QMessageBox_about(parent: *mut QWidget, title: &CxxString, text: &CxxString);
}
