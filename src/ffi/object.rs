unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QObject (base class for signal-slot connections) ---
    type QObject;

    // --- Generic signal-slot connection ---
    unsafe fn QObject_connect(
        sender: *mut QObject,
        sig: &CxxString,
        receiver: *mut QObject,
        slt: &CxxString,
        conn_type: i32,
    ) -> bool;

    unsafe fn QObject_disconnect(
        sender: *mut QObject,
        sig: &CxxString,
        receiver: *mut QObject,
        slt: &CxxString,
    ) -> bool;

    unsafe fn QObject_disconnectAll(obj: *mut QObject);

    // --- Thread safety ---
    unsafe fn QObject_isInGuiThread() -> bool;

    // --- Trampolines ---
    unsafe fn qtrs_setVoidTrampoline(trampoline: unsafe extern "C" fn(u64));
    unsafe fn qtrs_setBoolTrampoline(trampoline: unsafe extern "C" fn(u64, bool));
    unsafe fn qtrs_setIntTrampoline(trampoline: unsafe extern "C" fn(u64, i32));
    unsafe fn qtrs_setStringTrampoline(trampoline: unsafe extern "C" fn(u64, String));
}
