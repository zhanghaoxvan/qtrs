//! File dialog for opening and saving files.
//!
//! Wraps [`QFileDialog`](https://doc.qt.io/qt-6/qfiledialog.html).

use cxx::let_cxx_string;
use crate::ffi;
use crate::widget::AsWidget;

/// File dialog convenience functions.
///
/// Provides static methods for common file selection operations.
///
/// # Example
///
/// ```no_run
/// use qtrs::FileDialog;
///
/// // Open a file
/// if let Some(path) = FileDialog::open_file(None, "Open Image", "", "Images (*.png *.jpg)") {
///     println!("Selected: {}", path);
/// }
///
/// // Save a file
/// if let Some(path) = FileDialog::save_file(None, "Save Document", "", "Text Files (*.txt)") {
///     println!("Save to: {}", path);
/// }
///
/// // Select a directory
/// if let Some(path) = FileDialog::select_directory(None, "Select Folder", "") {
///     println!("Selected dir: {}", path);
/// }
/// ```
pub struct FileDialog;

impl FileDialog {
    /// Open a single file.
    ///
    /// # Parameters
    ///
    /// * `parent` — Optional parent widget
    /// * `caption` — Dialog title
    /// * `dir` — Starting directory (empty for default)
    /// * `filter` — File filter, e.g. `"Images (*.png *.jpg)"`
    ///
    /// # Returns
    ///
    /// `Some(path)` if a file was selected, `None` if cancelled.
    pub fn open_file(
        parent: Option<&dyn AsWidget>,
        caption: &str,
        dir: &str,
        filter: &str,
    ) -> Option<String> {
        let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
        let_cxx_string!(c_caption = caption);
        let_cxx_string!(c_dir = dir);
        let_cxx_string!(c_filter = filter);

        let result = unsafe {
            ffi::QFileDialog_getOpenFileName(
                parent_ptr,
                &c_caption,
                &c_dir,
                &c_filter,
            )
        };

        if result.is_empty() { None } else { Some(result) }
    }

    /// Open multiple files.
    ///
    /// # Parameters
    ///
    /// * `parent` — Optional parent widget
    /// * `caption` — Dialog title
    /// * `dir` — Starting directory (empty for default)
    /// * `filter` — File filter, e.g. `"Images (*.png *.jpg)"`
    ///
    /// # Returns
    ///
    /// A vector of selected file paths, empty if cancelled.
    pub fn open_files(
        parent: Option<&dyn AsWidget>,
        caption: &str,
        dir: &str,
        filter: &str,
    ) -> Vec<String> {
        let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
        let_cxx_string!(c_caption = caption);
        let_cxx_string!(c_dir = dir);
        let_cxx_string!(c_filter = filter);

        unsafe {
            ffi::QFileDialog_getOpenFileNames(
                parent_ptr,
                &c_caption,
                &c_dir,
                &c_filter,
            )
        }
    }

    /// Save a file.
    ///
    /// # Parameters
    ///
    /// * `parent` — Optional parent widget
    /// * `caption` — Dialog title
    /// * `dir` — Starting directory (empty for default)
    /// * `filter` — File filter, e.g. `"Text Files (*.txt)"`
    ///
    /// # Returns
    ///
    /// `Some(path)` if a file was selected, `None` if cancelled.
    pub fn save_file(
        parent: Option<&dyn AsWidget>,
        caption: &str,
        dir: &str,
        filter: &str,
    ) -> Option<String> {
        let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
        let_cxx_string!(c_caption = caption);
        let_cxx_string!(c_dir = dir);
        let_cxx_string!(c_filter = filter);

        let result = unsafe {
            ffi::QFileDialog_getSaveFileName(
                parent_ptr,
                &c_caption,
                &c_dir,
                &c_filter,
            )
        };

        if result.is_empty() { None } else { Some(result) }
    }

    /// Select an existing directory.
    ///
    /// # Parameters
    ///
    /// * `parent` — Optional parent widget
    /// * `caption` — Dialog title
    /// * `dir` — Starting directory (empty for default)
    ///
    /// # Returns
    ///
    /// `Some(path)` if a directory was selected, `None` if cancelled.
    pub fn select_directory(
        parent: Option<&dyn AsWidget>,
        caption: &str,
        dir: &str,
    ) -> Option<String> {
        let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
        let_cxx_string!(c_caption = caption);
        let_cxx_string!(c_dir = dir);

        let result = unsafe {
            ffi::QFileDialog_getExistingDirectory(
                parent_ptr,
                &c_caption,
                &c_dir,
            )
        };

        if result.is_empty() { None } else { Some(result) }
    }
}