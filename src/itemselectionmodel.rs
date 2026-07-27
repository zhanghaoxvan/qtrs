//! Selection model for tracking selections in Model/View views.
//!
//! Wraps [`QItemSelectionModel`](https://doc.qt.io/qt-6/qitemselectionmodel.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};

/// A non-owning wrapper around `QItemSelectionModel`.
///
/// The C++ object is owned by the view — do not use this wrapper after the
/// view is dropped. Signals are disconnected in [`Drop`] but the C++ object
/// is **not** deleted.
///
/// Obtained from the view's internal selection model. For now, constructed
/// manually via [`from_raw`](Self::from_raw).
///
/// # Signals
///
/// | Method | Qt signal |
/// |---|---|
/// | [`connect_selection_changed`](Self::connect_selection_changed) | `selectionChanged` |
/// | [`connect_current_changed`](Self::connect_current_changed) | `currentChanged` |
pub struct ItemSelectionModel {
    ptr: *mut ffi::QItemSelectionModel,
    signal_handles: Vec<SignalHandle>,
}

impl ItemSelectionModel {
    /// Wrap an existing `QItemSelectionModel*`.
    ///
    /// # Safety
    ///
    /// The pointer must be valid and the underlying view must outlive
    /// this wrapper.
    #[doc(hidden)]
    pub unsafe fn from_raw(ptr: *mut ffi::QItemSelectionModel) -> Self {
        Self { ptr, signal_handles: Vec::new() }
    }

    /// Returns `true` if the model has any selection.
    pub fn has_selection(&self) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QItemSelectionModel_hasSelection(self.ptr) }
    }

    /// Connect a callback that fires when the selection changes.
    pub fn connect_selection_changed<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QItemSelectionModel_onSelectionChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }

    /// Connect a callback that fires when the current (focused) item changes.
    pub fn connect_current_changed<F: Fn()>(&mut self, f: F) {
        debug_assert!(!self.ptr.is_null());
        let handle = signal::leak_void(f);
        unsafe { ffi::QItemSelectionModel_onCurrentChanged(self.ptr, handle.token); }
        self.signal_handles.push(handle);
    }
}

impl Drop for ItemSelectionModel {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        // Disconnect signals to prevent use-after-free.
        // The C++ object is NOT deleted — the view owns it.
        unsafe { ffi::QObject_disconnectAll(self.ptr as *mut _); }
        for h in self.signal_handles.drain(..) {
            unsafe { h.reclaim(); }
        }
        self.ptr = std::ptr::null_mut();
    }
}
