//! Desktop services for opening URLs/files.
//! Wraps [`QDesktopServices`](https://doc.qt.io/qt-6/qdesktopservices.html).

use cxx::let_cxx_string;
use crate::ffi;

/// Open a URL in the default browser or file in the default application.
/// Returns `true` on success.
pub fn open_url(url: &str) -> bool {
    let_cxx_string!(c = url);
    unsafe { ffi::QDesktopServices_openUrl(&c) }
}
