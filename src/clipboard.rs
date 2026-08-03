//! System clipboard access (static functions).
//! Wraps [`QClipboard`](https://doc.qt.io/qt-6/qclipboard.html).

use cxx::let_cxx_string;
use crate::ffi;

/// Copy text to the system clipboard.
pub fn set_text(text: &str) {
    let_cxx_string!(c = text);
    unsafe { ffi::QClipboard_setText(&c); }
}

/// Get text from the system clipboard.
pub fn text() -> String { unsafe { ffi::QClipboard_text() } }

/// Clear the clipboard.
pub fn clear() { unsafe { ffi::QClipboard_clear(); } }
