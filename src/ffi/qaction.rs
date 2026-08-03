unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
// --- QAction ---
        unsafe fn QAction_new(text: &CxxString, parent: *mut QWidget) -> *mut QAction;
        unsafe fn QAction_delete(action: *mut QAction);
        unsafe fn QAction_setText(action: *mut QAction, text: &CxxString);
        unsafe fn QAction_setIcon(action: *mut QAction, icon_path: &CxxString);
        unsafe fn QAction_setCheckable(action: *mut QAction, checkable: bool);
        unsafe fn QAction_setChecked(action: *mut QAction, checked: bool);
        unsafe fn QAction_setShortcut(action: *mut QAction, key: &CxxString);
        unsafe fn QAction_setEnabled(action: *mut QAction, enabled: bool);
        unsafe fn QAction_setToolTip(action: *mut QAction, tip: &CxxString);
        unsafe fn QAction_text(action: *mut QAction) -> String;
        unsafe fn QAction_isChecked(action: *mut QAction) -> bool;
        unsafe fn QAction_isEnabled(action: *mut QAction) -> bool;
        unsafe fn QAction_onTriggered(action: *mut QAction, ctx: u64);
        unsafe fn QAction_onToggled(action: *mut QAction, ctx: u64);
    }
