unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QCheckBox ---
    unsafe fn QCheckBox_new(text: &CxxString, parent: *mut QWidget) -> *mut QCheckBox;
    unsafe fn QCheckBox_isChecked(cb: *mut QCheckBox) -> bool;
    unsafe fn QCheckBox_setChecked(cb: *mut QCheckBox, checked: bool);
    unsafe fn QCheckBox_setTristate(cb: *mut QCheckBox, tri: bool);
    unsafe fn QCheckBox_isTristate(cb: *mut QCheckBox) -> bool;
    unsafe fn QCheckBox_delete(cb: *mut QCheckBox);
    unsafe fn QCheckBox_onToggled(cb: *mut QCheckBox, ctx: u64);
    }
