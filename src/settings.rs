//! Qt Settings — Cross-platform persistent application settings
//!
//! QSettings is an abstraction over the system registry, property list files,
//! and INI files, allowing you to save and restore application settings in a
//! portable way.
//!
//! # Examples
//! ```
//! use qtrs::Settings;
//!
//! let settings = Settings::new("MyCompany", "MyApp");
//! settings.set("theme", "dark");
//! assert_eq!(
//!     settings.get("theme", "light")
//!         .convert::<String>(),
//!     Some("dark".to_string())
//! );
//! ```

use crate::ffi::{QSettings, ffi_inner};
use crate::Variant;

// ============================================================
// Type Definitions
// ============================================================

/// Settings scope: user-specific or system-wide
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// Settings for the current user
    User = 0,
    /// Settings shared by all users on the system
    System = 1,
}

/// Storage format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Platform-native format (Windows: registry, macOS: plist, Linux: INI)
    Native = 0,
    /// INI file format
    Ini = 1,
    /// 32-bit registry (Windows only)
    Registry32 = 2,
    /// 64-bit registry (Windows only)
    Registry64 = 3,
}

/// Settings status code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// No error occurred
    NoError = 0,
    /// Access error (e.g., read-only file)
    AccessError = 1,
    /// Format error (e.g., malformed INI file)
    FormatError = 2,
}

// ============================================================
// Settings
// ============================================================

/// Rust wrapper for QSettings
///
/// Provides cross-platform persistent configuration management.
///
/// # Lifetimes
/// When `Settings` is dropped, `sync()` is automatically called to flush
/// changes to disk.
pub struct Settings {
    inner: *mut QSettings,
}

unsafe impl Send for Settings {}
unsafe impl Sync for Settings {}

impl Settings {
    // ============================================================
    // Constructors
    // ============================================================

    /// Creates a new settings object (user scope, native format)
    ///
    /// # Arguments
    /// - `organization`: Organization name (e.g., "MyCompany")
    /// - `application`: Application name (e.g., "MyApp")
    ///
    /// # Examples
    /// ```
    /// use qtrs::Settings;
    /// let settings = Settings::new("MySoft", "StarRunner");
    /// ```
    pub fn new(organization: &str, application: &str) -> Self {
        unsafe {
            Self {
                inner: ffi_inner::QSettings_new_user_app(&organization.to_string(), &application.to_string()),
            }
        }
    }

    /// Creates a settings object with the specified scope
    ///
    /// # Arguments
    /// - `scope`: `Scope::User` or `Scope::System`
    /// - `organization`: Organization name
    /// - `application`: Application name
    ///
    /// # Examples
    /// ```
    /// use qtrs::{Settings, Scope};
    /// let settings = Settings::new_with_scope(Scope::System, "MyCompany", "MyApp");
    /// ```
    pub fn new_with_scope(scope: Scope, organization: &str, application: &str) -> Self {
        unsafe {
            Self {
                inner: ffi_inner::QSettings_new_scope_app(scope as i32, &organization.to_string(), &application.to_string()),
            }
        }
    }

    /// Creates a settings object with the specified format and scope
    ///
    /// # Arguments
    /// - `format`: Storage format (`Format::Native` or `Format::Ini`)
    /// - `scope`: Scope (`Scope::User` or `Scope::System`)
    /// - `organization`: Organization name
    /// - `application`: Application name
    ///
    /// # Examples
    /// ```
    /// use qtrs::{Settings, Format, Scope};
    /// let settings = Settings::new_with_format(Format::Ini, Scope::User, "MyCompany", "MyApp");
    /// ```
    pub fn new_with_format(format: Format, scope: Scope, organization: &str, application: &str) -> Self {
        unsafe {
            Self {
                inner: ffi_inner::QSettings_new_format_scope(format as i32, scope as i32, &organization.to_string(), &application.to_string()),
            }
        }
    }

    /// Creates a settings object backed by a specific file
    ///
    /// # Arguments
    /// - `path`: File path
    /// - `format`: File format (usually `Format::Ini`)
    ///
    /// # Examples
    /// ```
    /// use qtrs::{Settings, Format};
    /// let settings = Settings::new_from_file("/etc/myapp.conf", Format::Ini);
    /// ```
    pub fn new_from_file(path: &str, format: Format) -> Self {
        unsafe {
            Self {
                inner: ffi_inner::QSettings_new_file(&path.to_string(), format as i32),
            }
        }
    }

    // ============================================================
    // Basic Read/Write
    // ============================================================

    /// Writes a setting
    ///
    /// # Arguments
    /// - `key`: Key name (supports `/` hierarchy, e.g., `"mainwindow/size"`)
    /// - `value`: String value
    ///
    /// # Examples
    /// ```
    /// # use qtrs::Settings;
    /// # let settings = Settings::new("MyCompany", "MyApp");
    /// settings.set("theme", "dark");
    /// ```
    pub fn set<T: Into<Variant>>(&self, key: &str, value: T) {
        let variant = value.into();
        unsafe {
            ffi_inner::QSettings_setValue(self.inner, &key.to_string(), variant.raw_ptr());
        }
    }

    /// Reads a setting
    ///
    /// # Arguments
    /// - `key`: Key name
    /// - `default`: Default value returned if the key does not exist
    ///
    /// # Returns
    /// The setting value, or `default` if the key is not found
    ///
    /// # Examples
    /// ```
    /// # use qtrs::Settings;
    /// # let settings = Settings::new("MyCompany", "MyApp");
    /// let theme = settings.get("theme", "light");
    /// ```
    pub fn get<T: Into<Variant>>(&self, key: &str, default: T) -> Variant {
        let default_value = default.into();
        unsafe {
            Variant {
                inner: ffi_inner::QSettings_value(
                    self.inner,
                    &key.to_string(),
                    default_value.raw_ptr()
                )
            }
        }
    }

    // ============================================================
    // Group Operations
    // ============================================================

    /// Begins a group
    ///
    /// All keys within the group are automatically prefixed.
    ///
    /// # Examples
    /// ```
    /// # use qtrs::Settings;
    /// # let settings = Settings::new("MyCompany", "MyApp");
    /// settings.begin_group("mainwindow");
    /// settings.set("size", "1024x768");  // Actual key: "mainwindow/size"
    /// settings.end_group();
    /// ```
    pub fn begin_group(&self, prefix: &str) {
        unsafe {
            ffi_inner::QSettings_beginGroup(self.inner, &prefix.to_string());
        }
    }

    /// Ends the current group
    pub fn end_group(&self) {
        unsafe {
            ffi_inner::QSettings_endGroup(self.inner);
        }
    }

    /// Returns the current group path
    pub fn group(&self) -> String {
        unsafe {
            ffi_inner::QSettings_group(self.inner)
        }
    }

    // ============================================================
    // Array Operations
    // ============================================================

    /// Begins reading an array
    ///
    /// Returns the array size. Must be used with `set_array_index()` and `end_array()`.
    ///
    /// # Examples
    /// ```
    /// # use qtrs::Settings;
    /// # let settings = Settings::new("MyCompany", "MyApp");
    /// let size = settings.begin_read_array("logins");
    /// for i in 0..size {
    ///     settings.set_array_index(i);
    ///     let name = settings.get("username", "");
    /// }
    /// settings.end_array();
    /// ```
    pub fn begin_read_array(&self, prefix: &str) -> i32 {
        unsafe {
            ffi_inner::QSettings_beginReadArray(self.inner, &prefix.to_string())
        }
    }

    /// Begins writing an array
    ///
    /// # Arguments
    /// - `prefix`: Array prefix
    /// - `size`: Array size (-1 for auto-detection)
    pub fn begin_write_array(&self, prefix: &str, size: i32) {
        unsafe {
            ffi_inner::QSettings_beginWriteArray(self.inner, &prefix.to_string(), size);
        }
    }

    /// Ends the current array
    pub fn end_array(&self) {
        unsafe {
            ffi_inner::QSettings_endArray(self.inner);
        }
    }

    /// Sets the current array index
    ///
    /// Must be called after `begin_read_array()` or `begin_write_array()`.
    pub fn set_array_index(&self, index: i32) {
        unsafe {
            ffi_inner::QSettings_setArrayIndex(self.inner, index);
        }
    }

    // ============================================================
    // Query Operations
    // ============================================================

    /// Checks whether a key exists
    pub fn contains(&self, key: &str) -> bool {
        unsafe {
            ffi_inner::QSettings_contains(self.inner, &key.to_string())
        }
    }

    /// Removes the specified key
    pub fn remove(&self, key: &str) {
        unsafe {
            ffi_inner::QSettings_remove(self.inner, &key.to_string());
        }
    }

    /// Removes all keys in the current group
    ///
    /// # Examples
    /// ```
    /// # use qtrs::Settings;
    /// # let settings = Settings::new("MyCompany", "MyApp");
    /// settings.begin_group("cache");
    /// settings.clear();  // Only removes keys under "cache"
    /// settings.end_group();
    /// ```
    pub fn clear(&self) {
        unsafe {
            ffi_inner::QSettings_clear(self.inner);
        }
    }

    /// Returns all keys
    pub fn all_keys(&self) -> Vec<String> {
        unsafe {
            ffi_inner::QSettings_allKeys(self.inner)
        }
    }

    /// Returns child keys in the current group
    pub fn child_keys(&self) -> Vec<String> {
        unsafe {
            ffi_inner::QSettings_childKeys(self.inner)
        }
    }

    /// Returns child groups in the current group
    pub fn child_groups(&self) -> Vec<String> {
        unsafe {
            ffi_inner::QSettings_childGroups(self.inner)
        }
    }

    // ============================================================
    // Status and Sync
    // ============================================================

    /// Syncs to disk
    ///
    /// Writes all unsaved changes to permanent storage.
    /// The destructor automatically calls this method, so manual calls
    /// are usually not necessary.
    pub fn sync(&self) {
        unsafe {
            ffi_inner::QSettings_sync(self.inner);
        }
    }

    /// Returns whether the settings are writable
    pub fn is_writable(&self) -> bool {
        unsafe {
            ffi_inner::QSettings_isWritable(self.inner)
        }
    }

    /// Returns the current status
    pub fn status(&self) -> Status {
        unsafe {
            match ffi_inner::QSettings_status(self.inner) {
                0 => Status::NoError,
                1 => Status::AccessError,
                2 => Status::FormatError,
                _ => Status::NoError,
            }
        }
    }

    /// Returns the file path / registry path
    pub fn file_name(&self) -> String {
        unsafe {
            ffi_inner::QSettings_fileName(self.inner)
        }
    }

    /// Returns whether fallbacks are enabled
    pub fn fallbacks_enabled(&self) -> bool {
        unsafe {
            ffi_inner::QSettings_fallbacksEnabled(self.inner)
        }
    }

    /// Enables or disables fallbacks
    ///
    /// When disabled, only reads from the current scope and does not
    /// search fallback locations.
    pub fn set_fallbacks_enabled(&self, enabled: bool) {
        unsafe {
            ffi_inner::QSettings_setFallbacksEnabled(self.inner, enabled);
        }
    }

    /// Returns the raw pointer (for advanced use)
    pub fn as_ptr(&self) -> *mut QSettings {
        self.inner
    }
}

// ============================================================
// Drop
// ============================================================

impl Drop for Settings {
    /// Automatically syncs and frees resources on drop
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                ffi_inner::QSettings_sync(self.inner);
                ffi_inner::QSettings_delete(self.inner);
            }
        }
    }
}

// ============================================================
// Debug & Display
// ============================================================

impl std::fmt::Debug for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys = self.all_keys();
        f.debug_struct("Settings")
            .field("file", &self.file_name())
            .field("key_count", &keys.len())
            .field("keys", &keys)
            .finish()
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Settings({} keys, file: {})", 
               self.all_keys().len(), 
               self.file_name())
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let settings = Settings::new("TestOrg", "TestApp");
        settings.set("key", "value");
        assert_eq!(settings.get("key", ""), Variant::from("value"));
        settings.remove("key");
        assert!(!settings.contains("key"));
    }

    #[test]
    fn test_group() {
        let settings = Settings::new("TestOrg", "TestApp");
        settings.begin_group("group");
        settings.set("key", "value");
        settings.end_group();
        assert_eq!(
            settings.get("group/key", "")
                .convert::<String>()
                .unwrap(),
            "value"
        );
    }
}