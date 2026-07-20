# Changelog

All notable changes to `qtrs` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
- **First public release** on crates.io.
- Core widgets: `Widget`, `MainWindow`, `Label`, `Button`, `LineEdit`, `TextEdit`,
  `PlainTextEdit`, `CheckBox`, `ComboBox`, `SpinBox`, `Slider`, `ProgressBar`,
  `ListWidget`, `TableWidget`, `TreeWidget`, `TabWidget`, `StackedWidget`,
  `Splitter`, `GroupBox`, `Frame`, `ScrollArea`.
- Layouts: `VBoxLayout`, `HBoxLayout`, `GridLayout`, `FormLayout`.
- Menus & toolbars: `Menu`, `MenuBar`, `ToolBar`, `Action`, `StatusBar`, `Shortcut`.
- Dialogs: `FileDialog`, `InputDialog`, `MessageBox`, `ProgressDialog`, `Dialog`.
- Type-safe signal-slot connections with compile-time checking.
- Qt Designer `.ui` file support via `UiLoader` (feature: `ui`).
- QML integration via `QmlEngine` (feature: `qml`).
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
- QML engine support (initial implementation, later removed and re-added as feature).
- Timer widget with `on_timeout` callback.
- Demo example showcasing `Slider` ↔ `SpinBox` ↔ `ProgressBar` synchronization.

### Changed
- Refined builder-pattern API for existing widgets.

### Fixed
- Cross-platform compatibility improvements.

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