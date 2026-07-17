//! Toolbar widget.
//!
//! Wraps [`QToolBar`](https://doc.qt.io/qt-6/qtoolbar.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A toolbar with action buttons and widgets.
///
/// `ToolBar` uses a **builder pattern**: call [`ToolBar::new`] to obtain
/// a [`Builder`], chain configuration, then call `.build()`.
///
/// # Signals
///
/// Actions are added via [`ToolBar::add_action`] with inline callbacks.
/// There are no signal fields on the builder — signals are connected
/// per-action at runtime.
///
/// # Example
///
/// ```no_run
/// use qtrs::ToolBar;
///
/// let toolbar = ToolBar::new("Edit")
///     .build();
/// toolbar.add_action("Undo", || println!("Undo"));
/// toolbar.add_action("Redo", || println!("Redo"));
/// ```
pub struct ToolBar {
    pub(crate) ptr: *mut ffi::QToolBar,
    pub(crate) has_parent: bool,
    pub(crate) signal_handles: Vec<SignalHandle>,
}

impl ToolBar {
    /// Start building a new toolbar.
    pub fn new(title: impl Into<String>) -> Builder {
        Builder::new(title.into())
    }

    // --- Action management ---

    /// Add an action with the given text and callback.
    ///
    /// The callback is a `Fn()` (no arguments) triggered by
    /// `QAction::triggered` (the no-overload, void version).
    pub fn add_action<F: Fn() + 'static>(&mut self, text: &str, f: F) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        let handle = signal::leak_void(f);
        unsafe { ffi::QToolBar_addAction(self.ptr, &c_text, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Add a separator.
    pub fn add_separator(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBar_addSeparator(self.ptr); }
    }

    /// Add a widget to the toolbar.
    pub fn add_widget(&self, w: &dyn AsWidget) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBar_addWidget(self.ptr, w.widget_ptr()); }
    }

    // --- Appearance ---

    /// Set whether the toolbar is movable.
    pub fn set_movable(&self, movable: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBar_setMovable(self.ptr, movable); }
    }

    /// Set whether the toolbar is floatable.
    pub fn set_floatable(&self, floatable: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBar_setFloatable(self.ptr, floatable); }
    }

    /// Set the icon size.
    pub fn set_icon_size(&self, w: i32, h: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBar_setIconSize(self.ptr, w, h); }
    }

    /// Set the allowed areas for the toolbar.
    pub fn set_allowed_areas(&self, areas: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBar_setAllowedAreas(self.ptr, areas); }
    }

    /// Set the tool button style.
    pub fn set_tool_button_style(&self, style: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBar_setToolButtonStyle(self.ptr, style); }
    }

    /// Remove all actions from the toolbar.
    pub fn clear(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolBar_clear(self.ptr); }
    }

    /// Wrap an existing `QToolBar*` obtained via `findChild` or from a main window.
    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QToolBar) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for ToolBar {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QToolBar(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for ToolBar {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QToolBar_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`ToolBar`].
pub struct Builder {
    title: String,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new(title: String) -> Self {
        Self { title, parent: None }
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QToolBar` and return the Rust wrapper.
    pub fn build(self) -> ToolBar {
        let_cxx_string!(c_title = &self.title);
        let ptr = unsafe {
            ffi::QToolBar_new(&c_title, self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        ToolBar {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        }
    }
}
