//! Base widget type and the [`AsWidget`] trait.
//!
//! [`Widget`] wraps [`QWidget`](https://doc.qt.io/qt-6/qwidget.html) —
//! it can serve as a top-level window or as a container for child widgets.
//! The [`AsWidget`] trait is implemented by every widget type in the
//! library so that layouts can accept any widget polymorphically.

use cxx::let_cxx_string;

use crate::ffi;

/// Polymorphic access to the underlying `QWidget*` pointer.
///
/// Every widget type in qtrs implements this trait. Layout containers
/// call [`widget_ptr`](AsWidget::widget_ptr) to add widgets without
/// knowing their concrete Rust type.
///
/// # Implementation note
///
/// The trait uses internal `set_has_parent` rather than a shared
/// ownership model. When a widget is added to a layout, the layout
/// calls `set_has_parent(true)` so that the widget's [`Drop`]
/// implementation skips C++ deletion (Qt's parent-child tree will
/// handle it instead).
pub trait AsWidget {
    /// Return the underlying `QWidget*` pointer.
    ///
    /// This is a raw C++ pointer — the caller must ensure the widget
    /// outlives any use of the pointer.
    fn widget_ptr(&self) -> *mut ffi::QWidget;

    /// Mark this widget as having a Qt parent.
    ///
    /// When `has_parent` is true, the [`Drop`] implementation will
    /// **not** delete the C++ object — Qt's parent-child ownership
    /// tree handles cleanup instead. This prevents double-free when
    /// a widget is added to a layout or created with an explicit parent.
    ///
    /// # Memory safety note
    ///
    /// When a widget has a parent and also has connected signals,
    /// the signal closures are **intentionally leaked** on Drop to
    /// prevent use-after-free (the C++ widget may still fire signals
    /// after the Rust wrapper is gone). Keep the Rust wrapper alive
    /// for the widget's full lifetime to avoid this leak.
    fn set_has_parent(&mut self);
}

/// A generic `QWidget` — can be a top-level window or a container.
///
/// `Widget` uses a **builder pattern**: call [`Widget::new`] to obtain
/// a [`Builder`], chain configuration methods, then call [`Builder::build`]
/// (or [`Builder::show`]) to construct the C++ object and return the Rust
/// wrapper.
///
/// # Memory safety
///
/// Every public method asserts (in debug builds) that the internal C++
/// pointer is non-null. This catches use-after-build-failure bugs early.
///
/// # Lifecycle
///
/// When a `Widget` is dropped:
/// - If the widget has **no** Qt parent: signal closures are reclaimed,
///   then the C++ `QWidget` is deleted via `delete`.
/// - If the widget **has** a Qt parent: signal closures are intentionally
///   **leaked** (to prevent use-after-free), and the C++ object is left
///   alone (Qt deletes it when the parent is destroyed).
///
/// # Example
///
/// ```no_run
/// use qtrs::Widget;
///
/// let window = Widget::new()
///     .title("My Window")
///     .size(800, 600)
///     .build();
/// window.show();
/// ```
pub struct Widget {
    ptr: *mut ffi::QWidget,
    has_parent: bool,
    #[allow(dead_code)]
    title: Option<String>,
    #[allow(dead_code)]
    width: i32,
    #[allow(dead_code)]
    height: i32,
    // Signal closure tokens. On Drop:
    //   has_parent=false → reclaimed (safe: C++ object is deleted right after)
    //   has_parent=true  → leaked   (safe: prevents use-after-free)
    signal_handles: Vec<crate::signal::SignalHandle>,
}

// Safety: Widget owns a unique C++ QWidget*. It is not Send/Sync because
// Qt GUI objects must only be accessed from the main thread.
// These negative impls are automatic due to the raw pointer field.

impl Widget {
    /// Start building a new, parentless `QWidget`.
    ///
    /// Returns a [`Builder`] — chain `.title()`, `.size()`, `.parent()`,
    /// then call `.build()` or `.show()`.
    pub fn new() -> Builder {
        Builder::new()
    }

    /// Create a `Widget` from a raw C++ pointer (internal use only).
    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QWidget, has_parent: bool) -> Self {
        debug_assert!(!ptr.is_null(), "from_raw called with null pointer");
        Self {
            ptr,
            has_parent,
            title: None,
            width: 0,
            height: 0,
            signal_handles: Vec::new(),
        }
    }

    /// Show this widget (makes it visible).
    ///
    /// For top-level windows, this displays the window. For child widgets
    /// added to a layout, visibility is managed by the parent.
    pub fn show(&self) {
        debug_assert!(!self.ptr.is_null(), "Widget::show on null pointer");
        unsafe { ffi::QWidget_show(self.ptr) };
    }

    /// Hide this widget.
    pub fn hide(&self) {
        debug_assert!(!self.ptr.is_null(), "Widget::hide on null pointer");
        unsafe { ffi::QWidget_hide(self.ptr) };
    }

    /// Set the window title at runtime.
    ///
    /// This is equivalent to
    /// [`QWidget::setWindowTitle`](https://doc.qt.io/qt-6/qwidget.html#windowTitle-prop).
    pub fn set_title(&self, title: &str) {
        debug_assert!(!self.ptr.is_null(), "Widget::set_title on null pointer");
        let_cxx_string!(c_title = title);
        unsafe {
            ffi::QWidget_setWindowTitle(self.ptr, &c_title);
        }
    }

    /// Resize the widget at runtime.
    ///
    /// Width and height are in logical pixels.
    pub fn resize(&self, width: i32, height: i32) {
        debug_assert!(!self.ptr.is_null(), "Widget::resize on null pointer");
        unsafe {
            ffi::QWidget_resize(self.ptr, width, height);
        }
    }

    /// Install a vertical box layout on this widget.
    ///
    /// After calling this, the layout manages the geometry of all child
    /// widgets added to it.
    ///
    /// # Safety note
    ///
    /// The layout must outlive this widget. Dropping the layout first is
    /// fine — children are dropped, then the C++ layout is deleted.
    pub fn set_vlayout(&mut self, layout_ptr: *mut ffi::QVBoxLayout) {
        debug_assert!(!self.ptr.is_null(), "Widget::set_vlayout on null pointer");
        debug_assert!(!layout_ptr.is_null(), "set_vlayout with null layout");
        unsafe {
            ffi::QWidget_setLayout(
                self.ptr,
                layout_ptr as *mut u8 as *mut ffi::QLayout,
            );
        }
    }

    /// Install a grid layout on this widget.
    pub fn set_grid(&mut self, grid: &crate::GridLayout) {
        debug_assert!(!self.ptr.is_null());
        unsafe {
            ffi::QWidget_setLayout(
                self.ptr,
                grid.layout_ptr() as *mut u8 as *mut ffi::QLayout,
            );
        }
    }

    /// Install a horizontal box layout on this widget.
    ///
    /// See [`set_vlayout`](Self::set_vlayout) for details.
    pub fn set_hlayout(&mut self, layout_ptr: *mut ffi::QHBoxLayout) {
        debug_assert!(!self.ptr.is_null(), "Widget::set_hlayout on null pointer");
        debug_assert!(!layout_ptr.is_null(), "set_hlayout with null layout");
        unsafe {
            ffi::QWidget_setLayout(
                self.ptr,
                layout_ptr as *mut u8 as *mut ffi::QLayout,
            );
        }
    }

    /// Set the window icon from an image file.
    pub fn set_icon(&self, icon_path: &str) {
        debug_assert!(!self.ptr.is_null(), "Widget::set_icon on null pointer");
        let_cxx_string!(c_path = icon_path);
        unsafe { ffi::QWidget_setWindowIcon(self.ptr, &c_path); }
    }

    /// Enable or disable this widget (and all children).
    pub fn set_enabled(&self, enabled: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QWidget_setEnabled(self.ptr, enabled); }
    }

    /// Show or hide this widget (alternative to [`show`](Self::show)/[`hide`](Self::hide)).
    pub fn set_visible(&self, visible: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QWidget_setVisible(self.ptr, visible); }
    }

    /// Set a tooltip that appears on hover.
    pub fn set_tooltip(&self, tip: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_tip = tip);
        unsafe { ffi::QWidget_setToolTip(self.ptr, &c_tip); }
    }

    /// Set the minimum size in pixels.
    pub fn set_min_size(&self, w: i32, h: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QWidget_setMinimumSize(self.ptr, w, h); }
    }

    /// Set the maximum size in pixels.
    pub fn set_max_size(&self, w: i32, h: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QWidget_setMaximumSize(self.ptr, w, h); }
    }

    /// Lock the widget to a fixed size (sets both min and max).
    pub fn set_fixed_size(&self, w: i32, h: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QWidget_setFixedSize(self.ptr, w, h); }
    }

    /// Apply a CSS stylesheet to this widget (cascades to children).
    ///
    /// Uses [`QWidget::setStyleSheet`](https://doc.qt.io/qt-6/stylesheet.html).
    pub fn set_style_sheet(&self, css: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_css = css);
        unsafe { ffi::QWidget_setStyleSheet(self.ptr, &c_css); }
    }

    /// Find a named child widget by its `objectName`.
    ///
    /// `kind` selects the widget type to find. Returns the wrapped widget
    /// on success, or `None` if no child with that name and type exists.
    ///
    /// ```no_run
    /// # use qtrs::*;
    /// let window = Widget::new().title("demo").build();
    /// if let Some(FoundWidget::PushButton(mut btn)) =
    ///     window.find(WidgetKind::PushButton, "myButton")
    /// {
    ///     btn.connect_clicked(|| println!("clicked!"));
    /// }
    /// ```
    pub fn find(&self, kind: WidgetKind, name: &str) -> Option<FoundWidget> {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_name = name);
        match kind {
            WidgetKind::PushButton => {
                let ptr = unsafe { ffi::QWidget_findPushButton(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::PushButton(crate::PushButton::from_raw(ptr, name))) }
            }
            WidgetKind::LineEdit => {
                let ptr = unsafe { ffi::QWidget_findLineEdit(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::LineEdit(crate::LineEdit::from_raw(ptr, name))) }
            }
            WidgetKind::CheckBox => {
                let ptr = unsafe { ffi::QWidget_findCheckBox(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::CheckBox(crate::CheckBox::from_raw(ptr, name))) }
            }
            WidgetKind::ComboBox => {
                let ptr = unsafe { ffi::QWidget_findComboBox(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::ComboBox(crate::ComboBox::from_raw(ptr, name))) }
            }
            WidgetKind::Slider => {
                let ptr = unsafe { ffi::QWidget_findSlider(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::Slider(crate::Slider::from_raw(ptr, name))) }
            }
            WidgetKind::TextEdit => {
                let ptr = unsafe { ffi::QWidget_findTextEdit(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::TextEdit(crate::TextEdit::from_raw(ptr, name))) }
            }
            WidgetKind::Label => {
                let ptr = unsafe { ffi::QWidget_findLabel(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::Label(crate::Label::from_raw(ptr, name))) }
            }
            WidgetKind::ProgressBar => {
                let ptr = unsafe { ffi::QWidget_findProgressBar(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::ProgressBar(crate::ProgressBar::from_raw(ptr))) }
            }
            WidgetKind::RadioButton => {
                let ptr = unsafe { ffi::QWidget_findRadioButton(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::RadioButton(crate::RadioButton::from_raw(ptr))) }
            }
            WidgetKind::GroupBox => {
                let ptr = unsafe { ffi::QWidget_findGroupBox(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::GroupBox(crate::GroupBox::from_raw(ptr))) }
            }
            WidgetKind::TabWidget => {
                let ptr = unsafe { ffi::QWidget_findTabWidget(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::TabWidget(crate::TabWidget::from_raw(ptr))) }
            }
            WidgetKind::SpinBox => {
                let ptr = unsafe { ffi::QWidget_findSpinBox(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::SpinBox(crate::SpinBox::from_raw(ptr))) }
            }
            WidgetKind::Any => {
                let ptr = unsafe { ffi::QWidget_findWidget(self.ptr, &c_name) };
                if ptr.is_null() { None }
                else { Some(FoundWidget::Widget(Widget::from_raw(ptr, true))) }
            }
        }
    }
}

// ============================================================
// Widget find enums
// ============================================================

/// Widget type selector for [`Widget::find`].
#[derive(Clone, Copy)]
pub enum WidgetKind {
    PushButton,
    LineEdit,
    CheckBox,
    ComboBox,
    Slider,
    TextEdit,
    Label,
    ProgressBar,
    RadioButton,
    GroupBox,
    TabWidget,
    SpinBox,
    /// Any `QWidget` (no signal support).
    Any,
}

/// Returned by [`Widget::find`] — match to unwrap and connect signals.
pub enum FoundWidget {
    PushButton(crate::PushButton),
    LineEdit(crate::LineEdit),
    CheckBox(crate::CheckBox),
    ComboBox(crate::ComboBox),
    Slider(crate::Slider),
    TextEdit(crate::TextEdit),
    Label(crate::Label),
    ProgressBar(crate::ProgressBar),
    RadioButton(crate::RadioButton),
    GroupBox(crate::GroupBox),
    TabWidget(crate::TabWidget),
    SpinBox(crate::SpinBox),
    Widget(Widget),
}

impl AsWidget for Widget {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null(), "widget_ptr on null pointer");
        unsafe { ffi::toQWidget_QWidget(self.ptr) }
    }

    fn set_has_parent(&mut self) {
        self.has_parent = true;
    }
}

impl Drop for Widget {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr); }
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
        } else {
            for h in self.signal_handles.drain(..) {
                unsafe { h.reclaim(); }
            }
            unsafe { ffi::QWidget_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`Widget`].
///
/// Collects configuration and creates the C++ `QWidget` in
/// [`build`](Self::build).
///
/// # Example
///
/// ```no_run
/// let window = Widget::new()
///     .title("Demo")
///     .size(640, 480)
///     .build();
/// ```
pub struct Builder {
    title: Option<String>,
    icon: Option<String>,
    width: i32,
    height: i32,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            title: None,
            icon: None,
            width: 400,
            height: 300,
            parent: None,
        }
    }

    /// Set the window title (displayed in the title bar).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the window icon from an image file path.
    ///
    /// Supports PNG, JPEG, BMP, GIF, SVG, and other formats Qt can read.
    /// The path is resolved relative to the working directory when the
    /// application runs.
    ///
    /// > **Wayland note:** Per-window icons may not display on Wayland.
    /// > Use [`Application::set_icon`] for reliable cross-platform icons.
    ///
    /// [`Application::set_icon`]: crate::Application::set_icon
    pub fn icon(mut self, path: impl Into<String>) -> Self {
        self.icon = Some(path.into());
        self
    }

    /// Set the window size in logical pixels.
    ///
    /// Default is 400×300.
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Attach this widget as a child of `parent`.
    ///
    /// The parent widget will manage this widget's C++ lifetime.
    /// Do **not** drop the parent before the child — Qt will delete
    /// the child automatically.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QWidget`, apply configuration, and return the Rust
    /// wrapper.
    ///
    /// This is the terminal method of the builder pattern.
    pub fn build(self) -> Widget {
        let ptr = unsafe {
            ffi::QWidget_new(
                self.parent.unwrap_or(std::ptr::null_mut()),
            )
        };
        debug_assert!(!ptr.is_null(), "QWidget_new returned null");

        let has_parent = self.parent.is_some();

        let widget = Widget {
            ptr,
            has_parent,
            title: self.title.clone(),
            width: self.width,
            height: self.height,
            signal_handles: Vec::new(),
        };

        // Apply initial configuration.
        if let Some(ref t) = self.title {
            let_cxx_string!(c_title = t);
            unsafe { ffi::QWidget_setWindowTitle(widget.ptr, &c_title) };
        }
        if let Some(ref icon_path) = self.icon {
            let_cxx_string!(c_icon = icon_path);
            unsafe { ffi::QWidget_setWindowIcon(widget.ptr, &c_icon) };
        }
        unsafe { ffi::QWidget_resize(widget.ptr, self.width, self.height) };

        widget
    }

    /// Build the widget and immediately call [`Widget::show`].
    ///
    /// Convenience shorthand for `.build()` followed by `.show()`.
    pub fn show(self) -> Widget {
        let w = self.build();
        w.show();
        w
    }
}
