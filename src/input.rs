//! Single-line text input widget.
//!
//! Wraps [`QLineEdit`](https://doc.qt.io/qt-6/qlineedit.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal;
use crate::widget::AsWidget;

/// A single-line text input field.
///
/// `LineEdit` uses a **builder pattern**: call [`LineEdit::new`] to obtain
/// a [`Builder`], chain `.on_return_pressed(f)`, `.parent(w)`, then call
/// `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | When |
/// |---|---|---|
/// | [`Builder::on_return_pressed`] | `QLineEdit::returnPressed` | User presses Enter/Return |
///
/// # Text caching
///
/// The widget stores a local copy of the text. It is updated when you
/// call [`set_text`](Self::set_text) or [`refresh_text`](Self::refresh_text).
/// The getter [`text`](Self::text) returns the cached copy without any
/// FFI call.
///
/// # Memory safety
///
/// See [`PushButton`] for signal-closure lifecycle rules — the same
/// parent/no-parent leak/reclaim logic applies here.
///
/// [`PushButton`]: crate::PushButton
///
/// # Example
///
/// ```no_run
/// use qtrs::LineEdit;
///
/// let edit = LineEdit::new("type here...")
///     .on_return_pressed(|| println!("Enter pressed!"))
///     .build();
/// ```
pub struct LineEdit {
    ptr: *mut ffi::QLineEdit,
    has_parent: bool,
    #[allow(dead_code)]
    text: String,
    signal_handles: Vec<crate::signal::SignalHandle>,
}

impl LineEdit {
    /// Start building a new `QLineEdit`.
    ///
    /// Returns a [`Builder`]. Set placeholder text (via the initial value),
    /// optional callbacks, and an optional parent, then call `.build()`.
    pub fn new(text: impl Into<String>) -> Builder {
        Builder::new(text.into())
    }

    /// Get the cached text.
    ///
    /// To read the live value from the Qt widget, call
    /// [`refresh_text`](Self::refresh_text) first.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Re-read the current text from the Qt widget.
    ///
    /// This fetches `QLineEdit::text()` via FFI and updates the local
    /// cache returned by [`text`](Self::text).
    pub fn refresh_text(&mut self) {
        debug_assert!(!self.ptr.is_null(), "LineEdit::refresh_text on null pointer");
        self.text = unsafe { ffi::QLineEdit_text(self.ptr) };
    }

    /// Set the text at runtime.
    ///
    /// This calls
    /// [`QLineEdit::setText`](https://doc.qt.io/qt-6/qlineedit.html#text-prop)
    /// and updates the local cache.
    pub fn set_text(&mut self, text: impl Into<String>) {
        debug_assert!(!self.ptr.is_null(), "LineEdit::set_text on null pointer");
        self.text = text.into();
        let_cxx_string!(c_text = &self.text);
        unsafe {
            ffi::QLineEdit_setText(self.ptr, &c_text);
        }
    }
}

impl AsWidget for LineEdit {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null(), "LineEdit::widget_ptr on null pointer");
        unsafe { ffi::toQWidget_QLineEdit(self.ptr) }
    }

    fn set_has_parent(&mut self) {
        self.has_parent = true;
    }
}

impl Drop for LineEdit {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.signal_handles.clear();
        } else {
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
            unsafe { ffi::QLineEdit_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`LineEdit`].
///
/// Collects initial text, signal callbacks, and parent, then creates the
/// C++ `QLineEdit` (and connects signals) in [`build`](Self::build).
pub struct Builder {
    text: String,
    on_return_pressed: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new(text: String) -> Self {
        Self {
            text,
            on_return_pressed: None,
            parent: None,
        }
    }

    /// Set the callback for when the user presses Enter/Return.
    ///
    /// The closure is stored on the heap and reclaimed when the widget
    /// is dropped (only if the widget has no Qt parent).
    pub fn on_return_pressed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_return_pressed = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    ///
    /// The parent manages the line-edit's C++ lifetime.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QLineEdit`, connect signals, and return the Rust
    /// wrapper.
    ///
    /// This is the terminal method of the builder pattern.
    pub fn build(self) -> LineEdit {
        let_cxx_string!(c_text = &self.text);
        let ptr = unsafe {
            ffi::QLineEdit_new(
                &c_text,
                self.parent.unwrap_or(std::ptr::null_mut()),
            )
        };
        debug_assert!(!ptr.is_null(), "QLineEdit_new returned null");

        let has_parent = self.parent.is_some();
        let mut signal_handles = Vec::new();

        if let Some(cb) = self.on_return_pressed {
            let handle = signal::leak_void(cb);
            unsafe { ffi::QLineEdit_onReturnPressed(ptr, handle.token); }
            signal_handles.push(handle);
        }

        LineEdit {
            ptr,
            has_parent,
            text: self.text,
            signal_handles,
        }
    }

    /// Build and show the widget.
    pub fn show(self) -> LineEdit {
        self.build()
    }
}
