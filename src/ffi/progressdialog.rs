unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QProgressDialog ---
    unsafe fn QProgressDialog_new(
    label: &CxxString,
    cancelText: &CxxString,
    min: i32,
    max: i32,
    parent: *mut QWidget,
    ) -> *mut QProgressDialog;
    unsafe fn QProgressDialog_delete(w: *mut QProgressDialog);
    unsafe fn QProgressDialog_setMinimum(w: *mut QProgressDialog, min: i32);
    unsafe fn QProgressDialog_setMaximum(w: *mut QProgressDialog, max: i32);
    unsafe fn QProgressDialog_setRange(w: *mut QProgressDialog, min: i32, max: i32);
    unsafe fn QProgressDialog_setValue(w: *mut QProgressDialog, value: i32);
    unsafe fn QProgressDialog_value(w: *mut QProgressDialog) -> i32;
    unsafe fn QProgressDialog_setLabelText(w: *mut QProgressDialog, text: &CxxString);
    unsafe fn QProgressDialog_setCancelButtonText(w: *mut QProgressDialog, text: &CxxString);
    unsafe fn QProgressDialog_wasCanceled(w: *mut QProgressDialog) -> bool;
    unsafe fn QProgressDialog_setMinimumDuration(w: *mut QProgressDialog, ms: i32);
    unsafe fn QProgressDialog_setAutoClose(w: *mut QProgressDialog, close: bool);
    unsafe fn QProgressDialog_setAutoReset(w: *mut QProgressDialog, reset: bool);
    unsafe fn QProgressDialog_show(w: *mut QProgressDialog);
    unsafe fn QProgressDialog_hide(w: *mut QProgressDialog);
    unsafe fn QProgressDialog_close(w: *mut QProgressDialog);
    }
