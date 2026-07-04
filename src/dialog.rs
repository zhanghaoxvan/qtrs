//! Standard dialog boxes (QMessageBox convenience wrappers).
//!
//! These are static convenience methods on
//! [`QMessageBox`](https://doc.qt.io/qt-6/qmessagebox.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::widget::AsWidget;

/// Show a modal **information** dialog.
pub fn information(parent: Option<&dyn AsWidget>, title: &str, text: &str) {
    let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
    let_cxx_string!(c_title = title);
    let_cxx_string!(c_text = text);
    unsafe { ffi::QMessageBox_information(parent_ptr, &c_title, &c_text); }
}

/// Show a modal **warning** dialog.
pub fn warning(parent: Option<&dyn AsWidget>, title: &str, text: &str) {
    let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
    let_cxx_string!(c_title = title);
    let_cxx_string!(c_text = text);
    unsafe { ffi::QMessageBox_warning(parent_ptr, &c_title, &c_text); }
}

/// Show a modal **critical** (error) dialog.
pub fn critical(parent: Option<&dyn AsWidget>, title: &str, text: &str) {
    let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
    let_cxx_string!(c_title = title);
    let_cxx_string!(c_text = text);
    unsafe { ffi::QMessageBox_critical(parent_ptr, &c_title, &c_text); }
}

/// Show a modal **question** dialog with Yes/No buttons.
///
/// Returns `true` if the user clicked **Yes**, `false` for **No**.
/// (Qt's `QMessageBox::Yes` is 0x00004000, `QMessageBox::No` is 0x00010000.)
pub fn question(parent: Option<&dyn AsWidget>, title: &str, text: &str) -> bool {
    let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
    let_cxx_string!(c_title = title);
    let_cxx_string!(c_text = text);
    let result = unsafe { ffi::QMessageBox_question(parent_ptr, &c_title, &c_text) };
    result == 0x00004000 // QMessageBox::Yes
}
