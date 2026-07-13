//! Static convenience functions for input dialogs.
//!
//! Wraps static methods on
//! [`QInputDialog`](https://doc.qt.io/qt-6/qinputdialog.html).
//!
//! # Note
//!
//! These are pure static functions — there is no struct or builder pattern.

use cxx::let_cxx_string;

use crate::ffi;
use crate::widget::AsWidget;

/// Show a modal dialog asking the user to enter a string.
///
/// Returns the entered text, or an empty string if the user cancelled.
pub fn get_text(parent: Option<&dyn AsWidget>, title: &str, label: &str, default_text: &str) -> String {
    let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
    let_cxx_string!(c_title = title);
    let_cxx_string!(c_label = label);
    let_cxx_string!(c_default = default_text);
    unsafe { ffi::QInputDialog_getText(parent_ptr, &c_title, &c_label, &c_default) }
}

/// Show a modal dialog asking the user to enter an integer.
///
/// Returns `0` if the user cancelled.
pub fn get_int(
    parent: Option<&dyn AsWidget>,
    title: &str,
    label: &str,
    value: i32,
    min: i32,
    max: i32,
    step: i32,
) -> i32 {
    let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
    let_cxx_string!(c_title = title);
    let_cxx_string!(c_label = label);
    unsafe { ffi::QInputDialog_getInt(parent_ptr, &c_title, &c_label, value, min, max, step) }
}

/// Show a modal dialog asking the user to enter a floating-point number.
///
/// Returns `0.0` if the user cancelled.
pub fn get_double(
    parent: Option<&dyn AsWidget>,
    title: &str,
    label: &str,
    value: f64,
    min: f64,
    max: f64,
    decimals: i32,
) -> f64 {
    let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
    let_cxx_string!(c_title = title);
    let_cxx_string!(c_label = label);
    unsafe { ffi::QInputDialog_getDouble(parent_ptr, &c_title, &c_label, value, min, max, decimals) }
}

/// Show a modal dialog asking the user to pick an item from a list.
///
/// Returns the selected item text, or an empty string if the user cancelled.
pub fn get_item(
    parent: Option<&dyn AsWidget>,
    title: &str,
    label: &str,
    items: &[&str],
    current: i32,
    editable: bool,
) -> String {
    let parent_ptr = parent.map(|p| p.widget_ptr()).unwrap_or(std::ptr::null_mut());
    let_cxx_string!(c_title = title);
    let_cxx_string!(c_label = label);
    let items_vec: Vec<String> = items.iter().map(|s| s.to_string()).collect();
    unsafe { ffi::QInputDialog_getItem(parent_ptr, &c_title, &c_label, items_vec, current, editable) }
}
