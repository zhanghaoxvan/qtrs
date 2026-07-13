//! # qtrs — Rust-style Qt6 bindings
//!
//! A type-safe, builder-pattern-driven Qt6 GUI library for Rust.
//! Built on [`cxx`] for C++ interop, with RAII memory management,
//! signal/callback bridging, and a fluent builder API.
//!
//! ## Installation
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! qtrs = "0.3.0"
//! ```
//!
//! ## Design principles
//!
//! - Builder pattern everywhere — chain `.title("X").size(800, 600).show()`
//! - RAII cleanup — widgets are automatically deleted on `Drop`, with
//!   parent-child awareness (widgets with a Qt parent skip C++ deletion to
//!   avoid double-free).
//! - Type-safe layout ownership — adding a widget to a layout transfers
//!   ownership; the layout drops its children when it goes out of scope.
//! - Zero-cost signal bridging — Qt signals invoke Rust closures via a
//!   single global trampoline; context tokens are plain `u64` values.
//!
//! ## Architecture
//!
//! | Layer | Crate module | What it does |
//! |---|---|---|
//! | C++ glue | *(internal)* | Inline wrappers around Qt5/Qt6 API |
//! | cxx bridge | [`ffi`] | Opaque C++ types + `extern "C++"` signatures |
//! | Safe wrappers | [`app`], [`widget`], [`button`], ... | Builder patterns, RAII, signals |
//! | Public API | *(this module)* | Re-exports of all safe types |
//!
//! ## Widget gallery
//!
//! ### Windows & Containers
//!
//! | Type | Qt class | Signals |
//! |---|---|---|
//! | [`Application`] | `QApplication` | — |
//! | [`Widget`] | `QWidget` | — |
//! | [`MainWindow`] | `QMainWindow` | — |
//! | [`GroupBox`] | `QGroupBox` | — |
//! | [`TabWidget`] | `QTabWidget` | `on_current_changed(i32)` |
//! | [`StackedWidget`] | `QStackedWidget` | `on_current_changed(i32)` |
//! | [`ScrollArea`] | `QScrollArea` | — |
//! | [`Splitter`] | `QSplitter` | — |
//!
//! ### Buttons & Controls
//!
//! | Type | Qt class | Signals |
//! |---|---|---|
//! | [`PushButton`] | `QPushButton` | `on_clicked` |
//! | [`CheckBox`] | `QCheckBox` | `on_toggled(bool)` |
//! | [`RadioButton`] | `QRadioButton` | `on_toggled(bool)` |
//! | [`Slider`] | `QSlider` | `on_value_changed(i32)` |
//! | [`SpinBox`] | `QSpinBox` | `on_value_changed(i32)` |
//! | [`ComboBox`] | `QComboBox` | `on_current_text_changed`<br>`on_current_index_changed(i32)` |
//!
//! ### Text & Display
//!
//! | Type | Qt class | Signals |
//! |---|---|---|
//! | [`Label`] | `QLabel` | — |
//! | [`LineEdit`] | `QLineEdit` | `on_return_pressed` |
//! | [`TextEdit`] | `QTextEdit` | `on_text_changed` |
//! | [`ProgressBar`] | `QProgressBar` | — |
//!
//! ### Item Views
//!
//! | Type | Qt class | Signals |
//! |---|---|---|
//! | [`ListWidget`] | `QListWidget` | `on_item_clicked(String)`<br>`on_item_double_clicked(String)`<br>`on_current_item_changed(String)` |
//! | [`TableWidget`] | `QTableWidget` | `on_cell_clicked`<br>`on_cell_double_clicked`<br>`on_current_cell_changed` |
//! | [`TreeWidget`] | `QTreeWidget` | `on_item_clicked(String)`<br>`on_item_double_clicked(String)`<br>`on_item_expanded(String)`<br>`on_item_collapsed(String)`<br>`on_current_item_changed(String)` |
//!
//! ### Menus & Toolbars
//!
//! | Type | Qt class | Signals |
//! |---|---|---|
//! | [`Action`] | `QAction` | `on_triggered(bool)`<br>`on_toggled(bool)` |
//! | [`Menu`] | `QMenu` | — |
//! | [`MenuBar`] | `QMenuBar` | — |
//! | [`ToolBar`] | `QToolBar` | per-action callbacks via `add_action` |
//! | [`StatusBar`] | `QStatusBar` | — |
//!
//! ### Dialogs
//!
//! | Type | Qt class | Notes |
//! |---|---|---|
//! | [`MessageBox`] | `QMessageBox` | Builder + `exec()`, static `about()` |
//! | [`FileDialog`] | `QFileDialog` | `open_file`, `save_file`, `select_directory` (static) |
//! | [`ProgressDialog`] | `QProgressDialog` | Builder with `set_value` / `was_canceled` |
//! | `inputdialog` | `QInputDialog` | `get_text`, `get_int`, `get_double`, `get_item` (static) |
//! | `dialog` | `QMessageBox` | `information`, `warning`, `critical`, `question` (static) |
//!
//! ### Layouts
//!
//! | Type | Qt class | Signals |
//! |---|---|---|
//! | [`VBoxLayout`] | `QVBoxLayout` | — |
//! | [`HBoxLayout`] | `QHBoxLayout` | — |
//! | [`GridLayout`] | `QGridLayout` | — |
//!
//! ### System
//!
//! | Type | Qt class | Signals |
//! |---|---|---|
//! | [`Timer`] | `QTimer` | `on_timeout`<br>`single_shot(ms, fn)` |
//! | [`UiLoader`] | `QUiLoader` | `.ui` file loading |
//! | [`Point`] | `QPoint` | — |
//!
//! ## Quick example
//!
//! ```no_run
//! use qtrs::prelude::*;
//!
//! fn main() {
//!     let app = Application::new();
//!
//!     let mut window = Widget::new()
//!         .title("Hello, qtrs!")
//!         .size(400, 300)
//!         .build();
//!
//!     let mut layout = VBoxLayout::with_parent(&window);
//!
//!     let btn = PushButton::new("Click me")
//!         .on_clicked(|| println!("clicked!"))
//!         .build();
//!     let label = Label::new("Welcome!").build();
//!
//!     layout.add_widget(Box::new(btn));
//!     layout.add_widget(Box::new(label));
//!
//!     window.set_vlayout(layout.layout_ptr());
//!     window.show();
//!
//!     app.exec();
//! }
//! ```
//!
//! ## Thread safety
//!
//! Qt GUI classes are **not thread-safe**. All widget creation, mutation,
//! and the event loop must happen on the main thread. This library does
//! not add any synchronisation — you are responsible for staying on the
//! right thread, just as in C++ Qt.
//!
//! ## Linking
//!
//! Qt6 (Core, Gui, Widgets) must be installed with development headers.
//! The build script uses `qmake` to locate them. If you see
//! `Qt not found!`, install the `qt6-base-dev` package
//! (or equivalent) for your distribution.

pub mod action;
pub mod app;
pub mod button;
pub mod checkbox;
pub mod combobox;
pub mod conn;
pub mod dialog;
pub mod ffi;
pub mod grid;
pub mod groupbox;
pub mod input;
pub mod inputdialog;
pub mod label;
pub mod layout;
pub mod listwidget;
pub mod menu;
pub mod mainwindow;
pub mod messagebox;
pub mod progressbar;
pub mod progressdialog;
pub mod scrollarea;
pub mod splitter;
pub mod stackedwidget;
pub mod tablewidget;
pub mod treewidget;
pub mod radiobutton;
pub mod signals;
pub mod slider;
pub mod spinbox;
pub mod tabwidget;
pub mod statusbar;
pub mod textedit;
pub mod timer;
pub mod toolbar;
mod signal;
pub mod widget;
pub mod filedialog;
pub mod point;
pub mod loader;

// ================================================
// Re-Exports
// ================================================

pub use action::Action;
pub use app::Application;
pub use button::PushButton;
pub use checkbox::CheckBox;
pub use combobox::ComboBox;
pub use grid::GridLayout;
pub use input::LineEdit;
pub use inputdialog::{get_double, get_int, get_item, get_text};
pub use label::Label;
pub use layout::{AsLayout, HBoxLayout, VBoxLayout};
pub use listwidget::ListWidget;
pub use slider::Slider;
pub use textedit::TextEdit;
pub use timer::Timer;
pub use widget::{AsWidget, FoundWidget, Widget, WidgetKind};
pub use progressbar::ProgressBar;
pub use radiobutton::RadioButton;
pub use groupbox::GroupBox;
pub use tabwidget::TabWidget;
pub use spinbox::SpinBox;
pub use progressdialog::ProgressDialog;
pub use scrollarea::ScrollArea;
pub use splitter::Splitter;
pub use stackedwidget::StackedWidget;
pub use statusbar::StatusBar;
pub use tablewidget::TableWidget;
pub use toolbar::ToolBar;
pub use treewidget::TreeWidget;
pub use mainwindow::MainWindow;
pub use menu::{Menu, MenuBar};
pub use messagebox::{
    MessageBox, NO_ICON, INFORMATION, WARNING, CRITICAL, QUESTION,
    OK, CANCEL, YES, NO, CLOSE, SAVE, DISCARD, APPLY, RESET,
    RESTORE_DEFAULTS, HELP, SAVE_ALL, YES_TO_ALL, NO_TO_ALL,
    ABORT, RETRY, IGNORE, NO_BUTTON,
};
pub use conn::{ConnectExt, ConnType, SignalMeta, SlotMeta};
pub use filedialog::FileDialog;
pub use point::Point;
pub use loader::UiLoader;


// ============================================================
// Prelude
// ============================================================

/// Prelude: commonly used types and traits.
pub mod prelude {
    pub use super::{
        Action, Application, AsLayout, AsWidget, CheckBox, ComboBox, FoundWidget,
        GridLayout, HBoxLayout, Label, LineEdit, PushButton, Slider,
        TextEdit, Timer, VBoxLayout, Widget, WidgetKind,
        ProgressBar, RadioButton, GroupBox, TabWidget, SpinBox, ListWidget,
        Menu, MenuBar,
        MainWindow, StatusBar, ToolBar, MessageBox,
        ProgressDialog, ScrollArea, Splitter, StackedWidget,
        TableWidget, TreeWidget,
        ConnectExt, ConnType, signals, FileDialog, Point, UiLoader,
        get_text, get_int, get_double, get_item,
    };
}