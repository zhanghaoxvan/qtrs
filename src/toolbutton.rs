//! Tool button widget with icon, text, and popup support.
//!
//! Wraps [`QToolButton`](https://doc.qt.io/qt-6/qtoolbutton.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// Tool button style constants matching Qt's `Qt::ToolButtonStyle`.
pub const TOOL_BUTTON_ICON_ONLY: i32 = 0;
pub const TOOL_BUTTON_TEXT_ONLY: i32 = 1;
pub const TOOL_BUTTON_TEXT_BESIDE_ICON: i32 = 2;
pub const TOOL_BUTTON_TEXT_UNDER_ICON: i32 = 3;

/// Popup mode constants matching Qt's `QToolButton::ToolButtonPopupMode`.
pub const DELAYED_POPUP: i32 = 0;
pub const MENU_BUTTON_POPUP: i32 = 1;
pub const INSTANT_POPUP: i32 = 2;

/// A tool button with icon, text, and optional popup menu.
///
/// Use a **builder pattern**: [`ToolButton::new`] returns a [`Builder`],
/// chain configuration, then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_clicked`] / [`ToolButton::connect_clicked`] | `QToolButton::clicked` | `()` |
/// | [`Builder::on_toggled`] / [`ToolButton::connect_toggled`] | `QToolButton::toggled` | `bool` (checked state) |
///
/// # Example
///
/// ```no_run
/// use qtrs::ToolButton;
///
/// let btn = ToolButton::new()
///     .text("Open")
///     .icon("/path/to/icon.png")
///     .on_clicked(|| println!("clicked!"))
///     .build();
/// ```
pub struct ToolButton {
    ptr: *mut ffi::QToolButton,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl ToolButton {
    /// Start building a new tool button.
    pub fn new() -> Builder {
        Builder::new()
    }

    // --- Properties ---

    /// Set the button text.
    pub fn set_text(&self, text: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_text = text);
        unsafe { ffi::QToolButton_setText(self.ptr, &c_text); }
    }

    /// Set the button icon from a file path.
    pub fn set_icon(&self, icon_path: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_path = icon_path);
        unsafe { ffi::QToolButton_setIcon(self.ptr, &c_path); }
    }

    /// Set the tool button style (icon only, text only, etc.).
    pub fn set_tool_button_style(&self, style: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolButton_setToolButtonStyle(self.ptr, style); }
    }

    /// Set the popup mode.
    pub fn set_popup_mode(&self, mode: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolButton_setPopupMode(self.ptr, mode); }
    }

    /// Set whether auto-raise is enabled.
    pub fn set_auto_raise(&self, enabled: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolButton_setAutoRaise(self.ptr, enabled); }
    }

    /// Make the button checkable.
    pub fn set_checkable(&self, checkable: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolButton_setCheckable(self.ptr, checkable); }
    }

    /// Set the checked state.
    pub fn set_checked(&self, checked: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QToolButton_setChecked(self.ptr, checked); }
    }

    /// Set a keyboard shortcut.
    pub fn set_shortcut(&self, key: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_key = key);
        unsafe { ffi::QToolButton_setShortcut(self.ptr, &c_key); }
    }

    // --- Signal connections (runtime) ---

    /// Connect a callback when the button is clicked.
    pub fn connect_clicked<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QToolButton_onClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback when the button toggles. Receives the checked state.
    pub fn connect_toggled<F: Fn(bool)>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_bool(f);
        unsafe { ffi::QToolButton_onToggled(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QToolButton, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for ToolButton {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QToolButton(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for ToolButton {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QToolButton_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`ToolButton`].
pub struct Builder {
    text: Option<String>,
    icon: Option<String>,
    tool_button_style: Option<i32>,
    popup_mode: Option<i32>,
    auto_raise: Option<bool>,
    checkable: Option<bool>,
    checked: Option<bool>,
    shortcut: Option<String>,
    on_clicked: Option<Box<dyn Fn()>>,
    on_toggled: Option<Box<dyn Fn(bool)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            text: None,
            icon: None,
            tool_button_style: None,
            popup_mode: None,
            auto_raise: None,
            checkable: None,
            checked: None,
            shortcut: None,
            on_clicked: None,
            on_toggled: None,
            parent: None,
        }
    }

    /// Set the button text.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set the button icon from a file path.
    pub fn icon(mut self, icon_path: impl Into<String>) -> Self {
        self.icon = Some(icon_path.into());
        self
    }

    /// Set the tool button style.
    pub fn tool_button_style(mut self, style: i32) -> Self {
        self.tool_button_style = Some(style);
        self
    }

    /// Set the popup mode.
    pub fn popup_mode(mut self, mode: i32) -> Self {
        self.popup_mode = Some(mode);
        self
    }

    /// Set whether auto-raise is enabled.
    pub fn auto_raise(mut self, enabled: bool) -> Self {
        self.auto_raise = Some(enabled);
        self
    }

    /// Make the button checkable.
    pub fn checkable(mut self, checkable: bool) -> Self {
        self.checkable = Some(checkable);
        self
    }

    /// Set the initial checked state.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// Set a keyboard shortcut.
    pub fn shortcut(mut self, key: impl Into<String>) -> Self {
        self.shortcut = Some(key.into());
        self
    }

    /// Called when the button is clicked.
    pub fn on_clicked<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_clicked = Some(Box::new(f));
        self
    }

    /// Called when the button toggles. Receives the checked state.
    pub fn on_toggled<F: Fn(bool) + 'static>(mut self, f: F) -> Self {
        self.on_toggled = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QToolButton` and return the Rust wrapper.
    pub fn build(self) -> ToolButton {
        let ptr = unsafe {
            ffi::QToolButton_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut tb = ToolButton {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };

        if let Some(text) = &self.text {
            let_cxx_string!(c_text = text);
            unsafe { ffi::QToolButton_setText(ptr, &c_text); }
        }
        if let Some(icon) = &self.icon {
            let_cxx_string!(c_icon = icon);
            unsafe { ffi::QToolButton_setIcon(ptr, &c_icon); }
        }
        if let Some(style) = self.tool_button_style {
            unsafe { ffi::QToolButton_setToolButtonStyle(ptr, style); }
        }
        if let Some(mode) = self.popup_mode {
            unsafe { ffi::QToolButton_setPopupMode(ptr, mode); }
        }
        if let Some(enabled) = self.auto_raise {
            unsafe { ffi::QToolButton_setAutoRaise(ptr, enabled); }
        }
        if let Some(checkable) = self.checkable {
            unsafe { ffi::QToolButton_setCheckable(ptr, checkable); }
        }
        if let Some(checked) = self.checked {
            unsafe { ffi::QToolButton_setChecked(ptr, checked); }
        }
        if let Some(key) = &self.shortcut {
            let_cxx_string!(c_key = key);
            unsafe { ffi::QToolButton_setShortcut(ptr, &c_key); }
        }

        if let Some(f) = self.on_clicked {
            let h = signal::leak_void(f);
            unsafe { ffi::QToolButton_onClicked(ptr, h.token); }
            tb.signal_handles.push(h);
        }
        if let Some(f) = self.on_toggled {
            let h = signal::leak_bool(f);
            unsafe { ffi::QToolButton_onToggled(ptr, h.token); }
            tb.signal_handles.push(h);
        }

        tb
    }
}
