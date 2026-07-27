//! Tree view for model-based hierarchical data display.
//!
//! Wraps [`QTreeView`](https://doc.qt.io/qt-6/qtreeview.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A tree view that displays hierarchical data from a
/// [`StandardItemModel`](crate::StandardItemModel).
///
/// Unlike [`TreeWidget`](crate::TreeWidget) (which manages its own items),
/// `TreeView` requires an external model.
///
/// # Signals
///
/// | Method | Qt signal |
/// |---|---|
/// | [`Builder::on_clicked`] / [`connect_clicked`](Self::connect_clicked) | `clicked` |
/// | [`Builder::on_double_clicked`] / [`connect_double_clicked`](Self::connect_double_clicked) | `doubleClicked` |
/// | [`Builder::on_expanded`] / [`connect_expanded`](Self::connect_expanded) | `expanded` |
/// | [`Builder::on_collapsed`] / [`connect_collapsed`](Self::connect_collapsed) | `collapsed` |
///
/// # Example
///
/// ```no_run
/// # use qtrs::prelude::*;
/// let model = StandardItemModel::new().build();
///
/// let tree = TreeView::new()
///     .model(&model)
///     .build();
/// ```
pub struct TreeView {
    ptr: *mut ffi::QTreeView,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl TreeView {
    /// Start building a new tree view.
    pub fn new() -> Builder { Builder::new() }

    /// Set the model for this view.
    pub fn set_model(&self, model: &crate::StandardItemModel) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeView_setModel(self.ptr, model.raw_ptr()); }
    }

    /// Get a pointer to the current model.
    #[doc(hidden)]
    pub fn model_ptr(&self) -> *mut ffi::QStandardItemModel {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeView_model(self.ptr) }
    }

    /// Set the selection mode.
    pub fn set_selection_mode(&self, mode: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeView_setSelectionMode(self.ptr, mode); }
    }

    /// Show or hide the header.
    pub fn set_header_hidden(&self, hidden: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeView_setHeaderHidden(self.ptr, hidden); }
    }

    /// Enable animated expand/collapse transitions.
    pub fn set_animated(&self, animated: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeView_setAnimated(self.ptr, animated); }
    }

    /// Set the indentation width for tree levels (pixels).
    pub fn set_indentation(&self, indent: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeView_setIndentation(self.ptr, indent); }
    }

    /// Show or hide the decoration (expand arrow) on root items.
    pub fn set_root_is_decorated(&self, decorated: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeView_setRootIsDecorated(self.ptr, decorated); }
    }

    /// Set whether items are expandable (whether expand arrows are shown).
    pub fn set_items_expandable(&self, expandable: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeView_setItemsExpandable(self.ptr, expandable); }
    }

    /// Expand all items.
    pub fn expand_all(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeView_expandAll(self.ptr); }
    }

    /// Collapse all items.
    pub fn collapse_all(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeView_collapseAll(self.ptr); }
    }

    // --- Signal connections ---

    /// Connect a callback that fires when an item is clicked.
    pub fn connect_clicked<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QTreeView_onClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when an item is double-clicked.
    pub fn connect_double_clicked<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QTreeView_onDoubleClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when an item is expanded.
    pub fn connect_expanded<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QTreeView_onExpanded(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when an item is collapsed.
    pub fn connect_collapsed<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QTreeView_onCollapsed(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QTreeView) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for TreeView {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QTreeView(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for TreeView {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QTreeView_delete(self.ptr); }
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`TreeView`].
pub struct Builder {
    model: Option<*mut ffi::QStandardItemModel>,
    header_hidden: Option<bool>,
    animated: Option<bool>,
    indentation: Option<i32>,
    on_clicked: Option<Box<dyn Fn()>>,
    on_double_clicked: Option<Box<dyn Fn()>>,
    on_expanded: Option<Box<dyn Fn()>>,
    on_collapsed: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            model: None,
            header_hidden: None,
            animated: None,
            indentation: None,
            on_clicked: None,
            on_double_clicked: None,
            on_expanded: None,
            on_collapsed: None,
            parent: None,
        }
    }

    /// Set the data model.
    pub fn model(mut self, model: &crate::StandardItemModel) -> Self {
        self.model = Some(model.raw_ptr());
        self
    }

    /// Hide the header row.
    pub fn header_hidden(mut self, hidden: bool) -> Self {
        self.header_hidden = Some(hidden);
        self
    }

    /// Enable animated transitions.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = Some(animated);
        self
    }

    /// Set the indentation width for tree levels.
    pub fn indentation(mut self, indent: i32) -> Self {
        self.indentation = Some(indent);
        self
    }

    /// Called when an item is clicked.
    pub fn on_clicked<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_clicked = Some(Box::new(f));
        self
    }

    /// Called when an item is double-clicked.
    pub fn on_double_clicked<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_double_clicked = Some(Box::new(f));
        self
    }

    /// Called when an item is expanded.
    pub fn on_expanded<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_expanded = Some(Box::new(f));
        self
    }

    /// Called when an item is collapsed.
    pub fn on_collapsed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_collapsed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the `QTreeView` and return the Rust wrapper.
    pub fn build(self) -> TreeView {
        let ptr = unsafe {
            ffi::QTreeView_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        assert!(!ptr.is_null(), "QTreeView_new returned null");
        let mut view = TreeView {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(model_ptr) = self.model {
            unsafe { ffi::QTreeView_setModel(ptr, model_ptr); }
        }
        if let Some(hidden) = self.header_hidden {
            unsafe { ffi::QTreeView_setHeaderHidden(ptr, hidden); }
        }
        if let Some(animated) = self.animated {
            unsafe { ffi::QTreeView_setAnimated(ptr, animated); }
        }
        if let Some(indent) = self.indentation {
            unsafe { ffi::QTreeView_setIndentation(ptr, indent); }
        }
        if let Some(f) = self.on_clicked {
            let h = signal::leak_void(f);
            unsafe { ffi::QTreeView_onClicked(ptr, h.token); }
            view.signal_handles.push(h);
        }
        if let Some(f) = self.on_double_clicked {
            let h = signal::leak_void(f);
            unsafe { ffi::QTreeView_onDoubleClicked(ptr, h.token); }
            view.signal_handles.push(h);
        }
        if let Some(f) = self.on_expanded {
            let h = signal::leak_void(f);
            unsafe { ffi::QTreeView_onExpanded(ptr, h.token); }
            view.signal_handles.push(h);
        }
        if let Some(f) = self.on_collapsed {
            let h = signal::leak_void(f);
            unsafe { ffi::QTreeView_onCollapsed(ptr, h.token); }
            view.signal_handles.push(h);
        }
        view
    }
}
