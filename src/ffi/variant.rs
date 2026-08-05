// src/ffi/variant.rs — FFI declarations
unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");

    // --- Constructors ---
    unsafe fn QVariant_from_int(v: i32) -> *mut QVariant;
    unsafe fn QVariant_from_uint(v: u32) -> *mut QVariant;
    unsafe fn QVariant_from_long(v: i64) -> *mut QVariant;
    unsafe fn QVariant_from_bool(v: bool) -> *mut QVariant;
    unsafe fn QVariant_from_double(v: f64) -> *mut QVariant;
    unsafe fn QVariant_from_string(s: String) -> *mut QVariant;
    unsafe fn QVariant_from_stringlist(v: Vec<String>) -> *mut QVariant;
    unsafe fn QVariant_from_bytearray(data: &[u8]) -> *mut QVariant;

    // --- Destructor ---
    unsafe fn QVariant_delete(v: *mut QVariant);

    // --- Type check ---
    unsafe fn QVariant_is_int(v: *mut QVariant) -> bool;
    unsafe fn QVariant_is_uint(v: *mut QVariant) -> bool;
    unsafe fn QVariant_is_long(v: *mut QVariant) -> bool;
    unsafe fn QVariant_is_bool(v: *mut QVariant) -> bool;
    unsafe fn QVariant_is_double(v: *mut QVariant) -> bool;
    unsafe fn QVariant_is_string(v: *mut QVariant) -> bool;
    unsafe fn QVariant_is_stringlist(v: *mut QVariant) -> bool;
    unsafe fn QVariant_is_bytearray(v: *mut QVariant) -> bool;

    // --- Getters ---
    unsafe fn QVariant_to_int(v: *mut QVariant) -> i32;
    unsafe fn QVariant_to_uint(v: *mut QVariant) -> u32;
    unsafe fn QVariant_to_long(v: *mut QVariant) -> i64;
    unsafe fn QVariant_to_bool(v: *mut QVariant) -> bool;
    unsafe fn QVariant_to_double(v: *mut QVariant) -> f64;
    unsafe fn QVariant_to_string(v: *mut QVariant) -> String;
    unsafe fn QVariant_to_stringlist(v: *mut QVariant) -> Vec<String>;
    unsafe fn QVariant_to_bytearray(v: *mut QVariant) -> Vec<u8>;
}