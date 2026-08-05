unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QMenu ---
    unsafe fn QMenu_new(title: &CxxString, parent: *mut QWidget) -> *mut QMenu;
    unsafe fn QMenu_addAction(menu: *mut QMenu, text: &CxxString, ctx: u64);
    unsafe fn QMenu_delete(menu: *mut QMenu);
    }
