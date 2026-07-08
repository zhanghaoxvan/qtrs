//! Group box widget.
//!
//! Wraps [`QGroupBox`](https://doc.qt.io/qt-6/qgroupbox.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::widget::AsWidget;

/// A group box with a title, used to group related widgets.
///
/// `GroupBox` uses a builder pattern: call [`GroupBox::new`] to obtain
/// a [`Builder`], then call `.build()`.
///
/// # Example
///
/// ```no_run
/// use qtrs::{GroupBox, VBoxLayout, RadioButton};
///
/// let group = GroupBox::new("Options")
///     .build();
///
/// // Add radio buttons to the group...
/// ```
pub struct GroupBox {
    ptr: *mut ffi::QGroupBox,
    has_parent: bool,
}

impl GroupBox {
    /// Start building a new group box.
    pub fn new(title: impl Into<String>) -> Builder {
        Builder::new(title.into())
    }

    /// Set the group box title at runtime.
    pub fn set_title(&self, title: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_title = title);
        unsafe { ffi::QGroupBox_setTitle(self.ptr, &c_title); }
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QGroupBox) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true }
    }
}

impl AsWidget for GroupBox {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QGroupBox(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for GroupBox {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if !self.has_parent {
            unsafe { ffi::QGroupBox_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`GroupBox`].
pub struct Builder {
    title: String,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new(title: String) -> Self {
        Self { title, parent: None }
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QGroupBox` and return the Rust wrapper.
    pub fn build(self) -> GroupBox {
        let_cxx_string!(c_title = &self.title);
        let ptr = unsafe {
            ffi::QGroupBox_new(&c_title, self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        GroupBox {
            ptr,
            has_parent: self.parent.is_some(),
        }
    }
}
