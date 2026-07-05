//! Checkbox widget with toggle callback.
//!
//! Wraps [`QCheckBox`](https://doc.qt.io/qt-6/qcheckbox.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A checkbox with an optional toggle callback.
///
/// Use a **builder pattern**: [`CheckBox::new`] returns a [`Builder`],
/// chain `.on_toggled(f)`, `.checked(true)`, `.parent(w)`, then `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_toggled`] | `QCheckBox::toggled` | `bool` (checked state) |
///
/// # Example
///
/// ```ignore
/// let cb = CheckBox::new("Enable feature")
///     .on_toggled(|checked| println!("toggled: {}", checked))
///     .build();
/// ```
pub struct CheckBox {
    ptr: *mut ffi::QCheckBox,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl CheckBox {
    pub fn new(text: impl Into<String>) -> Builder {
        Builder::new(text.into())
    }

    pub fn is_checked(&self) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QCheckBox_isChecked(self.ptr) }
    }

    pub fn set_checked(&self, checked: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QCheckBox_setChecked(self.ptr, checked) };
    }

    /// Connect a toggle callback to an already-existing checkbox.
    pub fn connect_toggled<F: Fn(bool) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_bool(f);
        unsafe { ffi::QCheckBox_onToggled(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QCheckBox, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for CheckBox {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QCheckBox(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for CheckBox {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.signal_handles.clear();
        } else {
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
            unsafe { ffi::QCheckBox_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

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

    /// Set initial checked state.
    pub fn checked(mut self, checked: bool) -> Self { self.checked = checked; self }

    /// Callback receives the new checked state (`true`/`false`).
    pub fn on_toggled<F: Fn(bool) + 'static>(mut self, f: F) -> Self {
        self.on_toggled = Some(Box::new(f)); self
    }

    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr()); self
    }

    pub fn build(self) -> CheckBox {
        let_cxx_string!(c_text = &self.text);
        let ptr = unsafe {
            ffi::QCheckBox_new(&c_text, self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut cb = CheckBox { ptr, has_parent: self.parent.is_some(), signal_handles: Vec::new() };
        if self.checked {
            unsafe { ffi::QCheckBox_setChecked(ptr, true) };
        }
        if let Some(f) = self.on_toggled {
            let h = signal::leak_bool(f);
            unsafe { ffi::QCheckBox_onToggled(ptr, h.token); }
            cb.signal_handles.push(h);
        }
        cb
    }
}
