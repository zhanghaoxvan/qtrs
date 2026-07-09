//! # Compile-time signal and slot identifiers
//!
//! This module provides zero-cost, compile-time checked signal and slot
//! constants for all widgets. Use these with [`ConnectExt::connect()`] to
//! establish type-safe signal-slot connections.
//!
//! # Example
//!
//! ```no_run
//! use qtrs::prelude::*;
//! use qtrs::signals::*;
//!
//! # fn main() {
//! # let app = Application::new();
//! # let window = Widget::new().build();
//! let slider = Slider::horizontal().parent(&window).build();
//! let spin = SpinBox::new().parent(&window).build();
//!
//! slider.connect(
//!     slider_signals::VALUE_CHANGED,
//!     &spin,
//!     spin_box_slots::SET_VALUE,
//!     ConnType::Auto,
//! );
//! # }
//! ```

// This is for the doc below
#[allow(unused_imports)]
use crate::conn::ConnectExt;

use crate::conn::{SignalMeta, SlotMeta};

// ============================================================
// Slider Signals
// ============================================================

/// Signal constants for [`Slider`](crate::Slider).
///
/// # Example
///
/// ```
/// # use qtrs::prelude::*;
/// # use qtrs::signals::slider_signals::VALUE_CHANGED;
/// # let slider = Slider::horizontal().build();
/// # let spin = SpinBox::new().build();
/// slider.connect(VALUE_CHANGED, &spin, spin_box_slots::SET_VALUE, ConnType::Auto);
/// ```
pub mod slider_signals {
    use super::*;

    /// Emitted when the slider's value changes.
    ///
    /// Qt signal: `valueChanged(int)`
    ///
    /// The new value is passed as `i32`.
    #[derive(Debug, Clone, Copy)]
    pub struct ValueChanged;
    impl SignalMeta for ValueChanged {
        const QT_SIGNATURE: &'static str = "2valueChanged(int)";
        type Args = (i32,);
    }
    pub const VALUE_CHANGED: ValueChanged = ValueChanged;

    /// Emitted when the user presses the slider handle.
    ///
    /// Qt signal: `sliderPressed()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct SliderPressed;
    impl SignalMeta for SliderPressed {
        const QT_SIGNATURE: &'static str = "2sliderPressed()";
        type Args = ();
    }
    pub const SLIDER_PRESSED: SliderPressed = SliderPressed;

    /// Emitted when the user releases the slider handle.
    ///
    /// Qt signal: `sliderReleased()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct SliderReleased;
    impl SignalMeta for SliderReleased {
        const QT_SIGNATURE: &'static str = "2sliderReleased()";
        type Args = ();
    }
    pub const SLIDER_RELEASED: SliderReleased = SliderReleased;

    /// Emitted when the slider is being dragged.
    ///
    /// Qt signal: `sliderMoved(int)`
    ///
    /// The current value during dragging is passed as `i32`.
    #[derive(Debug, Clone, Copy)]
    pub struct SliderMoved;
    impl SignalMeta for SliderMoved {
        const QT_SIGNATURE: &'static str = "2sliderMoved(int)";
        type Args = (i32,);
    }
    pub const SLIDER_MOVED: SliderMoved = SliderMoved;
}

// ============================================================
// SpinBox Slots
// ============================================================

/// Slot constants for [`SpinBox`](crate::SpinBox).
///
/// # Example
///
/// ```
/// # use qtrs::prelude::*;
/// # use qtrs::signals::spin_box_slots::SET_VALUE;
/// # let slider = Slider::horizontal().build();
/// # let spin = SpinBox::new().build();
/// slider.connect(slider_signals::VALUE_CHANGED, &spin, SET_VALUE, ConnType::Auto);
/// ```
pub mod spin_box_slots {
    use super::*;

    /// Sets the spin box's current value.
    ///
    /// Qt slot: `setValue(int)`
    ///
    /// The new value is passed as `i32`.
    #[derive(Debug, Clone, Copy)]
    pub struct SetValue;
    impl SlotMeta for SetValue {
        const QT_SIGNATURE: &'static str = "1setValue(int)";
        type Args = (i32,);
    }
    pub const SET_VALUE: SetValue = SetValue;

    /// Sets the spin box's minimum and maximum values.
    ///
    /// Qt slot: `setRange(int, int)`
    ///
    /// The minimum and maximum values are passed as `(i32, i32)`.
    #[derive(Debug, Clone, Copy)]
    pub struct SetRange;
    impl SlotMeta for SetRange {
        const QT_SIGNATURE: &'static str = "1setRange(int, int)";
        type Args = (i32, i32);
    }
    pub const SET_RANGE: SetRange = SetRange;

    /// Sets the suffix string displayed after the value.
    ///
    /// Qt slot: `setSuffix(QString)`
    ///
    /// The suffix text is passed as `String`.
    #[derive(Debug, Clone, Copy)]
    pub struct SetSuffix;
    impl SlotMeta for SetSuffix {
        const QT_SIGNATURE: &'static str = "1setSuffix(QString)";
        type Args = (String,);
    }
    pub const SET_SUFFIX: SetSuffix = SetSuffix;

    /// Sets the minimum value.
    ///
    /// Qt slot: `setMinimum(int)`
    ///
    /// The new minimum is passed as `i32`.
    #[derive(Debug, Clone, Copy)]
    pub struct SetMinimum;
    impl SlotMeta for SetMinimum {
        const QT_SIGNATURE: &'static str = "1setMinimum(int)";
        type Args = (i32,);
    }
    pub const SET_MINIMUM: SetMinimum = SetMinimum;

    /// Sets the maximum value.
    ///
    /// Qt slot: `setMaximum(int)`
    ///
    /// The new maximum is passed as `i32`.
    #[derive(Debug, Clone, Copy)]
    pub struct SetMaximum;
    impl SlotMeta for SetMaximum {
        const QT_SIGNATURE: &'static str = "1setMaximum(int)";
        type Args = (i32,);
    }
    pub const SET_MAXIMUM: SetMaximum = SetMaximum;
}

// ============================================================
// SpinBox Signals
// ============================================================

/// Signal constants for [`SpinBox`](crate::SpinBox).
pub mod spin_box_signals {
    use super::*;

    /// Emitted when the spin box value changes.
    ///
    /// Qt signal: `valueChanged(int)`
    ///
    /// The new value is passed as `i32`.
    #[derive(Debug, Clone, Copy)]
    pub struct ValueChanged;
    impl SignalMeta for ValueChanged {
        const QT_SIGNATURE: &'static str = "2valueChanged(int)";
        type Args = (i32,);
    }
    pub const VALUE_CHANGED: ValueChanged = ValueChanged;

    /// Emitted when the spin box text changes.
    ///
    /// Qt signal: `textChanged(QString)`
    ///
    /// The new text is passed as `String`.
    #[derive(Debug, Clone, Copy)]
    pub struct TextChanged;
    impl SignalMeta for TextChanged {
        const QT_SIGNATURE: &'static str = "2textChanged(QString)";
        type Args = (String,);
    }
    pub const TEXT_CHANGED: TextChanged = TextChanged;
}

// ============================================================
// PushButton Signals
// ============================================================

/// Signal constants for [`PushButton`](crate::PushButton).
pub mod push_button_signals {
    use super::*;

    /// Emitted when the button is clicked.
    ///
    /// Qt signal: `clicked()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Clicked;
    impl SignalMeta for Clicked {
        const QT_SIGNATURE: &'static str = "2clicked()";
        type Args = ();
    }
    pub const CLICKED: Clicked = Clicked;

    /// Emitted when the button is pressed down.
    ///
    /// Qt signal: `pressed()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Pressed;
    impl SignalMeta for Pressed {
        const QT_SIGNATURE: &'static str = "2pressed()";
        type Args = ();
    }
    pub const PRESSED: Pressed = Pressed;

    /// Emitted when the button is released.
    ///
    /// Qt signal: `released()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Released;
    impl SignalMeta for Released {
        const QT_SIGNATURE: &'static str = "2released()";
        type Args = ();
    }
    pub const RELEASED: Released = Released;
}

// ============================================================
// CheckBox Signals
// ============================================================

/// Signal constants for [`CheckBox`](crate::CheckBox).
pub mod check_box_signals {
    use super::*;

    /// Emitted when the checkbox is toggled.
    ///
    /// Qt signal: `toggled(bool)`
    ///
    /// `true` if checked, `false` if unchecked.
    #[derive(Debug, Clone, Copy)]
    pub struct Toggled;
    impl SignalMeta for Toggled {
        const QT_SIGNATURE: &'static str = "2toggled(bool)";
        type Args = (bool,);
    }
    pub const TOGGLED: Toggled = Toggled;

    /// Emitted when the checkbox state changes.
    ///
    /// Qt signal: `stateChanged(int)`
    ///
    /// The new state is passed as `i32` (0=unchecked, 1=partially, 2=checked).
    #[derive(Debug, Clone, Copy)]
    pub struct StateChanged;
    impl SignalMeta for StateChanged {
        const QT_SIGNATURE: &'static str = "2stateChanged(int)";
        type Args = (i32,);
    }
    pub const STATE_CHANGED: StateChanged = StateChanged;
}

// ============================================================
// CheckBox Slots
// ============================================================

/// Slot constants for [`CheckBox`](crate::CheckBox).
pub mod check_box_slots {
    use super::*;

    /// Sets the checked state.
    ///
    /// Qt slot: `setChecked(bool)`
    ///
    /// `true` to check, `false` to uncheck.
    #[derive(Debug, Clone, Copy)]
    pub struct SetChecked;
    impl SlotMeta for SetChecked {
        const QT_SIGNATURE: &'static str = "1setChecked(bool)";
        type Args = (bool,);
    }
    pub const SET_CHECKED: SetChecked = SetChecked;

    /// Sets whether the checkbox is tristate.
    ///
    /// Qt slot: `setTristate(bool)`
    ///
    /// `true` to enable tristate mode.
    #[derive(Debug, Clone, Copy)]
    pub struct SetTristate;
    impl SlotMeta for SetTristate {
        const QT_SIGNATURE: &'static str = "1setTristate(bool)";
        type Args = (bool,);
    }
    pub const SET_TRISTATE: SetTristate = SetTristate;
}

// ============================================================
// RadioButton Signals
// ============================================================

/// Signal constants for [`RadioButton`](crate::RadioButton).
pub mod radio_button_signals {
    use super::*;

    /// Emitted when the radio button is toggled.
    ///
    /// Qt signal: `toggled(bool)`
    ///
    /// `true` if selected, `false` if deselected.
    #[derive(Debug, Clone, Copy)]
    pub struct Toggled;
    impl SignalMeta for Toggled {
        const QT_SIGNATURE: &'static str = "2toggled(bool)";
        type Args = (bool,);
    }
    pub const TOGGLED: Toggled = Toggled;
}

// ============================================================
// RadioButton Slots
// ============================================================

/// Slot constants for [`RadioButton`](crate::RadioButton).
pub mod radio_button_slots {
    use super::*;

    /// Sets the checked state.
    ///
    /// Qt slot: `setChecked(bool)`
    ///
    /// `true` to select, `false` to deselect.
    #[derive(Debug, Clone, Copy)]
    pub struct SetChecked;
    impl SlotMeta for SetChecked {
        const QT_SIGNATURE: &'static str = "1setChecked(bool)";
        type Args = (bool,);
    }
    pub const SET_CHECKED: SetChecked = SetChecked;
}

// ============================================================
// LineEdit Signals
// ============================================================

/// Signal constants for [`LineEdit`](crate::LineEdit).
pub mod line_edit_signals {
    use super::*;

    /// Emitted when the user presses Enter.
    ///
    /// Qt signal: `returnPressed()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct ReturnPressed;
    impl SignalMeta for ReturnPressed {
        const QT_SIGNATURE: &'static str = "2returnPressed()";
        type Args = ();
    }
    pub const RETURN_PRESSED: ReturnPressed = ReturnPressed;

    /// Emitted when the text changes.
    ///
    /// Qt signal: `textChanged(QString)`
    ///
    /// The new text is passed as `String`.
    #[derive(Debug, Clone, Copy)]
    pub struct TextChanged;
    impl SignalMeta for TextChanged {
        const QT_SIGNATURE: &'static str = "2textChanged(QString)";
        type Args = (String,);
    }
    pub const TEXT_CHANGED: TextChanged = TextChanged;

    /// Emitted when the text is edited by the user.
    ///
    /// Qt signal: `textEdited(QString)`
    ///
    /// The new text is passed as `String`.
    #[derive(Debug, Clone, Copy)]
    pub struct TextEdited;
    impl SignalMeta for TextEdited {
        const QT_SIGNATURE: &'static str = "2textEdited(QString)";
        type Args = (String,);
    }
    pub const TEXT_EDITED: TextEdited = TextEdited;
}

// ============================================================
// LineEdit Slots
// ============================================================

/// Slot constants for [`LineEdit`](crate::LineEdit).
pub mod line_edit_slots {
    use super::*;

    /// Sets the text.
    ///
    /// Qt slot: `setText(QString)`
    ///
    /// The new text is passed as `String`.
    #[derive(Debug, Clone, Copy)]
    pub struct SetText;
    impl SlotMeta for SetText {
        const QT_SIGNATURE: &'static str = "1setText(QString)";
        type Args = (String,);
    }
    pub const SET_TEXT: SetText = SetText;

    /// Clears the text.
    ///
    /// Qt slot: `clear()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Clear;
    impl SlotMeta for Clear {
        const QT_SIGNATURE: &'static str = "1clear()";
        type Args = ();
    }
    pub const CLEAR: Clear = Clear;
}

// ============================================================
// ComboBox Signals
// ============================================================

/// Signal constants for [`ComboBox`](crate::ComboBox).
pub mod combo_box_signals {
    use super::*;

    /// Emitted when the current index changes.
    ///
    /// Qt signal: `currentIndexChanged(int)`
    ///
    /// The new index is passed as `i32`.
    #[derive(Debug, Clone, Copy)]
    pub struct CurrentIndexChanged;
    impl SignalMeta for CurrentIndexChanged {
        const QT_SIGNATURE: &'static str = "2currentIndexChanged(int)";
        type Args = (i32,);
    }
    pub const CURRENT_INDEX_CHANGED: CurrentIndexChanged = CurrentIndexChanged;

    /// Emitted when the current text changes.
    ///
    /// Qt signal: `currentTextChanged(QString)`
    ///
    /// The new text is passed as `String`.
    #[derive(Debug, Clone, Copy)]
    pub struct CurrentTextChanged;
    impl SignalMeta for CurrentTextChanged {
        const QT_SIGNATURE: &'static str = "2currentTextChanged(QString)";
        type Args = (String,);
    }
    pub const CURRENT_TEXT_CHANGED: CurrentTextChanged = CurrentTextChanged;
}

// ============================================================
// ComboBox Slots
// ============================================================

/// Slot constants for [`ComboBox`](crate::ComboBox).
pub mod combo_box_slots {
    use super::*;

    /// Sets the current index.
    ///
    /// Qt slot: `setCurrentIndex(int)`
    ///
    /// The new index is passed as `i32`.
    #[derive(Debug, Clone, Copy)]
    pub struct SetCurrentIndex;
    impl SlotMeta for SetCurrentIndex {
        const QT_SIGNATURE: &'static str = "1setCurrentIndex(int)";
        type Args = (i32,);
    }
    pub const SET_CURRENT_INDEX: SetCurrentIndex = SetCurrentIndex;

    /// Adds an item to the combobox.
    ///
    /// Qt slot: `addItem(QString)`
    ///
    /// The item text is passed as `String`.
    #[derive(Debug, Clone, Copy)]
    pub struct AddItem;
    impl SlotMeta for AddItem {
        const QT_SIGNATURE: &'static str = "1addItem(QString)";
        type Args = (String,);
    }
    pub const ADD_ITEM: AddItem = AddItem;

    /// Clears all items.
    ///
    /// Qt slot: `clear()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Clear;
    impl SlotMeta for Clear {
        const QT_SIGNATURE: &'static str = "1clear()";
        type Args = ();
    }
    pub const CLEAR: Clear = Clear;
}

// ============================================================
// ProgressBar Slots
// ============================================================

/// Slot constants for [`ProgressBar`](crate::ProgressBar).
pub mod progress_bar_slots {
    use super::*;

    /// Sets the progress value.
    ///
    /// Qt slot: `setValue(int)`
    ///
    /// The progress value is passed as `i32`.
    #[derive(Debug, Clone, Copy)]
    pub struct SetValue;
    impl SlotMeta for SetValue {
        const QT_SIGNATURE: &'static str = "1setValue(int)";
        type Args = (i32,);
    }
    pub const SET_VALUE: SetValue = SetValue;

    /// Sets the progress range.
    ///
    /// Qt slot: `setRange(int, int)`
    ///
    /// The minimum and maximum values are passed as `(i32, i32)`.
    #[derive(Debug, Clone, Copy)]
    pub struct SetRange;
    impl SlotMeta for SetRange {
        const QT_SIGNATURE: &'static str = "1setRange(int, int)";
        type Args = (i32, i32);
    }
    pub const SET_RANGE: SetRange = SetRange;

    /// Resets the progress bar.
    ///
    /// Qt slot: `reset()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Reset;
    impl SlotMeta for Reset {
        const QT_SIGNATURE: &'static str = "1reset()";
        type Args = ();
    }
    pub const RESET: Reset = Reset;
}

// ============================================================
// TabWidget Signals
// ============================================================

/// Signal constants for [`TabWidget`](crate::TabWidget).
pub mod tab_widget_signals {
    use super::*;

    /// Emitted when the current tab changes.
    ///
    /// Qt signal: `currentChanged(int)`
    ///
    /// The new tab index is passed as `i32`.
    #[derive(Debug, Clone, Copy)]
    pub struct CurrentChanged;
    impl SignalMeta for CurrentChanged {
        const QT_SIGNATURE: &'static str = "2currentChanged(int)";
        type Args = (i32,);
    }
    pub const CURRENT_CHANGED: CurrentChanged = CurrentChanged;
}

// ============================================================
// TabWidget Slots
// ============================================================

/// Slot constants for [`TabWidget`](crate::TabWidget).
pub mod tab_widget_slots {
    use super::*;

    /// Sets the current tab index.
    ///
    /// Qt slot: `setCurrentIndex(int)`
    ///
    /// The new tab index is passed as `i32`.
    #[derive(Debug, Clone, Copy)]
    pub struct SetCurrentIndex;
    impl SlotMeta for SetCurrentIndex {
        const QT_SIGNATURE: &'static str = "1setCurrentIndex(int)";
        type Args = (i32,);
    }
    pub const SET_CURRENT_INDEX: SetCurrentIndex = SetCurrentIndex;

    /// Adds a tab.
    ///
    /// Qt slot: `addTab(QWidget*, QString)`
    ///
    /// The page widget and label are passed as `(*mut QWidget, String)`.
    #[derive(Debug, Clone, Copy)]
    pub struct AddTab;
    impl SlotMeta for AddTab {
        const QT_SIGNATURE: &'static str = "1addTab(QWidget*, QString)";
        type Args = (*mut crate::ffi::QWidget, String);
    }
    pub const ADD_TAB: AddTab = AddTab;
}

// ============================================================
// TextEdit Signals
// ============================================================

/// Signal constants for [`TextEdit`](crate::TextEdit).
pub mod text_edit_signals {
    use super::*;

    /// Emitted when the text changes.
    ///
    /// Qt signal: `textChanged()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct TextChanged;
    impl SignalMeta for TextChanged {
        const QT_SIGNATURE: &'static str = "2textChanged()";
        type Args = ();
    }
    pub const TEXT_CHANGED: TextChanged = TextChanged;

    /// Emitted when text selection changes.
    ///
    /// Qt signal: `copyAvailable(bool)`
    ///
    /// `true` if text is selected and can be copied.
    #[derive(Debug, Clone, Copy)]
    pub struct CopyAvailable;
    impl SignalMeta for CopyAvailable {
        const QT_SIGNATURE: &'static str = "2copyAvailable(bool)";
        type Args = (bool,);
    }
    pub const COPY_AVAILABLE: CopyAvailable = CopyAvailable;
}

// ============================================================
// TextEdit Slots
// ============================================================

/// Slot constants for [`TextEdit`](crate::TextEdit).
pub mod text_edit_slots {
    use super::*;

    /// Sets the plain text content.
    ///
    /// Qt slot: `setPlainText(QString)`
    ///
    /// The text content is passed as `String`.
    #[derive(Debug, Clone, Copy)]
    pub struct SetPlainText;
    impl SlotMeta for SetPlainText {
        const QT_SIGNATURE: &'static str = "1setPlainText(QString)";
        type Args = (String,);
    }
    pub const SET_PLAIN_TEXT: SetPlainText = SetPlainText;

    /// Clears the text.
    ///
    /// Qt slot: `clear()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Clear;
    impl SlotMeta for Clear {
        const QT_SIGNATURE: &'static str = "1clear()";
        type Args = ();
    }
    pub const CLEAR: Clear = Clear;

    /// Copies selected text to clipboard.
    ///
    /// Qt slot: `copy()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Copy;
    impl SlotMeta for Copy {
        const QT_SIGNATURE: &'static str = "1copy()";
        type Args = ();
    }
    pub const COPY: Copy = Copy;

    /// Pastes text from clipboard.
    ///
    /// Qt slot: `paste()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Paste;
    impl SlotMeta for Paste {
        const QT_SIGNATURE: &'static str = "1paste()";
        type Args = ();
    }
    pub const PASTE: Paste = Paste;
}

// ============================================================
// Timer Signals
// ============================================================

/// Signal constants for [`Timer`](crate::Timer).
pub mod timer_signals {
    use super::*;

    /// Emitted when the timer times out.
    ///
    /// Qt signal: `timeout()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Timeout;
    impl SignalMeta for Timeout {
        const QT_SIGNATURE: &'static str = "2timeout()";
        type Args = ();
    }
    pub const TIMEOUT: Timeout = Timeout;
}

// ============================================================
// Timer Slots
// ============================================================

/// Slot constants for [`Timer`](crate::Timer).
pub mod timer_slots {
    use super::*;

    /// Starts the timer.
    ///
    /// Qt slot: `start()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Start;
    impl SlotMeta for Start {
        const QT_SIGNATURE: &'static str = "1start()";
        type Args = ();
    }
    pub const START: Start = Start;

    /// Stops the timer.
    ///
    /// Qt slot: `stop()`
    ///
    /// No parameters.
    #[derive(Debug, Clone, Copy)]
    pub struct Stop;
    impl SlotMeta for Stop {
        const QT_SIGNATURE: &'static str = "1stop()";
        type Args = ();
    }
    pub const STOP: Stop = Stop;
}

// ============================================================
// Widget (Base) Signals
// ============================================================

/// Signal constants for [`Widget`](crate::Widget) (base class).
pub mod widget_signals {
    use super::*;

    /// Emitted when the window title changes.
    ///
    /// Qt signal: `windowTitleChanged(QString)`
    ///
    /// The new title is passed as `String`.
    #[derive(Debug, Clone, Copy)]
    pub struct WindowTitleChanged;
    impl SignalMeta for WindowTitleChanged {
        const QT_SIGNATURE: &'static str = "2windowTitleChanged(QString)";
        type Args = (String,);
    }
    pub const WINDOW_TITLE_CHANGED: WindowTitleChanged = WindowTitleChanged;

    /// Emitted when the window icon changes.
    ///
    /// Qt signal: `windowIconChanged(QIcon)`
    ///
    /// The new icon is passed as `QIcon`.
    #[derive(Debug, Clone, Copy)]
    pub struct WindowIconChanged;
    impl SignalMeta for WindowIconChanged {
        const QT_SIGNATURE: &'static str = "2windowIconChanged(QIcon)";
        type Args = (crate::ffi::QIcon,);
    }
    pub const WINDOW_ICON_CHANGED: WindowIconChanged = WindowIconChanged;

    /// Emitted on context menu request.
    ///
    /// Qt signal: `customContextMenuRequested(QPoint)`
    ///
    /// The cursor position is passed as `QPoint`.
    #[derive(Debug, Clone, Copy)]
    pub struct CustomContextMenuRequested;
    impl SignalMeta for CustomContextMenuRequested {
        const QT_SIGNATURE: &'static str = "2customContextMenuRequested(QPoint)";
        type Args = (crate::ffi::QPoint,);
    }
    pub const CUSTOM_CONTEXT_MENU_REQUESTED: CustomContextMenuRequested =
        CustomContextMenuRequested;
}