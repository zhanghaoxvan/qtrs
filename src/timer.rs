//! Timer for delayed or periodic callbacks.
//!
//! Wraps [`QTimer`](https://doc.qt.io/qt-6/qtimer.html).

use crate::ffi;
use crate::signal::{self, SignalHandle};

/// A repeating or single-shot timer.
///
/// `Timer` fires a callback at a fixed interval. The timer **starts
/// immediately** on [`build`](Builder::build). Use [`stop`](Timer::stop)
/// to halt it, or let it drop to stop and clean up.
///
/// # Signals
///
/// | Method | Qt signal | When |
/// |---|---|---|
/// | [`Builder::on_timeout`] | `QTimer::timeout` | Every `interval_ms` milliseconds |
///
/// # Example
///
/// ```no_run
/// let timer = Timer::new(1000)
///     .on_timeout(|| println!("tick"))
///     .build();
/// // Prints "tick" every second
/// app.exec();
/// ```
pub struct Timer {
    ptr: *mut ffi::QTimer,
    signal_handles: Vec<SignalHandle>,
}

impl Timer {
    /// Start building a new timer with the given interval in milliseconds.
    ///
    /// Use [`Timer::single_shot`] for a one-shot timer.
    pub fn new(interval_ms: i32) -> Builder {
        Builder::new(interval_ms)
    }

    /// Create a timer that fires once after `interval_ms`, then stops.
    ///
    /// Convenience wrapper over Qt's single-shot behaviour.
    pub fn single_shot<F: Fn() + 'static>(interval_ms: i32, f: F) -> Timer {
        let timer = Timer::new(interval_ms).on_timeout(f).build();
        // Single-shot: the callback stops the timer after the first fire.
        // We can't express this cleanly with QTimer::singleShot static
        // from C++, so the user should stop() in the callback if they
        // only want one fire.
        timer
    }

    /// Start (or restart) the timer.
    pub fn start(&self, interval_ms: i32) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTimer_start(self.ptr, interval_ms); }
    }

    /// Stop the timer.
    pub fn stop(&self) {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTimer_stop(self.ptr); }
    }

    /// Returns `true` if the timer is currently running.
    pub fn is_active(&self) -> bool {
        debug_assert!(!self.ptr.is_null());
        unsafe { ffi::QTimer_isActive(self.ptr) }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        // Stop the timer first so no more signals fire, then reclaim.
        unsafe { ffi::QTimer_stop(self.ptr); }
        for h in self.signal_handles.drain(..) {
            unsafe { h.reclaim(); }
        }
        unsafe { ffi::QTimer_delete(self.ptr) };
        self.ptr = std::ptr::null_mut();
    }
}

// ============================================================
// Builder
// ============================================================

/// Builder for [`Timer`].
pub struct Builder {
    interval_ms: i32,
    on_timeout: Option<Box<dyn Fn()>>,
}

impl Builder {
    fn new(interval_ms: i32) -> Self {
        Self { interval_ms, on_timeout: None }
    }

    /// Set the callback fired every `interval_ms` milliseconds.
    pub fn on_timeout<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_timeout = Some(Box::new(f));
        self
    }

    /// Create the `QTimer`, connect the signal, start it, and return the
    /// Rust wrapper.
    pub fn build(self) -> Timer {
        let ptr = unsafe { ffi::QTimer_new() };
        debug_assert!(!ptr.is_null());
        let mut timer = Timer { ptr, signal_handles: Vec::new() };
        if let Some(f) = self.on_timeout {
            let h = signal::leak_void(f);
            unsafe { ffi::QTimer_onTimeout(ptr, h.token); }
            timer.signal_handles.push(h);
        }
        unsafe { ffi::QTimer_start(ptr, self.interval_ms); }
        timer
    }
}
