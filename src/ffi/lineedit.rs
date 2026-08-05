unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QLineEdit ---
    unsafe fn QLineEdit_new(text: &CxxString, parent: *mut QWidget) -> *mut QLineEdit;
    unsafe fn QLineEdit_text(edit: *mut QLineEdit) -> String;
    unsafe fn QLineEdit_setText(edit: *mut QLineEdit, text: &CxxString);
    unsafe fn QLineEdit_delete(edit: *mut QLineEdit);
    unsafe fn QLineEdit_clear(edit: *mut QLineEdit);
    unsafe fn QLineEdit_selectAll(edit: *mut QLineEdit);
    unsafe fn QLineEdit_copy(edit: *mut QLineEdit);
    unsafe fn QLineEdit_cut(edit: *mut QLineEdit);
    unsafe fn QLineEdit_paste(edit: *mut QLineEdit);
    unsafe fn QLineEdit_undo(edit: *mut QLineEdit);
    unsafe fn QLineEdit_redo(edit: *mut QLineEdit);
    unsafe fn QLineEdit_setReadOnly(edit: *mut QLineEdit, ro: bool);
    unsafe fn QLineEdit_isReadOnly(edit: *mut QLineEdit) -> bool;
    unsafe fn QLineEdit_setEchoMode(edit: *mut QLineEdit, mode: i32);
    unsafe fn QLineEdit_setMaxLength(edit: *mut QLineEdit, len: i32);
    unsafe fn QLineEdit_maxLength(edit: *mut QLineEdit) -> i32;
    unsafe fn QLineEdit_cursorPosition(edit: *mut QLineEdit) -> i32;
    unsafe fn QLineEdit_setCursorPosition(edit: *mut QLineEdit, pos: i32);
    unsafe fn QLineEdit_onReturnPressed(edit: *mut QLineEdit, ctx: u64);
    }
