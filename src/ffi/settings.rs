// src/ffi/settings.rs
unsafe extern "C++" {
    include!("src/cpp/settings.h");

    type QSettings;

    // --- Constructors ---
    unsafe fn QSettings_new_user_app(org: &CxxString, app: &CxxString) -> *mut QSettings;
    unsafe fn QSettings_new_scope_app(scope: i32, org: &CxxString, app: &CxxString) -> *mut QSettings;
    unsafe fn QSettings_new_format_scope(format: i32, scope: i32, org: &CxxString, app: &CxxString) -> *mut QSettings;
    unsafe fn QSettings_new_file(fileName: &CxxString, format: i32) -> *mut QSettings;
    unsafe fn QSettings_delete(s: *mut QSettings);

    // --- Group ---
    unsafe fn QSettings_beginGroup(s: *mut QSettings, prefix: &CxxString);
    unsafe fn QSettings_endGroup(s: *mut QSettings);
    unsafe fn QSettings_group(s: *mut QSettings) -> CxxString;

    // --- Array ---
    unsafe fn QSettings_beginReadArray(s: *mut QSettings, prefix: &CxxString) -> i32;
    unsafe fn QSettings_beginWriteArray(s: *mut QSettings, prefix: &CxxString, size: i32);
    unsafe fn QSettings_endArray(s: *mut QSettings);
    unsafe fn QSettings_setArrayIndex(s: *mut QSettings, i: i32);

    // --- Read ---
    unsafe fn QSettings_value_string(s: *mut QSettings, key: &CxxString, default_value: &CxxString) -> CxxString;
    unsafe fn QSettings_value_int(s: *mut QSettings, key: &CxxString, default_value: i32) -> i32;
    unsafe fn QSettings_value_bool(s: *mut QSettings, key: &CxxString, default_value: bool) -> bool;
    unsafe fn QSettings_value_double(s: *mut QSettings, key: &CxxString, default_value: f64) -> f64;

    // --- Write ---
    unsafe fn QSettings_setValue_string(s: *mut QSettings, key: &CxxString, value: &CxxString);
    unsafe fn QSettings_setValue_int(s: *mut QSettings, key: &CxxString, value: i32);
    unsafe fn QSettings_setValue_bool(s: *mut QSettings, key: &CxxString, value: bool);
    unsafe fn QSettings_setValue_double(s: *mut QSettings, key: &CxxString, value: f64);
    unsafe fn QSettings_setValue_variant(s: *mut QSettings, key: &CxxString, value: *mut crate::ffi_inner::QVariant);

    // --- Other ---
    unsafe fn QSettings_contains(s: *mut QSettings, key: &CxxString) -> bool;
    unsafe fn QSettings_remove(s: *mut QSettings, key: &CxxString);
    unsafe fn QSettings_sync(s: *mut QSettings);
    unsafe fn QSettings_clear(s: *mut QSettings);
    unsafe fn QSettings_isWritable(s: *mut QSettings) -> bool;
    unsafe fn QSettings_status(s: *mut QSettings) -> i32;
    unsafe fn QSettings_allKeys(s: *mut QSettings) -> CxxVector<CxxString>;
    unsafe fn QSettings_childKeys(s: *mut QSettings) -> CxxVector<CxxString>;
    unsafe fn QSettings_childGroups(s: *mut QSettings) -> CxxVector<CxxString>;
    unsafe fn QSettings_fileName(s: *mut QSettings) -> CxxString;
    unsafe fn QSettings_fallbacksEnabled(s: *mut QSettings) -> bool;
    unsafe fn QSettings_setFallbacksEnabled(s: *mut QSettings, enabled: bool);
}