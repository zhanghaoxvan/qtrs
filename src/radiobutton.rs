//! Radio button widget.
//!
//! Wraps [`QRadioButton`](https://doc.qt.io/qt-6/qradiobutton.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A radio button that can be toggled on/off.
///
/// Radio buttons typically belong to a group for mutual exclusivity.
///
/// `RadioButton` uses a builder pattern: call [`RadioButton::new`] to obtain
/// a [`Builder`], chain configuration methods, then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_toggled`] | `QRadioButton::toggled` | `bool` (checked state) |
///
/// # Example
///
/// ```no_run
/// use qtrs::RadioButton;
///
/// let rb = RadioButton::new("Enable feature")
///     .on_toggled(|checked| println!("toggled: {}", checked))
///     .build();
/// ```
pub struct RadioButton {
    ptr: *mut ffi::QRadioButton,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl RadioButton {
    /// Start building a new radio button.
    pub fn new(text: impl Into<String>) -> Builder {
        Builder::new(text.into())
    }

    /// Check if the radio button is selected.
    pub fn is_checked(&self) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QRadioButton_isChecked(self.ptr) }
    }

    /// Set the checked state.
    pub fn set_checked(&self, checked: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QRadioButton_setChecked(self.ptr, checked) };
    }

    /// Connect a toggle callback to an already-existing radio button.
    pub fn connect_toggled<F: Fn(bool) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_bool(f);
        unsafe { ffi::QRadioButton_onToggled(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QRadioButton) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for RadioButton {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QRadioButton(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for RadioButton {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
        } else {
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
            unsafe { ffi::QRadioButton_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`RadioButton`].
pub struct Builder {
    text: String,
    checked: bool,
    on_toggled: Option<Box<dyn Fn(bool)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new(text: String) -> Self {
        Self { text, checked: false, on_toggled: None, parent: None }
    }

    /// Set initial checked state. Default is `false`.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Set the toggle callback.
    ///
    /// The closure receives `true` when checked, `false` when unchecked.
    pub fn on_toggled<F: Fn(bool) + 'static>(mut self, f: F) -> Self {
        self.on_toggled = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QRadioButton` and return the Rust wrapper.
    pub fn build(self) -> RadioButton {
        let_cxx_string!(c_text = &self.text);
        let ptr = unsafe {
            ffi::QRadioButton_new(&c_text, self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut rb = RadioButton {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if self.checked {
            unsafe { ffi::QRadioButton_setChecked(ptr, true) };
        }
        if let Some(f) = self.on_toggled {
            let h = signal::leak_bool(f);
            unsafe { ffi::QRadioButton_onToggled(ptr, h.token); }
            rb.signal_handles.push(h);
        }
        rb
    }
}
