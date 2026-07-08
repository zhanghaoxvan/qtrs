//! Tab widget for multi-page interfaces.
//!
//! Wraps [`QTabWidget`](https://doc.qt.io/qt-6/qtabwidget.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A tab widget that displays pages with clickable tabs.
///
/// `TabWidget` uses a builder pattern: call [`TabWidget::new`] to obtain
/// a [`Builder`], then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_current_changed`] | `QTabWidget::currentChanged` | `i32` (new tab index) |
///
/// # Example
///
/// ```no_run
/// use qtrs::{TabWidget, Label, Widget};
///
/// let mut tabs = TabWidget::new()
///     .on_current_changed(|index| println!("Tab changed to {}", index))
///     .build();
///
/// let page1 = Widget::new().title("Page 1").build();
/// let page2 = Widget::new().title("Page 2").build();
///
/// tabs.add_tab(Box::new(page1), "Tab 1");
/// tabs.add_tab(Box::new(page2), "Tab 2");
/// ```
pub struct TabWidget {
    ptr: *mut ffi::QTabWidget,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
    pages: Vec<Box<dyn AsWidget>>,
}

impl TabWidget {
    /// Start building a new tab widget.
    pub fn new() -> Builder {
        Builder::new()
    }

    /// Add a tab with the given page widget and label.
    ///
    /// The page widget is moved into the tab widget and owned by it.
    pub fn add_tab(&mut self, page: Box<dyn AsWidget>, label: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_label = label);
        unsafe { ffi::QTabWidget_addTab(self.ptr, page.widget_ptr(), &c_label); }
        let mut page = page;
        page.set_has_parent();
        self.pages.push(page);
    }

    /// Get the index of the currently selected tab.
    pub fn current_index(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTabWidget_currentIndex(self.ptr) }
    }

    /// Set the currently selected tab by index.
    pub fn set_current_index(&self, index: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTabWidget_setCurrentIndex(self.ptr, index); }
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QTabWidget) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new(), pages: Vec::new() }
    }
}

impl AsWidget for TabWidget {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QTabWidget(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for TabWidget {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.pages.clear();
            self.signal_handles.clear();
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            self.pages.clear();
            unsafe { ffi::QTabWidget_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`TabWidget`].
pub struct Builder {
    on_current_changed: Option<Box<dyn Fn(i32)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self { on_current_changed: None, parent: None }
    }

    /// Called when the selected tab changes.
    ///
    /// The callback receives the new tab index (`i32`).
    pub fn on_current_changed<F: Fn(i32) + 'static>(mut self, f: F) -> Self {
        self.on_current_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QTabWidget` and return the Rust wrapper.
    pub fn build(self) -> TabWidget {
        let ptr = unsafe {
            ffi::QTabWidget_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut tw = TabWidget {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
            pages: Vec::new(),
        };
        if let Some(f) = self.on_current_changed {
            let h = signal::leak_int(f);
            unsafe { ffi::QTabWidget_onCurrentChanged(ptr, h.token); }
            tw.signal_handles.push(h);
        }
        tw
    }
}
