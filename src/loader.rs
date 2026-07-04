//! Qt Designer `.ui` file loading.
//!
//! Wraps [`QUiLoader`](https://doc.qt.io/qt-6/quiloader.html) to load
//! XML-based `.ui` files at runtime and instantiate the widget tree.
//!
//! Enable with feature `ui` in Cargo.toml.

use cxx::let_cxx_string;

use crate::ffi;
use crate::widget::{AsWidget, Widget};

/// Loader for `.ui` files created with Qt Designer.
///
/// `UiLoader` reads an XML `.ui` file at runtime and constructs the
/// widget tree it describes. The returned root widget must be kept
/// alive (e.g. stored in a variable and shown).
///
/// # Example
///
/// ```ignore
/// use qtrs::*;
///
/// let app = Application::new();
///
/// let loader = UiLoader::new();
/// let window = loader.load("ui/mainwindow.ui", None);
///
/// if let Some(mut w) = window {
///     w.show();
/// }
///
/// app.exec();
/// ```
pub struct UiLoader {
    ptr: *mut ffi::QUiLoader,
}

impl UiLoader {
    /// Create a new `QUiLoader`.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QUiLoader_new() };
        debug_assert!(!ptr.is_null(), "QUiLoader_new returned null");
        Self { ptr }
    }

    /// Load a `.ui` file and return the root widget.
    ///
    /// `parent` is an optional parent widget. Pass `None` for top-level
    /// windows, or `Some(&parent_widget)` to embed the loaded UI as a
    /// child.
    ///
    /// Returns `None` if the file cannot be opened (check that the path
    /// exists and is readable).
    pub fn load(
        &self,
        ui_path: &str,
        parent: Option<&dyn AsWidget>,
    ) -> Option<Widget> {
        debug_assert!(!self.ptr.is_null(), "UiLoader::load on null pointer");
        let_cxx_string!(c_path = ui_path);
        let parent_ptr = parent
            .map(|p| p.widget_ptr())
            .unwrap_or(std::ptr::null_mut());

        let widget_ptr =
            unsafe { ffi::QUiLoader_load(self.ptr, &c_path, parent_ptr) };

        if widget_ptr.is_null() {
            return None;
        }

        Some(Widget::from_raw(widget_ptr, parent.is_some()))
    }
}

impl Default for UiLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for UiLoader {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe { ffi::QUiLoader_delete(self.ptr) };
        self.ptr = std::ptr::null_mut();
    }
}
