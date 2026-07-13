//! Scroll area for providing scrollable views of widgets.
//!
//! Wraps [`QScrollArea`](https://doc.qt.io/qt-6/qscrollarea.html).

use crate::ffi;
use crate::signal::SignalHandle;
use crate::widget::AsWidget;

/// Scroll bar policy constants matching Qt's `Qt::ScrollBarPolicy`.
pub const SCROLL_BAR_ALWAYS_ON: i32 = 1;
pub const SCROLL_BAR_ALWAYS_OFF: i32 = 0;
pub const SCROLL_BAR_AS_NEEDED: i32 = 2;

/// A scroll area that provides a scrollable viewport for its child widget.
///
/// Use a **builder pattern**: [`ScrollArea::new`] returns a [`Builder`].
///
/// # Example
///
/// ```no_run
/// use qtrs::{ScrollArea, Label};
///
/// let label = Label::new("Hello").build();
/// let scroll = ScrollArea::new()
///     .set_widget_resizable(true)
///     .build();
/// scroll.set_widget(&label);
/// ```
pub struct ScrollArea {
    ptr: *mut ffi::QScrollArea,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl ScrollArea {
    /// Start building a new scroll area.
    pub fn new() -> Builder { Builder::new() }

    /// Set the scroll area's child widget.
    pub fn set_widget(&self, w: &dyn AsWidget) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollArea_setWidget(self.ptr, w.widget_ptr()); }
    }

    /// Set whether the scroll area resizes its child widget.
    pub fn set_widget_resizable(&self, resizable: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollArea_setWidgetResizable(self.ptr, resizable); }
    }

    /// Set the horizontal scroll bar policy.
    ///
    /// Use `SCROLL_BAR_ALWAYS_ON`, `SCROLL_BAR_ALWAYS_OFF`, or `SCROLL_BAR_AS_NEEDED`.
    pub fn set_horizontal_scroll_bar_policy(&self, policy: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollArea_setHorizontalScrollBarPolicy(self.ptr, policy); }
    }

    /// Set the vertical scroll bar policy.
    ///
    /// Use `SCROLL_BAR_ALWAYS_ON`, `SCROLL_BAR_ALWAYS_OFF`, or `SCROLL_BAR_AS_NEEDED`.
    pub fn set_vertical_scroll_bar_policy(&self, policy: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollArea_setVerticalScrollBarPolicy(self.ptr, policy); }
    }

    /// Scroll the contents so that the point (x, y) is visible.
    pub fn ensure_visible(&self, x: i32, y: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollArea_ensureVisible(self.ptr, x, y); }
    }

    /// Scroll the contents so that the given widget is visible.
    pub fn ensure_widget_visible(&self, w: &dyn AsWidget) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QScrollArea_ensureWidgetVisible(self.ptr, w.widget_ptr()); }
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QScrollArea) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for ScrollArea {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QScrollArea(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for ScrollArea {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.signal_handles.clear();
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QScrollArea_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`ScrollArea`].
pub struct Builder {
    widget_resizable: Option<bool>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self { widget_resizable: None, parent: None }
    }

    /// Set whether the scroll area resizes its child widget.
    pub fn set_widget_resizable(mut self, resizable: bool) -> Self {
        self.widget_resizable = Some(resizable);
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QScrollArea` and return the Rust wrapper.
    pub fn build(self) -> ScrollArea {
        let ptr = unsafe {
            ffi::QScrollArea_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let sa = ScrollArea {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(resizable) = self.widget_resizable {
            unsafe { ffi::QScrollArea_setWidgetResizable(ptr, resizable); }
        }
        sa
    }
}
