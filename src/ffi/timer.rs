unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QTimer ---
    unsafe fn QTimer_new() -> *mut QTimer;
    unsafe fn QTimer_start(timer: *mut QTimer, interval_ms: i32);
    unsafe fn QTimer_stop(timer: *mut QTimer);
    unsafe fn QTimer_isActive(timer: *mut QTimer) -> bool;
    unsafe fn QTimer_delete(timer: *mut QTimer);
    unsafe fn QTimer_onTimeout(timer: *mut QTimer, ctx: u64);
    unsafe fn QTimer_singleShot(interval_ms: i32, ctx: u64);
    }
