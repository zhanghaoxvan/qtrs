# Changelog

All notable changes to `qtrs` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.5.0] - 2026-07-27

### Added
- **Model/View architecture**: `StandardItemModel`, `TableView`, `ListView`,
  `TreeView`, and `ItemSelectionModel` — the full Qt Model/View framework
  with proper data-model separation. All views support signal callbacks
  (clicked, double-clicked, expanded, collapsed) and builder-pattern
  construction.
- **24 new QWidget methods** added as default methods on the `AsWidget`
  trait, available on every widget automatically:
  - Size/position getters: `width()`, `height()`, `x()`, `y()`,
    `pos() -> Point`, `size() -> (i32, i32)`
  - Geometry: `set_geometry()`, `geometry() -> (x, y, w, h)`
  - State queries: `is_visible()`, `is_enabled()`, `is_hidden()`,
    `is_minimized()`, `is_maximized()`
  - Window title getter: `window_title() -> String`
  - Focus management: `set_focus()`, `has_focus()`, `clear_focus()`
  - Object name: `set_object_name()`, `object_name() -> String`
  - Repaint: `update()`, `repaint()`
  - Window actions: `close()`, `raise_widget()`, `lower_widget()`
  - Size limit getters: `minimum_width()`, `minimum_height()`,
    `maximum_width()`, `maximum_height()`
  - Parent access: `parent_widget() -> Option<Widget>`
- `QObject_disconnectAll()` for safely disconnecting all signals from
  non-widget QObjects (e.g. `ItemSelectionModel`)
- `slider_slots` module with `SET_VALUE` slot constant for `Slider`

### Fixed
- Undefined behavior in `signal.rs`: replaced `std::mem::transmute` with `Box::from_raw` when reconstructing boxed closures from raw pointers
- `TextEdit::build()` now calls `setPlaceholderText` instead of `setPlainText` for placeholder text
- Outdated documentation claiming signal closures are "leaked" when widget has parent — they are correctly reclaimed
- All 19 broken doctests — examples now use correct signal/slot constants from `qtrs::signals`

### Changed
- Updated `demo.rs` example to use correct signal/slot imports
- Updated internal doctests to use `# use qtrs::signals::{...}` pattern

---

## [0.4.2] - 2026-7-25

### Fixed
- Undefined behavior in `signal.rs`: replaced `std::mem::transmute` with `Box::from_raw` when reconstructing boxed closures from raw pointers
- `TextEdit::build()` now calls `setPlaceholderText` instead of `setPlainText` for placeholder text
- Outdated documentation claiming signal closures are "leaked" when widget has parent — they are correctly reclaimed
- All 19 broken doctests — examples now use correct signal/slot constants from `qtrs::signals`

### Added
- `slider_slots` module with `SET_VALUE` slot constant for `Slider`

### Changed
- Updated `demo.rs` example to use correct signal/slot imports
- Updated internal doctests to use `# use qtrs::signals::{...}` pattern

---

## [0.4.1] - 2026-7-20

### Changed
- Signal closures no longer require `'static` bounds. Capturing local references directly is now
  possible without `Rc<RefCell<>>` wrappers.

---

## [0.4.0] - 2026-07-17

### Changed
- **Overhaul:** All widgets refactored for a more human-friendly and idiomatic Rust API.
- Consistent Builder patterns across all widget types.
- Improved type signatures and error messages for better IDE support.

---

## [0.3.1] - 2026-07-14

### Fixed
- Wrong version number in build metadata.
- Various compiler warnings.

---

## [0.3.0] - 2026-07-13

### Added
- Core widgets: `Widget`, `MainWindow`, `Label`, `Button`, `LineEdit`, `TextEdit`,
  `PlainTextEdit`, `CheckBox`, `ComboBox`, `SpinBox`, `Slider`, `ProgressBar`,
  `ListWidget`, `TableWidget`, `TreeWidget`, `TabWidget`, `StackedWidget`,
  `Splitter`, `GroupBox`, `Frame`, `ScrollArea`.
- Layouts: `VBoxLayout`, `HBoxLayout`, `GridLayout`, `FormLayout`.
- Menus & toolbars: `Menu`, `MenuBar`, `ToolBar`, `Action`, `StatusBar`, `Shortcut`.
- Dialogs: `FileDialog`, `InputDialog`, `MessageBox`, `ProgressDialog`, `Dialog`.
- Type-safe signal-slot connections with compile-time checking.
- Qt Designer `.ui` file support via `UiLoader`.
- Markdown Editor example.
- Font support with builder-pattern `Font` struct.
- Timer widget with `on_timeout` callback.
- SystemTrayIcon support.
- Builder-pattern API for all widgets (`.new().property(value).build()`).
- `prelude` module for convenient imports.
- Cross-platform support (Windows, Linux, macOS).

---

## [0.2.0] - 2026-07-08

### Added
- **Type-safe signal-slot connections** with compile-time checking.
  - `SignalMeta` and `SlotMeta` traits for compile-time type validation.
  - `ConnectExt` trait with `connect()` and `disconnect()` methods.
  - Compile-time signal/slot constants for all widgets.
  - Thread safety checks (`QObject_isInGuiThread`).
- Additional widgets: `CheckBox`, `ComboBox`, `TextEdit`, `Slider`, `GridLayout`.
- Qt Designer `.ui` file loading with slot connections.
- Timer widget with `on_timeout` callback.
- Demo example showcasing `Slider` ↔ `SpinBox` ↔ `ProgressBar` synchronization.

### Changed
- Refined builder-pattern API for existing widgets.

### Fixed
- Cross-platform compatibility improvements.

### Removed
- QML support (removed to keep focus on QWidgets).

---

## [0.1.0] - 2026-07-04

### Added
- **Initial release**.
- Core Qt bindings: `Widget`, `MainWindow`, `Label`, `Button`, `LineEdit`, `PlainTextEdit`.
- Basic layouts: `VBoxLayout`, `HBoxLayout`.
- Signal-slot connections via closures.
- RAII-based memory management (automatic `Drop`).
- Minimal documentation and examples.
- QML and `.ui` file support (initial implementation).