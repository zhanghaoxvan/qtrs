//! Qt application singleton.
//!
//! Wraps [`QApplication`](https://doc.qt.io/qt-6/qapplication.html),
//! the root object required by every Qt program. Only one instance can
//! exist per process — the C++ implementation uses a file-static pointer
//! so subsequent calls to [`Application::new`] return the same object.

use cxx::let_cxx_string;

use crate::ffi;

/// A Qt application instance (process-global singleton).
///
/// There can only be one `Application` per process. Creating multiple
/// Rust `Application` values is safe — the underlying C++ `QApplication`
/// is a static singleton — but you should hold exactly one for clarity.
///
/// `Application` is **not** [`Send`] or [`Sync`]. It must live on the
/// main thread alongside all widgets it manages.
///
/// # Example
///
/// ```ignore
/// use qtrs::Application;
///
/// let app = Application::new();
/// // ... create windows, widgets ...
/// app.exec(); // blocks until the last window closes
/// ```
pub struct Application {
    ptr: *mut ffi::QApplication,
}

impl Application {
    /// Create (or retrieve) the singleton Qt application.
    ///
    /// The first call constructs the actual `QApplication`; subsequent
    /// calls return the same underlying object.
    ///
    /// # Panics
    ///
    /// Panics if Qt cannot initialise (e.g. no display available on
    /// Linux without `QT_QPA_PLATFORM=offscreen`).
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QApplication_new() };
        debug_assert!(!ptr.is_null(), "QApplication_new returned null");
        Self { ptr }
    }

    /// Set the **application-wide** window icon from an image file.
    ///
    /// Unlike [`Widget::set_icon`], this sets the fallback icon for
    /// **all** windows in the application. This is required for the
    /// icon to appear on **Wayland** — Wayland compositors no_run
    /// per-widget icons and instead use the application-level icon
    /// set here (or the one from the `.desktop` file — see
    /// [`set_desktop_file_name`]).
    ///
    /// Call this **after** creating the `Application` but **before**
    /// showing any windows.
    ///
    /// [`Widget::set_icon`]: crate::Widget::set_icon
    /// [`set_desktop_file_name`]: Self::set_desktop_file_name
    pub fn set_icon(&self, icon_path: &str) {
        debug_assert!(!self.ptr.is_null(), "Application::set_icon on null pointer");
        let_cxx_string!(c_path = icon_path);
        unsafe {
            ffi::QApplication_setWindowIcon(self.ptr, &c_path);
        }
    }

    /// Tell Wayland which `.desktop` file identifies this app.
    ///
    /// Desktop file name should **not** include the `.desktop` suffix
    /// (e.g. pass `"myapp"` for `myapp.desktop`). Wayland compositors
    /// read the `Icon=` key from the matching desktop file to display
    /// the app icon in the task switcher and title bar.
    ///
    /// Call this **before** showing any windows.
    pub fn set_desktop_file_name(&self, name: &str) {
        debug_assert!(!self.ptr.is_null(), "Application::set_desktop_file_name on null pointer");
        let_cxx_string!(c_name = name);
        unsafe {
            ffi::QApplication_setDesktopFileName(self.ptr, &c_name);
        }
    }

    /// Enter the Qt event loop.
    ///
    /// This call **blocks** until
    /// [`QApplication::quit()`](https://doc.qt.io/qt-6/qapplication.html#quit)
    /// is triggered (normally when the last top-level window is closed).
    /// Returns the exit code — 0 for normal exit, non-zero for errors.
    ///
    /// Must be called on the main thread.
    pub fn exec(&self) -> i32 {
        debug_assert!(!self.ptr.is_null(), "Application::exec on null pointer");
        unsafe { ffi::QApplication_exec(self.ptr) }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

// No Drop needed: QApplication is a static singleton owned by C++.
