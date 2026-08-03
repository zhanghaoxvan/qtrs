//! Toolbox widget — a column of collapsible tabbed pages.
//!
//! Wraps [`QToolBox`](https://doc.qt.io/qt-6/qtoolbox.html).

use cxx::let_cxx_string;
use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A toolbox with collapsible item pages.
///
/// Items are stacked vertically; selecting an item expands it and collapses
/// the others.
///
/// # Signals
///
/// | Method | Qt signal |
/// |---|---|
/// | [`connect_current_changed`](Self::connect_current_changed) | `currentChanged` |
///
/// # Example
///
/// ```no_run
/// # use qtrs::prelude::*;
/// let mut toolbox = ToolBox::new().build();
///
/// let page1 = Widget::new().build();
/// toolbox.add_item(Box::new(page1), "Page 1");
/// ```
pub struct ToolBox {
    ptr: *mut ffi::QToolBox,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
    pages: Vec<Box<dyn AsWidget>>,
}

impl ToolBox {
    /// Start building a new toolbox.
    pub fn new() -> Builder { Builder::new() }

    /// Add a page with the given label. The widget is moved into the toolbox.
    pub fn add_item(&mut self, widget: Box<dyn AsWidget>, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        let mut widget = widget;
        widget.set_has_parent();
        unsafe {
            ffi::QToolBox_addItem(self.ptr, widget.widget_ptr(), &c_text);
        }
        self.pages.push(widget);
    }

    /// Add a page by value (auto-boxed).
    pub fn add<T: AsWidget + 'static>(&mut self, widget: T, text: &str) {
        self.add_item(Box::new(widget), text);
    }

    /// Insert a page at the given index.
    pub fn insert_item(&mut self, index: i32, widget: Box<dyn AsWidget>, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        let mut widget = widget;
        widget.set_has_parent();
        unsafe {
            ffi::QToolBox_insertItem(self.ptr, index, widget.widget_ptr(), &c_text);
        }
        self.pages.push(widget);
    }

    /// Remove the page at the given index.
    pub fn remove_item(&self, index: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBox_removeItem(self.ptr, index); }
    }

    /// Set the text of the item at the given index.
    pub fn set_item_text(&self, index: i32, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QToolBox_setItemText(self.ptr, index, &c_text); }
    }

    /// Get the text of the item at the given index.
    pub fn item_text(&self, index: i32) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBox_itemText(self.ptr, index) }
    }

    /// Set the icon for the item at the given index.
    pub fn set_item_icon(&self, index: i32, icon_path: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_path = icon_path);
        unsafe { ffi::QToolBox_setItemIcon(self.ptr, index, &c_path); }
    }

    /// Enable or disable the item at the given index.
    pub fn set_item_enabled(&self, index: i32, enabled: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBox_setItemEnabled(self.ptr, index, enabled); }
    }

    /// Return `true` if the item at the given index is enabled.
    pub fn is_item_enabled(&self, index: i32) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBox_isItemEnabled(self.ptr, index) }
    }

    /// Get the index of the currently selected (expanded) item.
    pub fn current_index(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBox_currentIndex(self.ptr) }
    }

    /// Set the currently selected (expanded) item by index.
    pub fn set_current_index(&self, index: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBox_setCurrentIndex(self.ptr, index); }
    }

    /// Get the number of items in the toolbox.
    pub fn count(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBox_count(self.ptr) }
    }

    /// Get a pointer to the widget at the given index.
    #[doc(hidden)]
    pub fn widget_ptr(&self, index: i32) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBox_widget(self.ptr, index) }
    }

    // --- Signal connections ---

    /// Connect a callback that fires when the current item changes.
    pub fn connect_current_changed<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QToolBox_onCurrentChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QToolBox) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new(), pages: Vec::new() }
    }
}

impl AsWidget for ToolBox {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QToolBox(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for ToolBox {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.pages.clear();
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            self.pages.clear();
            unsafe { ffi::QToolBox_delete(self.ptr); }
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`ToolBox`].
pub struct Builder {
    on_current_changed: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self { on_current_changed: None, parent: None }
    }

    /// Called when the current item changes.
    pub fn on_current_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_current_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the `QToolBox` and return the Rust wrapper.
    pub fn build(self) -> ToolBox {
        let ptr = unsafe {
            ffi::QToolBox_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        assert!(!ptr.is_null(), "QToolBox_new returned null");
        let mut tb = ToolBox {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
            pages: Vec::new(),
        };
        if let Some(f) = self.on_current_changed {
            let h = signal::leak_void(f);
            unsafe { ffi::QToolBox_onCurrentChanged(ptr, h.token); }
            tb.signal_handles.push(h);
        }
        tb
    }
}
