//! Font object for widget text styling.
//!
//! Wraps [`QFont`](https://doc.qt.io/qt-6/qfont.html).

use cxx::let_cxx_string;

use crate::ffi;

/// A font object that can be applied to widgets.
///
/// `Font` uses a **builder pattern**: call [`Font::new`] to obtain a
/// [`Builder`], chain `.family()`, `.point_size()`, `.bold()`, then call
/// `.build()`.
///
/// # Memory safety
///
/// The underlying `QFont` is allocated on the heap and automatically
/// deleted when the `Font` wrapper is dropped.
///
/// # Example
///
/// ```no_run
/// use qtrs::Font;
///
/// let font = Font::new()
///     .family("WenQuanYi Micro Hei")
///     .point_size(14)
///     .bold(true)
///     .build();
///
/// label.set_font(&font);
/// ```
pub struct Font {
    ptr: *mut ffi::QFont,
}

impl Font {
    /// Start building a new `QFont`.
    ///
    /// Returns a [`Builder`]. Chain configuration, then call `.build()`.
    pub fn new() -> Builder {
        Builder::new()
    }

    /// Get the font family name.
    pub fn family(&self) -> String {
        debug_assert!(!self.ptr.is_null(), "Font::family on null pointer");
        unsafe { ffi::QFont_family(self.ptr) }
    }

    /// Get the point size.
    pub fn point_size(&self) -> i32 {
        debug_assert!(!self.ptr.is_null(), "Font::point_size on null pointer");
        unsafe { ffi::QFont_pointSize(self.ptr) }
    }

    /// Get the pixel size (or 0 if not set).
    pub fn pixel_size(&self) -> i32 {
        debug_assert!(!self.ptr.is_null(), "Font::pixel_size on null pointer");
        unsafe { ffi::QFont_pixelSize(self.ptr) }
    }

    /// Check if the font is bold.
    pub fn bold(&self) -> bool {
        debug_assert!(!self.ptr.is_null(), "Font::bold on null pointer");
        unsafe { ffi::QFont_bold(self.ptr) }
    }

    /// Check if the font is italic.
    pub fn italic(&self) -> bool {
        debug_assert!(!self.ptr.is_null(), "Font::italic on null pointer");
        unsafe { ffi::QFont_italic(self.ptr) }
    }

    /// Check if the font is underlined.
    pub fn underline(&self) -> bool {
        debug_assert!(!self.ptr.is_null(), "Font::underline on null pointer");
        unsafe { ffi::QFont_underline(self.ptr) }
    }

    /// Check if the font has strikethrough.
    pub fn strike_out(&self) -> bool {
        debug_assert!(!self.ptr.is_null(), "Font::strike_out on null pointer");
        unsafe { ffi::QFont_strikeOut(self.ptr) }
    }

    /// Get the font weight (0-99, 50 = normal, 75 = bold).
    pub fn weight(&self) -> i32 {
        debug_assert!(!self.ptr.is_null(), "Font::weight on null pointer");
        unsafe { ffi::QFont_weight(self.ptr) }
    }

    /// Wrap an existing `QFont*`.
    #[doc(hidden)]
    pub(crate) fn from_raw(ptr: *mut ffi::QFont) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr }
    }

    /// Get the raw pointer for C++ interop.
    #[doc(hidden)]
    pub fn raw_ptr(&self) -> *mut ffi::QFont {
        self.ptr
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe { ffi::QFont_delete(self.ptr) };
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`Font`].
///
/// Collects font properties, then creates the C++ `QFont` in
/// [`build`](Self::build).
pub struct Builder {
    family: Option<String>,
    point_size: Option<i32>,
    pixel_size: Option<i32>,
    bold: Option<bool>,
    italic: Option<bool>,
    underline: Option<bool>,
    strike_out: Option<bool>,
    weight: Option<i32>,
}

impl Builder {
    fn new() -> Self {
        Self {
            family: None,
            point_size: None,
            pixel_size: None,
            bold: None,
            italic: None,
            underline: None,
            strike_out: None,
            weight: None,
        }
    }

    /// Set the font family (e.g., "Arial", "WenQuanYi Micro Hei").
    pub fn family(mut self, family: impl Into<String>) -> Self {
        self.family = Some(family.into());
        self
    }

    /// Set the font size in points (1/72 inch).
    pub fn point_size(mut self, size: i32) -> Self {
        self.point_size = Some(size);
        self
    }

    /// Set the font size in pixels.
    pub fn pixel_size(mut self, size: i32) -> Self {
        self.pixel_size = Some(size);
        self
    }

    /// Enable bold font weight.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    /// Enable italic style.
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    /// Enable underline.
    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = Some(underline);
        self
    }

    /// Enable strikethrough.
    pub fn strike_out(mut self, strike_out: bool) -> Self {
        self.strike_out = Some(strike_out);
        self
    }

    /// Set the font weight (0-99).
    ///
    /// Common values:
    /// - 50 = Normal (default)
    /// - 75 = Bold
    /// - 25 = Light
    /// - 87 = Black
    pub fn weight(mut self, weight: i32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Create the C++ `QFont` and return the Rust wrapper.
    ///
    /// This is the terminal method of the builder pattern.
    pub fn build(self) -> Font {
        let ptr = unsafe { ffi::QFont_new() };
        debug_assert!(!ptr.is_null(), "QFont_new returned null");

        let font = Font { ptr };

        // Apply all configured properties
        if let Some(family) = &self.family {
            let_cxx_string!(c_family = family);
            unsafe { ffi::QFont_setFamily(font.ptr, &c_family); }
        }

        if let Some(size) = self.point_size {
            unsafe { ffi::QFont_setPointSize(font.ptr, size); }
        }

        if let Some(size) = self.pixel_size {
            unsafe { ffi::QFont_setPixelSize(font.ptr, size); }
        }

        if let Some(bold) = self.bold {
            unsafe { ffi::QFont_setBold(font.ptr, bold); }
        }

        if let Some(italic) = self.italic {
            unsafe { ffi::QFont_setItalic(font.ptr, italic); }
        }

        if let Some(underline) = self.underline {
            unsafe { ffi::QFont_setUnderline(font.ptr, underline); }
        }

        if let Some(strike_out) = self.strike_out {
            unsafe { ffi::QFont_setStrikeOut(font.ptr, strike_out); }
        }

        if let Some(weight) = self.weight {
            unsafe { ffi::QFont_setWeight(font.ptr, weight); }
        }

        font
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new().build()
    }
}

// ============================================================
// FontExt — blanket impl over all AsWidget types
// ============================================================

use crate::widget::AsWidget;

/// Extension trait that gives every widget `font()` / `set_font()`.
///
/// Blanket-implemented for all types that implement [`AsWidget`], so every
/// widget in qtrs automatically gets these methods.
pub trait FontExt: AsWidget {
    /// Get the current font of this widget.
    ///
    /// Returns a **copy** of the widget's font. Changes to the returned
    /// [`Font`] do not affect the widget — call [`set_font`](FontExt::set_font)
    /// to apply changes.
    fn font(&self) -> Font {
        let ptr = unsafe { crate::ffi::QWidget_font(self.widget_ptr()) };
        debug_assert!(!ptr.is_null(), "QWidget_font returned null");
        Font::from_raw(ptr)
    }

    /// Set the font for this widget.
    fn set_font(&self, font: &Font) {
        unsafe { crate::ffi::QWidget_setFont(self.widget_ptr(), font.raw_ptr()); }
    }
}

impl<T: AsWidget> FontExt for T {}