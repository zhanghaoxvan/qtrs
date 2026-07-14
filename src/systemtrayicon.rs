//! System tray icon with context menu and activation signals.
//!
//! Wraps [`QSystemTrayIcon`](https://doc.qt.io/qt-6/qsystemtrayicon.html).
//!
//! # Note
//!
//! `QSystemTrayIcon` is a `QObject` subclass, **not** a `QWidget`. It does
//! **not** implement [`AsWidget`](crate::widget::AsWidget). Signal closures
//! are always reclaimed on [`Drop`]; the C++ object is always deleted.

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::menu::Menu;

/// Activation reason constants matching Qt's `QSystemTrayIcon::ActivationReason`.
pub const UNKNOWN: i32 = 0;
pub const CONTEXT: i32 = 1;
pub const DOUBLE_CLICK: i32 = 2;
pub const TRIGGER: i32 = 3;
pub const MIDDLE_CLICK: i32 = 4;

/// A system tray icon with tooltip, context menu, and activation signals.
///
/// `SystemTrayIcon` uses a **builder pattern**: call [`SystemTrayIcon::new`]
/// to obtain a [`Builder`], chain configuration, then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_activated`] / [`SystemTrayIcon::connect_activated`] | `QSystemTrayIcon::activated` | `i32` (reason — `UNKNOWN`, `CONTEXT`, `DOUBLE_CLICK`, `TRIGGER`, `MIDDLE_CLICK`) |
///
/// # Example
///
/// ```no_run
/// use qtrs::SystemTrayIcon;
///
/// let tray = SystemTrayIcon::new("/path/to/icon.png")
///     .tool_tip("My App")
///     .on_activated(|reason| println!("clicked reason={}", reason))
///     .build();
/// tray.show();
/// ```
pub struct SystemTrayIcon {
    ptr: *mut ffi::QSystemTrayIcon,
    signal_handles: Vec<SignalHandle>,
}

impl SystemTrayIcon {
    /// Start building a new system tray icon.
    pub fn new(icon_path: impl Into<String>) -> Builder {
        Builder::new(icon_path.into())
    }

    // --- Properties ---

    /// Set the tray icon from a file path.
    pub fn set_icon(&self, icon_path: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_path = icon_path);
        unsafe { ffi::QSystemTrayIcon_setIcon(self.ptr, &c_path); }
    }

    /// Set the tooltip text.
    pub fn set_tool_tip(&self, tip: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_tip = tip);
        unsafe { ffi::QSystemTrayIcon_setToolTip(self.ptr, &c_tip); }
    }

    /// Show the tray icon.
    pub fn show(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSystemTrayIcon_show(self.ptr); }
    }

    /// Hide the tray icon.
    pub fn hide(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSystemTrayIcon_hide(self.ptr); }
    }

    /// Returns `true` if the tray icon is currently visible.
    pub fn is_visible(&self) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSystemTrayIcon_isVisible(self.ptr) }
    }

    /// Set the context menu.
    pub fn set_context_menu(&self, menu: &mut Menu) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QSystemTrayIcon_setContextMenu(self.ptr, menu.ptr); }
    }

    // --- Runtime signal connections ---

    /// Connect a callback when the tray icon is activated.
    /// Receives a reason constant (`UNKNOWN`, `CONTEXT`, `DOUBLE_CLICK`, `TRIGGER`, `MIDDLE_CLICK`).
    pub fn connect_activated<F: Fn(i32) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_int(f);
        unsafe { ffi::QSystemTrayIcon_onActivated(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }
}

impl Drop for SystemTrayIcon {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        for h in self.signal_handles.drain(..) {
            unsafe { h.reclaim(); }
        }
        unsafe { ffi::QSystemTrayIcon_delete(self.ptr); }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`SystemTrayIcon`].
pub struct Builder {
    icon_path: String,
    tool_tip: Option<String>,
    on_activated: Option<Box<dyn Fn(i32)>>,
}

impl Builder {
    fn new(icon_path: String) -> Self {
        Self {
            icon_path,
            tool_tip: None,
            on_activated: None,
        }
    }

    /// Set the tooltip text.
    pub fn tool_tip(mut self, tip: impl Into<String>) -> Self {
        self.tool_tip = Some(tip.into());
        self
    }

    /// Called when the tray icon is activated. Receives a reason constant.
    pub fn on_activated<F: Fn(i32) + 'static>(mut self, f: F) -> Self {
        self.on_activated = Some(Box::new(f));
        self
    }

    /// Create the C++ `QSystemTrayIcon` and return the Rust wrapper.
    pub fn build(self) -> SystemTrayIcon {
        let_cxx_string!(c_icon = &self.icon_path);
        let ptr = unsafe {
            ffi::QSystemTrayIcon_new(&c_icon, std::ptr::null_mut())
        };
        debug_assert!(!ptr.is_null());
        let mut tray = SystemTrayIcon { ptr, signal_handles: Vec::new() };

        if let Some(tip) = &self.tool_tip {
            let_cxx_string!(c_tip = tip);
            unsafe { ffi::QSystemTrayIcon_setToolTip(ptr, &c_tip); }
        }

        if let Some(f) = self.on_activated {
            let h = signal::leak_int(f);
            unsafe { ffi::QSystemTrayIcon_onActivated(ptr, h.token); }
            tray.signal_handles.push(h);
        }

        tray
    }
}
