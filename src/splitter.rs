//! Splitter widget for resizable horizontal or vertical layouts.
//!
//! Wraps [`QSplitter`](https://doc.qt.io/qt-6/qsplitter.html).

use crate::ffi;
use crate::signal::SignalHandle;
use crate::widget::AsWidget;

/// Orientation constants matching Qt's `Qt::Orientation`.
pub const HORIZONTAL: i32 = 1;
pub const VERTICAL: i32 = 2;

/// A splitter that lays out its children in a row or column, separated
/// by draggable handles.
///
/// Use a **builder pattern**: [`Splitter::new`] returns a [`Builder`].
///
/// # Example
///
/// ```no_run
/// use qtrs::{Splitter, Label};
///
/// let mut splitter = Splitter::new(HORIZONTAL).build();
///
/// let left = Label::new("Left").build();
/// let right = Label::new("Right").build();
///
/// splitter.add_widget(Box::new(left));
/// splitter.add_widget(Box::new(right));
/// ```
pub struct Splitter {
    ptr: *mut ffi::QSplitter,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
    children: Vec<Box<dyn AsWidget>>,
}

impl Splitter {
    /// Start building a new splitter with the given orientation.
    ///
    /// Use `HORIZONTAL` or `VERTICAL`.
    pub fn new(orientation: i32) -> Builder { Builder::new(orientation) }

    /// Add a widget to the splitter's layout.
    ///
    /// The widget is moved into the splitter and owned by it.
    pub fn add_widget(&mut self, w: Box<dyn AsWidget>) {
        debug_assert!(!self.ptr.is_null());
        let mut w = w;
        w.set_has_parent();
        unsafe { ffi::QSplitter_addWidget(self.ptr, w.widget_ptr()); }
        self.children.push(w);
    }

    /// Add a widget by value (auto-boxed).
    pub fn add<T: AsWidget + 'static>(&mut self, widget: T) {
        self.add_widget(Box::new(widget));
    }

    /// Insert a widget at the given index.
    ///
    /// The widget is moved into the splitter and owned by it.
    pub fn insert_widget(&mut self, index: i32, w: Box<dyn AsWidget>) {
        debug_assert!(!self.ptr.is_null());
        let mut w = w;
        w.set_has_parent();
        unsafe { ffi::QSplitter_insertWidget(self.ptr, index, w.widget_ptr()); }
        self.children.push(w);
    }

    /// Set the stretch factor at the given index.
    pub fn set_stretch_factor(&self, index: i32, stretch: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSplitter_setStretchFactor(self.ptr, index, stretch); }
    }

    /// Set the sizes of the splitter's children in pixels.
    pub fn set_sizes(&self, sizes: &[i32]) {
        debug_assert!(!self.ptr.is_null());
        let vec: Vec<i32> = sizes.iter().copied().collect();
        unsafe { ffi::QSplitter_setSizes(self.ptr, vec); }
    }

    /// Get the sizes of the splitter's children in pixels.
    pub fn sizes(&self) -> Vec<i32> {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSplitter_sizes(self.ptr) }
    }

    /// Set the orientation (`HORIZONTAL` or `VERTICAL`).
    pub fn set_orientation(&self, orientation: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSplitter_setOrientation(self.ptr, orientation); }
    }

    /// Get the number of children in the splitter.
    pub fn count(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSplitter_count(self.ptr) }
    }

    /// Set the width of the splitter handle in pixels.
    pub fn set_handle_width(&self, width: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSplitter_setHandleWidth(self.ptr, width); }
    }

    /// Set whether children can be collapsed to zero size.
    pub fn set_children_collapsible(&self, collapsible: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSplitter_setChildrenCollapsible(self.ptr, collapsible); }
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QSplitter) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new(), children: Vec::new() }
    }
}

impl AsWidget for Splitter {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QSplitter(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for Splitter {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.children.clear();
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            self.children.clear();
            unsafe { ffi::QSplitter_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`Splitter`].
pub struct Builder {
    orientation: i32,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new(orientation: i32) -> Self {
        Self { orientation, parent: None }
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QSplitter` and return the Rust wrapper.
    pub fn build(self) -> Splitter {
        let ptr = unsafe {
            ffi::QSplitter_new(
                self.orientation,
                self.parent.unwrap_or(std::ptr::null_mut()),
            )
        };
        debug_assert!(!ptr.is_null());
        Splitter {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
            children: Vec::new(),
        }
    }
}
