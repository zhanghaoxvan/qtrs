unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QMainWindow ---
    unsafe fn QMainWindow_new(parent: *mut QWidget) -> *mut QMainWindow;
    unsafe fn QMainWindow_delete(w: *mut QMainWindow);
    unsafe fn QMainWindow_setMenuBar(w: *mut QMainWindow, menuBar: *mut QMenuBar);
    unsafe fn QMainWindow_setCentralWidget(w: *mut QMainWindow, central: *mut QWidget);
    unsafe fn QMainWindow_setStatusBar(w: *mut QMainWindow, statusBar: *mut QStatusBar);
    unsafe fn QMainWindow_addToolBar(w: *mut QMainWindow, title: &CxxString) -> *mut QToolBar;
    unsafe fn QMainWindow_addToolBarBreak(w: *mut QMainWindow);
    unsafe fn QMainWindow_setWindowTitle(w: *mut QMainWindow, title: &CxxString);
    unsafe fn QMainWindow_resize(w: *mut QMainWindow, width: i32, height: i32);
    unsafe fn QMainWindow_show(w: *mut QMainWindow);
    unsafe fn QMainWindow_hide(w: *mut QMainWindow);
    unsafe fn QMainWindow_setDockOptions(w: *mut QMainWindow, options: i32);
    unsafe fn QMainWindow_setTabPosition(w: *mut QMainWindow, areas: i32, tabPos: i32);
    }
