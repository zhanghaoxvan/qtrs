# qtrs — Rust-style Qt6 bindings

[![Crates.io](https://img.shields.io/crates/v/qtrs.svg)](https://crates.io/crates/qtrs)
[![Docs.rs](https://docs.rs/qtrs/badge.svg)](https://docs.rs/qtrs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

![Demo](https://github.com/zhanghaoxvan/qtrs/blob/main/assets/demo.png)

A type-safe, builder-pattern-driven Qt6 GUI library for Rust. Built on
[`cxx`](https://cxx.rs) for zero-cost C++ interop, with RAII memory
management and signal/callback bridging.

## Features

- Builder pattern — chain `.title("X").size(800, 600).build()`
- RAII cleanup — automatic C++ deletion, parent-child aware (no double-free)
- Signal bridging — Qt signals invoke Rust closures via a global trampoline
- Layout ownership — adding a widget to a layout transfers ownership
- .ui file loading — load Qt Designer `.ui` files at runtime
- Compile-time embedding — ship `.ui` and `.qrc` resources inside the binary (`qtrs-build`)
- Zero unsafe in public API — all FFI is encapsulated

## Quick Start

```rust
use qtrs::prelude::*;

fn main() {
    let app = Application::new();

    let mut window = Widget::new()
        .title("Hello, qtrs!")
        .size(400, 300)
        .build();

    let mut layout = VBoxLayout::with_parent(&window);

    let btn = PushButton::new("Click me")
        .on_clicked(|| println!("clicked!"))
        .build();
    let label = Label::new("Welcome!").build();

    layout.add(btn);
    layout.add(label);

    window.set_layout(&layout);
    window.show();

    app.exec();
}
```

You can see [demo.rs](https://github.com/zhanghaoxvan/qtrs/blob/main/examples/demo/demo.rs) for details.

## Widgets

### Windows & Containers
| Type | Qt Class | Signals |
|------|----------|---------|
| `Application` | `QApplication` | — |
| `Widget` | `QWidget` | — |
| `MainWindow` | `QMainWindow` | — |
| `GroupBox` | `QGroupBox` | — |
| `TabWidget` | `QTabWidget` | `on_current_changed(i32)` |
| `StackedWidget` | `QStackedWidget` | `on_current_changed(i32)` |
| `ScrollArea` | `QScrollArea` | — |
| `Splitter` | `QSplitter` | — |

### Buttons & Controls
| Type | Qt Class | Signals |
|------|----------|---------|
| `PushButton` | `QPushButton` | `on_clicked` |
| `ToolButton` | `QToolButton` | `on_clicked`<br>`on_toggled(bool)` |
| `CheckBox` | `QCheckBox` | `on_toggled(bool)` |
| `RadioButton` | `QRadioButton` | `on_toggled(bool)` |
| `Slider` | `QSlider` | `on_value_changed(i32)` |
| `SpinBox` | `QSpinBox` | `on_value_changed(i32)` |
| `ComboBox` | `QComboBox` | `on_current_text_changed`<br>`on_current_index_changed(i32)` |

### Text & Display
| Type | Qt Class | Signals |
|------|----------|---------|
| `Label` | `QLabel` | — |
| `LineEdit` | `QLineEdit` | `on_return_pressed` |
| `TextEdit` | `QTextEdit` | `on_text_changed` |
| `PlainTextEdit` | `QPlainTextEdit` | `on_text_changed`<br>`on_cursor_position_changed` |
| `TextBrowser` | `QTextBrowser` | `on_anchor_clicked(String)`<br>`on_text_changed` |
| `ProgressBar` | `QProgressBar` | — |

### Item Views
| Type | Qt Class | Signals |
|------|----------|---------|
| `ListWidget` | `QListWidget` | `on_item_clicked(String)`<br>`on_item_double_clicked(String)`<br>`on_current_item_changed(String)` |
| `TableWidget` | `QTableWidget` | `on_cell_clicked`<br>`on_cell_double_clicked`<br>`on_current_cell_changed` |
| `TreeWidget` | `QTreeWidget` | `on_item_clicked(String)`<br>`on_item_double_clicked(String)`<br>`on_item_expanded(String)`<br>`on_item_collapsed(String)`<br>`on_current_item_changed(String)` |

### Date & Time
| Type | Qt Class | Signals |
|------|----------|---------|
| `DateEdit` | `QDateEdit` | `on_date_changed(String)` |
| `TimeEdit` | `QTimeEdit` | `on_time_changed(String)` |
| `DateTimeEdit` | `QDateTimeEdit` | `on_date_time_changed(String)` |
| `CalendarWidget` | `QCalendarWidget` | `on_selection_changed`<br>`on_activated(String)` |

### Menus & Toolbars
| Type | Qt Class | Signals |
|------|----------|---------|
| `Action` | `QAction` | `on_triggered(bool)`<br>`on_toggled(bool)` |
| `Menu` | `QMenu` | — |
| `MenuBar` | `QMenuBar` | — |
| `ToolBar` | `QToolBar` | per-action callbacks via `add_action` |
| `StatusBar` | `QStatusBar` | — |

### Dialogs
| Type | Qt Class | Notes |
|------|----------|-------|
| `MessageBox` | `QMessageBox` | Builder + `exec()`, static `about()` |
| `FileDialog` | `QFileDialog` | `open_file`, `save_file`, `select_directory` (static) |
| `ProgressDialog` | `QProgressDialog` | Builder with `set_value` / `was_canceled` |
| `InputDialog` | `QInputDialog` | `get_text`, `get_int`, `get_double`, `get_item` (static) |
| `MessageBox` | `QMessageBox` | `information`, `warning`, `critical`, `question` (static) |

### Layouts
| Type | Qt Class | Signals |
|------|----------|---------|
| `VBoxLayout` | `QVBoxLayout` | — |
| `HBoxLayout` | `QHBoxLayout` | — |
| `GridLayout` | `QGridLayout` | — |

### Styling & Utilities
| Type | Qt Class | Signals |
|------|----------|---------|
| `Frame` | `QFrame` | — (shape/shadow) |
| `Font` | `QFont` | — (family, size, weight builder) |
| `FontExt` | *(trait)* | `font()` / `set_font()` blanket-impl |
| `Shortcut` | `QShortcut` | `on_activated()` (QObject) |

### System
| Type | Qt Class | Signals |
|------|----------|---------|
| `Timer` | `QTimer` | `on_timeout`<br>`single_shot(ms, fn)` |
| `SystemTrayIcon` | `QSystemTrayIcon` | `on_activated(TrayIconReason)` |
| `UiLoader` | `QUiLoader` | `.ui` file loading |
| `Point` | `QPoint` | — |

## Prerequisites

Qt6 with development headers:

| Platform | Instructions |
|---|---|
| Debian / Ubuntu | `sudo apt install qt6-base-dev qt6-declarative-dev` |
| Fedora | `sudo dnf install qt6-qtbase-devel qt6-qtdeclarative-devel` |
| Arch | `sudo pacman -S qt6-base qt6-declarative` |
| macOS (Homebrew) | `brew install qt@6` |
| Windows | Install from [qt.io](https://www.qt.io/download-open-source) or via [vcpkg](https://vcpkg.io) |

Remember to set a `PATH` environment to tell `qtrs` where the `qmake` are.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
qtrs = "0.5.9"
```

## Compile-time `.ui` / `.qrc` embedding

By default `.ui` files are loaded from disk at runtime, and images / icons /
fonts are read from the filesystem. To ship everything inside a single
binary, use the companion [`qtrs-build`](qtrs-build/) crate: it runs Qt's
own `uic` and `rcc` at *compile time* and links the results into your final
executable.

Add it to your `build.rs`:

```toml
# Cargo.toml
[build-dependencies]
qtrs-build = "0.1"
```

```rust
// build.rs
fn main() {
    qtrs_build::Ui::embed().expect("embed ui files");
    qtrs_build::Rcc::embed().expect("embed resource files");
}
```

Directory conventions:

- Every `*.ui` file under `<package>/ui` becomes an embedded resource at
  `:/qrc/<name>.ui` — load it with `UiLoader::load(":/qrc/<name>.ui", None)`.
- Every `*.qrc` file under `<package>/resources` is embedded verbatim; its
  resources are available under `:/...` (e.g. a `<file alias="icon.png">`
  with prefix `/` resolves as `:/icon.png`).

Inside the binary the `.ui` XML and `.qrc` files are registered as Qt
resources by their `qInitResources_*` initializers before `main()` runs, so
no filesystem access is needed at runtime. See the
[`qtrs-build`](qtrs-build/) crate docs for customising the Qt tool lookup
(e.g. Qt5 vs Qt6 paths).

## Memory management

Widgets are deleted automatically on `Drop` — unless they have a Qt
parent (set explicitly or via layout). In that case Qt's parent-child tree
handles deletion, preventing double-free.

```
Widget created without parent  ->  Drop deletes C++ object
Widget created with parent     ->  Drop skips deletion (Qt handles it)
Widget added to layout         ->  Layout takes ownership, Drop skips deletion
```

## Thread safety

Qt GUI classes are **not thread-safe**. All widget creation, mutation, and
the event loop must happen on the main thread.

## License

MIT OR Apache-2.0 — see [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
