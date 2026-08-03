unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
// --- QRadioButton ---
        unsafe fn QRadioButton_new(text: &CxxString, parent: *mut QWidget) -> *mut QRadioButton;
        unsafe fn QRadioButton_isChecked(rb: *mut QRadioButton) -> bool;
        unsafe fn QRadioButton_setChecked(rb: *mut QRadioButton, checked: bool);
        unsafe fn QRadioButton_delete(rb: *mut QRadioButton);
        unsafe fn QRadioButton_onToggled(rb: *mut QRadioButton, ctx: u64);
    }
