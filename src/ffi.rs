// src/ffi.rs — cxx bridge declarations for qtrs
//
// All C++ functions are free functions (not member functions), so we use
// raw `*mut T` parameters instead of `self: &T`.

#[cxx::bridge]
pub mod ffi_inner {

    // ========================================================================
    // Qt Core
    // ========================================================================

    unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");

        // --- Qt opaque types ---
        type QApplication;
        type QWidget;
        type QPushButton;
        type QLabel;
        type QLineEdit;
        type QCheckBox;
        type QComboBox;
        type QTextEdit;
        type QSlider;
        type QTimer;
        type QVBoxLayout;
        type QHBoxLayout;
        type QGridLayout;
        type QLayout;

        // --- Trampolines ---
        unsafe fn qtrs_setVoidTrampoline(trampoline: unsafe extern "C" fn(u64));
        unsafe fn qtrs_setBoolTrampoline(trampoline: unsafe extern "C" fn(u64, bool));
        unsafe fn qtrs_setIntTrampoline(trampoline: unsafe extern "C" fn(u64, i32));

        // --- QApplication ---
        unsafe fn QApplication_new() -> *mut QApplication;
        unsafe fn QApplication_exec(app: *mut QApplication) -> i32;
        unsafe fn QApplication_setWindowIcon(app: *mut QApplication, icon_path: &CxxString);
        unsafe fn QApplication_setDesktopFileName(app: *mut QApplication, name: &CxxString);

        // --- QWidget ---
        unsafe fn QWidget_new(parent: *mut QWidget) -> *mut QWidget;
        unsafe fn QWidget_show(widget: *mut QWidget);
        unsafe fn QWidget_hide(widget: *mut QWidget);
        unsafe fn QWidget_setWindowTitle(widget: *mut QWidget, title: &CxxString);
        unsafe fn QWidget_resize(widget: *mut QWidget, width: i32, height: i32);
        unsafe fn QWidget_delete(widget: *mut QWidget);
        unsafe fn QWidget_setLayout(widget: *mut QWidget, layout: *mut QLayout);
        unsafe fn QWidget_setWindowIcon(widget: *mut QWidget, icon_path: &CxxString);

        // --- Common QWidget properties ---
        unsafe fn QWidget_setEnabled(widget: *mut QWidget, enabled: bool);
        unsafe fn QWidget_setVisible(widget: *mut QWidget, visible: bool);
        unsafe fn QWidget_setToolTip(widget: *mut QWidget, tip: &CxxString);
        unsafe fn QWidget_setMinimumSize(widget: *mut QWidget, width: i32, height: i32);
        unsafe fn QWidget_setMaximumSize(widget: *mut QWidget, width: i32, height: i32);
        unsafe fn QWidget_setFixedSize(widget: *mut QWidget, width: i32, height: i32);

        // --- toQWidget upcasts ---
        unsafe fn toQWidget_QWidget(w: *mut QWidget) -> *mut QWidget;
        unsafe fn toQWidget_QPushButton(btn: *mut QPushButton) -> *mut QWidget;
        unsafe fn toQWidget_QLabel(label: *mut QLabel) -> *mut QWidget;
        unsafe fn toQWidget_QLineEdit(edit: *mut QLineEdit) -> *mut QWidget;
        unsafe fn toQWidget_QCheckBox(cb: *mut QCheckBox) -> *mut QWidget;
        unsafe fn toQWidget_QComboBox(cb: *mut QComboBox) -> *mut QWidget;
        unsafe fn toQWidget_QTextEdit(edit: *mut QTextEdit) -> *mut QWidget;
        unsafe fn toQWidget_QSlider(s: *mut QSlider) -> *mut QWidget;

        // --- QPushButton ---
        unsafe fn QPushButton_new(text: &CxxString, parent: *mut QWidget) -> *mut QPushButton;
        unsafe fn QPushButton_show(btn: *mut QPushButton);
        unsafe fn QPushButton_setText(btn: *mut QPushButton, text: &CxxString);
        unsafe fn QPushButton_delete(btn: *mut QPushButton);
        unsafe fn QPushButton_onClicked(btn: *mut QPushButton, ctx: u64);

        // --- QLabel ---
        unsafe fn QLabel_new(text: &CxxString, parent: *mut QWidget) -> *mut QLabel;
        unsafe fn QLabel_setText(label: *mut QLabel, text: &CxxString);
        unsafe fn QLabel_delete(label: *mut QLabel);

        // --- QLineEdit ---
        unsafe fn QLineEdit_new(text: &CxxString, parent: *mut QWidget) -> *mut QLineEdit;
        unsafe fn QLineEdit_text(edit: *mut QLineEdit) -> String;
        unsafe fn QLineEdit_setText(edit: *mut QLineEdit, text: &CxxString);
        unsafe fn QLineEdit_delete(edit: *mut QLineEdit);
        unsafe fn QLineEdit_onReturnPressed(edit: *mut QLineEdit, ctx: u64);

        // --- QCheckBox ---
        unsafe fn QCheckBox_new(text: &CxxString, parent: *mut QWidget) -> *mut QCheckBox;
        unsafe fn QCheckBox_isChecked(cb: *mut QCheckBox) -> bool;
        unsafe fn QCheckBox_setChecked(cb: *mut QCheckBox, checked: bool);
        unsafe fn QCheckBox_delete(cb: *mut QCheckBox);
        unsafe fn QCheckBox_onToggled(cb: *mut QCheckBox, ctx: u64);

        // --- QComboBox ---
        unsafe fn QComboBox_new(parent: *mut QWidget) -> *mut QComboBox;
        unsafe fn QComboBox_addItem(cb: *mut QComboBox, text: &CxxString);
        unsafe fn QComboBox_currentText(cb: *mut QComboBox) -> String;
        unsafe fn QComboBox_setCurrentIndex(cb: *mut QComboBox, index: i32);
        unsafe fn QComboBox_delete(cb: *mut QComboBox);
        unsafe fn QComboBox_onCurrentTextChanged(cb: *mut QComboBox, ctx: u64);

        // --- QTextEdit ---
        unsafe fn QTextEdit_new(parent: *mut QWidget) -> *mut QTextEdit;
        unsafe fn QTextEdit_toPlainText(edit: *mut QTextEdit) -> String;
        unsafe fn QTextEdit_setPlainText(edit: *mut QTextEdit, text: &CxxString);
        unsafe fn QTextEdit_delete(edit: *mut QTextEdit);
        unsafe fn QTextEdit_onTextChanged(edit: *mut QTextEdit, ctx: u64);

        // --- QSlider ---
        unsafe fn QSlider_new(orientation: i32, parent: *mut QWidget) -> *mut QSlider;
        unsafe fn QSlider_value(s: *mut QSlider) -> i32;
        unsafe fn QSlider_setValue(s: *mut QSlider, value: i32);
        unsafe fn QSlider_setRange(s: *mut QSlider, min: i32, max: i32);
        unsafe fn QSlider_delete(s: *mut QSlider);
        unsafe fn QSlider_onValueChanged(s: *mut QSlider, ctx: u64);

        // --- QVBoxLayout ---
        unsafe fn QVBoxLayout_new(parent: *mut QWidget) -> *mut QVBoxLayout;
        unsafe fn QVBoxLayout_addWidget(layout: *mut QVBoxLayout, widget: *mut QWidget);
        unsafe fn QVBoxLayout_delete(layout: *mut QVBoxLayout);

        // --- QHBoxLayout ---
        unsafe fn QHBoxLayout_new(parent: *mut QWidget) -> *mut QHBoxLayout;
        unsafe fn QHBoxLayout_addWidget(layout: *mut QHBoxLayout, widget: *mut QWidget);
        unsafe fn QHBoxLayout_delete(layout: *mut QHBoxLayout);

        // --- QGridLayout ---
        unsafe fn QGridLayout_new(parent: *mut QWidget) -> *mut QGridLayout;
        unsafe fn QGridLayout_addWidget(
            layout: *mut QGridLayout, widget: *mut QWidget,
            row: i32, col: i32, rowSpan: i32, colSpan: i32,
        );
        unsafe fn QGridLayout_delete(layout: *mut QGridLayout);

        // --- QTimer ---
        unsafe fn QTimer_new() -> *mut QTimer;
        unsafe fn QTimer_start(timer: *mut QTimer, interval_ms: i32);
        unsafe fn QTimer_stop(timer: *mut QTimer);
        unsafe fn QTimer_isActive(timer: *mut QTimer) -> bool;
        unsafe fn QTimer_delete(timer: *mut QTimer);
        unsafe fn QTimer_onTimeout(timer: *mut QTimer, ctx: u64);

        // --- QMessageBox ---
        unsafe fn QMessageBox_information(
            parent: *mut QWidget, title: &CxxString, text: &CxxString,
        );
        unsafe fn QMessageBox_warning(
            parent: *mut QWidget, title: &CxxString, text: &CxxString,
        );
        unsafe fn QMessageBox_critical(
            parent: *mut QWidget, title: &CxxString, text: &CxxString,
        );
        unsafe fn QMessageBox_question(
            parent: *mut QWidget, title: &CxxString, text: &CxxString,
        ) -> i32;
    }

    #[cfg(feature = "ui")]
    unsafe extern "C++" {
        include!("src/cpp/uiloader.h");

        type QUiLoader;
        unsafe fn QUiLoader_new() -> *mut QUiLoader;
        unsafe fn QUiLoader_load(loader: *mut QUiLoader, ui_path: &CxxString, parent: *mut QWidget) -> *mut QWidget;
        unsafe fn QUiLoader_delete(loader: *mut QUiLoader);
    }
}

pub use ffi_inner::*;