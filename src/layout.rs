//! Layout managers for arranging widgets.
//!
//! Two layout types are provided:
//! - [`VBoxLayout`] — vertical stacking (top-to-bottom).
//! - [`HBoxLayout`] — horizontal arrangement (left-to-right).
//!
//! Both wrap Qt's
//! [`QVBoxLayout`](https://doc.qt.io/qt-6/qvboxlayout.html) and
//! [`QHBoxLayout`](https://doc.qt.io/qt-6/qhboxlayout.html).

use crate::ffi;
use crate::widget::AsWidget;

/// Trait for layout types that can be installed on a [`Widget`].
///
/// [`Widget`]: crate::Widget
pub trait AsLayout {
    /// Return the underlying `QLayout*` pointer.
    fn layout_ptr(&self) -> *mut ffi::QLayout;
}

// ============================================================
// VBoxLayout
// ============================================================

/// A vertical box layout.
///
/// Widgets are stacked top-to-bottom in the order they are added via
/// [`add_widget`](Self::add_widget).
///
/// # Ownership
///
/// `VBoxLayout` takes ownership of child widgets. When the layout is
/// dropped:
/// 1. All children are dropped first (their `Drop` sees `has_parent=true`
///    and skips C++ deletion — Qt handles it).
/// 2. Then the C++ layout is deleted.
///
/// # Memory safety
///
/// The layout pointer must outlive any `set_vlayout()` call that
/// references it. The normal pattern (create layout, install on
/// window, drop layout at end of scope) is safe.
///
/// # Example
///
/// ```no_run
/// use qtrs::{VBoxLayout, PushButton};
///
/// let mut layout = VBoxLayout::new();
/// let btn = PushButton::new("OK").build();
/// layout.add_widget(Box::new(btn));
/// ```
pub struct VBoxLayout {
    ptr: *mut ffi::QVBoxLayout,
    children: Vec<Box<dyn AsWidget>>,
}

impl VBoxLayout {
    /// Create a new, empty vertical box layout.
    ///
    /// Use [`Widget::set_vlayout`] to install it on a widget later.
    ///
    /// [`Widget::set_vlayout`]: crate::Widget::set_vlayout
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QVBoxLayout_new(std::ptr::null_mut()) };
        assert!(!ptr.is_null(), "QVBoxLayout_new returned null");
        Self {
            ptr,
            children: Vec::new(),
        }
    }

    /// Create a layout and immediately attach it to `parent`.
    ///
    /// This is equivalent to calling [`new`](Self::new) followed by
    /// [`Widget::set_vlayout`], but does not require a `&mut` reference
    /// to the parent during layout construction.
    ///
    /// [`Widget::set_vlayout`]: crate::Widget::set_vlayout
    pub fn with_parent(parent: &dyn AsWidget) -> Self {
        let ptr = unsafe { ffi::QVBoxLayout_new(parent.widget_ptr()) };
        assert!(!ptr.is_null(), "QVBoxLayout_new returned null");
        Self {
            ptr,
            children: Vec::new(),
        }
    }

    /// Add a widget to this layout, taking ownership of it.
    ///
    /// The widget is moved into the layout — you cannot use it through
    /// the original variable afterwards.
    pub fn add_widget(&mut self, mut widget: Box<dyn AsWidget>) {
        debug_assert!(!self.ptr.is_null(), "VBoxLayout::add_widget on null pointer");
        unsafe {
            ffi::QVBoxLayout_addWidget(self.ptr, widget.widget_ptr());
        }
        widget.set_has_parent(); // Qt parent manages the C++ lifetime
        self.children.push(widget);
    }

    /// Add a widget by value (auto-boxed).  Equivalent to `add_widget(Box::new(w))`.
    ///
    /// ```no_run
    /// layout.add(PushButton::new("OK").build());
    /// ```
    pub fn add<T: AsWidget + 'static>(&mut self, widget: T) {
        self.add_widget(Box::new(widget));
    }

    /// Get the raw `QVBoxLayout*` pointer.
    pub fn layout_ptr(&self) -> *mut ffi::QVBoxLayout {
        self.ptr
    }

    /// Set spacing between items (pixels). Default is platform-dependent (~6px).
    pub fn set_spacing(&self, spacing: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QVBoxLayout_setSpacing(self.ptr, spacing); }
    }

    /// Set margins around the layout in pixels (left, top, right, bottom).
    pub fn set_contents_margins(&self, left: i32, top: i32, right: i32, bottom: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QVBoxLayout_setContentsMargins(self.ptr, left, top, right, bottom); }
    }
}

impl AsLayout for VBoxLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout {
        self.ptr as *mut u8 as *mut ffi::QLayout
    }
}

impl Drop for VBoxLayout {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        // Drop children first — their Drop sees has_parent=true
        // and skips C++ deletion (Qt handles it).
        self.children.clear();
        unsafe { ffi::QVBoxLayout_delete(self.ptr) };
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// HBoxLayout
// ============================================================

/// A horizontal box layout.
///
/// Widgets are arranged left-to-right in the order they are added via
/// [`add_widget`](Self::add_widget).
///
/// See [`VBoxLayout`] for ownership semantics and safety notes.
///
/// # Example
///
/// ```no_run
/// use qtrs::{HBoxLayout, Label, PushButton};
///
/// let mut layout = HBoxLayout::new();
/// layout.add_widget(Box::new(Label::new("Name:").build()));
/// layout.add_widget(Box::new(PushButton::new("Submit").build()));
/// ```
pub struct HBoxLayout {
    ptr: *mut ffi::QHBoxLayout,
    children: Vec<Box<dyn AsWidget>>,
}

impl HBoxLayout {
    /// Create a new, empty horizontal box layout.
    ///
    /// Use [`Widget::set_hlayout`] to install it on a widget later.
    ///
    /// [`Widget::set_hlayout`]: crate::Widget::set_hlayout
    pub fn new() -> Self {
        let ptr = unsafe { ffi::QHBoxLayout_new(std::ptr::null_mut()) };
        assert!(!ptr.is_null(), "QHBoxLayout_new returned null");
        Self {
            ptr,
            children: Vec::new(),
        }
    }

    /// Create a layout and immediately attach it to `parent`.
    pub fn with_parent(parent: &dyn AsWidget) -> Self {
        let ptr = unsafe { ffi::QHBoxLayout_new(parent.widget_ptr()) };
        assert!(!ptr.is_null(), "QHBoxLayout_new returned null");
        Self {
            ptr,
            children: Vec::new(),
        }
    }

    /// Add a widget to this layout, taking ownership of it.
    ///
    /// See [`VBoxLayout::add_widget`] for ownership semantics.
    pub fn add_widget(&mut self, mut widget: Box<dyn AsWidget>) {
        debug_assert!(!self.ptr.is_null(), "HBoxLayout::add_widget on null pointer");
        unsafe {
            ffi::QHBoxLayout_addWidget(self.ptr, widget.widget_ptr());
        }
        widget.set_has_parent();
        self.children.push(widget);
    }

    /// Add a widget by value (auto-boxed).
    pub fn add<T: AsWidget + 'static>(&mut self, widget: T) {
        self.add_widget(Box::new(widget));
    }

    /// Get the raw `QHBoxLayout*` pointer.
    pub fn layout_ptr(&self) -> *mut ffi::QHBoxLayout {
        self.ptr
    }

    /// Set spacing between items (pixels).
    pub fn set_spacing(&self, spacing: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QHBoxLayout_setSpacing(self.ptr, spacing); }
    }

    /// Set margins around the layout in pixels.
    pub fn set_contents_margins(&self, left: i32, top: i32, right: i32, bottom: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QHBoxLayout_setContentsMargins(self.ptr, left, top, right, bottom); }
    }
}

impl AsLayout for HBoxLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout {
        self.ptr as *mut u8 as *mut ffi::QLayout
    }
}

impl Drop for HBoxLayout {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        self.children.clear();
        unsafe { ffi::QHBoxLayout_delete(self.ptr) };
        self.ptr = std::ptr::null_mut();
    }
}
