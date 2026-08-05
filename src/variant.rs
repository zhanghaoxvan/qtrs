//! Qt Variant — A type-erased value container
//!
//! QVariant can store values of many different types, similar to `std::any`.
//! This module provides a type-safe Rust wrapper around QVariant.

use crate::ffi::ffi_inner;
use std::fmt;

/// A type-erased value container that can hold various Qt types.
///
/// # Examples
/// ```
/// use qtrs::Variant;
///
/// let v = Variant::from(42);
/// assert_eq!(v.convert::<i32>(), Some(42));
///
/// let v = Variant::from("hello");
/// assert_eq!(v.convert::<String>(), Some("hello".to_string()));
/// ```
pub struct Variant {
    pub(crate) inner: *mut ffi_inner::QVariant,
}

unsafe impl Send for Variant {}
unsafe impl Sync for Variant {}

impl Variant {
    // ============================================================
    // Type-safe extraction
    // ============================================================

    /// Extract a value of type `T` from the Variant
    ///
    /// Returns `Some(T)` if the Variant contains the correct type,
    /// otherwise returns `None`.
    ///
    /// # Examples
    /// ```
    /// # use qtrs::Variant;
    /// let v = Variant::from(42);
    /// assert_eq!(v.convert::<i32>(), Some(42));
    /// assert_eq!(v.convert::<String>(), None);
    /// ```
    pub fn convert<T: VariantType>(&self) -> Option<T> {
        if T::is_type(self.inner) {
            Some(unsafe { T::extract(self.inner) })
        } else {
            None
        }
    }

    /// Extract a value of type `T` or return a default value
    pub fn convert_or<T: VariantType + Default>(&self) -> T {
        self.convert::<T>().unwrap_or_default()
    }

    /// Extract a value of type `T` or return a provided default
    pub fn convert_or_else<T: VariantType>(&self, default: T) -> T {
        self.convert::<T>().unwrap_or(default)
    }

    /// Returns the raw pointer (for advanced use)
    pub(crate) fn raw_ptr(&self) -> *mut ffi_inner::QVariant {
        self.inner
    }
}

// ============================================================
// Drop
// ============================================================

impl Drop for Variant {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                ffi_inner::QVariant_delete(self.inner);
            }
        }
    }
}

// ============================================================
// VariantType trait
// ============================================================

/// Trait for types that can be stored in a Variant
pub trait VariantType: Sized + 'static {
    /// Check if the variant contains this type
    fn is_type(ptr: *mut ffi_inner::QVariant) -> bool;

    /// Extract the value from the variant
    /// # Safety
    /// Caller must ensure the variant contains the correct type
    unsafe fn extract(ptr: *mut ffi_inner::QVariant) -> Self;

    /// Create a Variant from this type
    fn into_variant(self) -> Variant;
}

// ============================================================
// Implementations for concrete types
// ============================================================

impl VariantType for i32 {
    fn is_type(ptr: *mut ffi_inner::QVariant) -> bool {
        unsafe { ffi_inner::QVariant_is_int(ptr) }
    }

    unsafe fn extract(ptr: *mut ffi_inner::QVariant) -> Self {
        ffi_inner::QVariant_to_int(ptr)
    }

    fn into_variant(self) -> Variant {
        unsafe {
            Variant {
                inner: ffi_inner::QVariant_from_int(self),
            }
        }
    }
}

impl VariantType for u32 {
    fn is_type(ptr: *mut ffi_inner::QVariant) -> bool {
        unsafe { ffi_inner::QVariant_is_uint(ptr) }
    }

    unsafe fn extract(ptr: *mut ffi_inner::QVariant) -> Self {
        ffi_inner::QVariant_to_uint(ptr)
    }

    fn into_variant(self) -> Variant {
        unsafe {
            Variant {
                inner: ffi_inner::QVariant_from_uint(self),
            }
        }
    }
}

impl VariantType for i64 {
    fn is_type(ptr: *mut ffi_inner::QVariant) -> bool {
        unsafe { ffi_inner::QVariant_is_long(ptr) }
    }

    unsafe fn extract(ptr: *mut ffi_inner::QVariant) -> Self {
        ffi_inner::QVariant_to_long(ptr)
    }

    fn into_variant(self) -> Variant {
        unsafe {
            Variant {
                inner: ffi_inner::QVariant_from_long(self),
            }
        }
    }
}

impl VariantType for bool {
    fn is_type(ptr: *mut ffi_inner::QVariant) -> bool {
        unsafe { ffi_inner::QVariant_is_bool(ptr) }
    }

    unsafe fn extract(ptr: *mut ffi_inner::QVariant) -> Self {
        ffi_inner::QVariant_to_bool(ptr)
    }

    fn into_variant(self) -> Variant {
        unsafe {
            Variant {
                inner: ffi_inner::QVariant_from_bool(self),
            }
        }
    }
}

impl VariantType for f64 {
    fn is_type(ptr: *mut ffi_inner::QVariant) -> bool {
        unsafe { ffi_inner::QVariant_is_double(ptr) }
    }

    unsafe fn extract(ptr: *mut ffi_inner::QVariant) -> Self {
        ffi_inner::QVariant_to_double(ptr)
    }

    fn into_variant(self) -> Variant {
        unsafe {
            Variant {
                inner: ffi_inner::QVariant_from_double(self),
            }
        }
    }
}

impl VariantType for String {
    fn is_type(ptr: *mut ffi_inner::QVariant) -> bool {
        unsafe { ffi_inner::QVariant_is_string(ptr) }
    }

    unsafe fn extract(ptr: *mut ffi_inner::QVariant) -> Self {
        ffi_inner::QVariant_to_string(ptr)
    }

    fn into_variant(self) -> Variant {
        unsafe {
            Variant {
                inner: ffi_inner::QVariant_from_string(self),
            }
        }
    }
}

impl VariantType for Vec<String> {
    fn is_type(ptr: *mut ffi_inner::QVariant) -> bool {
        unsafe { ffi_inner::QVariant_is_stringlist(ptr) }
    }

    unsafe fn extract(ptr: *mut ffi_inner::QVariant) -> Self {
        ffi_inner::QVariant_to_stringlist(ptr)
    }

    fn into_variant(self) -> Variant {
        unsafe {
            Variant {
                inner: ffi_inner::QVariant_from_stringlist(self),
            }
        }
    }
}

impl VariantType for Vec<u8> {
    fn is_type(ptr: *mut ffi_inner::QVariant) -> bool {
        unsafe { ffi_inner::QVariant_is_bytearray(ptr) }
    }

    unsafe fn extract(ptr: *mut ffi_inner::QVariant) -> Self {
        ffi_inner::QVariant_to_bytearray(ptr)
    }

    fn into_variant(self) -> Variant {
        unsafe {
            Variant {
                inner: ffi_inner::QVariant_from_bytearray(&self),
            }
        }
    }
}

// ============================================================
// From / Into traits (convenience)
// ============================================================

impl<T: VariantType> From<T> for Variant {
    fn from(value: T) -> Self {
        value.into_variant()
    }
}

impl From<&str> for Variant {
    fn from(value: &str) -> Self {
        Variant::from(value.to_string())
    }
}

// ============================================================
// PartialEq trait(for assert_eq!, etc.)
// ============================================================

impl PartialEq for Variant {
    fn eq(&self, other: &Self) -> bool {
        // i32
        if let (Some(v1), Some(v2)) = (self.convert::<i32>(), other.convert::<i32>()) {
            return v1 == v2;
        }
        // u32
        if let (Some(v1), Some(v2)) = (self.convert::<u32>(), other.convert::<u32>()) {
            return v1 == v2;
        }
        // i64
        if let (Some(v1), Some(v2)) = (self.convert::<i64>(), other.convert::<i64>()) {
            return v1 == v2;
        }
        // String
        if let (Some(v1), Some(v2)) = (self.convert::<String>(), other.convert::<String>()) {
            return v1 == v2;
        }
        // bool
        if let (Some(v1), Some(v2)) = (self.convert::<bool>(), other.convert::<bool>()) {
            return v1 == v2;
        }
        // f64
        if let (Some(v1), Some(v2)) = (self.convert::<f64>(), other.convert::<f64>()) {
            // oh my god why the world has a thing named nan!
            // i want to impl Eq btw
            return v1 == v2;
        }
        // Vec<String>
        if let (Some(v1), Some(v2)) = (self.convert::<Vec<String>>(), other.convert::<Vec<String>>()) {
            return v1 == v2;
        }
        // Vec<u8>
        if let (Some(v1), Some(v2)) = (self.convert::<Vec<u8>>(), other.convert::<Vec<u8>>()) {
            return v1 == v2;
        }
        false
    }
}

// ============================================================
// Debug trait
// ============================================================

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(v) = self.convert::<i32>() {
            return write!(f, "Variant({})", v);
        }
        if let Some(v) = self.convert::<u32>() {
            return write!(f, "Variant({})", v);
        }
        if let Some(v) = self.convert::<i64>() {
            return write!(f, "Variant({})", v);
        }
        if let Some(v) = self.convert::<String>() {
            return write!(f, "Variant({:?})", v);
        }
        if let Some(v) = self.convert::<bool>() {
            return write!(f, "Variant({})", v);
        }
        if let Some(v) = self.convert::<f64>() {
            if v.is_nan() {
                return write!(f, "Variant(NaN)");
            } else if v.is_infinite() {
                return write!(f, "Variant({})", if v.is_sign_positive() { "inf" } else { "-inf" });
            }
            return write!(f, "Variant({})", v);
        }
        if let Some(v) = self.convert::<Vec<String>>() {
            return write!(f, "Variant({:?})", v);
        }
        if let Some(v) = self.convert::<Vec<u8>>() {
            return write!(f, "Variant({:?})", v);
        }
        
        // 未知类型
        write!(f, "Variant(<unknown>)")
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        let v = Variant::from(42);
        assert_eq!(v.convert::<i32>(), Some(42));
        assert_eq!(v.convert::<String>(), None);

        let v = Variant::from("hello");
        assert_eq!(v.convert::<String>(), Some("hello".to_string()));
        assert_eq!(v.convert::<i32>(), None);
    }

    #[test]
    fn test_convert_or() {
        let v = Variant::from(42);
        assert_eq!(v.convert_or::<i32>(), 42);
        assert_eq!(v.convert_or::<String>(), String::default());

        let v = Variant::from("hello".to_string());
        assert_eq!(v.convert_or_else::<String>("default".to_string()), "hello");
        assert_eq!(v.convert_or::<i32>(), 0);
    }

    #[test]
    fn test_all_types() {
        let v = Variant::from(42_i32);
        assert_eq!(v.convert::<i32>(), Some(42));

        let v = Variant::from(42_u32);
        assert_eq!(v.convert::<u32>(), Some(42));

        let v = Variant::from(42_i64);
        assert_eq!(v.convert::<i64>(), Some(42));

        let v = Variant::from(true);
        assert_eq!(v.convert::<bool>(), Some(true));

        let v = Variant::from(3.14);
        assert_eq!(v.convert::<f64>(), Some(3.14));

        let v = Variant::from("hello".to_string());
        assert_eq!(v.convert::<String>(), Some("hello".to_string()));

        let v = Variant::from(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(v.convert::<Vec<String>>(), Some(vec!["a".to_string(), "b".to_string()]));

        let v = Variant::from(vec![1, 2, 3]);
        assert_eq!(v.convert::<Vec<u8>>(), Some(vec![1, 2, 3]));
    }
}