unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QProgressBar ---
    unsafe fn QProgressBar_new(parent: *mut QWidget) -> *mut QProgressBar;
    unsafe fn QProgressBar_setValue(bar: *mut QProgressBar, value: i32);
    unsafe fn QProgressBar_value(bar: *mut QProgressBar) -> i32;
    unsafe fn QProgressBar_setRange(bar: *mut QProgressBar, min: i32, max: i32);
    unsafe fn QProgressBar_setMinimum(bar: *mut QProgressBar, min: i32);
    unsafe fn QProgressBar_setMaximum(bar: *mut QProgressBar, max: i32);
    unsafe fn QProgressBar_setFormat(bar: *mut QProgressBar, format: &CxxString);
    unsafe fn QProgressBar_delete(bar: *mut QProgressBar);
}
