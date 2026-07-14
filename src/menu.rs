//! Menu and menu bar widgets.
//!
//! Wraps [`QMenu`](https://doc.qt.io/qt-6/qmenu.html) and
//! [`QMenuBar`](https://doc.qt.io/qt-6/qmenubar.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

// ============================================================
// Menu
// ============================================================

/// A drop-down menu containing actions.
///
/// `Menu` uses a builder pattern: call [`Menu::new`] to obtain
/// a [`MenuBuilder`], then call `.build()`.
///
/// # Example
///
/// ```no_run
/// use qtrs::{Menu, MenuBar};
///
/// let mut file_menu = Menu::new("File")
///     .action("New", || println!("New file"))
///     .action("Open", || println!("Open file"))
///     .build();
/// ```
pub struct Menu {
    pub(crate) ptr: *mut ffi::QMenu,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl Menu {
    /// Start building a new menu.
    pub fn new(title: impl Into<String>) -> MenuBuilder {
        MenuBuilder::new(title.into())
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QMenu) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }

    /// Get the raw menu pointer for use with [`MenuBarBuilder::add_menu`].
    pub fn menu_ptr(&self) -> *mut ffi::QMenu {
        self.ptr
    }
}

impl AsWidget for Menu {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QMenu(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for Menu {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
        } else {
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
            unsafe { ffi::QMenu_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// MenuBuilder for [`Menu`].
pub struct MenuBuilder {
    title: String,
    actions: Vec<(String, Box<dyn Fn()>)>,
    parent: Option<*mut ffi::QWidget>,
}

impl MenuBuilder {
    fn new(title: String) -> Self {
        Self { title, actions: Vec::new(), parent: None }
    }

    /// Add a menu action with a callback.
    ///
    /// The callback is triggered when the menu item is selected.
    pub fn action<F: Fn() + 'static>(mut self, text: impl Into<String>, f: F) -> Self {
        self.actions.push((text.into(), Box::new(f)));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QMenu` and return the Rust wrapper.
    pub fn build(self) -> Menu {
        let_cxx_string!(c_title = &self.title);
        let ptr = unsafe {
            ffi::QMenu_new(&c_title, self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());

        let mut menu = Menu {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };

        for (text, cb) in self.actions {
            let_cxx_string!(c_text = &text);
            let handle = signal::leak_void(cb);
            unsafe {
                ffi::QMenu_addAction(ptr, &c_text, handle.token);
            }
            menu.signal_handles.push(handle);
        }

        menu
    }
}

// ============================================================
// MenuBar
// ============================================================

/// A menu bar at the top of a window.
///
/// `MenuBar` uses a builder pattern: call [`MenuBar::new`] to obtain
/// a [`MenuBarBuilder`], then call `.build()`.
///
/// # Example
///
/// ```no_run
/// use qtrs::{MenuBar, Menu};
///
/// let file_menu = Menu::new("File")
///     .action("Quit", || std::process::exit(0))
///     .build();
///
/// let mut menubar = MenuBar::new()
///     .add_menu(file_menu)
///     .build();
/// ```
pub struct MenuBar {
    ptr: *mut ffi::QMenuBar,
    has_parent: bool,
    menus: Vec<Menu>,
}

impl MenuBar {
    /// Start building a new menu bar.
    pub fn new() -> MenuBarBuilder {
        MenuBarBuilder::new()
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QMenuBar) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, menus: Vec::new() }
    }

    /// Get the raw menu bar pointer for use with [`MainWindow::set_menu_bar`].
    pub(crate) fn menubar_ptr(&self) -> *mut ffi::QMenuBar {
        self.ptr
    }
}

impl AsWidget for MenuBar {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QMenuBar(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for MenuBar {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if !self.has_parent {
            self.menus.clear();
            unsafe { ffi::QMenuBar_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// MenuBuilder for [`MenuBar`].
pub struct MenuBarBuilder {
    menus: Vec<Menu>,
    parent: Option<*mut ffi::QWidget>,
}

impl MenuBarBuilder {
    fn new() -> Self {
        Self { menus: Vec::new(), parent: None }
    }

    /// Add a menu to the menu bar.
    ///
    /// The menu is moved into the menu bar and owned by it.
    pub fn add_menu(mut self, menu: Menu) -> Self {
        self.menus.push(menu);
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QMenuBar` and return the Rust wrapper.
    pub fn build(self) -> MenuBar {
        let ptr = unsafe {
            ffi::QMenuBar_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());

        let mut menubar = MenuBar {
            ptr,
            has_parent: self.parent.is_some(),
            menus: Vec::new(),
        };

        for menu in self.menus {
            unsafe {
                ffi::QMenuBar_addMenu(ptr, menu.menu_ptr());
            }
            menubar.menus.push(menu);
        }

        menubar
    }
}
