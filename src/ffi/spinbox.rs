unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QSpinBox ---
    unsafe fn QSpinBox_new(parent: *mut QWidget) -> *mut QSpinBox;
    unsafe fn QSpinBox_setValue(sb: *mut QSpinBox, value: i32);
    unsafe fn QSpinBox_value(sb: *mut QSpinBox) -> i32;
    unsafe fn QSpinBox_setRange(sb: *mut QSpinBox, min: i32, max: i32);
    unsafe fn QSpinBox_setSuffix(sb: *mut QSpinBox, suffix: &CxxString);
    unsafe fn QSpinBox_setPrefix(sb: *mut QSpinBox, prefix: &CxxString);
    unsafe fn QSpinBox_delete(sb: *mut QSpinBox);
    unsafe fn QSpinBox_onValueChanged(sb: *mut QSpinBox, ctx: u64);
}
