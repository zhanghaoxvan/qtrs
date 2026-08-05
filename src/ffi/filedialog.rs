unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QFileDialog ---
    unsafe fn QFileDialog_getOpenFileName(
        parent: *mut QWidget,
        caption: &CxxString,
        dir: &CxxString,
        filter: &CxxString,
    ) -> String;

    unsafe fn QFileDialog_getOpenFileNames(
        parent: *mut QWidget,
        caption: &CxxString,
        dir: &CxxString,
        filter: &CxxString,
    ) -> Vec<String>;

    unsafe fn QFileDialog_getSaveFileName(
        parent: *mut QWidget,
        caption: &CxxString,
        dir: &CxxString,
        filter: &CxxString,
    ) -> String;

    unsafe fn QFileDialog_getExistingDirectory(
        parent: *mut QWidget,
        caption: &CxxString,
        dir: &CxxString,
    ) -> String;
}
