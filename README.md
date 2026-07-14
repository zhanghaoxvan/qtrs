# qtrs — Rust-style Qt6 bindings

[![Crates.io](https://img.shields.io/crates/v/qtrs.svg)](https://crates.io/crates/qtrs)
[![Docs.rs](https://docs.rs/qtrs/badge.svg)](https://docs.rs/qtrs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

![Demo](https://github.com/zhanghaoxvan/qtrs/blob/main/assets/demo.png)

A type-safe, builder-pattern-driven Qt6 GUI library for Rust. Built on
[`cxx`](https://cxx.rs) for zero-cost C++ interop, with RAII memory
management and signal/callback bridging.

## Features

- Builder pattern — chain `.title("X").size(800, 600).show()`
- RAII cleanup — automatic C++ deletion, parent-child aware (no double-free)
- Signal bridging — Qt signals invoke Rust closures via a global trampoline
- Layout ownership — adding a widget to a layout transfers ownership
- .ui file loading — load Qt Designer `.ui` files at runtime
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

    layout.add_widget(Box::new(btn));
    layout.add_widget(Box::new(label));

    window.set_vlayout(layout.layout_ptr());
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
| `ProgressBar` | `QProgressBar` | — |

### Item Views
| Type | Qt Class | Signals |
|------|----------|---------|
| `ListWidget` | `QListWidget` | `on_item_clicked(String)`<br>`on_item_double_clicked(String)`<br>`on_current_item_changed(String)` |
| `TableWidget` | `QTableWidget` | `on_cell_clicked`<br>`on_cell_double_clicked`<br>`on_current_cell_changed` |
| `TreeWidget` | `QTreeWidget` | `on_item_clicked(String)`<br>`on_item_double_clicked(String)`<br>`on_item_expanded(String)`<br>`on_item_collapsed(String)`<br>`on_current_item_changed(String)` |

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

### System
| Type | Qt Class | Signals |
|------|----------|---------|
| `Timer` | `QTimer` | `on_timeout`<br>`single_shot(ms, fn)` |
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
qtrs = "0.3.1"
```

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
