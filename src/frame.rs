//! Frame widget with customizable shape and shadow.
//!
//! Wraps [`QFrame`](https://doc.qt.io/qt-6/qframe.html).

use crate::ffi;
use crate::signal::SignalHandle;
use crate::widget::AsWidget;

/// Frame shape constants matching Qt's `QFrame::Shape`.
pub const NO_FRAME: i32 = 0;
pub const BOX: i32 = 1;
pub const PANEL: i32 = 2;
pub const STYLED_PANEL: i32 = 3;
pub const H_LINE: i32 = 4;
pub const V_LINE: i32 = 5;

/// Frame shadow constants matching Qt's `QFrame::Shadow`.
pub const PLAIN: i32 = 16;
pub const RAISED: i32 = 32;
pub const SUNKEN: i32 = 48;

/// A frame widget with configurable shape and shadow.
///
/// Use a **builder pattern**: [`Frame::new`] returns a [`Builder`],
/// chain configuration, then call `.build()`.
///
/// # Example
///
/// ```no_run
/// use qtrs::Frame;
///
/// let frame = Frame::new()
///     .set_frame_shape(qtrs::frame::BOX)
///     .set_frame_shadow(qtrs::frame::RAISED)
///     .build();
/// ```
pub struct Frame {
    ptr: *mut ffi::QFrame,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl Frame {
    /// Start building a new frame.
    pub fn new() -> Builder {
        Builder::new()
    }

    // --- Properties ---

    /// Set the frame shape.
    pub fn set_frame_shape(&self, shape: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QFrame_setFrameShape(self.ptr, shape); }
    }

    /// Set the frame shadow.
    pub fn set_frame_shadow(&self, shadow: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QFrame_setFrameShadow(self.ptr, shadow); }
    }

    /// Set the line width.
    pub fn set_line_width(&self, width: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QFrame_setLineWidth(self.ptr, width); }
    }

    /// Set the mid-line width.
    pub fn set_mid_line_width(&self, width: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QFrame_setMidLineWidth(self.ptr, width); }
    }

    /// Set the frame style (combined shape + shadow).
    pub fn set_frame_style(&self, style: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QFrame_setFrameStyle(self.ptr, style); }
    }

    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QFrame, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for Frame {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QFrame(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for Frame {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.signal_handles.clear();
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QFrame_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`Frame`].
pub struct Builder {
    frame_shape: Option<i32>,
    frame_shadow: Option<i32>,
    line_width: Option<i32>,
    mid_line_width: Option<i32>,
    frame_style: Option<i32>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            frame_shape: None,
            frame_shadow: None,
            line_width: None,
            mid_line_width: None,
            frame_style: None,
            parent: None,
        }
    }

    /// Set the frame shape.
    pub fn set_frame_shape(mut self, shape: i32) -> Self {
        self.frame_shape = Some(shape);
        self
    }

    /// Set the frame shadow.
    pub fn set_frame_shadow(mut self, shadow: i32) -> Self {
        self.frame_shadow = Some(shadow);
        self
    }

    /// Set the line width.
    pub fn set_line_width(mut self, width: i32) -> Self {
        self.line_width = Some(width);
        self
    }

    /// Set the mid-line width.
    pub fn set_mid_line_width(mut self, width: i32) -> Self {
        self.mid_line_width = Some(width);
        self
    }

    /// Set the frame style (combined shape + shadow).
    pub fn set_frame_style(mut self, style: i32) -> Self {
        self.frame_style = Some(style);
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QFrame` and return the Rust wrapper.
    pub fn build(self) -> Frame {
        let ptr = unsafe {
            ffi::QFrame_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let frame = Frame {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };
        if let Some(shape) = self.frame_shape {
            unsafe { ffi::QFrame_setFrameShape(ptr, shape); }
        }
        if let Some(shadow) = self.frame_shadow {
            unsafe { ffi::QFrame_setFrameShadow(ptr, shadow); }
        }
        if let Some(width) = self.line_width {
            unsafe { ffi::QFrame_setLineWidth(ptr, width); }
        }
        if let Some(width) = self.mid_line_width {
            unsafe { ffi::QFrame_setMidLineWidth(ptr, width); }
        }
        if let Some(style) = self.frame_style {
            unsafe { ffi::QFrame_setFrameStyle(ptr, style); }
        }
        frame
    }
}
