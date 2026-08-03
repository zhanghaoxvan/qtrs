//! Dock widget — a floating/dockable panel within a [`MainWindow`].
//!
//! Wraps [`QDockWidget`](https://doc.qt.io/qt-6/qdockwidget.html).

use cxx::let_cxx_string;
use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A dock widget that can be dragged, floated, or docked inside a
/// [`MainWindow`](crate::MainWindow).
///
/// # Signals
///
/// | Method | Qt signal |
/// |---|---|
/// | [`connect_visibility_changed`](Self::connect_visibility_changed) | `visibilityChanged` |
/// | [`connect_features_changed`](Self::connect_features_changed) | `featuresChanged` |
///
/// # Example
///
/// ```no_run
/// # use qtrs::prelude::*;
/// let dock = DockWidget::new()
///     .window_title("Tools")
///     .build();
/// ```
pub struct DockWidget {
    ptr: *mut ffi::QDockWidget,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl DockWidget {
    /// Start building a new dock widget.
    pub fn new() -> Builder { Builder::new() }

    /// Set the window title (shown in the dock title bar).
    pub fn set_window_title(&self, title: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_title = title);
        unsafe { ffi::QDockWidget_setWindowTitle(self.ptr, &c_title); }
    }

    /// Set the content widget inside the dock.
    pub fn set_widget(&self, w: &dyn AsWidget) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDockWidget_setWidget(self.ptr, w.widget_ptr()); }
    }

    /// Get the content widget, or null.
    #[doc(hidden)]
    pub fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDockWidget_widget(self.ptr) }
    }

    /// Set allowed dock features (bitwise OR of feature constants).
    pub fn set_features(&self, features: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDockWidget_setFeatures(self.ptr, features); }
    }

    /// Get the current features.
    pub fn features(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDockWidget_features(self.ptr) }
    }

    /// Set allowed dock areas (bitwise OR of area constants).
    pub fn set_allowed_areas(&self, areas: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDockWidget_setAllowedAreas(self.ptr, areas); }
    }

    /// Get the allowed areas.
    pub fn allowed_areas(&self) -> i32 {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDockWidget_allowedAreas(self.ptr) }
    }

    /// Set whether the dock is floating (not docked).
    pub fn set_floating(&self, floating: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDockWidget_setFloating(self.ptr, floating); }
    }

    /// Return `true` if the dock is floating.
    pub fn is_floating(&self) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDockWidget_isFloating(self.ptr) }
    }

    /// Show the dock widget.
    pub fn show(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDockWidget_show(self.ptr); }
    }

    /// Hide the dock widget.
    pub fn hide(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QDockWidget_hide(self.ptr); }
    }

    // --- Signal connections ---

    /// Connect a callback that fires when visibility changes.
    pub fn connect_visibility_changed<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QDockWidget_onVisibilityChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when features change.
    pub fn connect_features_changed<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QDockWidget_onFeaturesChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QDockWidget) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for DockWidget {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QDockWidget(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for DockWidget {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            unsafe { ffi::QWidget_disconnectAll(self.ptr as *mut _); }
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QDockWidget_delete(self.ptr); }
        }
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`DockWidget`].
pub struct Builder {
    window_title: Option<String>,
    features: Option<i32>,
    allowed_areas: Option<i32>,
    floating: Option<bool>,
    on_visibility_changed: Option<Box<dyn Fn()>>,
    on_features_changed: Option<Box<dyn Fn()>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            window_title: None,
            features: None,
            allowed_areas: None,
            floating: None,
            on_visibility_changed: None,
            on_features_changed: None,
            parent: None,
        }
    }

    /// Set the dock title.
    pub fn window_title(mut self, title: impl Into<String>) -> Self {
        self.window_title = Some(title.into());
        self
    }

    /// Set the allowed features.
    pub fn features(mut self, features: i32) -> Self {
        self.features = Some(features);
        self
    }

    /// Set the allowed dock areas.
    pub fn allowed_areas(mut self, areas: i32) -> Self {
        self.allowed_areas = Some(areas);
        self
    }

    /// Set whether the dock starts floating.
    pub fn floating(mut self, floating: bool) -> Self {
        self.floating = Some(floating);
        self
    }

    /// Called when visibility changes.
    pub fn on_visibility_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_visibility_changed = Some(Box::new(f));
        self
    }

    /// Called when features change.
    pub fn on_features_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_features_changed = Some(Box::new(f));
        self
    }

    /// Set the parent widget (usually a [`MainWindow`](crate::MainWindow)).
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the `QDockWidget` and return the Rust wrapper.
    pub fn build(self) -> DockWidget {
        let ptr = unsafe {
            ffi::QDockWidget_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        assert!(!ptr.is_null(), "QDockWidget_new returned null");
        let mut dw = DockWidget {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(ref title) = self.window_title {
            dw.set_window_title(title);
        }
        if let Some(features) = self.features {
            unsafe { ffi::QDockWidget_setFeatures(ptr, features); }
        }
        if let Some(areas) = self.allowed_areas {
            unsafe { ffi::QDockWidget_setAllowedAreas(ptr, areas); }
        }
        if let Some(floating) = self.floating {
            unsafe { ffi::QDockWidget_setFloating(ptr, floating); }
        }
        if let Some(f) = self.on_visibility_changed {
            let h = signal::leak_void(f);
            unsafe { ffi::QDockWidget_onVisibilityChanged(ptr, h.token); }
            dw.signal_handles.push(h);
        }
        if let Some(f) = self.on_features_changed {
            let h = signal::leak_void(f);
            unsafe { ffi::QDockWidget_onFeaturesChanged(ptr, h.token); }
            dw.signal_handles.push(h);
        }
        dw
    }
}
