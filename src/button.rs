//! Push-button widget with click callbacks.
//!
//! Wraps [`QPushButton`](https://doc.qt.io/qt-6/qpushbutton.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal;
use crate::widget::AsWidget;

/// A pressable button with an optional click callback.
///
/// `PushButton` uses a **builder pattern**: call [`PushButton::new`] to
/// obtain a [`Builder`], chain `.on_clicked(f)`, `.parent(w)`, then call
/// `.build()` or `.show()`.
///
/// # Signals
///
/// | Method | Qt signal | When |
/// |---|---|---|
/// | [`Builder::on_clicked`] | `QPushButton::clicked` | Button is pressed and released |
///
/// # Memory safety
///
/// Signal closures are stored on the heap and passed to C++ as `u64`
/// tokens. On [`Drop`]:
///
/// - **No Qt parent:** closures are reclaimed, then the C++ object is
///   deleted. Safe — no more signals can fire after deletion.
/// - **Has Qt parent:** closures are **intentionally leaked** to
///   prevent use-after-free (the C++ widget outlives the Rust wrapper
///   and could still fire signals). Keep the Rust wrapper alive for
///   the widget's full lifetime to avoid this leak.
///
/// # Example
///
/// ```ignore
/// use qtrs::PushButton;
///
/// let btn = PushButton::new("Click me")
///     .on_clicked(|| println!("clicked!"))
///     .build();
/// ```
pub struct PushButton {
    ptr: *mut ffi::QPushButton,
    has_parent: bool,
    #[allow(dead_code)]
    text: String,
    signal_handles: Vec<crate::signal::SignalHandle>,
}

impl PushButton {
    /// Start building a new `QPushButton`.
    ///
    /// Returns a [`Builder`]. Chain configuration, then call `.build()`.
    pub fn new(text: impl Into<String>) -> Builder {
        Builder::new(text.into())
    }

    /// Get the current button text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Update the button text at runtime.
    ///
    /// This calls
    /// [`QPushButton::setText`](https://doc.qt.io/qt-6/qabstractbutton.html#text-prop).
    pub fn set_text(&mut self, text: impl Into<String>) {
        debug_assert!(!self.ptr.is_null(), "PushButton::set_text on null pointer");
        self.text = text.into();
        let_cxx_string!(c_text = &self.text);
        unsafe {
            ffi::QPushButton_setText(self.ptr, &c_text);
        }
    }

    /// Show this button.
    ///
    /// Normally child widgets are shown automatically by their parent;
    /// use this only for standalone buttons.
    pub fn show(&self) {
        debug_assert!(!self.ptr.is_null(), "PushButton::show on null pointer");
        unsafe { ffi::QPushButton_show(self.ptr) };
    }

    /// Connect a click callback to an already-existing button.
    ///
    /// This is the runtime equivalent of
    /// [`Builder::on_clicked`] — useful when the button was loaded
    /// from a `.ui` file rather than built in Rust.
    pub fn connect_clicked<F: Fn() + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QPushButton_onClicked(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Wrap an existing `QPushButton*` obtained via `findChild`.
    /// The button already has a Qt parent, so Drop will not delete it.
    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QPushButton, text: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, text: text.to_string(), signal_handles: Vec::new() }
    }
}

impl AsWidget for PushButton {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null(), "PushButton::widget_ptr on null pointer");
        unsafe { ffi::toQWidget_QPushButton(self.ptr) }
    }

    fn set_has_parent(&mut self) {
        self.has_parent = true;
    }
}

impl Drop for PushButton {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            // Disconnect all Qt signals so no more callbacks fire,
            // then reclaim closures safely (no use-after-free).
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
        } else {
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
            unsafe { ffi::QPushButton_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`PushButton`].
///
/// Collects text, parent, and signal callbacks, then creates the C++
/// `QPushButton` (and connects signals) in [`build`](Self::build).
pub struct Builder {
    text: String,
    on_clicked: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new(text: String) -> Self {
        Self {
            text,
            on_clicked: None,
            parent: None,
        }
    }

    /// Set the click callback.
    ///
    /// The closure will be called **each time** the button is clicked.
    /// It is stored on the heap and reclaimed when the button is dropped
    /// (only if the button has no Qt parent — see the [memory safety]
    /// note on [`PushButton`]).
    pub fn on_clicked<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_clicked = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    ///
    /// The parent manages this button's C++ lifetime. Do not drop the
    /// parent before the button.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QPushButton`, connect signals, and return the
    /// Rust wrapper.
    ///
    /// This is the terminal method of the builder pattern.
    pub fn build(self) -> PushButton {
        let_cxx_string!(c_text = &self.text);
        let ptr = unsafe {
            ffi::QPushButton_new(
                &c_text,
                self.parent.unwrap_or(std::ptr::null_mut()),
            )
        };
        debug_assert!(!ptr.is_null(), "QPushButton_new returned null");

        let has_parent = self.parent.is_some();
        let mut signal_handles = Vec::new();

        // Connect click signal if a callback was provided.
        if let Some(cb) = self.on_clicked {
            let handle = signal::leak_void(cb);
            unsafe { ffi::QPushButton_onClicked(ptr, handle.token); }
            signal_handles.push(handle);
        }

        PushButton {
            ptr,
            has_parent,
            text: self.text,
            signal_handles,
        }
    }

    /// Build and immediately show the button.
    pub fn show(self) -> PushButton {
        let btn = self.build();
        btn.show();
        btn
    }
}
