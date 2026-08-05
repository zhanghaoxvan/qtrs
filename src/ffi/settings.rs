// src/ffi/settings.rs
unsafe extern "C++" {
    include!("src/cpp/settings.h");

    // --- Constructors ---
    unsafe fn QSettings_new_user_app(org: &String, app: &String) -> *mut QSettings;
    unsafe fn QSettings_new_scope_app(scope: i32, org: &String, app: &String) -> *mut QSettings;
    unsafe fn QSettings_new_format_scope(format: i32, scope: i32, org: &String, app: &String) -> *mut QSettings;
    unsafe fn QSettings_new_file(fileName: &String, format: i32) -> *mut QSettings;
    unsafe fn QSettings_delete(s: *mut QSettings);

    // --- Group ---
    unsafe fn QSettings_beginGroup(s: *mut QSettings, prefix: &String);
    unsafe fn QSettings_endGroup(s: *mut QSettings);
    unsafe fn QSettings_group(s: *mut QSettings) -> String;

    // --- Array ---
    unsafe fn QSettings_beginReadArray(s: *mut QSettings, prefix: &String) -> i32;
    unsafe fn QSettings_beginWriteArray(s: *mut QSettings, prefix: &String, size: i32);
    unsafe fn QSettings_endArray(s: *mut QSettings);
    unsafe fn QSettings_setArrayIndex(s: *mut QSettings, i: i32);

    // --- Read ---
    unsafe fn QSettings_value(s: *mut QSettings, key: &String, default_value: *mut QVariant) -> *mut QVariant;

    // --- Write ---
    unsafe fn QSettings_setValue(s: *mut QSettings, key: &String, value: *mut QVariant);

    // --- Other ---
    unsafe fn QSettings_contains(s: *mut QSettings, key: &String) -> bool;
    unsafe fn QSettings_remove(s: *mut QSettings, key: &String);
    unsafe fn QSettings_sync(s: *mut QSettings);
    unsafe fn QSettings_clear(s: *mut QSettings);
    unsafe fn QSettings_isWritable(s: *mut QSettings) -> bool;
    unsafe fn QSettings_status(s: *mut QSettings) -> i32;
    unsafe fn QSettings_allKeys(s: *mut QSettings) -> Vec<String>;
    unsafe fn QSettings_childKeys(s: *mut QSettings) -> Vec<String>;
    unsafe fn QSettings_childGroups(s: *mut QSettings) -> Vec<String>;
    unsafe fn QSettings_fileName(s: *mut QSettings) -> String;
    unsafe fn QSettings_fallbacksEnabled(s: *mut QSettings) -> bool;
    unsafe fn QSettings_setFallbacksEnabled(s: *mut QSettings, enabled: bool);
}