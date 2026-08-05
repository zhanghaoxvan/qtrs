unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QComboBox ---
    unsafe fn QComboBox_new(parent: *mut QWidget) -> *mut QComboBox;
    unsafe fn QComboBox_addItem(cb: *mut QComboBox, text: &CxxString);
    unsafe fn QComboBox_currentText(cb: *mut QComboBox) -> String;
    unsafe fn QComboBox_setCurrentIndex(cb: *mut QComboBox, index: i32);
    unsafe fn QComboBox_count(cb: *mut QComboBox) -> i32;
    unsafe fn QComboBox_removeItem(cb: *mut QComboBox, index: i32);
    unsafe fn QComboBox_clear(cb: *mut QComboBox);
    unsafe fn QComboBox_setEditable(cb: *mut QComboBox, edit: bool);
    unsafe fn QComboBox_isEditable(cb: *mut QComboBox) -> bool;
    unsafe fn QComboBox_setMaxCount(cb: *mut QComboBox, max: i32);
    unsafe fn QComboBox_delete(cb: *mut QComboBox);
    unsafe fn QComboBox_onCurrentTextChanged(cb: *mut QComboBox, ctx: u64);
    unsafe fn QComboBox_onCurrentIndexChanged(cb: *mut QComboBox, ctx: u64);
}
