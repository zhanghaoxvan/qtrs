unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
// --- QPushButton ---
        unsafe fn QPushButton_new(text: &CxxString, parent: *mut QWidget) -> *mut QPushButton;
        unsafe fn QPushButton_show(btn: *mut QPushButton);
        unsafe fn QPushButton_setText(btn: *mut QPushButton, text: &CxxString);
        unsafe fn QPushButton_delete(btn: *mut QPushButton);
        unsafe fn QPushButton_setIcon(btn: *mut QPushButton, path: &CxxString);
        unsafe fn QPushButton_setFlat(btn: *mut QPushButton, flat: bool);
        unsafe fn QPushButton_isFlat(btn: *mut QPushButton) -> bool;
        unsafe fn QPushButton_setDefault(btn: *mut QPushButton, def: bool);
        unsafe fn QPushButton_setAutoDefault(btn: *mut QPushButton, def: bool);
        unsafe fn QPushButton_onClicked(btn: *mut QPushButton, ctx: u64);
    }
