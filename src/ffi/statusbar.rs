unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QStatusBar ---
    unsafe fn QStatusBar_new(parent: *mut QWidget) -> *mut QStatusBar;
    unsafe fn QStatusBar_delete(bar: *mut QStatusBar);
    unsafe fn QStatusBar_showMessage(bar: *mut QStatusBar, text: &CxxString, timeout_ms: i32);
    unsafe fn QStatusBar_clearMessage(bar: *mut QStatusBar);
    unsafe fn QStatusBar_addWidget(bar: *mut QStatusBar, widget: *mut QWidget);
    unsafe fn QStatusBar_addPermanentWidget(bar: *mut QStatusBar, widget: *mut QWidget);
    unsafe fn QStatusBar_removeWidget(bar: *mut QStatusBar, widget: *mut QWidget);
    unsafe fn QStatusBar_setSizeGripEnabled(bar: *mut QStatusBar, enabled: bool);
    }
