//! List widget for displaying scrollable lists of items.
//!
//! Wraps [`QListWidget`](https://doc.qt.io/qt-6/qlistwidget.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// Selection mode constants matching Qt's `QAbstractItemView::SelectionMode`.
pub const SINGLE_SELECTION: i32 = 0;
pub const MULTI_SELECTION: i32 = 1;
pub const EXTENDED_SELECTION: i32 = 2;
pub const CONTIGUOUS_SELECTION: i32 = 3;
pub const NO_SELECTION: i32 = 4;

/// A scrollable list of text items.
///
/// Use a **builder pattern**: [`ListWidget::new`] returns a [`Builder`].
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_item_clicked`] | `QListWidget::itemClicked` | `String` (item text) |
/// | [`Builder::on_item_double_clicked`] | `QListWidget::itemDoubleClicked` | `String` (item text) |
/// | [`Builder::on_current_item_changed`] | `QListWidget::currentItemChanged` | `String` (item text) |
///
/// # Example
///
/// ```no_run
/// use qtrs::ListWidget;
///
/// let list = ListWidget::new()
///     .items(&["One", "Two", "Three"])
///     .on_item_clicked(|text| println!("clicked: {}", text))
///     .build();
/// ```
pub struct ListWidget {
    ptr: *mut ffi::QListWidget,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl ListWidget {
    /// Start building a new list widget.
    pub fn new() -> Builder { Builder::new() }

    // --- Item management ---

    /// Add a single item to the end of the list.
    pub fn add_item(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QListWidget_addItem(self.ptr, &c_text); }
    }

    /// Add multiple items to the end of the list.
    pub fn add_items(&self, items: &[&str]) {
        debug_assert!(!self.ptr.is_null());
        for text in items {
            let_cxx_string!(c_text = text);
            unsafe { ffi::QListWidget_addItem(self.ptr, &c_text); }
        }
    }

    /// Insert an item at the given row.
    pub fn insert_item(&self, row: i32, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QListWidget_insertItem(self.ptr, row, &c_text); }
    }

    /// Remove all items.
    pub fn clear(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListWidget_clear(self.ptr); }
    }

    /// Remove the item at the given row.
    pub fn remove_item(&self, row: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListWidget_removeItem(self.ptr, row); }
    }

    // --- Getters ---

    /// Number of items in the list.
    pub fn count(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListWidget_count(self.ptr) }
    }

    /// Get the text of the item at the given row.
    pub fn item_text(&self, row: i32) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListWidget_itemText(self.ptr, row) }
    }

    /// Get the current selected row, or `-1` if nothing is selected.
    pub fn current_row(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListWidget_currentRow(self.ptr) }
    }

    /// Get the text of the currently selected item.
    pub fn current_text(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListWidget_currentText(self.ptr) }
    }

    // --- Selection ---

    /// Set the current selected row.
    pub fn set_current_row(&self, row: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListWidget_setCurrentRow(self.ptr, row); }
    }

    /// Set the selection mode (see `SINGLE_SELECTION`, `MULTI_SELECTION`, etc.).
    pub fn set_selection_mode(&self, mode: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QListWidget_setSelectionMode(self.ptr, mode); }
    }

    // --- Signal connections (runtime) ---

    /// Connect callback when an item is clicked. Receives the item text.
    pub fn connect_item_clicked<F: Fn(String) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QListWidget_onItemClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect callback when an item is double-clicked. Receives the item text.
    pub fn connect_item_double_clicked<F: Fn(String) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QListWidget_onItemDoubleClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect callback when the current item changes. Receives the item text.
    pub fn connect_current_item_changed<F: Fn(String) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QListWidget_onCurrentItemChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QListWidget, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for ListWidget {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QListWidget(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for ListWidget {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QListWidget_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`ListWidget`].
pub struct Builder {
    items: Vec<String>,
    on_item_clicked: Option<Box<dyn Fn(String)>>,
    on_item_double_clicked: Option<Box<dyn Fn(String)>>,
    on_current_item_changed: Option<Box<dyn Fn(String)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            on_item_clicked: None,
            on_item_double_clicked: None,
            on_current_item_changed: None,
            parent: None,
        }
    }

    /// Set the list of items (replaces any previous items).
    pub fn items(mut self, items: &[&str]) -> Self {
        self.items = items.iter().map(|s| s.to_string()).collect();
        self
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

    /// Create the C++ `QListWidget` and return the Rust wrapper.
    pub fn build(self) -> ListWidget {
        let ptr = unsafe {
            ffi::QListWidget_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut lw = ListWidget {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        for item in &self.items {
            let_cxx_string!(c_item = item);
            unsafe { ffi::QListWidget_addItem(ptr, &c_item); }
        }
        if let Some(f) = self.on_item_clicked {
            let h = signal::leak_string(f);
            unsafe { ffi::QListWidget_onItemClicked(ptr, h.token); }
            lw.signal_handles.push(h);
        }
        if let Some(f) = self.on_item_double_clicked {
            let h = signal::leak_string(f);
            unsafe { ffi::QListWidget_onItemDoubleClicked(ptr, h.token); }
            lw.signal_handles.push(h);
        }
        if let Some(f) = self.on_current_item_changed {
            let h = signal::leak_string(f);
            unsafe { ffi::QListWidget_onCurrentItemChanged(ptr, h.token); }
            lw.signal_handles.push(h);
        }
        lw
    }
}
