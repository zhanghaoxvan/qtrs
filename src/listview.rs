//! List view for model-based data display.
//!
//! Wraps [`QListView`](https://doc.qt.io/qt-6/qlistview.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// View modes for [`ListView`].
///
/// Mirrors Qt's `QListView::ViewMode`.
pub const LIST_MODE: i32 = 0;
pub const ICON_MODE: i32 = 1;

/// A list view that displays data from a [`StandardItemModel`](crate::StandardItemModel).
///
/// Unlike [`ListWidget`](crate::ListWidget) (which manages its own items),
/// `ListView` requires an external model.
///
/// # Signals
///
/// | Method | Qt signal |
/// |---|---|
/// | [`Builder::on_clicked`] / [`connect_clicked`](Self::connect_clicked) | `clicked` |
/// | [`Builder::on_double_clicked`] / [`connect_double_clicked`](Self::connect_double_clicked) | `doubleClicked` |
///
/// # Example
///
/// ```no_run
/// # use qtrs::prelude::*;
/// let model = StandardItemModel::new().build();
/// model.append_row(&["Item 1"]);
/// model.append_row(&["Item 2"]);
///
/// let list = ListView::new()
///     .model(&model)
///     .build();
/// ```
pub struct ListView {
    ptr: *mut ffi::QListView,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl ListView {
    /// Start building a new list view.
    pub fn new() -> Builder { Builder::new() }

    /// Set the model for this view.
    pub fn set_model(&self, model: &crate::StandardItemModel) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListView_setModel(self.ptr, model.raw_ptr()); }
    }

    /// Get a pointer to the current model.
    #[doc(hidden)]
    pub fn model_ptr(&self) -> *mut ffi::QStandardItemModel {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListView_model(self.ptr) }
    }

    /// Set the selection mode.
    pub fn set_selection_mode(&self, mode: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListView_setSelectionMode(self.ptr, mode); }
    }

    /// Set the view mode ([`LIST_MODE`] or [`ICON_MODE`]).
    pub fn set_view_mode(&self, mode: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListView_setViewMode(self.ptr, mode); }
    }

    // --- Signal connections ---

    /// Connect a callback that fires when an item is clicked.
    pub fn connect_clicked<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QListView_onClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when an item is double-clicked.
    pub fn connect_double_clicked<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QListView_onDoubleClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QListView) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for ListView {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QListView(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for ListView {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QListView_delete(self.ptr); }
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`ListView`].
pub struct Builder {
    model: Option<*mut ffi::QStandardItemModel>,
    view_mode: Option<i32>,
    on_clicked: Option<Box<dyn Fn()>>,
    on_double_clicked: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            model: None,
            view_mode: None,
            on_clicked: None,
            on_double_clicked: None,
            parent: None,
        }
    }

    /// Set the data model.
    pub fn model(mut self, model: &crate::StandardItemModel) -> Self {
        self.model = Some(model.raw_ptr());
        self
    }

    /// Set the view mode ([`LIST_MODE`] or [`ICON_MODE`]).
    pub fn view_mode(mut self, mode: i32) -> Self {
        self.view_mode = Some(mode);
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

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the `QListView` and return the Rust wrapper.
    pub fn build(self) -> ListView {
        let ptr = unsafe {
            ffi::QListView_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        assert!(!ptr.is_null(), "QListView_new returned null");
        let mut view = ListView {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(model_ptr) = self.model {
            unsafe { ffi::QListView_setModel(ptr, model_ptr); }
        }
        if let Some(mode) = self.view_mode {
            unsafe { ffi::QListView_setViewMode(ptr, mode); }
        }
        if let Some(f) = self.on_clicked {
            let h = signal::leak_void(f);
            unsafe { ffi::QListView_onClicked(ptr, h.token); }
            view.signal_handles.push(h);
        }
        if let Some(f) = self.on_double_clicked {
            let h = signal::leak_void(f);
            unsafe { ffi::QListView_onDoubleClicked(ptr, h.token); }
            view.signal_handles.push(h);
        }
        view
    }
}
