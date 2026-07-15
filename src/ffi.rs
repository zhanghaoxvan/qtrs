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
        type QListWidget;
        type QAction;
        type QMainWindow;
        type QToolBar;
        type QStatusBar;
        type QMessageBox;
        type QProgressDialog;
        type QTableWidget;
        type QTreeWidget;
        type QScrollArea;
        type QStackedWidget;
        type QSplitter;
        type QDateEdit;
        type QTimeEdit;
        type QDateTimeEdit;
        type QPlainTextEdit;
        type QTextBrowser;
        type QFrame;
        type QToolButton;
        type QCalendarWidget;
        type QShortcut;
        type QSystemTrayIcon;
        type QFont;

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
        unsafe fn qtrs_setStringTrampoline(trampoline: unsafe extern "C" fn(u64, String));

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
        unsafe fn QWidget_setFont(widget: *mut QWidget, font: *mut QFont);
        unsafe fn QWidget_font(widget: *mut QWidget) -> *mut QFont;
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
        unsafe fn QWidget_findListWidget(parent: *mut QWidget, name: &CxxString) -> *mut QListWidget;
        unsafe fn QWidget_findProgressDialog(parent: *mut QWidget, name: &CxxString) -> *mut QProgressDialog;
        unsafe fn QWidget_findScrollArea(parent: *mut QWidget, name: &CxxString) -> *mut QScrollArea;
        unsafe fn QWidget_findTableWidget(parent: *mut QWidget, name: &CxxString) -> *mut QTableWidget;
        unsafe fn QWidget_findTreeWidget(parent: *mut QWidget, name: &CxxString) -> *mut QTreeWidget;
        unsafe fn QWidget_findStackedWidget(parent: *mut QWidget, name: &CxxString) -> *mut QStackedWidget;
        unsafe fn QWidget_findSplitter(parent: *mut QWidget, name: &CxxString) -> *mut QSplitter;
        unsafe fn QWidget_findFrame(parent: *mut QWidget, name: &CxxString) -> *mut QFrame;
        unsafe fn QWidget_findToolButton(parent: *mut QWidget, name: &CxxString) -> *mut QToolButton;
        unsafe fn QWidget_findCalendarWidget(parent: *mut QWidget, name: &CxxString) -> *mut QCalendarWidget;
        unsafe fn QWidget_findDateEdit(parent: *mut QWidget, name: &CxxString) -> *mut QDateEdit;
        unsafe fn QWidget_findTimeEdit(parent: *mut QWidget, name: &CxxString) -> *mut QTimeEdit;
        unsafe fn QWidget_findDateTimeEdit(parent: *mut QWidget, name: &CxxString) -> *mut QDateTimeEdit;
        unsafe fn QWidget_findPlainTextEdit(parent: *mut QWidget, name: &CxxString) -> *mut QPlainTextEdit;
        unsafe fn QWidget_findTextBrowser(parent: *mut QWidget, name: &CxxString) -> *mut QTextBrowser;

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

        // ============================================================
        // ListWidget
        // ============================================================
        

        unsafe fn QListWidget_new(parent: *mut QWidget) -> *mut QListWidget;
        unsafe fn QListWidget_delete(w: *mut QListWidget);
        unsafe fn QListWidget_addItem(w: *mut QListWidget, text: &CxxString);
        unsafe fn QListWidget_addItems(w: *mut QListWidget, items: &CxxVector<CxxString>);
        unsafe fn QListWidget_insertItem(w: *mut QListWidget, row: i32, text: &CxxString);
        unsafe fn QListWidget_clear(w: *mut QListWidget);
        unsafe fn QListWidget_removeItem(w: *mut QListWidget, row: i32);
        unsafe fn QListWidget_count(w: *mut QListWidget) -> i32;
        unsafe fn QListWidget_itemText(w: *mut QListWidget, row: i32) -> String;
        unsafe fn QListWidget_currentRow(w: *mut QListWidget) -> i32;
        unsafe fn QListWidget_currentText(w: *mut QListWidget) -> String;
        unsafe fn QListWidget_setCurrentRow(w: *mut QListWidget, row: i32);
        unsafe fn QListWidget_setSelectionMode(w: *mut QListWidget, mode: i32);
        unsafe fn QListWidget_onItemClicked(w: *mut QListWidget, ctx: u64);
        unsafe fn QListWidget_onItemDoubleClicked(w: *mut QListWidget, ctx: u64);
        unsafe fn QListWidget_onCurrentItemChanged(w: *mut QListWidget, ctx: u64);
        unsafe fn toQWidget_QListWidget(w: *mut QListWidget) -> *mut QWidget;
        unsafe fn toQWidget_QMainWindow(w: *mut QMainWindow) -> *mut QWidget;
        unsafe fn toQWidget_QToolBar(toolbar: *mut QToolBar) -> *mut QWidget;
        unsafe fn toQWidget_QStatusBar(bar: *mut QStatusBar) -> *mut QWidget;
        unsafe fn toQWidget_QMessageBox(w: *mut QMessageBox) -> *mut QWidget;
        unsafe fn toQWidget_QProgressDialog(w: *mut QProgressDialog) -> *mut QWidget;
        unsafe fn toQWidget_QTableWidget(w: *mut QTableWidget) -> *mut QWidget;
        unsafe fn toQWidget_QTreeWidget(w: *mut QTreeWidget) -> *mut QWidget;
        unsafe fn toQWidget_QScrollArea(w: *mut QScrollArea) -> *mut QWidget;
        unsafe fn toQWidget_QStackedWidget(w: *mut QStackedWidget) -> *mut QWidget;
        unsafe fn toQWidget_QSplitter(w: *mut QSplitter) -> *mut QWidget;
        unsafe fn toQWidget_QDateEdit(w: *mut QDateEdit) -> *mut QWidget;
        unsafe fn toQWidget_QTimeEdit(w: *mut QTimeEdit) -> *mut QWidget;
        unsafe fn toQWidget_QDateTimeEdit(w: *mut QDateTimeEdit) -> *mut QWidget;
        unsafe fn toQWidget_QPlainTextEdit(w: *mut QPlainTextEdit) -> *mut QWidget;
        unsafe fn toQWidget_QTextBrowser(w: *mut QTextBrowser) -> *mut QWidget;
        unsafe fn toQWidget_QFrame(w: *mut QFrame) -> *mut QWidget;
        unsafe fn toQWidget_QToolButton(btn: *mut QToolButton) -> *mut QWidget;
        unsafe fn toQWidget_QCalendarWidget(cal: *mut QCalendarWidget) -> *mut QWidget;

        // --- QAction ---
        unsafe fn QAction_new(text: &CxxString, parent: *mut QWidget) -> *mut QAction;
        unsafe fn QAction_delete(action: *mut QAction);
        unsafe fn QAction_setText(action: *mut QAction, text: &CxxString);
        unsafe fn QAction_setIcon(action: *mut QAction, icon_path: &CxxString);
        unsafe fn QAction_setCheckable(action: *mut QAction, checkable: bool);
        unsafe fn QAction_setChecked(action: *mut QAction, checked: bool);
        unsafe fn QAction_setShortcut(action: *mut QAction, key: &CxxString);
        unsafe fn QAction_setEnabled(action: *mut QAction, enabled: bool);
        unsafe fn QAction_setToolTip(action: *mut QAction, tip: &CxxString);
        unsafe fn QAction_text(action: *mut QAction) -> String;
        unsafe fn QAction_isChecked(action: *mut QAction) -> bool;
        unsafe fn QAction_isEnabled(action: *mut QAction) -> bool;
        unsafe fn QAction_onTriggered(action: *mut QAction, ctx: u64);
        unsafe fn QAction_onToggled(action: *mut QAction, ctx: u64);

        // --- QMainWindow ---
        unsafe fn QMainWindow_new(parent: *mut QWidget) -> *mut QMainWindow;
        unsafe fn QMainWindow_delete(w: *mut QMainWindow);
        unsafe fn QMainWindow_setMenuBar(w: *mut QMainWindow, menuBar: *mut QMenuBar);
        unsafe fn QMainWindow_setCentralWidget(w: *mut QMainWindow, central: *mut QWidget);
        unsafe fn QMainWindow_setStatusBar(w: *mut QMainWindow, statusBar: *mut QStatusBar);
        unsafe fn QMainWindow_addToolBar(w: *mut QMainWindow, title: &CxxString) -> *mut QToolBar;
        unsafe fn QMainWindow_addToolBarBreak(w: *mut QMainWindow);
        unsafe fn QMainWindow_setWindowTitle(w: *mut QMainWindow, title: &CxxString);
        unsafe fn QMainWindow_resize(w: *mut QMainWindow, width: i32, height: i32);
        unsafe fn QMainWindow_show(w: *mut QMainWindow);
        unsafe fn QMainWindow_hide(w: *mut QMainWindow);
        unsafe fn QMainWindow_setDockOptions(w: *mut QMainWindow, options: i32);
        unsafe fn QMainWindow_setTabPosition(w: *mut QMainWindow, areas: i32, tabPos: i32);

        // --- QToolBar ---
        unsafe fn QToolBar_new(title: &CxxString, parent: *mut QWidget) -> *mut QToolBar;
        unsafe fn QToolBar_delete(toolbar: *mut QToolBar);
        unsafe fn QToolBar_addAction(toolbar: *mut QToolBar, text: &CxxString, ctx: u64);
        unsafe fn QToolBar_addSeparator(toolbar: *mut QToolBar);
        unsafe fn QToolBar_addWidget(toolbar: *mut QToolBar, widget: *mut QWidget);
        unsafe fn QToolBar_setMovable(toolbar: *mut QToolBar, movable: bool);
        unsafe fn QToolBar_setFloatable(toolbar: *mut QToolBar, floatable: bool);
        unsafe fn QToolBar_setIconSize(toolbar: *mut QToolBar, w: i32, h: i32);
        unsafe fn QToolBar_setAllowedAreas(toolbar: *mut QToolBar, areas: i32);
        unsafe fn QToolBar_setToolButtonStyle(toolbar: *mut QToolBar, style: i32);
        unsafe fn QToolBar_clear(toolbar: *mut QToolBar);

        // --- QStatusBar ---
        unsafe fn QStatusBar_new(parent: *mut QWidget) -> *mut QStatusBar;
        unsafe fn QStatusBar_delete(bar: *mut QStatusBar);
        unsafe fn QStatusBar_showMessage(bar: *mut QStatusBar, text: &CxxString, timeout_ms: i32);
        unsafe fn QStatusBar_clearMessage(bar: *mut QStatusBar);
        unsafe fn QStatusBar_addWidget(bar: *mut QStatusBar, widget: *mut QWidget);
        unsafe fn QStatusBar_addPermanentWidget(bar: *mut QStatusBar, widget: *mut QWidget);
        unsafe fn QStatusBar_removeWidget(bar: *mut QStatusBar, widget: *mut QWidget);
        unsafe fn QStatusBar_setSizeGripEnabled(bar: *mut QStatusBar, enabled: bool);

        // --- QMessageBox (object-based) ---
        unsafe fn QMessageBox_new(parent: *mut QWidget) -> *mut QMessageBox;
        unsafe fn QMessageBox_delete(w: *mut QMessageBox);
        unsafe fn QMessageBox_setIcon(w: *mut QMessageBox, icon: i32);
        unsafe fn QMessageBox_setText(w: *mut QMessageBox, text: &CxxString);
        unsafe fn QMessageBox_setInformativeText(w: *mut QMessageBox, text: &CxxString);
        unsafe fn QMessageBox_setWindowTitle(w: *mut QMessageBox, title: &CxxString);
        unsafe fn QMessageBox_setStandardButtons(w: *mut QMessageBox, buttons: i32);
        unsafe fn QMessageBox_setDefaultButton(w: *mut QMessageBox, button: i32);
        unsafe fn QMessageBox_setDetailedText(w: *mut QMessageBox, text: &CxxString);
        unsafe fn QMessageBox_exec(w: *mut QMessageBox) -> i32;
        unsafe fn QMessageBox_about(parent: *mut QWidget, title: &CxxString, text: &CxxString);

        // --- QInputDialog ---
        unsafe fn QInputDialog_getText(
            parent: *mut QWidget,
            title: &CxxString,
            label: &CxxString,
            text: &CxxString,
        ) -> String;
        unsafe fn QInputDialog_getInt(
            parent: *mut QWidget,
            title: &CxxString,
            label: &CxxString,
            value: i32,
            min: i32,
            max: i32,
            step: i32,
        ) -> i32;
        unsafe fn QInputDialog_getDouble(
            parent: *mut QWidget,
            title: &CxxString,
            label: &CxxString,
            value: f64,
            min: f64,
            max: f64,
            decimals: i32,
        ) -> f64;
        unsafe fn QInputDialog_getItem(
            parent: *mut QWidget,
            title: &CxxString,
            label: &CxxString,
            items: Vec<String>,
            current: i32,
            editable: bool,
        ) -> String;

        // --- QProgressDialog ---
        unsafe fn QProgressDialog_new(
            label: &CxxString,
            cancelText: &CxxString,
            min: i32,
            max: i32,
            parent: *mut QWidget,
        ) -> *mut QProgressDialog;
        unsafe fn QProgressDialog_delete(w: *mut QProgressDialog);
        unsafe fn QProgressDialog_setMinimum(w: *mut QProgressDialog, min: i32);
        unsafe fn QProgressDialog_setMaximum(w: *mut QProgressDialog, max: i32);
        unsafe fn QProgressDialog_setRange(w: *mut QProgressDialog, min: i32, max: i32);
        unsafe fn QProgressDialog_setValue(w: *mut QProgressDialog, value: i32);
        unsafe fn QProgressDialog_value(w: *mut QProgressDialog) -> i32;
        unsafe fn QProgressDialog_setLabelText(w: *mut QProgressDialog, text: &CxxString);
        unsafe fn QProgressDialog_setCancelButtonText(w: *mut QProgressDialog, text: &CxxString);
        unsafe fn QProgressDialog_wasCanceled(w: *mut QProgressDialog) -> bool;
        unsafe fn QProgressDialog_setMinimumDuration(w: *mut QProgressDialog, ms: i32);
        unsafe fn QProgressDialog_setAutoClose(w: *mut QProgressDialog, close: bool);
        unsafe fn QProgressDialog_setAutoReset(w: *mut QProgressDialog, reset: bool);
        unsafe fn QProgressDialog_show(w: *mut QProgressDialog);
        unsafe fn QProgressDialog_hide(w: *mut QProgressDialog);
        unsafe fn QProgressDialog_close(w: *mut QProgressDialog);

        // --- QTableWidget ---
        unsafe fn QTableWidget_new(rows: i32, cols: i32, parent: *mut QWidget) -> *mut QTableWidget;
        unsafe fn QTableWidget_delete(w: *mut QTableWidget);
        unsafe fn QTableWidget_setRowCount(w: *mut QTableWidget, rows: i32);
        unsafe fn QTableWidget_setColumnCount(w: *mut QTableWidget, cols: i32);
        unsafe fn QTableWidget_setItem(w: *mut QTableWidget, row: i32, col: i32, text: &CxxString);
        unsafe fn QTableWidget_itemText(w: *mut QTableWidget, row: i32, col: i32) -> String;
        unsafe fn QTableWidget_setHorizontalHeaderLabels(w: *mut QTableWidget, labels: Vec<String>);
        unsafe fn QTableWidget_setVerticalHeaderLabels(w: *mut QTableWidget, labels: Vec<String>);
        unsafe fn QTableWidget_setCurrentCell(w: *mut QTableWidget, row: i32, col: i32);
        unsafe fn QTableWidget_currentRow(w: *mut QTableWidget) -> i32;
        unsafe fn QTableWidget_currentColumn(w: *mut QTableWidget) -> i32;
        unsafe fn QTableWidget_selectedRows(w: *mut QTableWidget) -> Vec<i32>;
        unsafe fn QTableWidget_clear(w: *mut QTableWidget);
        unsafe fn QTableWidget_clearContents(w: *mut QTableWidget);
        unsafe fn QTableWidget_setSelectionMode(w: *mut QTableWidget, mode: i32);
        unsafe fn QTableWidget_setSelectionBehavior(w: *mut QTableWidget, behavior: i32);
        unsafe fn QTableWidget_removeRow(w: *mut QTableWidget, row: i32);
        unsafe fn QTableWidget_insertRow(w: *mut QTableWidget, row: i32);
        unsafe fn QTableWidget_setColumnWidth(w: *mut QTableWidget, col: i32, width: i32);
        unsafe fn QTableWidget_setRowHeight(w: *mut QTableWidget, row: i32, height: i32);
        unsafe fn QTableWidget_onCellClicked(w: *mut QTableWidget, ctx: u64);
        unsafe fn QTableWidget_onCellDoubleClicked(w: *mut QTableWidget, ctx: u64);
        unsafe fn QTableWidget_onCurrentCellChanged(w: *mut QTableWidget, ctx: u64);

        // --- QTreeWidget ---
        unsafe fn QTreeWidget_new(parent: *mut QWidget) -> *mut QTreeWidget;
        unsafe fn QTreeWidget_delete(w: *mut QTreeWidget);
        unsafe fn QTreeWidget_addTopLevelItem(w: *mut QTreeWidget, text: &CxxString);
        unsafe fn QTreeWidget_clear(w: *mut QTreeWidget);
        unsafe fn QTreeWidget_currentItemText(w: *mut QTreeWidget) -> String;
        unsafe fn QTreeWidget_setHeaderLabel(w: *mut QTreeWidget, text: &CxxString);
        unsafe fn QTreeWidget_setHeaderLabels(w: *mut QTreeWidget, labels: Vec<String>);
        unsafe fn QTreeWidget_expandAll(w: *mut QTreeWidget);
        unsafe fn QTreeWidget_collapseAll(w: *mut QTreeWidget);
        unsafe fn QTreeWidget_expandItem(w: *mut QTreeWidget, text: &CxxString);
        unsafe fn QTreeWidget_setCurrentItem(w: *mut QTreeWidget, text: &CxxString);
        unsafe fn QTreeWidget_topLevelItemCount(w: *mut QTreeWidget) -> i32;
        unsafe fn QTreeWidget_onItemClicked(w: *mut QTreeWidget, ctx: u64);
        unsafe fn QTreeWidget_onItemDoubleClicked(w: *mut QTreeWidget, ctx: u64);
        unsafe fn QTreeWidget_onItemExpanded(w: *mut QTreeWidget, ctx: u64);
        unsafe fn QTreeWidget_onItemCollapsed(w: *mut QTreeWidget, ctx: u64);
        unsafe fn QTreeWidget_onCurrentItemChanged(w: *mut QTreeWidget, ctx: u64);

        // --- QScrollArea ---
        unsafe fn QScrollArea_new(parent: *mut QWidget) -> *mut QScrollArea;
        unsafe fn QScrollArea_delete(w: *mut QScrollArea);
        unsafe fn QScrollArea_setWidget(w: *mut QScrollArea, widget: *mut QWidget);
        unsafe fn QScrollArea_takeWidget(w: *mut QScrollArea) -> *mut QWidget;
        unsafe fn QScrollArea_setWidgetResizable(w: *mut QScrollArea, resizable: bool);
        unsafe fn QScrollArea_setHorizontalScrollBarPolicy(w: *mut QScrollArea, policy: i32);
        unsafe fn QScrollArea_setVerticalScrollBarPolicy(w: *mut QScrollArea, policy: i32);
        unsafe fn QScrollArea_ensureVisible(w: *mut QScrollArea, x: i32, y: i32);
        unsafe fn QScrollArea_ensureWidgetVisible(w: *mut QScrollArea, widget: *mut QWidget);

        // --- QStackedWidget ---
        unsafe fn QStackedWidget_new(parent: *mut QWidget) -> *mut QStackedWidget;
        unsafe fn QStackedWidget_delete(w: *mut QStackedWidget);
        unsafe fn QStackedWidget_addWidget(w: *mut QStackedWidget, page: *mut QWidget) -> i32;
        unsafe fn QStackedWidget_insertWidget(w: *mut QStackedWidget, index: i32, page: *mut QWidget) -> i32;
        unsafe fn QStackedWidget_removeWidget(w: *mut QStackedWidget, page: *mut QWidget);
        unsafe fn QStackedWidget_setCurrentIndex(w: *mut QStackedWidget, index: i32);
        unsafe fn QStackedWidget_currentIndex(w: *mut QStackedWidget) -> i32;
        unsafe fn QStackedWidget_count(w: *mut QStackedWidget) -> i32;
        unsafe fn QStackedWidget_widget(w: *mut QStackedWidget, index: i32) -> *mut QWidget;
        unsafe fn QStackedWidget_onCurrentChanged(w: *mut QStackedWidget, ctx: u64);

        // --- QSplitter ---
        unsafe fn QSplitter_new(orientation: i32, parent: *mut QWidget) -> *mut QSplitter;
        unsafe fn QSplitter_delete(w: *mut QSplitter);
        unsafe fn QSplitter_addWidget(w: *mut QSplitter, widget: *mut QWidget);
        unsafe fn QSplitter_insertWidget(w: *mut QSplitter, index: i32, widget: *mut QWidget);
        unsafe fn QSplitter_setStretchFactor(w: *mut QSplitter, index: i32, stretch: i32);
        unsafe fn QSplitter_setSizes(w: *mut QSplitter, sizes: Vec<i32>);
        unsafe fn QSplitter_sizes(w: *mut QSplitter) -> Vec<i32>;
        unsafe fn QSplitter_setOrientation(w: *mut QSplitter, orientation: i32);
        unsafe fn QSplitter_count(w: *mut QSplitter) -> i32;
        unsafe fn QSplitter_setHandleWidth(w: *mut QSplitter, width: i32);
        unsafe fn QSplitter_setChildrenCollapsible(w: *mut QSplitter, collapsible: bool);

        // ============================================================
        // QDateEdit
        // ============================================================

        unsafe fn QDateEdit_new(parent: *mut QWidget) -> *mut QDateEdit;
        unsafe fn QDateEdit_delete(w: *mut QDateEdit);
        unsafe fn QDateEdit_setDate(w: *mut QDateEdit, date: &CxxString);
        unsafe fn QDateEdit_date(w: *mut QDateEdit) -> String;
        unsafe fn QDateEdit_setMinimumDate(w: *mut QDateEdit, date: &CxxString);
        unsafe fn QDateEdit_setMaximumDate(w: *mut QDateEdit, date: &CxxString);
        unsafe fn QDateEdit_clearMinimumDate(w: *mut QDateEdit);
        unsafe fn QDateEdit_clearMaximumDate(w: *mut QDateEdit);
        unsafe fn QDateEdit_setDisplayFormat(w: *mut QDateEdit, format: &CxxString);
        unsafe fn QDateEdit_setCalendarPopup(w: *mut QDateEdit, enabled: bool);
        unsafe fn QDateEdit_onDateChanged(w: *mut QDateEdit, ctx: u64);

        // ============================================================
        // QTimeEdit
        // ============================================================

        unsafe fn QTimeEdit_new(parent: *mut QWidget) -> *mut QTimeEdit;
        unsafe fn QTimeEdit_delete(w: *mut QTimeEdit);
        unsafe fn QTimeEdit_setTime(w: *mut QTimeEdit, time: &CxxString);
        unsafe fn QTimeEdit_time(w: *mut QTimeEdit) -> String;
        unsafe fn QTimeEdit_setDisplayFormat(w: *mut QTimeEdit, format: &CxxString);
        unsafe fn QTimeEdit_onTimeChanged(w: *mut QTimeEdit, ctx: u64);

        // ============================================================
        // QDateTimeEdit
        // ============================================================

        unsafe fn QDateTimeEdit_new(parent: *mut QWidget) -> *mut QDateTimeEdit;
        unsafe fn QDateTimeEdit_delete(w: *mut QDateTimeEdit);
        unsafe fn QDateTimeEdit_setDateTime(w: *mut QDateTimeEdit, dt: &CxxString);
        unsafe fn QDateTimeEdit_dateTime(w: *mut QDateTimeEdit) -> String;
        unsafe fn QDateTimeEdit_setDisplayFormat(w: *mut QDateTimeEdit, format: &CxxString);
        unsafe fn QDateTimeEdit_setCalendarPopup(w: *mut QDateTimeEdit, enabled: bool);
        unsafe fn QDateTimeEdit_onDateTimeChanged(w: *mut QDateTimeEdit, ctx: u64);

        // ============================================================
        // QPlainTextEdit
        // ============================================================

        unsafe fn QPlainTextEdit_new(parent: *mut QWidget) -> *mut QPlainTextEdit;
        unsafe fn QPlainTextEdit_delete(w: *mut QPlainTextEdit);
        unsafe fn QPlainTextEdit_setPlainText(w: *mut QPlainTextEdit, text: &CxxString);
        unsafe fn QPlainTextEdit_plainText(w: *mut QPlainTextEdit) -> String;
        unsafe fn QPlainTextEdit_setPlaceholderText(w: *mut QPlainTextEdit, text: &CxxString);
        unsafe fn QPlainTextEdit_setReadOnly(w: *mut QPlainTextEdit, readOnly: bool);
        unsafe fn QPlainTextEdit_setLineWrapMode(w: *mut QPlainTextEdit, mode: i32);
        unsafe fn QPlainTextEdit_appendPlainText(w: *mut QPlainTextEdit, text: &CxxString);
        unsafe fn QPlainTextEdit_clear(w: *mut QPlainTextEdit);
        unsafe fn QPlainTextEdit_onTextChanged(w: *mut QPlainTextEdit, ctx: u64);
        unsafe fn QPlainTextEdit_onCursorPositionChanged(w: *mut QPlainTextEdit, ctx: u64);

        // ============================================================
        // QTextBrowser
        // ============================================================

        unsafe fn QTextBrowser_new(parent: *mut QWidget) -> *mut QTextBrowser;
        unsafe fn QTextBrowser_delete(w: *mut QTextBrowser);
        unsafe fn QTextBrowser_setHtml(w: *mut QTextBrowser, html: &CxxString);
        unsafe fn QTextBrowser_setPlainText(w: *mut QTextBrowser, text: &CxxString);
        unsafe fn QTextBrowser_plainText(w: *mut QTextBrowser) -> String;
        unsafe fn QTextBrowser_toHtml(w: *mut QTextBrowser) -> String;
        unsafe fn QTextBrowser_setOpenExternalLinks(w: *mut QTextBrowser, open: bool);
        unsafe fn QTextBrowser_setOpenLinks(w: *mut QTextBrowser, open: bool);
        unsafe fn QTextBrowser_setSource(w: *mut QTextBrowser, url: &CxxString);
        unsafe fn QTextBrowser_source(w: *mut QTextBrowser) -> String;
        unsafe fn QTextBrowser_clear(w: *mut QTextBrowser);
        unsafe fn QTextBrowser_append(w: *mut QTextBrowser, text: &CxxString);
        unsafe fn QTextBrowser_setSearchPaths(w: *mut QTextBrowser, paths: Vec<String>);
        unsafe fn QTextBrowser_onAnchorClicked(w: *mut QTextBrowser, ctx: u64);
        unsafe fn QTextBrowser_onTextChanged(w: *mut QTextBrowser, ctx: u64);

        // ============================================================
        // QFrame
        // ============================================================

        unsafe fn QFrame_new(parent: *mut QWidget) -> *mut QFrame;
        unsafe fn QFrame_delete(frame: *mut QFrame);
        unsafe fn QFrame_setFrameShape(frame: *mut QFrame, shape: i32);
        unsafe fn QFrame_setFrameShadow(frame: *mut QFrame, shadow: i32);
        unsafe fn QFrame_setLineWidth(frame: *mut QFrame, width: i32);
        unsafe fn QFrame_setMidLineWidth(frame: *mut QFrame, width: i32);
        unsafe fn QFrame_setFrameStyle(frame: *mut QFrame, style: i32);

        // ============================================================
        // QToolButton
        // ============================================================

        unsafe fn QToolButton_new(parent: *mut QWidget) -> *mut QToolButton;
        unsafe fn QToolButton_delete(btn: *mut QToolButton);
        unsafe fn QToolButton_setText(btn: *mut QToolButton, text: &CxxString);
        unsafe fn QToolButton_setIcon(btn: *mut QToolButton, icon_path: &CxxString);
        unsafe fn QToolButton_setToolButtonStyle(btn: *mut QToolButton, style: i32);
        unsafe fn QToolButton_setPopupMode(btn: *mut QToolButton, mode: i32);
        unsafe fn QToolButton_setAutoRaise(btn: *mut QToolButton, enabled: bool);
        unsafe fn QToolButton_setCheckable(btn: *mut QToolButton, checkable: bool);
        unsafe fn QToolButton_setChecked(btn: *mut QToolButton, checked: bool);
        unsafe fn QToolButton_setShortcut(btn: *mut QToolButton, key: &CxxString);
        unsafe fn QToolButton_onClicked(btn: *mut QToolButton, ctx: u64);
        unsafe fn QToolButton_onToggled(btn: *mut QToolButton, ctx: u64);

        // ============================================================
        // QCalendarWidget
        // ============================================================

        unsafe fn QCalendarWidget_new(parent: *mut QWidget) -> *mut QCalendarWidget;
        unsafe fn QCalendarWidget_delete(cal: *mut QCalendarWidget);
        unsafe fn QCalendarWidget_setSelectedDate(cal: *mut QCalendarWidget, date_str: &CxxString);
        unsafe fn QCalendarWidget_selectedDate(cal: *mut QCalendarWidget) -> String;
        unsafe fn QCalendarWidget_setMinimumDate(cal: *mut QCalendarWidget, date_str: &CxxString);
        unsafe fn QCalendarWidget_setMaximumDate(cal: *mut QCalendarWidget, date_str: &CxxString);
        unsafe fn QCalendarWidget_setFirstDayOfWeek(cal: *mut QCalendarWidget, day: i32);
        unsafe fn QCalendarWidget_setGridVisible(cal: *mut QCalendarWidget, visible: bool);
        unsafe fn QCalendarWidget_setNavigationBarVisible(cal: *mut QCalendarWidget, visible: bool);
        unsafe fn QCalendarWidget_onSelectionChanged(cal: *mut QCalendarWidget, ctx: u64);
        unsafe fn QCalendarWidget_onActivated(cal: *mut QCalendarWidget, ctx: u64);

        // ============================================================
        // QShortcut
        // ============================================================

        unsafe fn QShortcut_new(key: &CxxString, parent: *mut QWidget) -> *mut QShortcut;
        unsafe fn QShortcut_delete(sc: *mut QShortcut);
        unsafe fn QShortcut_setKey(sc: *mut QShortcut, key: &CxxString);
        unsafe fn QShortcut_setEnabled(sc: *mut QShortcut, enabled: bool);
        unsafe fn QShortcut_setAutoRepeat(sc: *mut QShortcut, repeat: bool);
        unsafe fn QShortcut_onActivated(sc: *mut QShortcut, ctx: u64);

        // ============================================================
        // QFont
        // ============================================================

        unsafe fn QFont_new() -> *mut QFont;
        unsafe fn QFont_setFamily(font: *mut QFont, family: &CxxString);
        unsafe fn QFont_setPointSize(font: *mut QFont, size: i32);
        unsafe fn QFont_setPixelSize(font: *mut QFont, size: i32);
        unsafe fn QFont_setBold(font: *mut QFont, bold: bool);
        unsafe fn QFont_setItalic(font: *mut QFont, italic: bool);
        unsafe fn QFont_setUnderline(font: *mut QFont, underline: bool);
        unsafe fn QFont_setStrikeOut(font: *mut QFont, strike: bool);
        unsafe fn QFont_setWeight(font: *mut QFont, weight: i32);
        unsafe fn QFont_family(font: *mut QFont) -> String;
        unsafe fn QFont_pointSize(font: *mut QFont) -> i32;
        unsafe fn QFont_pixelSize(font: *mut QFont) -> i32;
        unsafe fn QFont_bold(font: *mut QFont) -> bool;
        unsafe fn QFont_italic(font: *mut QFont) -> bool;
        unsafe fn QFont_underline(font: *mut QFont) -> bool;
        unsafe fn QFont_strikeOut(font: *mut QFont) -> bool;
        unsafe fn QFont_weight(font: *mut QFont) -> i32;
        unsafe fn QFont_delete(font: *mut QFont);

        // ============================================================
        // QSystemTrayIcon
        // ============================================================

        unsafe fn QSystemTrayIcon_new(icon_path: &CxxString, parent: *mut QObject) -> *mut QSystemTrayIcon;
        unsafe fn QSystemTrayIcon_delete(tray: *mut QSystemTrayIcon);
        unsafe fn QSystemTrayIcon_setIcon(tray: *mut QSystemTrayIcon, icon_path: &CxxString);
        unsafe fn QSystemTrayIcon_setToolTip(tray: *mut QSystemTrayIcon, tip: &CxxString);
        unsafe fn QSystemTrayIcon_show(tray: *mut QSystemTrayIcon);
        unsafe fn QSystemTrayIcon_hide(tray: *mut QSystemTrayIcon);
        unsafe fn QSystemTrayIcon_isVisible(tray: *mut QSystemTrayIcon) -> bool;
        unsafe fn QSystemTrayIcon_setContextMenu(tray: *mut QSystemTrayIcon, menu: *mut QMenu);
        unsafe fn QSystemTrayIcon_onActivated(tray: *mut QSystemTrayIcon, ctx: u64);

    }

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