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
//! qtrs = "0.2.4"
//! ```
//!
//! ## Features
//!
//! - **`ui`** — enables `.ui` file loading via `QUiLoader`.
//!   Requires `qt6-tools-dev` (Debian) or equivalent.
//!
//! ### Enable features:
//!
//! ```toml
//! [dependencies]
//! qtrs = { version = "0.2.4", features = ["ui"] }
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
//! | C++ glue | *(internal)* | Inline wrappers around Qt6 API |
//! | cxx bridge | [`ffi`] | Opaque C++ types + `extern "C++"` signatures |
//! | Safe wrappers | [`app`], [`widget`], [`button`], [`label`], [`input`], [`layout`] | Builder patterns, RAII, signals |
//! | Public API | *(this module)* | Re-exports of all safe types |
//!
//! ## Widget gallery
//!
//! | Type | Qt class | Signals |
//! |---|---|---|
//! | [`Application`] | `QApplication` | — |
//! | [`Widget`] | `QWidget` | — |
//! | [`PushButton`] | `QPushButton` | `on_clicked` |
//! | [`Label`] | `QLabel` | — |
//! | [`LineEdit`] | `QLineEdit` | `on_return_pressed` |
//! | [`CheckBox`] | `QCheckBox` | `on_toggled(bool)` |
//! | [`ComboBox`] | `QComboBox` | `on_current_text_changed`<br>`on_current_index_changed(i32)` |
//! | [`TextEdit`] | `QTextEdit` | `on_text_changed` |
//! | [`Slider`] | `QSlider` | `on_value_changed(i32)` |
//! | [`ProgressBar`] | `QProgressBar` | — |
//! | [`RadioButton`] | `QRadioButton` | `on_toggled(bool)` |
//! | [`GroupBox`] | `QGroupBox` | — |
//! | [`TabWidget`] | `QTabWidget` | `on_current_changed(i32)` |
//! | [`SpinBox`] | `QSpinBox` | `on_value_changed(i32)` |
//! | [`Menu`] | `QMenu` | — |
//! | [`MenuBar`] | `QMenuBar` | — |
//! | [`Timer`] | `QTimer` | `on_timeout`<br>`single_shot(ms, fn)` |
//! | [`VBoxLayout`] | `QVBoxLayout` | — |
//! | [`HBoxLayout`] | `QHBoxLayout` | — |
//! | [`GridLayout`] | `QGridLayout` | — |
//! | [`FileDialog`] | `QFileDialog` | `open_file`, `save_file`, `select_directory` (static) |
//! | `dialog` | `QMessageBox` | `information`, `warning`, `critical`, `question` |
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
//! The build script uses `pkg-config` to locate them. If you see
//! `Qt6 Library is not installed!`, install the `qt6-base-dev` package
//! (or equivalent) for your distribution.

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
pub mod label;
pub mod layout;
pub mod menu;
pub mod progressbar;
pub mod radiobutton;
pub mod signals;
pub mod slider;
pub mod spinbox;
pub mod tabwidget;
pub mod textedit;
pub mod timer;
mod signal;
pub mod widget;
pub mod filedialog;

#[cfg(feature = "ui")]
pub mod loader;

// ================================================
// Re-Exports
// ================================================

pub use app::Application;
pub use button::PushButton;
pub use checkbox::CheckBox;
pub use combobox::ComboBox;
pub use grid::GridLayout;
pub use input::LineEdit;
pub use label::Label;
pub use layout::{AsLayout, HBoxLayout, VBoxLayout};
pub use slider::Slider;
pub use textedit::TextEdit;
pub use timer::Timer;
pub use widget::{AsWidget, FoundWidget, Widget, WidgetKind};
pub use progressbar::ProgressBar;
pub use radiobutton::RadioButton;
pub use groupbox::GroupBox;
pub use tabwidget::TabWidget;
pub use spinbox::SpinBox;
pub use menu::{Menu, MenuBar};
pub use conn::{ConnectExt, ConnType, SignalMeta, SlotMeta};
pub use filedialog::FileDialog;

#[cfg(feature = "ui")]
pub use loader::UiLoader;


// ============================================================
// Prelude
// ============================================================

/// Prelude: commonly used types and traits.
pub mod prelude {
    pub use super::{
        Application, AsLayout, AsWidget, CheckBox, ComboBox, FoundWidget,
        GridLayout, HBoxLayout, Label, LineEdit, PushButton, Slider,
        TextEdit, Timer, VBoxLayout, Widget, WidgetKind,
        ProgressBar, RadioButton, GroupBox, TabWidget, SpinBox, Menu, MenuBar,
        ConnectExt, ConnType, signals, FileDialog
    };
    #[cfg(feature = "ui")]
    pub use super::UiLoader;
}