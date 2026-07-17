//! Main application window with menu bar, toolbars, and status bar.
//!
//! Wraps [`QMainWindow`](https://doc.qt.io/qt-6/qmainwindow.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::menu::MenuBar;
use crate::signal::SignalHandle;
use crate::widget::AsWidget;

/// A main application window.
///
/// `MainWindow` uses a **builder pattern**: call [`MainWindow::new`] to
/// obtain a [`Builder`], chain configuration, then call `.build()`.
///
/// # Example
///
/// ```no_run
/// use qtrs::{MainWindow, PushButton};
///
/// let mut window = MainWindow::new()
///     .window_title("My App")
///     .size(800, 600)
///     .build();
/// window.show();
/// ```
pub struct MainWindow {
    ptr: *mut ffi::QMainWindow,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl MainWindow {
    /// Start building a new main window.
    pub fn new() -> Builder { Builder::new() }

    // --- Widget management ---

    /// Set the menu bar.
    pub fn set_menu_bar(&self, menubar: &MenuBar) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMainWindow_setMenuBar(self.ptr, menubar.menubar_ptr()); }
    }

    /// Set the central widget.
    pub fn set_central_widget(&self, w: &dyn AsWidget) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMainWindow_setCentralWidget(self.ptr, w.widget_ptr()); }
    }

    /// Set the status bar.
    pub fn set_status_bar(&self, bar: &StatusBar) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMainWindow_setStatusBar(self.ptr, bar.statusbar_ptr()); }
    }

    /// Add a toolbar with the given title. The toolbar is owned by the window.
    pub fn add_tool_bar(&self, title: &str) -> ToolBar {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_title = title);
        let ptr = unsafe { ffi::QMainWindow_addToolBar(self.ptr, &c_title) };
        debug_assert!(!ptr.is_null());
        ToolBar::from_raw(ptr)
    }

    /// Insert a toolbar break.
    pub fn add_tool_bar_break(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMainWindow_addToolBarBreak(self.ptr); }
    }

    // --- Window management ---

    /// Set the window title.
    pub fn set_window_title(&self, title: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_title = title);
        unsafe { ffi::QMainWindow_setWindowTitle(self.ptr, &c_title); }
    }

    /// Resize the window.
    pub fn resize(&self, width: i32, height: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMainWindow_resize(self.ptr, width, height); }
    }

    /// Show the window.
    pub fn show(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMainWindow_show(self.ptr); }
    }

    /// Hide the window.
    pub fn hide(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMainWindow_hide(self.ptr); }
    }

    // --- Dock options ---

    /// Set dock options.
    pub fn set_dock_options(&self, options: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMainWindow_setDockOptions(self.ptr, options); }
    }

    /// Set the tab position for dock widget areas.
    pub fn set_tab_position(&self, areas: i32, tab_pos: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QMainWindow_setTabPosition(self.ptr, areas, tab_pos); }
    }

    /// Wrap an existing `QMainWindow*` obtained via `findChild`.
    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QMainWindow, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for MainWindow {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QMainWindow(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for MainWindow {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QMainWindow_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`MainWindow`].
pub struct Builder {
    window_title: Option<String>,
    size: Option<(i32, i32)>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self { window_title: None, size: None, parent: None }
    }

    /// Set the window title.
    pub fn window_title(mut self, title: impl Into<String>) -> Self {
        self.window_title = Some(title.into());
        self
    }

    /// Set the window size.
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.size = Some((width, height));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QMainWindow` and return the Rust wrapper.
    pub fn build(self) -> MainWindow {
        let ptr = unsafe {
            ffi::QMainWindow_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mw = MainWindow {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(title) = &self.window_title {
            mw.set_window_title(title);
        }
        if let Some((w, h)) = self.size {
            mw.resize(w, h);
        }
        mw
    }
}

// Forward-declare StatusBar and ToolBar so we don't get circular deps
// through mainwindow → statusbar/toolbar. We use them in set_status_bar
// and add_tool_bar. The actual definitions are in statusbar.rs / toolbar.rs.
use crate::statusbar::StatusBar;
use crate::toolbar::ToolBar;
