unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
// --- QMessageBox ---
        unsafe fn QMessageBox_information(
            parent: *mut QWidget,
            title: &CxxString,
            text: &CxxString,
        );
        unsafe fn QMessageBox_warning(
            parent: *mut QWidget,
            title: &CxxString,
            text: &CxxString,
        );
        unsafe fn QMessageBox_critical(
            parent: *mut QWidget,
            title: &CxxString,
            text: &CxxString,
        );
        unsafe fn QMessageBox_question(
            parent: *mut QWidget,
            title: &CxxString,
            text: &CxxString,
        ) -> i32;
    }
