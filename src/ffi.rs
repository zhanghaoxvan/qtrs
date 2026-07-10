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
        type QProgressBar;
        type QTimer;
        type QVBoxLayout;
        type QHBoxLayout;
        type QGridLayout;
        type QLayout;
        type QRadioButton;
        type QGroupBox;
        type QTabWidget;
        type QSpinBox;
        type QMenu;
        type QMenuBar;
        type QIcon;
        type QPoint;

        // --- QObject (base class for signal-slot connections) ---
        type QObject;

        // --- Generic signal-slot connection ---
        unsafe fn QObject_connect(
            sender: *mut QObject,
            sig: &CxxString,
            receiver: *mut QObject,
            slt: &CxxString,
            conn_type: i32,
        ) -> bool;

        unsafe fn QObject_disconnect(
            sender: *mut QObject,
            sig: &CxxString,
            receiver: *mut QObject,
            slt: &CxxString,
        ) -> bool;

        // --- Thread safety ---
        unsafe fn QObject_isInGuiThread() -> bool;

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
        unsafe fn QWidget_move(widget: *mut QWidget, x: i32, y: i32);
        unsafe fn QWidget_moveToPoint(widget: *mut QWidget, point: *mut QPoint);

        // --- Common QWidget properties ---
        unsafe fn QWidget_setEnabled(widget: *mut QWidget, enabled: bool);
        unsafe fn QWidget_setVisible(widget: *mut QWidget, visible: bool);
        unsafe fn QWidget_setToolTip(widget: *mut QWidget, tip: &CxxString);
        unsafe fn QWidget_setMinimumSize(widget: *mut QWidget, width: i32, height: i32);
        unsafe fn QWidget_setMaximumSize(widget: *mut QWidget, width: i32, height: i32);
        unsafe fn QWidget_setFixedSize(widget: *mut QWidget, width: i32, height: i32);
        unsafe fn QWidget_setStyleSheet(widget: *mut QWidget, css: &CxxString);
        unsafe fn QWidget_disconnectAll(widget: *mut QWidget);

        // --- toQWidget upcasts ---
        unsafe fn toQWidget_QWidget(w: *mut QWidget) -> *mut QWidget;
        unsafe fn toQWidget_QPushButton(btn: *mut QPushButton) -> *mut QWidget;
        unsafe fn toQWidget_QLabel(label: *mut QLabel) -> *mut QWidget;
        unsafe fn toQWidget_QLineEdit(edit: *mut QLineEdit) -> *mut QWidget;
        unsafe fn toQWidget_QCheckBox(cb: *mut QCheckBox) -> *mut QWidget;
        unsafe fn toQWidget_QComboBox(cb: *mut QComboBox) -> *mut QWidget;
        unsafe fn toQWidget_QTextEdit(edit: *mut QTextEdit) -> *mut QWidget;
        unsafe fn toQWidget_QSlider(s: *mut QSlider) -> *mut QWidget;
        unsafe fn toQWidget_QProgressBar(bar: *mut QProgressBar) -> *mut QWidget;
        unsafe fn toQWidget_QRadioButton(rb: *mut QRadioButton) -> *mut QWidget;
        unsafe fn toQWidget_QGroupBox(gb: *mut QGroupBox) -> *mut QWidget;
        unsafe fn toQWidget_QTabWidget(tw: *mut QTabWidget) -> *mut QWidget;
        unsafe fn toQWidget_QSpinBox(sb: *mut QSpinBox) -> *mut QWidget;
        unsafe fn toQWidget_QMenu(menu: *mut QMenu) -> *mut QWidget;
        unsafe fn toQWidget_QMenuBar(mb: *mut QMenuBar) -> *mut QWidget;

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
        unsafe fn QComboBox_onCurrentIndexChanged(cb: *mut QComboBox, ctx: u64);

        // --- QTextEdit ---
        unsafe fn QTextEdit_new(parent: *mut QWidget) -> *mut QTextEdit;
        unsafe fn QTextEdit_toPlainText(edit: *mut QTextEdit) -> String;
        unsafe fn QTextEdit_setPlainText(edit: *mut QTextEdit, text: &CxxString);
        unsafe fn QTextEdit_setPlaceholderText(edit: *mut QTextEdit, text: &CxxString);
        unsafe fn QTextEdit_delete(edit: *mut QTextEdit);
        unsafe fn QTextEdit_onTextChanged(edit: *mut QTextEdit, ctx: u64);

        // --- QSlider ---
        unsafe fn QSlider_new(orientation: i32, parent: *mut QWidget) -> *mut QSlider;
        unsafe fn QSlider_value(s: *mut QSlider) -> i32;
        unsafe fn QSlider_setValue(s: *mut QSlider, value: i32);
        unsafe fn QSlider_setRange(s: *mut QSlider, min: i32, max: i32);
        unsafe fn QSlider_delete(s: *mut QSlider);
        unsafe fn QSlider_onValueChanged(s: *mut QSlider, ctx: u64);

        // --- QProgressBar ---
        unsafe fn QProgressBar_new(parent: *mut QWidget) -> *mut QProgressBar;
        unsafe fn QProgressBar_setValue(bar: *mut QProgressBar, value: i32);
        unsafe fn QProgressBar_value(bar: *mut QProgressBar) -> i32;
        unsafe fn QProgressBar_setRange(bar: *mut QProgressBar, min: i32, max: i32);
        unsafe fn QProgressBar_setMinimum(bar: *mut QProgressBar, min: i32);
        unsafe fn QProgressBar_setMaximum(bar: *mut QProgressBar, max: i32);
        unsafe fn QProgressBar_setFormat(bar: *mut QProgressBar, format: &CxxString);
        unsafe fn QProgressBar_delete(bar: *mut QProgressBar);

        // --- QVBoxLayout ---
        unsafe fn QVBoxLayout_new(parent: *mut QWidget) -> *mut QVBoxLayout;
        unsafe fn QVBoxLayout_addWidget(layout: *mut QVBoxLayout, widget: *mut QWidget);
        unsafe fn QVBoxLayout_delete(layout: *mut QVBoxLayout);
        unsafe fn QVBoxLayout_setSpacing(layout: *mut QVBoxLayout, spacing: i32);
        unsafe fn QVBoxLayout_setContentsMargins(
            layout: *mut QVBoxLayout,
            left: i32,
            top: i32,
            right: i32,
            bottom: i32,
        );

        // --- QHBoxLayout ---
        unsafe fn QHBoxLayout_new(parent: *mut QWidget) -> *mut QHBoxLayout;
        unsafe fn QHBoxLayout_addWidget(layout: *mut QHBoxLayout, widget: *mut QWidget);
        unsafe fn QHBoxLayout_delete(layout: *mut QHBoxLayout);
        unsafe fn QHBoxLayout_setSpacing(layout: *mut QHBoxLayout, spacing: i32);
        unsafe fn QHBoxLayout_setContentsMargins(
            layout: *mut QHBoxLayout,
            left: i32,
            top: i32,
            right: i32,
            bottom: i32,
        );

        // --- QGridLayout ---
        unsafe fn QGridLayout_new(parent: *mut QWidget) -> *mut QGridLayout;
        unsafe fn QGridLayout_addWidget(
            layout: *mut QGridLayout,
            widget: *mut QWidget,
            row: i32,
            col: i32,
            rowSpan: i32,
            colSpan: i32,
        );
        unsafe fn QGridLayout_delete(layout: *mut QGridLayout);

        // --- findChild helpers (for widgets loaded from .ui files) ---
        unsafe fn QWidget_findWidget(parent: *mut QWidget, name: &CxxString) -> *mut QWidget;
        unsafe fn QWidget_findPushButton(parent: *mut QWidget, name: &CxxString) -> *mut QPushButton;
        unsafe fn QWidget_findLineEdit(parent: *mut QWidget, name: &CxxString) -> *mut QLineEdit;
        unsafe fn QWidget_findCheckBox(parent: *mut QWidget, name: &CxxString) -> *mut QCheckBox;
        unsafe fn QWidget_findLabel(parent: *mut QWidget, name: &CxxString) -> *mut QLabel;
        unsafe fn QWidget_findComboBox(parent: *mut QWidget, name: &CxxString) -> *mut QComboBox;
        unsafe fn QWidget_findSlider(parent: *mut QWidget, name: &CxxString) -> *mut QSlider;
        unsafe fn QWidget_findTextEdit(parent: *mut QWidget, name: &CxxString) -> *mut QTextEdit;
        unsafe fn QWidget_findProgressBar(parent: *mut QWidget, name: &CxxString) -> *mut QProgressBar;
        unsafe fn QWidget_findRadioButton(parent: *mut QWidget, name: &CxxString) -> *mut QRadioButton;
        unsafe fn QWidget_findGroupBox(parent: *mut QWidget, name: &CxxString) -> *mut QGroupBox;
        unsafe fn QWidget_findTabWidget(parent: *mut QWidget, name: &CxxString) -> *mut QTabWidget;
        unsafe fn QWidget_findSpinBox(parent: *mut QWidget, name: &CxxString) -> *mut QSpinBox;

        // --- QTimer ---
        unsafe fn QTimer_new() -> *mut QTimer;
        unsafe fn QTimer_start(timer: *mut QTimer, interval_ms: i32);
        unsafe fn QTimer_stop(timer: *mut QTimer);
        unsafe fn QTimer_isActive(timer: *mut QTimer) -> bool;
        unsafe fn QTimer_delete(timer: *mut QTimer);
        unsafe fn QTimer_onTimeout(timer: *mut QTimer, ctx: u64);
        unsafe fn QTimer_singleShot(interval_ms: i32, ctx: u64);

        // --- QMessageBox ---
        unsafe fn QMessageBox_information(
            parent: *mut QWidget,
            title: &CxxString,
            text: &CxxString,
        );
        unsafe fn QMessageBox_warning(
            parent: *mut QWidget,
            title: &CxxString,
            text: &CxxString,
        );
        unsafe fn QMessageBox_critical(
            parent: *mut QWidget,
            title: &CxxString,
            text: &CxxString,
        );
        unsafe fn QMessageBox_question(
            parent: *mut QWidget,
            title: &CxxString,
            text: &CxxString,
        ) -> i32;

        // --- QRadioButton ---
        unsafe fn QRadioButton_new(text: &CxxString, parent: *mut QWidget) -> *mut QRadioButton;
        unsafe fn QRadioButton_isChecked(rb: *mut QRadioButton) -> bool;
        unsafe fn QRadioButton_setChecked(rb: *mut QRadioButton, checked: bool);
        unsafe fn QRadioButton_delete(rb: *mut QRadioButton);
        unsafe fn QRadioButton_onToggled(rb: *mut QRadioButton, ctx: u64);

        // --- QGroupBox ---
        unsafe fn QGroupBox_new(title: &CxxString, parent: *mut QWidget) -> *mut QGroupBox;
        unsafe fn QGroupBox_setTitle(gb: *mut QGroupBox, title: &CxxString);
        unsafe fn QGroupBox_delete(gb: *mut QGroupBox);

        // --- QTabWidget ---
        unsafe fn QTabWidget_new(parent: *mut QWidget) -> *mut QTabWidget;
        unsafe fn QTabWidget_addTab(tw: *mut QTabWidget, page: *mut QWidget, label: &CxxString);
        unsafe fn QTabWidget_currentIndex(tw: *mut QTabWidget) -> i32;
        unsafe fn QTabWidget_setCurrentIndex(tw: *mut QTabWidget, index: i32);
        unsafe fn QTabWidget_delete(tw: *mut QTabWidget);
        unsafe fn QTabWidget_onCurrentChanged(tw: *mut QTabWidget, ctx: u64);

        // --- QSpinBox ---
        unsafe fn QSpinBox_new(parent: *mut QWidget) -> *mut QSpinBox;
        unsafe fn QSpinBox_setValue(sb: *mut QSpinBox, value: i32);
        unsafe fn QSpinBox_value(sb: *mut QSpinBox) -> i32;
        unsafe fn QSpinBox_setRange(sb: *mut QSpinBox, min: i32, max: i32);
        unsafe fn QSpinBox_setSuffix(sb: *mut QSpinBox, suffix: &CxxString);
        unsafe fn QSpinBox_delete(sb: *mut QSpinBox);
        unsafe fn QSpinBox_onValueChanged(sb: *mut QSpinBox, ctx: u64);

        // --- QMenu ---
        unsafe fn QMenu_new(title: &CxxString, parent: *mut QWidget) -> *mut QMenu;
        unsafe fn QMenu_addAction(menu: *mut QMenu, text: &CxxString, ctx: u64);
        unsafe fn QMenu_delete(menu: *mut QMenu);

        // --- QMenuBar ---
        unsafe fn QMenuBar_new(parent: *mut QWidget) -> *mut QMenuBar;
        unsafe fn QMenuBar_addMenu(mb: *mut QMenuBar, menu: *mut QMenu);
        unsafe fn QMenuBar_delete(mb: *mut QMenuBar);

        // --- QFileDialog ---
        unsafe fn QFileDialog_getOpenFileName(
            parent: *mut QWidget,
            caption: &CxxString,
            dir: &CxxString,
            filter: &CxxString,
        ) -> String;

        unsafe fn QFileDialog_getOpenFileNames(
            parent: *mut QWidget,
            caption: &CxxString,
            dir: &CxxString,
            filter: &CxxString,
        ) -> Vec<String>;

        unsafe fn QFileDialog_getSaveFileName(
            parent: *mut QWidget,
            caption: &CxxString,
            dir: &CxxString,
            filter: &CxxString,
        ) -> String;

        unsafe fn QFileDialog_getExistingDirectory(
            parent: *mut QWidget,
            caption: &CxxString,
            dir: &CxxString,
        ) -> String;

        // --- QPoint ---
        unsafe fn QPoint_new(x: i32, y: i32) -> *mut QPoint;
        unsafe fn QPoint_delete(point: *mut QPoint);

    }

    #[cfg(feature = "ui")]
    unsafe extern "C++" {
        include!("src/cpp/uiloader.h");

        type QUiLoader;
        unsafe fn QUiLoader_new() -> *mut QUiLoader;
        unsafe fn QUiLoader_load(
            loader: *mut QUiLoader,
            ui_path: &CxxString,
            parent: *mut QWidget,
        ) -> *mut QWidget;
        unsafe fn QUiLoader_delete(loader: *mut QUiLoader);
    }
}

pub use ffi_inner::*;