//! Base widget type and the [`AsWidget`] trait.
//!
//! [`Widget`] wraps [`QWidget`](https://doc.qt.io/qt-6/qwidget.html) —
//! it can serve as a top-level window or as a container for child widgets.
//! The [`AsWidget`] trait is implemented by every widget type in the
//! library so that layouts can accept any widget polymorphically.

use cxx::let_cxx_string;

use crate::ffi;
use crate::Point;

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
    /// the signals are **disconnected** first on Drop, then closures
    /// are reclaimed. This prevents use-after-free (the C++ widget
    /// may outlive the Rust wrapper). Keep the Rust wrapper alive
    /// for the widget's full lifetime for best results.
    fn set_has_parent(&mut self);

    // ============================================================
    // Default methods — every widget gets these for free
    // ============================================================

    /// Get the widget width in pixels.
    fn width(&self) -> i32 {
        unsafe { ffi::QWidget_width(self.widget_ptr()) }
    }

    /// Get the widget height in pixels.
    fn height(&self) -> i32 {
        unsafe { ffi::QWidget_height(self.widget_ptr()) }
    }

    /// Get the widget's x position relative to its parent.
    fn x(&self) -> i32 {
        unsafe { ffi::QWidget_x(self.widget_ptr()) }
    }

    /// Get the widget's y position relative to its parent.
    fn y(&self) -> i32 {
        unsafe { ffi::QWidget_y(self.widget_ptr()) }
    }

    /// Get the widget's position as a `Point`.
    fn pos(&self) -> Point {
        Point::new(self.x(), self.y())
    }

    /// Get the widget's size as `(width, height)`.
    fn size(&self) -> (i32, i32) {
        (self.width(), self.height())
    }

    /// Move the widget to `(x, y)` relative to its parent.
    fn move_to(&self, x: i32, y: i32) {
        unsafe { ffi::QWidget_move(self.widget_ptr(), x, y); }
    }

    /// Move the widget to a `Point` position.
    fn move_to_point(&self, point: Point) {
        unsafe { ffi::QWidget_moveToPoint(self.widget_ptr(), point.to_raw()); }
    }

    /// Set the widget's geometry (position and size) in one call.
    fn set_geometry(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { ffi::QWidget_setGeometry(self.widget_ptr(), x, y, w, h); }
    }

    /// Get the widget's geometry as `(x, y, width, height)`.
    fn geometry(&self) -> (i32, i32, i32, i32) {
        (self.x(), self.y(), self.width(), self.height())
    }

    /// Returns `true` if the widget is visible.
    fn is_visible(&self) -> bool {
        unsafe { ffi::QWidget_isVisible(self.widget_ptr()) }
    }

    /// Returns `true` if the widget is enabled.
    fn is_enabled(&self) -> bool {
        unsafe { ffi::QWidget_isEnabled(self.widget_ptr()) }
    }

    /// Returns `true` if the widget is hidden.
    fn is_hidden(&self) -> bool {
        unsafe { ffi::QWidget_isHidden(self.widget_ptr()) }
    }

    /// Get the window title (returns empty string if no title set).
    fn window_title(&self) -> String {
        unsafe { ffi::QWidget_windowTitle(self.widget_ptr()) }
    }

    /// Give keyboard focus to this widget.
    fn set_focus(&self) {
        unsafe { ffi::QWidget_setFocus(self.widget_ptr()); }
    }

    /// Returns `true` if the widget has keyboard focus.
    fn has_focus(&self) -> bool {
        unsafe { ffi::QWidget_hasFocus(self.widget_ptr()) }
    }

    /// Remove keyboard focus from this widget.
    fn clear_focus(&self) {
        unsafe { ffi::QWidget_clearFocus(self.widget_ptr()); }
    }

    /// Set the Qt object name (used for `findChild` and stylesheets).
    fn set_object_name(&self, name: &str) {
        let_cxx_string!(c_name = name);
        unsafe { ffi::QWidget_setObjectName(self.widget_ptr(), &c_name); }
    }

    /// Get the Qt object name.
    fn object_name(&self) -> String {
        unsafe { ffi::QWidget_objectName(self.widget_ptr()) }
    }

    /// Schedule a repaint. Qt coalesces multiple `update()` calls
    /// into a single paint event for efficiency.
    fn update(&self) {
        unsafe { ffi::QWidget_update(self.widget_ptr()); }
    }

    /// Immediately repaint the widget (use sparingly; prefer [`update`](Self::update)).
    fn repaint(&self) {
        unsafe { ffi::QWidget_repaint(self.widget_ptr()); }
    }

    /// Close this widget. Returns `true` if the widget was closed.
    fn close(&self) {
        unsafe { ffi::QWidget_close(self.widget_ptr()); }
    }

    /// Get the parent widget, or `None` if this is a top-level window.
    fn parent_widget(&self) -> Option<Widget> {
        let ptr = unsafe { ffi::QWidget_parentWidget(self.widget_ptr()) };
        if ptr.is_null() { None }
        else { Some(Widget::from_raw(ptr, true)) }
    }

    /// Get the minimum width constraint.
    fn minimum_width(&self) -> i32 {
        unsafe { ffi::QWidget_minimumWidth(self.widget_ptr()) }
    }

    /// Get the minimum height constraint.
    fn minimum_height(&self) -> i32 {
        unsafe { ffi::QWidget_minimumHeight(self.widget_ptr()) }
    }

    /// Get the maximum width constraint.
    fn maximum_width(&self) -> i32 {
        unsafe { ffi::QWidget_maximumWidth(self.widget_ptr()) }
    }

    /// Get the maximum height constraint.
    fn maximum_height(&self) -> i32 {
        unsafe { ffi::QWidget_maximumHeight(self.widget_ptr()) }
    }

    /// Raise this widget to the top of the parent's widget stack.
    fn raise_widget(&self) {
        unsafe { ffi::QWidget_raiseWidget(self.widget_ptr()); }
    }

    /// Lower this widget to the bottom of the parent's widget stack.
    fn lower_widget(&self) {
        unsafe { ffi::QWidget_lowerWidget(self.widget_ptr()); }
    }

    /// Returns `true` if the widget is minimized (iconified).
    fn is_minimized(&self) -> bool {
        unsafe { ffi::QWidget_isMinimized(self.widget_ptr()) }
    }

    /// Returns `true` if the widget is maximized.
    fn is_maximized(&self) -> bool {
        unsafe { ffi::QWidget_isMaximized(self.widget_ptr()) }
    }

    /// Set the mouse cursor shape for this widget.
    fn set_cursor(&self, shape: i32) {
        unsafe { ffi::QWidget_setCursor(self.widget_ptr(), shape); }
    }

    /// Restore the default cursor for this widget.
    fn unset_cursor(&self) {
        unsafe { ffi::QWidget_unsetCursor(self.widget_ptr()); }
    }

    /// Adjust the widget size to fit its contents.
    fn adjust_size(&self) {
        unsafe { ffi::QWidget_adjustSize(self.widget_ptr()); }
    }

    /// Returns `true` if this widget's window is the active window.
    fn is_active_window(&self) -> bool {
        unsafe { ffi::QWidget_isActiveWindow(self.widget_ptr()) }
    }

    /// Returns `true` if the mouse cursor is over this widget.
    fn under_mouse(&self) -> bool {
        unsafe { ffi::QWidget_underMouse(self.widget_ptr()) }
    }

    /// Returns `true` if this widget is an independent window.
    fn is_window(&self) -> bool {
        unsafe { ffi::QWidget_isWindow(self.widget_ptr()) }
    }

    /// Get the top-level ancestor window.
    fn window(&self) -> Option<Widget> {
        let ptr = unsafe { ffi::QWidget_window(self.widget_ptr()) };
        if ptr.is_null() { None } else { Some(Widget::from_raw(ptr, true)) }
    }

    /// Set the window opacity (0.0 = transparent, 1.0 = opaque).
    fn set_window_opacity(&self, opacity: f64) {
        unsafe { ffi::QWidget_setWindowOpacity(self.widget_ptr(), opacity); }
    }

    /// Set a fixed width for this widget.
    fn set_fixed_width(&self, width: i32) {
        unsafe { ffi::QWidget_setFixedWidth(self.widget_ptr(), width); }
    }

    /// Set a fixed height for this widget.
    fn set_fixed_height(&self, height: i32) {
        unsafe { ffi::QWidget_setFixedHeight(self.widget_ptr(), height); }
    }

    /// Enable mouse tracking (mouse move events even without buttons pressed).
    fn set_mouse_tracking(&self, enable: bool) {
        unsafe { ffi::QWidget_setMouseTracking(self.widget_ptr(), enable); }
    }

    /// Returns `true` if mouse tracking is enabled.
    fn has_mouse_tracking(&self) -> bool {
        unsafe { ffi::QWidget_hasMouseTracking(self.widget_ptr()) }
    }

    /// Enable or disable drop events for this widget.
    fn set_accept_drops(&self, enable: bool) {
        unsafe { ffi::QWidget_setAcceptDrops(self.widget_ptr(), enable); }
    }

    /// Set whether the widget background is filled automatically.
    fn set_auto_fill_background(&self, enable: bool) {
        unsafe { ffi::QWidget_setAutoFillBackground(self.widget_ptr(), enable); }
    }

    /// Show the widget in full-screen mode.
    fn show_full_screen(&self) {
        unsafe { ffi::QWidget_showFullScreen(self.widget_ptr()); }
    }

    /// Show the widget maximized.
    fn show_maximized(&self) {
        unsafe { ffi::QWidget_showMaximized(self.widget_ptr()); }
    }

    /// Show the widget minimized (iconified).
    fn show_minimized(&self) {
        unsafe { ffi::QWidget_showMinimized(self.widget_ptr()); }
    }

    /// Restore the widget to normal size after being maximized or minimized.
    fn show_normal(&self) {
        unsafe { ffi::QWidget_showNormal(self.widget_ptr()); }
    }

    /// Set the size policy for both horizontal and vertical directions.
    fn set_size_policy(&self, h_policy: i32, v_policy: i32) {
        unsafe { ffi::QWidget_setSizePolicy(self.widget_ptr(), h_policy, v_policy); }
    }
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
/// - If the widget **has** a Qt parent: all signals are disconnected first,
///   then closures are reclaimed. The C++ object is left alone (Qt deletes
///   it when the parent is destroyed).
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

    /// Install any layout (unified API — works with all layout types).
    ///
    /// ```no_run
    /// # use qtrs::*;
    /// let mut window = Widget::new().build();
    /// let vbox = VBoxLayout::with_parent(&window);
    /// window.set_layout(&vbox);
    /// ```
    pub fn set_layout(&mut self, layout: &impl crate::layout::AsLayout) {
        assert!(!self.ptr.is_null(), "Widget::set_layout on null pointer");
        let lp = layout.layout_ptr();
        assert!(!lp.is_null(), "set_layout with null layout");
        unsafe { ffi::QWidget_setLayout(self.ptr, lp); }
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

    /// Move widget to (x, y) coordinates.
    pub fn move_to(&self, x: i32, y: i32) {
        debug_assert!(!self.ptr.is_null(), "Widget::move_to on null pointer");
        unsafe { ffi::QWidget_move(self.ptr, x, y); }
    }

    /// Move widget to a Point position.
    pub fn move_to_point(&self, point: Point) {
        debug_assert!(!self.ptr.is_null(), "Widget::move_to_point on null pointer");
        unsafe { ffi::QWidget_moveToPoint(self.ptr, point.to_raw()); }
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
        assert!(!self.ptr.is_null(), "Widget::find on null pointer");
        let_cxx_string!(c_name = name);

        // Generate all find arms from a compact spec in a single macro call.
        // Entries separated by `;`. Last token before `;` is `0` (no name) or `1` (with name).
        macro_rules! find_match {
            ($($kind:ident $ffi:ident $found:ident $use_name:ident $ty:path);* $(;)?) => {
                match kind {
                    $(
                        WidgetKind::$kind => {
                            let ptr = unsafe { ffi::$ffi(self.ptr, &c_name) };
                            if ptr.is_null() { None }
                            else {
                                find_match!(@raw $use_name, $found, $ty, ptr)
                            }
                        }
                    ),*
                    WidgetKind::Any => {
                        let ptr = unsafe { ffi::QWidget_findWidget(self.ptr, &c_name) };
                        if ptr.is_null() { None }
                        else { Some(FoundWidget::Widget(Widget::from_raw(ptr, true))) }
                    }
                }
            };
            (@raw YES, $found:ident, $ty:path, $ptr:ident) => {
                Some(FoundWidget::$found(<$ty>::from_raw($ptr, name)))
            };
            (@raw NO, $found:ident, $ty:path, $ptr:ident) => {
                Some(FoundWidget::$found(<$ty>::from_raw($ptr)))
            };
        }

        find_match! {
            PushButton QWidget_findPushButton PushButton YES crate::PushButton;
            LineEdit QWidget_findLineEdit LineEdit YES crate::LineEdit;
            CheckBox QWidget_findCheckBox CheckBox YES crate::CheckBox;
            ComboBox QWidget_findComboBox ComboBox YES crate::ComboBox;
            Slider QWidget_findSlider Slider YES crate::Slider;
            TextEdit QWidget_findTextEdit TextEdit YES crate::TextEdit;
            Label QWidget_findLabel Label YES crate::Label;
            ProgressBar QWidget_findProgressBar ProgressBar NO crate::ProgressBar;
            RadioButton QWidget_findRadioButton RadioButton NO crate::RadioButton;
            GroupBox QWidget_findGroupBox GroupBox NO crate::GroupBox;
            TabWidget QWidget_findTabWidget TabWidget NO crate::TabWidget;
            SpinBox QWidget_findSpinBox SpinBox NO crate::SpinBox;
            ListWidget QWidget_findListWidget ListWidget YES crate::ListWidget;
            ProgressDialog QWidget_findProgressDialog ProgressDialog NO crate::ProgressDialog;
            ScrollArea QWidget_findScrollArea ScrollArea NO crate::ScrollArea;
            TableWidget QWidget_findTableWidget TableWidget NO crate::TableWidget;
            TreeWidget QWidget_findTreeWidget TreeWidget NO crate::TreeWidget;
            StackedWidget QWidget_findStackedWidget StackedWidget NO crate::StackedWidget;
            Splitter QWidget_findSplitter Splitter NO crate::Splitter;
            DateEdit QWidget_findDateEdit DateEdit YES crate::DateEdit;
            TimeEdit QWidget_findTimeEdit TimeEdit YES crate::TimeEdit;
            DateTimeEdit QWidget_findDateTimeEdit DateTimeEdit YES crate::DateTimeEdit;
            PlainTextEdit QWidget_findPlainTextEdit PlainTextEdit YES crate::PlainTextEdit;
            TextBrowser QWidget_findTextBrowser TextBrowser YES crate::TextBrowser;
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
    ListWidget,
    ProgressDialog,
    ScrollArea,
    TableWidget,
    TreeWidget,
    StackedWidget,
    Splitter,
    DateEdit,
    TimeEdit,
    DateTimeEdit,
    PlainTextEdit,
    TextBrowser,
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
    ListWidget(crate::ListWidget),
    ProgressDialog(crate::ProgressDialog),
    ScrollArea(crate::ScrollArea),
    TableWidget(crate::TableWidget),
    TreeWidget(crate::TreeWidget),
    StackedWidget(crate::StackedWidget),
    Splitter(crate::Splitter),
    DateEdit(crate::DateEdit),
    TimeEdit(crate::TimeEdit),
    DateTimeEdit(crate::DateTimeEdit),
    PlainTextEdit(crate::PlainTextEdit),
    TextBrowser(crate::TextBrowser),
    Widget(Widget),
}

#[macro_export]
macro_rules! find {
    ($w:expr, $kind:ident, $name:literal) => {
        match $w.find(WidgetKind::$kind, $name) {
            Some(FoundWidget::$kind(w)) => w,
            _ => panic!("widget '{}' not found", $name),
        }
    };
    ($w:expr, $kind:ident, $name:literal, $msg:literal) => {
        match $w.find(WidgetKind::$kind, $name) {
            Some(FoundWidget::$kind(w)) => w,
            _ => panic!($msg),
        }
    };
    ($w:expr, $kind:ident, $name:literal, $msg:expr) => {
        match $w.find(WidgetKind::$kind, $name) {
            Some(FoundWidget::$kind(w)) => w,
            _ => panic!("{}", $msg),
        }
    };
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
/// # use qtrs::prelude::*;
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
        assert!(!ptr.is_null(), "QWidget_new returned null");

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
