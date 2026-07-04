//! QML / Qt Quick support.
//!
//! Wraps [`QQmlApplicationEngine`](https://doc.qt.io/qt-6/qqmlapplicationengine.html)
//! for loading `.qml` files.
//!
//! Enable with feature `qml` in Cargo.toml.

use cxx::let_cxx_string;

use crate::ffi;

/// A QML application engine.
///
/// Loads and runs `.qml` files for Qt Quick UIs. Once the QML is loaded,
/// control returns to the caller — call [`Application::exec`] afterwards
/// to enter the event loop.
///
/// [`Application::exec`]: crate::Application::exec
///
/// # Example
///
/// ```ignore
/// use qtrs::*;
///
/// let app = Application::new();
///
/// let engine = QmlEngine::new();
/// engine.load("ui/main.qml");
///
/// app.exec();
/// ```
pub struct QmlEngine {
    ptr: *mut ffi::QQmlApplicationEngine,
}

impl QmlEngine {
    /// Create a new `QQmlApplicationEngine`.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QQmlApplicationEngine_new() };
        debug_assert!(!ptr.is_null(), "QQmlApplicationEngine_new returned null");
        Self { ptr }
    }

    /// Return the raw `QQmlApplicationEngine*` pointer for FFI use.
    pub fn raw_ptr(&self) -> *mut std::ffi::c_void {
        self.ptr as *mut std::ffi::c_void
    }

    /// Load a `.qml` file.
    ///
    /// The path is resolved relative to the working directory, or use a
    /// Qt resource path (`qrc:/...`).
    ///
    /// # Panics
    ///
    /// Qt may emit warnings to stderr if the file is not found or
    /// contains errors. The engine does not return an error code;
    /// check the console output for QML parsing errors.
    pub fn load(&self, qml_path: &str) {
        debug_assert!(!self.ptr.is_null(), "QmlEngine::load on null pointer");
        let_cxx_string!(c_path = qml_path);
        unsafe {
            ffi::QQmlApplicationEngine_load(self.ptr, &c_path);
        }
    }
}

impl Default for QmlEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for QmlEngine {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe { ffi::QQmlApplicationEngine_delete(self.ptr) };
        self.ptr = std::ptr::null_mut();
    }
}
