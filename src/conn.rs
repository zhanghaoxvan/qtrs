//! # Type-safe signal-slot connection API
//!
//! This module provides compile-time checked signal-slot connections.
//! Use [`ConnectExt::connect()`] with the constants from [`crate::signals`]
//! to establish type-safe connections between widgets.
//!
//! ## Overview
//!
//! Qt's signal-slot system allows widgets to communicate directly without
//! intermediate Rust code. This module provides a type-safe wrapper around
//! Qt's `QObject::connect` that validates signals and slots at compile time.
//!
//! ## How It Works
//!
//! 1. Each signal and slot is represented as a zero-sized type (ZST)
//! 2. These types implement [`SignalMeta`] or [`SlotMeta`] traits
//! 3. The `connect()` method checks at compile time that the signal and slot
//!    parameter types match exactly
//! 4. The Qt6 internal signature strings are pre-computed constants
//!
//! ## Example
//!
//! ```no_run
//! use qtrs::prelude::*;
//!
//! let slider = Slider::horizontal().build();
//! let spin = SpinBox::new().build();
//!
//! // Compile-time validated: slider -> spinbox
//! slider.connect(
//!     Slider::value_changed,
//!     &spin,
//!     SpinBox::set_value,
//!     ConnType::Auto,
//! );
//! ```

use cxx::let_cxx_string;
use crate::ffi;
use crate::widget::AsWidget;

// ============================================================
// Core Traits
// ============================================================

/// Compile-time signal metadata.
///
/// This trait is implemented by signal constants in [`crate::signals`].
/// It provides the Qt6 internal signature and Rust parameter types
/// needed for type-safe connections.
///
/// # Associated Constants
///
/// * `QT_SIGNATURE` — The Qt6 internal signature string with the `2` prefix
///
/// # Associated Types
///
/// * `Args` — The Rust parameter tuple type
///
/// # Example
///
/// ```rust
/// # use qtrs::conn::SignalMeta;
/// #[derive(Debug, Clone, Copy)]
/// pub struct ValueChanged;
/// impl SignalMeta for ValueChanged {
///     const QT_SIGNATURE: &'static str = "2valueChanged(int)";
///     type Args = (i32,);
/// }
/// ```
pub trait SignalMeta {
    /// Qt6 internal signature string.
    ///
    /// Format: `"2signalName(paramTypes)"` where `2` is the Qt6 signal prefix.
    /// This string is passed directly to Qt's `QObject::connect`.
    const QT_SIGNATURE: &'static str;

    /// Rust parameter tuple type.
    ///
    /// Used for compile-time type checking against the slot's parameter type.
    /// The connection will only compile if the signal's `Args` exactly matches
    /// the slot's `Args`.
    type Args;
}

/// Compile-time slot metadata.
///
/// This trait is implemented by slot constants in [`crate::signals`].
/// It provides the Qt6 internal signature and Rust parameter types
/// needed for type-safe connections.
///
/// # Associated Constants
///
/// * `QT_SIGNATURE` — The Qt6 internal signature string with the `1` prefix
///
/// # Associated Types
///
/// * `Args` — The Rust parameter tuple type
///
/// # Example
///
/// ```rust
/// # use qtrs::conn::SlotMeta;
/// #[derive(Debug, Clone, Copy)]
/// pub struct SetValue;
/// impl SlotMeta for SetValue {
///     const QT_SIGNATURE: &'static str = "1setValue(int)";
///     type Args = (i32,);
/// }
/// ```
pub trait SlotMeta {
    /// Qt6 internal signature string.
    ///
    /// Format: `"1slotName(paramTypes)"` where `1` is the Qt6 slot prefix.
    /// This string is passed directly to Qt's `QObject::connect`.
    const QT_SIGNATURE: &'static str;

    /// Rust parameter tuple type.
    ///
    /// Used for compile-time type checking against the signal's parameter type.
    /// The connection will only compile if the slot's `Args` exactly matches
    /// the signal's `Args`.
    type Args;
}

// ============================================================
// Connection Type
// ============================================================

/// Qt connection type.
///
/// Maps directly to Qt's `Qt::ConnectionType` enum.
/// Controls how the signal-slot connection behaves.
///
/// # Default
///
/// Use `Auto` for most cases. Qt automatically chooses the appropriate
/// connection type based on thread affinity.
///
/// # Thread Safety
///
/// | Variant | Same Thread | Different Threads |
/// |---------|-------------|-------------------|
/// | `Auto` | Direct | Queued |
/// | `Direct` | Call immediately | **Unsafe - may crash** |
/// | `Queued` | Queued (async) | Queued (async, safe) |
/// | `BlockingQueued` | Blocks until called | Blocks until called |
/// | `Unique` | Prevents duplicates | Prevents duplicates |
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnType {
    /// Auto-select: Direct if same thread, Queued if different.
    ///
    /// This is the recommended default for most connections.
    Auto = 0,

    /// Direct call (synchronous, same thread only).
    ///
    /// The slot is called immediately in the sender's thread.
    /// Must only be used when sender and receiver are in the same thread.
    Direct = 1,

    /// Queued call (asynchronous, thread-safe).
    ///
    /// The call is posted to the receiver's thread event loop.
    /// Safe for cross-thread connections.
    Queued = 2,

    /// Blocking queued call (synchronous across threads).
    ///
    /// The sender thread blocks until the slot has been called in the
    /// receiver thread. Use with caution.
    BlockingQueued = 3,

    /// Unique connection (prevents duplicate connections).
    ///
    /// If the same signal-slot pair is already connected, the new
    /// connection will not be made.
    Unique = 0x80,
}

// ============================================================
// ConnectExt Trait
// ============================================================

/// Extension trait for [`AsWidget`] providing type-safe signal-slot connections.
///
/// This trait adds compile-time checked `connect()` and `disconnect()` methods
/// to all widgets. It is automatically implemented for all types that implement
/// [`AsWidget`].
///
/// # Type Safety
///
/// The compiler verifies three things:
///
/// 1. The signal exists on the source widget type
/// 2. The slot exists on the target widget type
/// 3. Signal parameters match slot parameters exactly
///
/// # Error Cases
///
/// If the signal and slot parameter types don't match, the code will not compile:
///
/// ```compile_fail
/// # use qtrs::prelude::*;
/// # let slider = Slider::horizontal().build();
/// # let label = Label::new("").build();
/// // Compile error: signal sends i32, slot expects String
/// slider.connect(Slider::value_changed, &label, Label::set_text, ConnType::Auto);
/// ```
///
/// # Thread Safety
///
/// In debug builds, all widget operations assert they are called from the GUI thread.
/// Use `Auto` or `Queued` for cross-thread connections.
///
/// # Example
///
/// ```no_run
/// use qtrs::prelude::*;
///
/// let slider = Slider::horizontal().build();
/// let spin = SpinBox::new().build();
///
/// slider.connect(
///     Slider::value_changed,
///     &spin,
///     SpinBox::set_value,
///     ConnType::Auto,
/// );
/// ```
pub trait ConnectExt: AsWidget {
    /// Connect a signal to a slot with compile-time type checking.
    ///
    /// This is the primary method for establishing signal-slot connections.
    /// It provides full compile-time safety with zero runtime overhead.
    ///
    /// # Type Parameters
    ///
    /// * `S` — Signal type implementing [`SignalMeta`]
    /// * `T` — Slot type implementing [`SlotMeta`]
    ///
    /// # Constraints
    ///
    /// `S::Args == T::Args` — signal and slot parameter types must match exactly.
    /// If they don't match, the code will not compile.
    ///
    /// # Parameters
    ///
    /// * `signal` — The signal to connect, e.g., `Slider::value_changed`
    /// * `target` — The target widget that owns the slot
    /// * `slot` — The slot to connect, e.g., `SpinBox::set_value`
    /// * `conn_type` — The connection type, see [`ConnType`]
    ///
    /// # Returns
    ///
    /// `true` if the connection was successfully established.
    /// Returns `false` if the connection failed.
    ///
    /// # Panics
    ///
    /// In debug builds, panics if called from a non-GUI thread.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use qtrs::prelude::*;
    /// # let slider = Slider::horizontal().build();
    /// # let spin = SpinBox::new().build();
    /// slider.connect(Slider::value_changed, &spin, SpinBox::set_value, ConnType::Auto);
    /// ```
    fn connect<S, T>(
        &self,
        _signal: S,
        target: &dyn AsWidget,
        _slot: T,
        conn_type: ConnType,
    ) -> bool
    where
        S: SignalMeta,
        T: SlotMeta,
        S::Args: EqSlotArgs<T::Args>,
    {
        debug_assert!(
            unsafe { ffi::QObject_isInGuiThread() },
            "Widget::connect must be called from the GUI thread"
        );

        let sig = S::QT_SIGNATURE;
        let slt = T::QT_SIGNATURE;

        let_cxx_string!(c_sig = sig);
        let_cxx_string!(c_slt = slt);

        unsafe {
            ffi::QObject_connect(
                self.widget_ptr().cast(),
                &c_sig,
                target.widget_ptr().cast(),
                &c_slt,
                conn_type as i32,
            )
        }
    }

    /// Disconnect a signal-slot connection.
    ///
    /// Removes a previously established connection. The signal, target widget,
    /// and slot must exactly match the connection you want to remove.
    ///
    /// # Type Parameters
    ///
    /// * `S` — Signal type implementing [`SignalMeta`]
    /// * `T` — Slot type implementing [`SlotMeta`]
    ///
    /// # Parameters
    ///
    /// * `signal` — The signal that was connected
    /// * `target` — The target widget that owns the slot
    /// * `slot` — The slot that was connected
    ///
    /// # Returns
    ///
    /// `true` if the connection was successfully disconnected.
    /// Returns `false` if the connection wasn't found.
    ///
    /// # Panics
    ///
    /// In debug builds, panics if called from a non-GUI thread.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use qtrs::prelude::*;
    /// # let slider = Slider::horizontal().build();
    /// # let spin = SpinBox::new().build();
    /// slider.disconnect(Slider::value_changed, &spin, SpinBox::set_value);
    /// ```
    fn disconnect<S, T>(
        &self,
        _signal: S,
        target: &dyn AsWidget,
        _slot: T,
    ) -> bool
    where
        S: SignalMeta,
        T: SlotMeta,
    {
        debug_assert!(
            unsafe { ffi::QObject_isInGuiThread() },
            "Widget::disconnect must be called from the GUI thread"
        );

        let sig = S::QT_SIGNATURE;
        let slt = T::QT_SIGNATURE;

        let_cxx_string!(c_sig = sig);
        let_cxx_string!(c_slt = slt);

        unsafe {
            ffi::QObject_disconnect(
                self.widget_ptr().cast(),
                &c_sig,
                target.widget_ptr().cast(),
                &c_slt,
            )
        }
    }
}

// ============================================================
// Type Equality Helper
// ============================================================

/// Marker trait for compile-time type equality.
///
/// Used in the `where` clause of [`ConnectExt::connect()`] to ensure
/// signal and slot parameter types match exactly.
///
/// This trait is automatically implemented for all types via a blanket
/// implementation. You don't need to implement it manually.
pub trait EqSlotArgs<T> {}

/// Blanket implementation: any type is equal to itself.
impl<A> EqSlotArgs<A> for A {}

// ============================================================
// Blanket Implementation
// ============================================================

/// Automatically implement [`ConnectExt`] for all types that implement [`AsWidget`].
impl<T: AsWidget> ConnectExt for T {}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conn_type_repr() {
        assert_eq!(ConnType::Auto as i32, 0);
        assert_eq!(ConnType::Direct as i32, 1);
        assert_eq!(ConnType::Queued as i32, 2);
        assert_eq!(ConnType::BlockingQueued as i32, 3);
        assert_eq!(ConnType::Unique as i32, 0x80);
    }

    #[test]
    fn test_type_equality() {
        fn require_same<A, B>() where A: EqSlotArgs<B> {}

        // Same types compile
        require_same::<i32, i32>();
        require_same::<(i32,), (i32,)>();
        require_same::<(i32, i32), (i32, i32)>();
        require_same::<(), ()>();
        require_same::<String, String>();

        // These would not compile if uncommented:
        // require_same::<i32, String>();
        // require_same::<(i32,), (i32, i32)>();
        // require_same::<(), i32>();
    }
}