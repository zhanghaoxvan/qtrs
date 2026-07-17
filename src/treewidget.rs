//! Tree widget for displaying hierarchical data.
//!
//! Wraps [`QTreeWidget`](https://doc.qt.io/qt-6/qtreewidget.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A tree widget for displaying hierarchical items.
///
/// Use a **builder pattern**: [`TreeWidget::new`] returns a [`Builder`].
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_item_clicked`] | `QTreeWidget::itemClicked` | `String` (item text) |
/// | [`Builder::on_item_double_clicked`] | `QTreeWidget::itemDoubleClicked` | `String` (item text) |
/// | [`Builder::on_item_expanded`] | `QTreeWidget::itemExpanded` | `String` (item text) |
/// | [`Builder::on_item_collapsed`] | `QTreeWidget::itemCollapsed` | `String` (item text) |
/// | [`Builder::on_current_item_changed`] | `QTreeWidget::currentItemChanged` | `String` (item text) |
///
/// # Example
///
/// ```no_run
/// use qtrs::TreeWidget;
///
/// let tree = TreeWidget::new()
///     .on_item_clicked(|text| println!("clicked: {}", text))
///     .build();
///
/// tree.add_top_level_item("Root");
/// tree.set_header_label("Name");
/// ```
pub struct TreeWidget {
    ptr: *mut ffi::QTreeWidget,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl TreeWidget {
    /// Start building a new tree widget.
    pub fn new() -> Builder { Builder::new() }

    // --- Item management ---

    /// Add a top-level item with the given text.
    pub fn add_top_level_item(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QTreeWidget_addTopLevelItem(self.ptr, &c_text); }
    }

    /// Remove all items.
    pub fn clear(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeWidget_clear(self.ptr); }
    }

    // --- Getters ---

    /// Get the text of the currently selected item.
    pub fn current_item_text(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeWidget_currentItemText(self.ptr) }
    }

    // --- Header ---

    /// Set the header label (single column).
    pub fn set_header_label(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QTreeWidget_setHeaderLabel(self.ptr, &c_text); }
    }

    /// Set multiple header labels.
    pub fn set_header_labels(&self, labels: &[&str]) {
        debug_assert!(!self.ptr.is_null());
        let vec: Vec<String> = labels.iter().map(|s| s.to_string()).collect();
        unsafe { ffi::QTreeWidget_setHeaderLabels(self.ptr, vec); }
    }

    // --- Expand/Collapse ---

    /// Expand all items.
    pub fn expand_all(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeWidget_expandAll(self.ptr); }
    }

    /// Collapse all items.
    pub fn collapse_all(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeWidget_collapseAll(self.ptr); }
    }

    /// Expand the item with the given text.
    pub fn expand_item(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QTreeWidget_expandItem(self.ptr, &c_text); }
    }

    // --- Current item ---

    /// Set the current item by text.
    pub fn set_current_item(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QTreeWidget_setCurrentItem(self.ptr, &c_text); }
    }

    /// Get the number of top-level items.
    pub fn top_level_item_count(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTreeWidget_topLevelItemCount(self.ptr) }
    }

    // --- Signal connections (runtime) ---

    /// Connect callback when an item is clicked. Receives the item text.
    pub fn connect_item_clicked<F: Fn(String) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QTreeWidget_onItemClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect callback when an item is double-clicked. Receives the item text.
    pub fn connect_item_double_clicked<F: Fn(String) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QTreeWidget_onItemDoubleClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect callback when an item is expanded. Receives the item text.
    pub fn connect_item_expanded<F: Fn(String) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QTreeWidget_onItemExpanded(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect callback when an item is collapsed. Receives the item text.
    pub fn connect_item_collapsed<F: Fn(String) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QTreeWidget_onItemCollapsed(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect callback when the current item changes. Receives the item text.
    pub fn connect_current_item_changed<F: Fn(String) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QTreeWidget_onCurrentItemChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QTreeWidget) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for TreeWidget {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QTreeWidget(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for TreeWidget {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QTreeWidget_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`TreeWidget`].
pub struct Builder {
    on_item_clicked: Option<Box<dyn Fn(String)>>,
    on_item_double_clicked: Option<Box<dyn Fn(String)>>,
    on_item_expanded: Option<Box<dyn Fn(String)>>,
    on_item_collapsed: Option<Box<dyn Fn(String)>>,
    on_current_item_changed: Option<Box<dyn Fn(String)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            on_item_clicked: None,
            on_item_double_clicked: None,
            on_item_expanded: None,
            on_item_collapsed: None,
            on_current_item_changed: None,
            parent: None,
        }
    }

    /// Called when an item is clicked. Receives the item text.
    pub fn on_item_clicked<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_item_clicked = Some(Box::new(f));
        self
    }

    /// Called when an item is double-clicked. Receives the item text.
    pub fn on_item_double_clicked<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_item_double_clicked = Some(Box::new(f));
        self
    }

    /// Called when an item is expanded. Receives the item text.
    pub fn on_item_expanded<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_item_expanded = Some(Box::new(f));
        self
    }

    /// Called when an item is collapsed. Receives the item text.
    pub fn on_item_collapsed<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_item_collapsed = Some(Box::new(f));
        self
    }

    /// Called when the current item changes. Receives the item text.
    pub fn on_current_item_changed<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_current_item_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QTreeWidget` and return the Rust wrapper.
    pub fn build(self) -> TreeWidget {
        let ptr = unsafe {
            ffi::QTreeWidget_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut tw = TreeWidget {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(f) = self.on_item_clicked {
            let h = signal::leak_string(f);
            unsafe { ffi::QTreeWidget_onItemClicked(ptr, h.token); }
            tw.signal_handles.push(h);
        }
        if let Some(f) = self.on_item_double_clicked {
            let h = signal::leak_string(f);
            unsafe { ffi::QTreeWidget_onItemDoubleClicked(ptr, h.token); }
            tw.signal_handles.push(h);
        }
        if let Some(f) = self.on_item_expanded {
            let h = signal::leak_string(f);
            unsafe { ffi::QTreeWidget_onItemExpanded(ptr, h.token); }
            tw.signal_handles.push(h);
        }
        if let Some(f) = self.on_item_collapsed {
            let h = signal::leak_string(f);
            unsafe { ffi::QTreeWidget_onItemCollapsed(ptr, h.token); }
            tw.signal_handles.push(h);
        }
        if let Some(f) = self.on_current_item_changed {
            let h = signal::leak_string(f);
            unsafe { ffi::QTreeWidget_onCurrentItemChanged(ptr, h.token); }
            tw.signal_handles.push(h);
        }
        tw
    }
}
