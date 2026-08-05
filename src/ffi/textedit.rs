unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QTextEdit ---
    unsafe fn QTextEdit_new(parent: *mut QWidget) -> *mut QTextEdit;
    unsafe fn QTextEdit_toPlainText(edit: *mut QTextEdit) -> String;
    unsafe fn QTextEdit_setPlainText(edit: *mut QTextEdit, text: &CxxString);
    unsafe fn QTextEdit_setPlaceholderText(edit: *mut QTextEdit, text: &CxxString);
    unsafe fn QTextEdit_setReadOnly(edit: *mut QTextEdit, ro: bool);
    unsafe fn QTextEdit_isReadOnly(edit: *mut QTextEdit) -> bool;
    unsafe fn QTextEdit_append(edit: *mut QTextEdit, text: &CxxString);
    unsafe fn QTextEdit_copy(edit: *mut QTextEdit);
    unsafe fn QTextEdit_cut(edit: *mut QTextEdit);
    unsafe fn QTextEdit_paste(edit: *mut QTextEdit);
    unsafe fn QTextEdit_undo(edit: *mut QTextEdit);
    unsafe fn QTextEdit_redo(edit: *mut QTextEdit);
    unsafe fn QTextEdit_selectAll(edit: *mut QTextEdit);
    unsafe fn QTextEdit_delete(edit: *mut QTextEdit);
    unsafe fn QTextEdit_onTextChanged(edit: *mut QTextEdit, ctx: u64);
    }
