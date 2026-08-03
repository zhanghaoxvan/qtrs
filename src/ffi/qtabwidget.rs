unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
// --- QTabWidget ---
        unsafe fn QTabWidget_new(parent: *mut QWidget) -> *mut QTabWidget;
        unsafe fn QTabWidget_addTab(tw: *mut QTabWidget, page: *mut QWidget, label: &CxxString);
        unsafe fn QTabWidget_currentIndex(tw: *mut QTabWidget) -> i32;
        unsafe fn QTabWidget_setCurrentIndex(tw: *mut QTabWidget, index: i32);
        unsafe fn QTabWidget_delete(tw: *mut QTabWidget);
        unsafe fn QTabWidget_onCurrentChanged(tw: *mut QTabWidget, ctx: u64);
    }
