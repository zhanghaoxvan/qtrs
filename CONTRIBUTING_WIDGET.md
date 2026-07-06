# qtrs Widget Addition Guide

## Adding a New Qt Widget to qtrs

---

## Overview

This guide documents the **standard 5-layer process** for adding a new Qt widget to the qtrs framework. The process is demonstrated using a `Picture` widget (based on `QLabel`) as a working example, but the pattern applies to **any Qt widget**.

---

## Prerequisites

Before adding a new widget, you should be familiar with:
- **cxx crate**: C++/Rust interop fundamentals
- **Qt Widgets lifecycle**: Parent-child ownership, RAII vs manual `delete`
- **qtrs architecture**: `AsWidget` trait, Builder pattern, signal bridging
- Required Qt version: **Qt 6.2+**
- Required Rust version: **1.70+** (MSRV)

---

## Project Structure Reference

```
qtrs/
├── src/
│   ├── cpp/                    # C++ glue layer
│   │   ├── qt_widget.h         # Umbrella header (Layer 2)
│   │   ├── {widget}.h          # New widget glue (Layer 1)
│   │   ├── signal.h            # Trampoline globals
│   │   ├── app.h
│   │   ├── button.h
│   │   └── ...
│   ├── ffi.rs                  # cxx bridge (Layer 3)
│   ├── {widget}.rs             # Rust wrapper (Layer 4)
│   ├── lib.rs                  # Library exports (Layer 5)
│   └── ...
└── Cargo.toml
```

---

## Quick Reference Checklist

| Step | File | Action |
|------|------|--------|
| 1 | `src/cpp/{widget}.h` | Create C++ inline wrapper functions |
| 2 | `src/cpp/qt_widget.h` | Add `#include "{widget}.h"` |
| 3 | `src/ffi.rs` | Add cxx bridge declarations |
| 4 | `src/{widget}.rs` | Create safe Rust wrapper + Builder + Drop |
| 5 | `src/lib.rs` | Add `pub mod {widget};` + `pub use {widget}::*;` |

---

## Layer 1: C++ Glue Code

Create `src/cpp/picture.h`:

```cpp
// src/cpp/picture.h — QLabel-based image widget
#pragma once

#include <QLabel>
#include <QPixmap>
#include <QString>
#include <string>

// ─── Constructor ──────────────────────────────────────────────
inline QLabel *Picture_new(QWidget *parent) {
    return new QLabel(parent);
}

// ─── Setters ──────────────────────────────────────────────────
inline void Picture_setPixmap(QLabel *pic, const std::string &path) {
    pic->setPixmap(QPixmap(QString::fromStdString(path)));
}

inline void Picture_setPixmapScaled(QLabel *pic, const std::string &path, int w, int h) {
    QPixmap pixmap(QString::fromStdString(path));
    pic->setPixmap(pixmap.scaled(w, h, Qt::KeepAspectRatio, Qt::SmoothTransformation));
}

inline void Picture_clear(QLabel *pic) {
    pic->clear();
}

// ─── Destructor ──────────────────────────────────────────────
inline void Picture_delete(QLabel *pic) { delete pic; }
```

### C++ Signal Support (Optional)

If your widget emits signals (e.g., `clicked`), add a custom subclass:

```cpp
// src/cpp/clickablelabel.h
#pragma once

#include <QLabel>
#include <QMouseEvent>
#include "signal.h"

class ClickableLabel : public QLabel {
    Q_OBJECT
public:
    explicit ClickableLabel(QWidget *parent = nullptr) : QLabel(parent) {}
    
protected:
    void mousePressEvent(QMouseEvent *event) override {
        Q_UNUSED(event);
        emit clicked();
    }
    
signals:
    void clicked();
};

// ─── Wrapper functions ──────────────────────────────────────
inline ClickableLabel *ClickableLabel_new(QWidget *parent) {
    return new ClickableLabel(parent);
}

inline void ClickableLabel_setPixmap(ClickableLabel *label, const std::string &path) {
    label->setPixmap(QPixmap(QString::fromStdString(path)));
}

inline void ClickableLabel_onClicked(ClickableLabel *label, uint64_t ctx) {
    QObject::connect(label, &ClickableLabel::clicked, [ctx]() {
        g_voidTrampoline(ctx);
    });
}

inline void ClickableLabel_delete(ClickableLabel *label) { delete label; }
```

---

## Layer 2: Umbrella Header

Add the include to `src/cpp/qt_widget.h`:

```cpp
// src/cpp/qt_widget.h — umbrella header for all C++ glue modules
#pragma once

#include "signal.h"
#include "app.h"
#include "widget.h"
#include "button.h"
#include "label.h"
#include "input.h"
#include "checkbox.h"
#include "combobox.h"
#include "textedit.h"
#include "slider.h"
#include "timer.h"
#include "layout.h"
#include "dialog.h"
#include "picture.h"      // <── ADD THIS LINE

#ifdef QTRS_HAS_UI
#    include "uiloader.h"
#endif
```

---

## Layer 3: cxx Bridge (`src/ffi.rs`)

Add these declarations inside the `unsafe extern "C++"` block:

```rust
// src/ffi.rs — add inside unsafe extern "C++" block

// ─── Picture (QLabel-based image widget) ────────────────────
unsafe fn Picture_new(parent: *mut QWidget) -> *mut QLabel;
unsafe fn Picture_setPixmap(pic: *mut QLabel, path: &CxxString);
unsafe fn Picture_setPixmapScaled(pic: *mut QLabel, path: &CxxString, width: i32, height: i32);
unsafe fn Picture_clear(pic: *mut QLabel);
unsafe fn Picture_delete(pic: *mut QLabel);

// ─── Upcast (already exists, ensure available) ──────────────
// unsafe fn toQWidget_QLabel(label: *mut QLabel) -> *mut QWidget;
```

### For `Widget::find()` Support

Add to the bridge:

```rust
unsafe fn QWidget_findPicture(parent: *mut QWidget, name: &CxxString) -> *mut QLabel;
```

And add to `src/cpp/layout.h`:

```cpp
inline QLabel *QWidget_findPicture(QWidget *parent, const std::string &name) {
    return parent->findChild<QLabel *>(QString::fromStdString(name));
}
```

---

## Layer 4: Rust Wrapper (`src/picture.rs`)

Create the complete Rust wrapper:

```rust
// src/picture.rs — Image display widget

use cxx::let_cxx_string;
use crate::ffi;
use crate::widget::AsWidget;

/// An image display widget.
///
/// Wraps [`QLabel`](https://doc.qt.io/qt-6/qlabel.html) to display
/// pixmaps from image files. Supports PNG, JPEG, BMP, GIF, SVG,
/// and other formats Qt can read.
///
/// # Example
///
/// ```ignore
/// use qtrs::Picture;
///
/// let image = Picture::new()
///     .pixmap("assets/logo.png")
///     .build();
/// ```
pub struct Picture {
    ptr: *mut ffi::QLabel,
    has_parent: bool,
}

impl Picture {
    /// Start building a new Picture widget.
    pub fn new() -> Builder {
        Builder::new()
    }

    /// Set the displayed image from a file path.
    pub fn set_pixmap(&self, path: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_path = path);
        unsafe { ffi::Picture_setPixmap(self.ptr, &c_path); }
    }

    /// Set image scaled to specific dimensions.
    pub fn set_pixmap_scaled(&self, path: &str, width: i32, height: i32) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_path = path);
        unsafe { ffi::Picture_setPixmapScaled(self.ptr, &c_path, width, height); }
    }

    /// Clear the displayed image.
    pub fn clear(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::Picture_clear(self.ptr); }
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QLabel) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true }
    }
}

impl AsWidget for Picture {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QLabel(self.ptr) }
    }

    fn set_has_parent(&mut self) {
        self.has_parent = true;
    }
}

impl Drop for Picture {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if !self.has_parent {
            unsafe { ffi::Picture_delete(self.ptr); }
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`Picture`].
pub struct Builder {
    pixmap: Option<String>,
    pixmap_scaled: Option<(String, i32, i32)>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            pixmap: None,
            pixmap_scaled: None,
            parent: None,
        }
    }

    /// Set the image from a file path (original size).
    pub fn pixmap(mut self, path: impl Into<String>) -> Self {
        self.pixmap = Some(path.into());
        self
    }

    /// Set the image scaled to specific dimensions.
    pub fn pixmap_scaled(mut self, path: impl Into<String>, width: i32, height: i32) -> Self {
        self.pixmap_scaled = Some((path.into(), width, height));
        self
    }

    /// Set parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Build the C++ QLabel and return the Rust wrapper.
    pub fn build(self) -> Picture {
        let ptr = unsafe {
            ffi::Picture_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());

        let pic = Picture {
            ptr,
            has_parent: self.parent.is_some(),
        };

        if let Some(path) = self.pixmap {
            pic.set_pixmap(&path);
        }
        if let Some((path, w, h)) = self.pixmap_scaled {
            pic.set_pixmap_scaled(&path, w, h);
        }

        pic
    }

    /// Build and immediately show.
    pub fn show(self) -> Picture {
        let pic = self.build();
        unsafe { ffi::QWidget_show(pic.widget_ptr()); }
        pic
    }
}
```

---

## Layer 5: Library Exports (`src/lib.rs`)

Add the new widget to the public API:

```rust
// src/lib.rs

pub mod picture;
pub use picture::Picture;

// ─── Add to prelude ──────────────────────────────────────────
pub mod prelude {
    pub use super::{
        Application, AsLayout, AsWidget, CheckBox, ComboBox, FoundWidget,
        GridLayout, HBoxLayout, Label, LineEdit, PushButton, Slider,
        TextEdit, Timer, VBoxLayout, Widget, WidgetKind,
        Picture,  // <── ADD THIS
    };
    #[cfg(feature = "ui")]
    pub use super::UiLoader;
}
```

---

## Complete Usage Example

```rust
use qtrs::prelude::*;

fn main() {
    let app = Application::new();

    let window = Widget::new()
        .title("Image Viewer")
        .size(800, 600)
        .show();

    let mut layout = VBoxLayout::with_parent(&window);

    // Display an image
    let logo = Picture::new()
        .pixmap_scaled("assets/logo.png", 400, 300)
        .build();

    let label = Label::new("Hello, qtrs!").build();

    layout.add_widget(Box::new(logo));
    layout.add_widget(Box::new(label));

    window.set_vlayout(layout.layout_ptr());

    app.exec();
}
```

---

## Adding `Widget::find()` Support (Optional)

To enable runtime widget lookup by `objectName`:

### 1. Update `WidgetKind` enum (`src/widget.rs`)

```rust
pub enum WidgetKind {
    PushButton,
    LineEdit,
    CheckBox,
    Label,
    Picture,   // <── ADD
    Any,
}
```

### 2. Update `FoundWidget` enum (`src/widget.rs`)

```rust
pub enum FoundWidget {
    PushButton(crate::PushButton),
    LineEdit(crate::LineEdit),
    CheckBox(crate::CheckBox),
    Label(crate::Label),
    Picture(crate::Picture),   // <── ADD
    Widget(Widget),
}
```

### 3. Add match arm in `Widget::find()` (`src/widget.rs`)

```rust
WidgetKind::Picture => {
    let ptr = unsafe { ffi::QWidget_findPicture(self.ptr, &c_name) };
    if ptr.is_null() { None }
    else { Some(FoundWidget::Picture(crate::Picture::from_raw(ptr))) }
}
```

---

## Supporting Widget Signals (Clickable Picture)

For widgets that emit signals, here are two approaches:

### Option A: Use `QPushButton` with Icon (Simpler)

```rust
let btn = PushButton::new("")
    .on_clicked(|| println!("Image clicked!"))
    .build();

// Set icon via C++ helper (not shown)
```

### Option B: Custom C++ Subclass (Full Control)

See the `ClickableLabel` example in Layer 1. Then follow the same 5-layer pattern:

1. C++ glue with `onClicked` signal connection
2. Include in umbrella header
3. FFI bridge with `ClickableLabel_onClicked`
4. Rust wrapper with `on_clicked` builder method
5. Export in lib.rs

---

## Troubleshooting

### cxx Compilation Errors

| Error | Likely Cause | Fix |
|-------|--------------|-----|
| `unknown type 'QLabel'` | Missing include in C++ header | Add `#include <QLabel>` |
| `cannot find function` | Header not included in `qt_widget.h` | Add `#include "{widget}.h"` |
| `undefined reference` | Function declared but not defined | Check C++ function signatures match FFI |

### Runtime Panics

| Panic | Likely Cause | Fix |
|-------|--------------|-----|
| `debug_assert!(!self.ptr.is_null())` | Widget not constructed | Check `*_new` returns non-null |
| Signal not firing | Trampoline not registered | Ensure `ensure_trampolines_registered()` called in `leak_*` functions |

### Memory Leaks

| Symptom | Likely Cause | Fix |
|---------|--------------|-----|
| Widget not destroyed on Drop | `has_parent` incorrectly set to `true` | Set `has_parent = false` for parentless widgets |
| Closures leak | Widget has parent and is dropped early | Keep Rust wrapper alive; or review `has_parent` logic |

---

## Summary Reference Table

| Component | What to Add |
|-----------|-------------|
| **C++ Header** | Constructor, setters, getters, destructor, optional signal connections |
| **Umbrella Header** | `#include "{widget}.h"` |
| **FFI Bridge** | `unsafe fn` declarations for all C++ functions |
| **Rust Wrapper** | Struct with `ptr` + `has_parent`, `AsWidget` impl, `Drop`, Builder |
| **Library Export** | `pub mod` + `pub use` in `lib.rs` + prelude |

---

## Applying the Pattern to Other Widgets

This 5-layer pattern works for **any Qt widget**:

| Qt Widget | C++ Type | Rust Type |
|-----------|----------|-----------|
| Progress bar | `QProgressBar` | `ProgressBar` |
| Radio button | `QRadioButton` | `RadioButton` |
| Group box | `QGroupBox` | `GroupBox` |
| Tab widget | `QTabWidget` | `TabWidget` |
| Stacked widget | `QStackedWidget` | `StackedWidget` |
| Scroll area | `QScrollArea` | `ScrollArea` |
| List widget | `QListWidget` | `ListWidget` |
| Tree widget | `QTreeWidget` | `TreeWidget` |
| Table widget | `QTableWidget` | `TableWidget` |

Simply replace `QLabel` with the target Qt class and adapt the setter functions accordingly.

---

## Document Version

- **Version:** 1.0
- **Qt Version:** 6.2+
- **Rust MSRV:** 1.70+
- **Last Updated:** 2026-07-06

---

*This document is part of the qtrs project contribution guidelines.*
