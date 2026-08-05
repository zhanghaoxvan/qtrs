unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QToolBar ---
    unsafe fn QToolBar_new(title: &CxxString, parent: *mut QWidget) -> *mut QToolBar;
    unsafe fn QToolBar_delete(toolbar: *mut QToolBar);
    unsafe fn QToolBar_addAction(toolbar: *mut QToolBar, text: &CxxString, ctx: u64);
    unsafe fn QToolBar_addSeparator(toolbar: *mut QToolBar);
    unsafe fn QToolBar_addWidget(toolbar: *mut QToolBar, widget: *mut QWidget);
    unsafe fn QToolBar_setMovable(toolbar: *mut QToolBar, movable: bool);
    unsafe fn QToolBar_setFloatable(toolbar: *mut QToolBar, floatable: bool);
    unsafe fn QToolBar_setIconSize(toolbar: *mut QToolBar, w: i32, h: i32);
    unsafe fn QToolBar_setAllowedAreas(toolbar: *mut QToolBar, areas: i32);
    unsafe fn QToolBar_setToolButtonStyle(toolbar: *mut QToolBar, style: i32);
    unsafe fn QToolBar_clear(toolbar: *mut QToolBar);
    }
