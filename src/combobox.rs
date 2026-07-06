//! Drop-down combo box widget.
//!
//! Wraps [`QComboBox`](https://doc.qt.io/qt-6/qcombobox.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A drop-down selection list.
///
/// Use a **builder pattern**: [`ComboBox::new`] returns a [`Builder`].
///
/// # Signals
///
/// | Method | Qt signal | Note |
/// |---|---|---|
/// | [`Builder::on_current_text_changed`] | `QComboBox::currentTextChanged` | Void — call `current_text()` to read value |
///
/// # Example
///
/// ```ignore
/// let combo = ComboBox::new()
///     .items(&["Red", "Green", "Blue"])
///     .on_current_text_changed(|| println!("selection changed"))
///     .build();
/// ```
pub struct ComboBox {
    ptr: *mut ffi::QComboBox,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl ComboBox {
    pub fn new() -> Builder { Builder::new() }

    /// Add an item to the drop-down list.
    pub fn add_item(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QComboBox_addItem(self.ptr, &c_text); }
    }

    /// Get the currently selected text.
    pub fn current_text(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QComboBox_currentText(self.ptr) }
    }

    /// Set the selected index.
    pub fn set_current_index(&self, index: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QComboBox_setCurrentIndex(self.ptr, index); }
    }

    /// Connect a callback that fires when the selected index changes.
    /// The callback receives the new index as `i32`.
    pub fn connect_current_index_changed<F: Fn(i32) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_int(f);
        unsafe { ffi::QComboBox_onCurrentIndexChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QComboBox, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for ComboBox {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QComboBox(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for ComboBox {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.signal_handles.clear();
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QComboBox_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

pub struct Builder {
    items: Vec<String>,
    on_current_text_changed: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self { Self { items: Vec::new(), on_current_text_changed: None, parent: None } }

    /// Set the list of items (replaces any previous items).
    pub fn items(mut self, items: &[&str]) -> Self {
        self.items = items.iter().map(|s| s.to_string()).collect(); self
    }

    /// Called when the selected item changes (void — call `current_text()` to read).
    pub fn on_current_text_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_current_text_changed = Some(Box::new(f)); self
    }

    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr()); self
    }

    pub fn build(self) -> ComboBox {
        let ptr = unsafe {
            ffi::QComboBox_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut cb = ComboBox { ptr, has_parent: self.parent.is_some(), signal_handles: Vec::new() };
        for item in &self.items {
            let_cxx_string!(c_item = item);
            unsafe { ffi::QComboBox_addItem(ptr, &c_item); }
        }
        if let Some(f) = self.on_current_text_changed {
            let h = signal::leak_void(f);
            unsafe { ffi::QComboBox_onCurrentTextChanged(ptr, h.token); }
            cb.signal_handles.push(h);
        }
        cb
    }
}
