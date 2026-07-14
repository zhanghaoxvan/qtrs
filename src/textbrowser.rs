//! Rich-text browser widget.
//!
//! Wraps [`QTextBrowser`](https://doc.qt.io/qt-6/qtextbrowser.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A rich-text browser for displaying (read-only) HTML content with hyperlink
/// navigation.
///
/// Use a **builder pattern**: [`TextBrowser::new`] returns a [`Builder`].
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_anchor_clicked`] | `QTextBrowser::anchorClicked` | `String` (URL) |
/// | [`Builder::on_text_changed`] | `QTextBrowser::textChanged` | `()` (void) |
pub struct TextBrowser {
    ptr: *mut ffi::QTextBrowser,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl TextBrowser {
    /// Start building a new text browser.
    pub fn new() -> Builder { Builder::new() }

    /// Set the HTML content.
    pub fn set_html(&self, html: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_html = html);
        unsafe { ffi::QTextBrowser_setHtml(self.ptr, &c_html); }
    }

    /// Set the plain text content (replaces any HTML).
    pub fn set_plain_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QTextBrowser_setPlainText(self.ptr, &c_text); }
    }

    /// Get the plain text content.
    pub fn plain_text(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTextBrowser_plainText(self.ptr) }
    }

    /// Get the HTML content.
    pub fn to_html(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTextBrowser_toHtml(self.ptr) }
    }

    /// Set whether external links should be opened automatically.
    pub fn set_open_external_links(&self, open: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTextBrowser_setOpenExternalLinks(self.ptr, open); }
    }

    /// Set whether links are clickable.
    pub fn set_open_links(&self, open: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTextBrowser_setOpenLinks(self.ptr, open); }
    }

    /// Set the source URL.
    pub fn set_source(&self, url: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_url = url);
        unsafe { ffi::QTextBrowser_setSource(self.ptr, &c_url); }
    }

    /// Get the source URL.
    pub fn source(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTextBrowser_source(self.ptr) }
    }

    /// Clear all content.
    pub fn clear(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTextBrowser_clear(self.ptr); }
    }

    /// Append text to the end.
    pub fn append(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QTextBrowser_append(self.ptr, &c_text); }
    }

    /// Set the search paths for resources.
    pub fn set_search_paths(&self, paths: &[&str]) {
        debug_assert!(!self.ptr.is_null());
        let vec: Vec<String> = paths.iter().map(|s| s.to_string()).collect();
        unsafe { ffi::QTextBrowser_setSearchPaths(self.ptr, vec); }
    }

    /// Connect a callback that fires when an anchor is clicked. Receives the URL string.
    pub fn connect_anchor_clicked<F: Fn(String) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QTextBrowser_onAnchorClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when the text changes.
    pub fn connect_text_changed<F: Fn() + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QTextBrowser_onTextChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QTextBrowser, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for TextBrowser {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QTextBrowser(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for TextBrowser {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.signal_handles.clear();
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QTextBrowser_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`TextBrowser`].
pub struct Builder {
    html: Option<String>,
    on_anchor_clicked: Option<Box<dyn Fn(String)>>,
    on_text_changed: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            html: None,
            on_anchor_clicked: None,
            on_text_changed: None,
            parent: None,
        }
    }

    /// Set the initial HTML content.
    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    /// Called when an anchor is clicked. Receives the URL string.
    pub fn on_anchor_clicked<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_anchor_clicked = Some(Box::new(f));
        self
    }

    /// Called when the text changes.
    pub fn on_text_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_text_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QTextBrowser` and return the Rust wrapper.
    pub fn build(self) -> TextBrowser {
        let ptr = unsafe {
            ffi::QTextBrowser_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut tb = TextBrowser {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(html) = &self.html {
            let_cxx_string!(c_html = html);
            unsafe { ffi::QTextBrowser_setHtml(ptr, &c_html); }
        }
        if let Some(f) = self.on_anchor_clicked {
            let h = signal::leak_string(f);
            unsafe { ffi::QTextBrowser_onAnchorClicked(ptr, h.token); }
            tb.signal_handles.push(h);
        }
        if let Some(f) = self.on_text_changed {
            let h = signal::leak_void(f);
            unsafe { ffi::QTextBrowser_onTextChanged(ptr, h.token); }
            tb.signal_handles.push(h);
        }
        tb
    }
}
