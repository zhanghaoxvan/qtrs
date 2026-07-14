//! Calendar widget for date selection.
//!
//! Wraps [`QCalendarWidget`](https://doc.qt.io/qt-6/qcalendarwidget.html).

use cxx::let_cxx_string;

use crate::ffi;
use crate::signal::{self, SignalHandle};
use crate::widget::AsWidget;

/// A calendar widget for date selection.
///
/// Use a **builder pattern**: [`CalendarWidget::new`] returns a [`Builder`],
/// chain configuration, then call `.build()`.
///
/// # Signals
///
/// | Method | Qt signal | Callback receives |
/// |---|---|---|
/// | [`Builder::on_selection_changed`] / [`CalendarWidget::connect_selection_changed`] | `QCalendarWidget::selectionChanged` | `()` |
/// | [`Builder::on_activated`] / [`CalendarWidget::connect_activated`] | `QCalendarWidget::activated` | `String` (date in `yyyy-MM-dd` format) |
///
/// # Example
///
/// ```no_run
/// use qtrs::CalendarWidget;
///
/// let cal = CalendarWidget::new()
///     .on_selection_changed(|| println!("date selected!"))
///     .build();
/// ```
pub struct CalendarWidget {
    ptr: *mut ffi::QCalendarWidget,
    has_parent: bool,
    signal_handles: Vec<SignalHandle>,
}

impl CalendarWidget {
    /// Start building a new calendar widget.
    pub fn new() -> Builder {
        Builder::new()
    }

    // --- Properties ---

    /// Set the selected date. Expects a date string in `"yyyy-MM-dd"` format.
    pub fn set_selected_date(&self, date: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_date = date);
        unsafe { ffi::QCalendarWidget_setSelectedDate(self.ptr, &c_date); }
    }

    /// Get the currently selected date as a `"yyyy-MM-dd"` string.
    pub fn selected_date(&self) -> String {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QCalendarWidget_selectedDate(self.ptr) }
    }

    /// Set the minimum date. Expects `"yyyy-MM-dd"` format.
    pub fn set_minimum_date(&self, date: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_date = date);
        unsafe { ffi::QCalendarWidget_setMinimumDate(self.ptr, &c_date); }
    }

    /// Set the maximum date. Expects `"yyyy-MM-dd"` format.
    pub fn set_maximum_date(&self, date: &str) {
        debug_assert!(!self.ptr.is_null());
        let_cxx_string!(c_date = date);
        unsafe { ffi::QCalendarWidget_setMaximumDate(self.ptr, &c_date); }
    }

    /// Set the first day of the week (0=Sunday, 1=Monday, ..., 6=Saturday).
    pub fn set_first_day_of_week(&self, day: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QCalendarWidget_setFirstDayOfWeek(self.ptr, day); }
    }

    /// Show or hide the grid.
    pub fn set_grid_visible(&self, visible: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QCalendarWidget_setGridVisible(self.ptr, visible); }
    }

    /// Show or hide the navigation bar.
    pub fn set_navigation_bar_visible(&self, visible: bool) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QCalendarWidget_setNavigationBarVisible(self.ptr, visible); }
    }

    // --- Signal connections (runtime) ---

    /// Connect a callback when the selection changes.
    pub fn connect_selection_changed<F: Fn() + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QCalendarWidget_onSelectionChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback when a date is activated (double-clicked or pressed).
    /// Receives the date as a `"yyyy-MM-dd"` string.
    pub fn connect_activated<F: Fn(String) + 'static>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_string(f);
        unsafe { ffi::QCalendarWidget_onActivated(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn from_raw(ptr: *mut ffi::QCalendarWidget, _name: &str) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr, has_parent: true, signal_handles: Vec::new() }
    }
}

impl AsWidget for CalendarWidget {
    fn widget_ptr(&self) -> *mut ffi::QWidget {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::toQWidget_QCalendarWidget(self.ptr) }
    }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}

impl Drop for CalendarWidget {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if self.has_parent {
            self.signal_handles.clear();
        } else {
            for h in self.signal_handles.drain(..) { unsafe { h.reclaim(); } }
            unsafe { ffi::QCalendarWidget_delete(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
    }
}

/// Builder for [`CalendarWidget`].
pub struct Builder {
    selected_date: Option<String>,
    minimum_date: Option<String>,
    maximum_date: Option<String>,
    first_day_of_week: Option<i32>,
    grid_visible: Option<bool>,
    navigation_bar_visible: Option<bool>,
    on_selection_changed: Option<Box<dyn Fn()>>,
    on_activated: Option<Box<dyn Fn(String)>>,
    parent: Option<*mut ffi::QWidget>,
}

impl Builder {
    fn new() -> Self {
        Self {
            selected_date: None,
            minimum_date: None,
            maximum_date: None,
            first_day_of_week: None,
            grid_visible: None,
            navigation_bar_visible: None,
            on_selection_changed: None,
            on_activated: None,
            parent: None,
        }
    }

    /// Set the initially selected date. Expects `"yyyy-MM-dd"` format.
    pub fn selected_date(mut self, date: impl Into<String>) -> Self {
        self.selected_date = Some(date.into());
        self
    }

    /// Set the minimum date. Expects `"yyyy-MM-dd"` format.
    pub fn minimum_date(mut self, date: impl Into<String>) -> Self {
        self.minimum_date = Some(date.into());
        self
    }

    /// Set the maximum date. Expects `"yyyy-MM-dd"` format.
    pub fn maximum_date(mut self, date: impl Into<String>) -> Self {
        self.maximum_date = Some(date.into());
        self
    }

    /// Set the first day of the week (0=Sunday, 1=Monday, ..., 6=Saturday).
    pub fn first_day_of_week(mut self, day: i32) -> Self {
        self.first_day_of_week = Some(day);
        self
    }

    /// Show or hide the grid.
    pub fn grid_visible(mut self, visible: bool) -> Self {
        self.grid_visible = Some(visible);
        self
    }

    /// Show or hide the navigation bar.
    pub fn navigation_bar_visible(mut self, visible: bool) -> Self {
        self.navigation_bar_visible = Some(visible);
        self
    }

    /// Called when the selection changes.
    pub fn on_selection_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_selection_changed = Some(Box::new(f));
        self
    }

    /// Called when a date is activated. Receives the date as `"yyyy-MM-dd"`.
    pub fn on_activated<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_activated = Some(Box::new(f));
        self
    }

    /// Set the parent widget.
    pub fn parent(mut self, parent: &dyn AsWidget) -> Self {
        self.parent = Some(parent.widget_ptr());
        self
    }

    /// Create the C++ `QCalendarWidget` and return the Rust wrapper.
    pub fn build(self) -> CalendarWidget {
        let ptr = unsafe {
            ffi::QCalendarWidget_new(self.parent.unwrap_or(std::ptr::null_mut()))
        };
        debug_assert!(!ptr.is_null());
        let mut cal = CalendarWidget {
            ptr,
            has_parent: self.parent.is_some(),
            signal_handles: Vec::new(),
        };

        if let Some(date) = &self.selected_date {
            let_cxx_string!(c_date = date);
            unsafe { ffi::QCalendarWidget_setSelectedDate(ptr, &c_date); }
        }
        if let Some(date) = &self.minimum_date {
            let_cxx_string!(c_date = date);
            unsafe { ffi::QCalendarWidget_setMinimumDate(ptr, &c_date); }
        }
        if let Some(date) = &self.maximum_date {
            let_cxx_string!(c_date = date);
            unsafe { ffi::QCalendarWidget_setMaximumDate(ptr, &c_date); }
        }
        if let Some(day) = self.first_day_of_week {
            unsafe { ffi::QCalendarWidget_setFirstDayOfWeek(ptr, day); }
        }
        if let Some(visible) = self.grid_visible {
            unsafe { ffi::QCalendarWidget_setGridVisible(ptr, visible); }
        }
        if let Some(visible) = self.navigation_bar_visible {
            unsafe { ffi::QCalendarWidget_setNavigationBarVisible(ptr, visible); }
        }

        if let Some(f) = self.on_selection_changed {
            let h = signal::leak_void(f);
            unsafe { ffi::QCalendarWidget_onSelectionChanged(ptr, h.token); }
            cal.signal_handles.push(h);
        }
        if let Some(f) = self.on_activated {
            let h = signal::leak_string(f);
            unsafe { ffi::QCalendarWidget_onActivated(ptr, h.token); }
            cal.signal_handles.push(h);
        }

        cal
    }
}
