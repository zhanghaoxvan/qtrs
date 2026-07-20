//! Stacked widget for multi-page interfaces where only one page is visible at a time.
//!
//! Wraps [`QStackedWidget`](https://doc.qt.io/qt-6/qstackedwidget.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A stacked widget that shows one child widget at a time.
///
/// Use a **builder pattern**: [`StackedWidget::new`] returns a [`Builder`].
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_current_changed`] | `QStackedWidget::currentChanged` | `i32` (new page index) |
///
/// # Example
///
/// ```no_run
/// use qtrs::{StackedWidget, Label};
///
/// let mut stack = StackedWidget::new()
///     .on_current_changed(|index| println!("page {}", index))
///     .build();
///
/// let page1 = Label::new("Page 1").build();
/// let page2 = Label::new("Page 2").build();
///
/// stack.add_widget(Box::new(page1));
/// stack.add_widget(Box::new(page2));
/// stack.set_current_index(1);
/// ```
pub struct StackedWidget {
    ptr: *mut ffi::QStackedWidget,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
    pages: Vec<Box<dyn AsWidget>>,
}

impl StackedWidget {
    /// Start building a new stacked widget.
    pub fn new() -> Builder { Builder::new() }

    /// Add a page to the stacked widget and return its index.
    ///
    /// The page widget is moved into the stacked widget and owned by it.
    pub fn add_widget(&mut self, page: Box<dyn AsWidget>) -> i32 {
        debug_assert!(!self.ptr.is_null());
        let mut page = page;
        page.set_has_parent();
        let index = unsafe { ffi::QStackedWidget_addWidget(self.ptr, page.widget_ptr()) };
        self.pages.push(page);
        index
    }

    /// Insert a page at the given index and return its index.
    ///
    /// The page widget is moved into the stacked widget and owned by it.
    pub fn insert_widget(&mut self, index: i32, page: Box<dyn AsWidget>) -> i32 {
        debug_assert!(!self.ptr.is_null());
        let mut page = page;
        page.set_has_parent();
        let actual_index = unsafe { ffi::QStackedWidget_insertWidget(self.ptr, index, page.widget_ptr()) };
        self.pages.push(page);
        actual_index
    }

    /// Remove a page widget from the stacked widget (does not delete it).
    pub fn remove_widget(&mut self, w: &dyn AsWidget) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStackedWidget_removeWidget(self.ptr, w.widget_ptr()); }
        self.pages.retain(|p| !std::ptr::eq(p.widget_ptr(), w.widget_ptr()));
    }

    /// Set the currently visible page by index.
    pub fn set_current_index(&self, index: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStackedWidget_setCurrentIndex(self.ptr, index); }
    }

    /// Get the index of the currently visible page.
    pub fn current_index(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStackedWidget_currentIndex(self.ptr) }
    }

    /// Get the number of pages.
    pub fn count(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QStackedWidget_count(self.ptr) }
    }

    // --- Signal connections (runtime) ---

    /// Connect a callback that fires when the current page changes.
    /// The callback receives the new page index as `i32`.
    pub fn connect_current_changed<F: Fn(i32)>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_int(f);
        unsafe { ffi::QStackedWidget_onCurrentChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QStackedWidget) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new(), pages: Vec::new() }
    }
}

impl AsWidget for StackedWidget {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QStackedWidget(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for StackedWidget {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.pages.clear();
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            self.pages.clear();
            unsafe { ffi::QStackedWidget_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`StackedWidget`].
pub struct Builder {
    on_current_changed: Option<Box<dyn Fn(i32)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self { on_current_changed: None, parent: None }
    }

    /// Called when the current page changes. Receives the new page index (`i32`).
    pub fn on_current_changed<F: Fn(i32) + 'static>(mut self, f: F) -> Self {
        self.on_current_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QStackedWidget` and return the Rust wrapper.
    pub fn build(self) -> StackedWidget {
        let ptr = unsafe {
            ffi::QStackedWidget_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut sw = StackedWidget {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
            pages: Vec::new(),
        };
        if let Some(f) = self.on_current_changed {
            let h = signal::leak_int(f);
            unsafe { ffi::QStackedWidget_onCurrentChanged(ptr, h.token); }
            sw.signal_handles.push(h);
        }
        sw
    }
}
