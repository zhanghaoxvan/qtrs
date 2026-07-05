//! Static text label widget.
//!
//! Wraps [`QLabel`](https://doc.qt.io/qt-6/qlabel.html) for displaying
//! read-only text in a window.

use cxx::let_cxx_string;

use crate::ffi;
use crate::widget::AsWidget;

/// A static text label.
///
/// `Label` uses a **builder pattern**: call [`Label::new`] to obtain a
/// [`Builder`], set the parent, then call `.build()`.
///
/// # Example
///
/// ```ignore
/// use qtrs::Label;
///
/// let label = Label::new("Hello, world!").build();
/// ```
pub struct Label {
    ptr: *mut ffi::QLabel,
    has_parent: bool,
    #[allow(dead_code)]
    text: String,
}

impl Label {
    /// Start building a new `QLabel`.
    ///
    /// Returns a [`Builder`]. Set an optional parent, then call `.build()`.
    pub fn new(text: impl Into<String>) -> Builder {
        Builder::new(text.into())
    }

    /// Get the current label text.
    ///
    /// Returns the cached copy — this is updated automatically when you
    /// call [`set_text`](Self::set_text).
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Update the label text at runtime.
    ///
    /// This calls
    /// [`QLabel::setText`](https://doc.qt.io/qt-6/qlabel.html#text-prop).
    pub fn set_text(&mut self, text: impl Into<String>) {
        debug_assert!(!self.ptr.is_null(), "Label::set_text on null pointer");
        self.text = text.into();
        let_cxx_string!(c_text = &self.text);
        unsafe {
            ffi::QLabel_setText(self.ptr, &c_text);
        }
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QLabel, text: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, text: text.to_string() }
    }
}

impl AsWidget for Label {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null(), "Label::widget_ptr on null pointer");
        unsafe { ffi::toQWidget_QLabel(self.ptr) }
    }

    fn set_has_parent(&mut self) {
        self.has_parent = true;
    }
}

impl Drop for Label {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        if !self.has_parent {
            unsafe { ffi::QLabel_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`Label`].
pub struct Builder {
    text: String,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new(text: String) -> Self {
        Self {
            text,
            parent: None,
        }
    }

    /// Set the parent widget.
    ///
    /// The parent manages the label's C++ lifetime.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QLabel` and return the Rust wrapper.
    pub fn build(self) -> Label {
        let_cxx_string!(c_text = &self.text);
        let ptr = unsafe {
            ffi::QLabel_new(
                &c_text,
                self.parent.unwrap_or(std::ptr::null_mut()),
            )
        };
        debug_assert!(!ptr.is_null(), "QLabel_new returned null");

        Label {
            ptr,
            has_parent: self.parent.is_some(),
            text: self.text,
        }
    }
}
