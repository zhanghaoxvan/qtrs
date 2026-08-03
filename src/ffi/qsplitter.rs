unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
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

        // ============================================================
        // QDial
        // ============================================================

        unsafe fn QDial_new(parent: *mut QWidget) -> *mut QDial;
        unsafe fn QDial_delete(dial: *mut QDial);
        unsafe fn QDial_value(dial: *mut QDial) -> i32;
        unsafe fn QDial_setValue(dial: *mut QDial, value: i32);
        unsafe fn QDial_setRange(dial: *mut QDial, min: i32, max: i32);
        unsafe fn QDial_setSingleStep(dial: *mut QDial, step: i32);
        unsafe fn QDial_setPageStep(dial: *mut QDial, step: i32);
        unsafe fn QDial_minimum(dial: *mut QDial) -> i32;
        unsafe fn QDial_maximum(dial: *mut QDial) -> i32;
        unsafe fn QDial_setMinimum(dial: *mut QDial, min: i32);
        unsafe fn QDial_setMaximum(dial: *mut QDial, max: i32);
        unsafe fn QDial_setNotchesVisible(dial: *mut QDial, visible: bool);
        unsafe fn QDial_notchesVisible(dial: *mut QDial) -> bool;
        unsafe fn QDial_setWrapping(dial: *mut QDial, wrapping: bool);
        unsafe fn QDial_wrapping(dial: *mut QDial) -> bool;
        unsafe fn QDial_onValueChanged(dial: *mut QDial, ctx: u64);
        unsafe fn toQWidget_QDial(dial: *mut QDial) -> *mut QWidget;

        // ============================================================
        // QDoubleSpinBox
        // ============================================================

        unsafe fn QDoubleSpinBox_new(parent: *mut QWidget) -> *mut QDoubleSpinBox;
        unsafe fn QDoubleSpinBox_delete(sb: *mut QDoubleSpinBox);
        unsafe fn QDoubleSpinBox_setValue(sb: *mut QDoubleSpinBox, value: f64);
        unsafe fn QDoubleSpinBox_value(sb: *mut QDoubleSpinBox) -> f64;
        unsafe fn QDoubleSpinBox_setRange(sb: *mut QDoubleSpinBox, min: f64, max: f64);
        unsafe fn QDoubleSpinBox_setSingleStep(sb: *mut QDoubleSpinBox, step: f64);
        unsafe fn QDoubleSpinBox_singleStep(sb: *mut QDoubleSpinBox) -> f64;
        unsafe fn QDoubleSpinBox_setDecimals(sb: *mut QDoubleSpinBox, decimals: i32);
        unsafe fn QDoubleSpinBox_decimals(sb: *mut QDoubleSpinBox) -> i32;
        unsafe fn QDoubleSpinBox_setPrefix(sb: *mut QDoubleSpinBox, prefix: &CxxString);
        unsafe fn QDoubleSpinBox_setSuffix(sb: *mut QDoubleSpinBox, suffix: &CxxString);
        unsafe fn QDoubleSpinBox_setMinimum(sb: *mut QDoubleSpinBox, min: f64);
        unsafe fn QDoubleSpinBox_setMaximum(sb: *mut QDoubleSpinBox, max: f64);
        unsafe fn QDoubleSpinBox_minimum(sb: *mut QDoubleSpinBox) -> f64;
        unsafe fn QDoubleSpinBox_maximum(sb: *mut QDoubleSpinBox) -> f64;
        unsafe fn QDoubleSpinBox_setReadOnly(sb: *mut QDoubleSpinBox, read_only: bool);
        unsafe fn QDoubleSpinBox_setGroupSeparatorShown(sb: *mut QDoubleSpinBox, shown: bool);
        unsafe fn QDoubleSpinBox_onValueChanged(sb: *mut QDoubleSpinBox, ctx: u64);
        unsafe fn toQWidget_QDoubleSpinBox(sb: *mut QDoubleSpinBox) -> *mut QWidget;

        // ============================================================
        // QLCDNumber
        // ============================================================

        unsafe fn QLCDNumber_new(parent: *mut QWidget) -> *mut QLCDNumber;
        unsafe fn QLCDNumber_delete(lcd: *mut QLCDNumber);
        unsafe fn QLCDNumber_displayInt(lcd: *mut QLCDNumber, value: i32);
        unsafe fn QLCDNumber_displayStr(lcd: *mut QLCDNumber, text: &CxxString);
        unsafe fn QLCDNumber_setDigitCount(lcd: *mut QLCDNumber, n: i32);
        unsafe fn QLCDNumber_digitCount(lcd: *mut QLCDNumber) -> i32;
        unsafe fn QLCDNumber_setMode(lcd: *mut QLCDNumber, mode: i32);
        unsafe fn QLCDNumber_setSegmentStyle(lcd: *mut QLCDNumber, style: i32);
        unsafe fn QLCDNumber_setSmallDecimalPoint(lcd: *mut QLCDNumber, enabled: bool);
        unsafe fn QLCDNumber_checkOverflow(lcd: *mut QLCDNumber, value: i32) -> bool;
        unsafe fn QLCDNumber_onOverflow(lcd: *mut QLCDNumber, ctx: u64);
        unsafe fn toQWidget_QLCDNumber(lcd: *mut QLCDNumber) -> *mut QWidget;

        // ============================================================
        // QScrollBar
        // ============================================================

        unsafe fn QScrollBar_new(orientation: i32, parent: *mut QWidget) -> *mut QScrollBar;
        unsafe fn QScrollBar_delete(sb: *mut QScrollBar);
        unsafe fn QScrollBar_value(sb: *mut QScrollBar) -> i32;
        unsafe fn QScrollBar_setValue(sb: *mut QScrollBar, value: i32);
        unsafe fn QScrollBar_setRange(sb: *mut QScrollBar, min: i32, max: i32);
        unsafe fn QScrollBar_setSingleStep(sb: *mut QScrollBar, step: i32);
        unsafe fn QScrollBar_setPageStep(sb: *mut QScrollBar, step: i32);
        unsafe fn QScrollBar_minimum(sb: *mut QScrollBar) -> i32;
        unsafe fn QScrollBar_maximum(sb: *mut QScrollBar) -> i32;
        unsafe fn QScrollBar_setMinimum(sb: *mut QScrollBar, min: i32);
        unsafe fn QScrollBar_setMaximum(sb: *mut QScrollBar, max: i32);
        unsafe fn QScrollBar_setOrientation(sb: *mut QScrollBar, orientation: i32);
        unsafe fn QScrollBar_setInvertedAppearance(sb: *mut QScrollBar, inverted: bool);
        unsafe fn QScrollBar_setInvertedControls(sb: *mut QScrollBar, inverted: bool);
        unsafe fn QScrollBar_setSliderPosition(sb: *mut QScrollBar, pos: i32);
        unsafe fn QScrollBar_sliderPosition(sb: *mut QScrollBar) -> i32;
        unsafe fn QScrollBar_onValueChanged(sb: *mut QScrollBar, ctx: u64);
        unsafe fn toQWidget_QScrollBar(sb: *mut QScrollBar) -> *mut QWidget;

        // ============================================================
        // QDialogButtonBox
        // ============================================================

        unsafe fn QDialogButtonBox_new(parent: *mut QWidget) -> *mut QDialogButtonBox;
        unsafe fn QDialogButtonBox_setStandardButtons(
            button_box: *mut QDialogButtonBox, buttons: i32,
        );
        unsafe fn QDialogButtonBox_button(
            box_: *mut QDialogButtonBox, button: i32,
        ) -> *mut QPushButton;
        unsafe fn QDialogButtonBox_delete(box_: *mut QDialogButtonBox);

        // ============================================================
        // QDockWidget
        // ============================================================

        unsafe fn QDockWidget_new(parent: *mut QWidget) -> *mut QDockWidget;
        unsafe fn QDockWidget_delete(dw: *mut QDockWidget);
        unsafe fn QDockWidget_setWindowTitle(dw: *mut QDockWidget, title: &CxxString);
        unsafe fn QDockWidget_setWidget(dw: *mut QDockWidget, widget: *mut QWidget);
        unsafe fn QDockWidget_widget(dw: *mut QDockWidget) -> *mut QWidget;
        unsafe fn QDockWidget_setFeatures(dw: *mut QDockWidget, features: i32);
        unsafe fn QDockWidget_features(dw: *mut QDockWidget) -> i32;
        unsafe fn QDockWidget_setAllowedAreas(dw: *mut QDockWidget, areas: i32);
        unsafe fn QDockWidget_allowedAreas(dw: *mut QDockWidget) -> i32;
        unsafe fn QDockWidget_setFloating(dw: *mut QDockWidget, floating: bool);
        unsafe fn QDockWidget_isFloating(dw: *mut QDockWidget) -> bool;
        unsafe fn QDockWidget_setVisible(dw: *mut QDockWidget, visible: bool);
        unsafe fn QDockWidget_show(dw: *mut QDockWidget);
        unsafe fn QDockWidget_hide(dw: *mut QDockWidget);
        unsafe fn QDockWidget_onVisibilityChanged(dw: *mut QDockWidget, ctx: u64);
        unsafe fn QDockWidget_onFeaturesChanged(dw: *mut QDockWidget, ctx: u64);
        unsafe fn toQWidget_QDockWidget(dw: *mut QDockWidget) -> *mut QWidget;

        // ============================================================
        // QToolBox
        // ============================================================

        unsafe fn QToolBox_new(parent: *mut QWidget) -> *mut QToolBox;
        unsafe fn QToolBox_delete(tb: *mut QToolBox);
        unsafe fn QToolBox_addItem(tb: *mut QToolBox, widget: *mut QWidget, text: &CxxString);
        unsafe fn QToolBox_insertItem(
            tb: *mut QToolBox, index: i32, widget: *mut QWidget, text: &CxxString,
        );
        unsafe fn QToolBox_removeItem(tb: *mut QToolBox, index: i32);
        unsafe fn QToolBox_setItemText(tb: *mut QToolBox, index: i32, text: &CxxString);
        unsafe fn QToolBox_itemText(tb: *mut QToolBox, index: i32) -> String;
        unsafe fn QToolBox_setItemIcon(tb: *mut QToolBox, index: i32, icon_path: &CxxString);
        unsafe fn QToolBox_setItemEnabled(tb: *mut QToolBox, index: i32, enabled: bool);
        unsafe fn QToolBox_isItemEnabled(tb: *mut QToolBox, index: i32) -> bool;
        unsafe fn QToolBox_currentIndex(tb: *mut QToolBox) -> i32;
        unsafe fn QToolBox_setCurrentIndex(tb: *mut QToolBox, index: i32);
        unsafe fn QToolBox_count(tb: *mut QToolBox) -> i32;
        unsafe fn QToolBox_widget(tb: *mut QToolBox, index: i32) -> *mut QWidget;
        unsafe fn QToolBox_onCurrentChanged(tb: *mut QToolBox, ctx: u64);
        unsafe fn toQWidget_QToolBox(tb: *mut QToolBox) -> *mut QWidget;

        // ============================================================
        // QFontComboBox
        // ============================================================
        unsafe fn QFontComboBox_new(parent: *mut QWidget) -> *mut QFontComboBox;
        unsafe fn QFontComboBox_delete(cb: *mut QFontComboBox);
        unsafe fn QFontComboBox_setCurrentFont(cb: *mut QFontComboBox, family: &CxxString);
        unsafe fn QFontComboBox_currentFont(cb: *mut QFontComboBox) -> String;
        unsafe fn QFontComboBox_setFontFilters(cb: *mut QFontComboBox, filters: i32);
        unsafe fn QFontComboBox_onCurrentFontChanged(cb: *mut QFontComboBox, ctx: u64);
        unsafe fn toQWidget_QFontComboBox(cb: *mut QFontComboBox) -> *mut QWidget;

        // ============================================================
        // QButtonGroup
        // ============================================================
        unsafe fn QButtonGroup_new(parent: *mut QObject) -> *mut QButtonGroup;
        unsafe fn QButtonGroup_delete(bg: *mut QButtonGroup);
        unsafe fn QButtonGroup_addButton(bg: *mut QButtonGroup, btn: *mut QAbstractButton, id: i32);
        unsafe fn QButtonGroup_setExclusive(bg: *mut QButtonGroup, exclusive: bool);
        unsafe fn QButtonGroup_onButtonClicked(bg: *mut QButtonGroup, ctx: u64);

        // ============================================================
        // QKeySequenceEdit
        // ============================================================
        unsafe fn QKeySequenceEdit_new(parent: *mut QWidget) -> *mut QKeySequenceEdit;
        unsafe fn QKeySequenceEdit_delete(e: *mut QKeySequenceEdit);
        unsafe fn QKeySequenceEdit_clear(e: *mut QKeySequenceEdit);
        unsafe fn QKeySequenceEdit_onEditingFinished(e: *mut QKeySequenceEdit, ctx: u64);
        unsafe fn toQWidget_QKeySequenceEdit(e: *mut QKeySequenceEdit) -> *mut QWidget;

        // ============================================================
        // QStringListModel
        // ============================================================
        unsafe fn QStringListModel_new(parent: *mut QObject) -> *mut QStringListModel;
        unsafe fn QStringListModel_delete(m: *mut QStringListModel);
        unsafe fn QStringListModel_setStringList(m: *mut QStringListModel, list: Vec<String>);
        unsafe fn QStringListModel_data(m: *mut QStringListModel, row: i32) -> String;
        unsafe fn QStringListModel_rowCount(m: *mut QStringListModel) -> i32;

        // ============================================================
        // QSortFilterProxyModel
        // ============================================================
        unsafe fn QSortFilterProxyModel_new(parent: *mut QObject) -> *mut QSortFilterProxyModel;
        unsafe fn QSortFilterProxyModel_delete(m: *mut QSortFilterProxyModel);
        unsafe fn QSortFilterProxyModel_setSourceModel(
            m: *mut QSortFilterProxyModel, src: *mut QStandardItemModel,
        );
        unsafe fn QSortFilterProxyModel_setFilterRole(m: *mut QSortFilterProxyModel, role: i32);
        unsafe fn QSortFilterProxyModel_setFilterFixedString(
            m: *mut QSortFilterProxyModel, text: &CxxString,
        );
        unsafe fn QSortFilterProxyModel_setFilterCaseSensitivity(
            m: *mut QSortFilterProxyModel, cs: i32,
        );
        unsafe fn QSortFilterProxyModel_setSortRole(m: *mut QSortFilterProxyModel, role: i32);
        unsafe fn QSortFilterProxyModel_sort(m: *mut QSortFilterProxyModel, col: i32, order: i32);

        // ============================================================
        // QCompleter
        // ============================================================
        unsafe fn QCompleter_new(
            model: *mut QStringListModel, parent: *mut QObject,
        ) -> *mut QCompleter;
        unsafe fn QCompleter_delete(c: *mut QCompleter);
        unsafe fn QCompleter_setCompletionMode(c: *mut QCompleter, mode: i32);
        unsafe fn QCompleter_setCaseSensitivity(c: *mut QCompleter, cs: i32);
        unsafe fn QCompleter_setFilterMode(c: *mut QCompleter, mode: i32);
        unsafe fn QCompleter_onActivated(c: *mut QCompleter, ctx: u64);

        // ============================================================
        // QClipboard (static functions)
        // ============================================================
        unsafe fn QClipboard_setText(text: &CxxString);
        unsafe fn QClipboard_text() -> String;
        unsafe fn QClipboard_clear();

        // ============================================================
        // QDesktopServices (static function)
        // ============================================================
        unsafe fn QDesktopServices_openUrl(url: &CxxString) -> bool;

        // ============================================================
        // QUndoStack
        // ============================================================
        unsafe fn QUndoStack_new(parent: *mut QObject) -> *mut QUndoStack;
        unsafe fn QUndoStack_delete(s: *mut QUndoStack);
        unsafe fn QUndoStack_undo(s: *mut QUndoStack);
        unsafe fn QUndoStack_redo(s: *mut QUndoStack);
        unsafe fn QUndoStack_clear(s: *mut QUndoStack);
        unsafe fn QUndoStack_canUndo(s: *mut QUndoStack) -> bool;
        unsafe fn QUndoStack_canRedo(s: *mut QUndoStack) -> bool;
        unsafe fn QUndoStack_count(s: *mut QUndoStack) -> i32;

        // ============================================================
        // QIntValidator / QDoubleValidator
        // ============================================================
        unsafe fn QIntValidator_new(parent: *mut QObject) -> *mut QIntValidator;
        unsafe fn QIntValidator_delete(v: *mut QIntValidator);
        unsafe fn QIntValidator_setRange(v: *mut QIntValidator, min: i32, max: i32);
        unsafe fn QIntValidator_setBottom(v: *mut QIntValidator, bottom: i32);
        unsafe fn QIntValidator_setTop(v: *mut QIntValidator, top: i32);
        unsafe fn QDoubleValidator_new(parent: *mut QObject) -> *mut QDoubleValidator;
        unsafe fn QDoubleValidator_delete(v: *mut QDoubleValidator);
        unsafe fn QDoubleValidator_setRange(
            v: *mut QDoubleValidator, min: f64, max: f64, decimals: i32,
        );

        // ============================================================
        // Cursor (on QWidget)
        // ============================================================
        unsafe fn QWidget_setCursor(widget: *mut QWidget, shape: i32);
        unsafe fn QWidget_unsetCursor(widget: *mut QWidget);

        // ============================================================
        // QFileSystemModel
        // ============================================================
        unsafe fn QFileSystemModel_new(parent: *mut QObject) -> *mut QFileSystemModel;
        unsafe fn QFileSystemModel_delete(m: *mut QFileSystemModel);
        unsafe fn QFileSystemModel_setRootPath(m: *mut QFileSystemModel, path: &CxxString);
        unsafe fn QFileSystemModel_rootPath(m: *mut QFileSystemModel) -> String;
        unsafe fn QFileSystemModel_filePath(m: *mut QFileSystemModel, idx_row: i32, idx_col: i32) -> String;
        unsafe fn QFileSystemModel_isDir(m: *mut QFileSystemModel, idx_row: i32, idx_col: i32) -> bool;

        // ============================================================
        // QHeaderView
        // ============================================================
        unsafe fn QHeaderView_new(orientation: i32, parent: *mut QWidget) -> *mut QHeaderView;
        unsafe fn QHeaderView_delete(h: *mut QHeaderView);
        unsafe fn QHeaderView_setStretchLastSection(h: *mut QHeaderView, stretch: bool);
        unsafe fn QHeaderView_resizeSection(h: *mut QHeaderView, section: i32, size: i32);
        unsafe fn QHeaderView_hideSection(h: *mut QHeaderView, section: i32);
        unsafe fn QHeaderView_showSection(h: *mut QHeaderView, section: i32);
        unsafe fn QHeaderView_setSectionResizeMode(h: *mut QHeaderView, mode: i32);
        unsafe fn toQWidget_QHeaderView(h: *mut QHeaderView) -> *mut QWidget;

        // ============================================================
        // QWizard + QWizardPage
        // ============================================================
        unsafe fn QWizard_new(parent: *mut QWidget) -> *mut QWizard;
        unsafe fn QWizard_delete(w: *mut QWizard);
        unsafe fn QWizard_addPage(w: *mut QWizard, page: *mut QWizardPage);
        unsafe fn QWizard_setWindowTitle(w: *mut QWizard, title: &CxxString);
        unsafe fn QWizard_next(w: *mut QWizard);
        unsafe fn QWizard_back(w: *mut QWizard);
        unsafe fn QWizard_restart(w: *mut QWizard);
        unsafe fn QWizard_currentId(w: *mut QWizard) -> i32;
        unsafe fn toQWidget_QWizard(w: *mut QWizard) -> *mut QWidget;
        unsafe fn QWizardPage_new(parent: *mut QWidget) -> *mut QWizardPage;
        unsafe fn QWizardPage_delete(p: *mut QWizardPage);
        unsafe fn QWizardPage_setTitle(p: *mut QWizardPage, title: &CxxString);
        unsafe fn QWizardPage_setSubTitle(p: *mut QWizardPage, subtitle: &CxxString);
        unsafe fn toQWidget_QWizardPage(p: *mut QWizardPage) -> *mut QWidget;

        // ============================================================
        // QColumnView
        // ============================================================
        unsafe fn QColumnView_new(parent: *mut QWidget) -> *mut QColumnView;
        unsafe fn QColumnView_delete(v: *mut QColumnView);
        unsafe fn QColumnView_setModel(v: *mut QColumnView, model: *mut QStandardItemModel);
        unsafe fn toQWidget_QColumnView(v: *mut QColumnView) -> *mut QWidget;

        // ============================================================
        // QStandardItemModel
        // ============================================================

        unsafe fn QStandardItemModel_new(parent: *mut QObject) -> *mut QStandardItemModel;
        unsafe fn QStandardItemModel_delete(model: *mut QStandardItemModel);
        unsafe fn QStandardItemModel_rowCount(model: *mut QStandardItemModel) -> i32;
        unsafe fn QStandardItemModel_columnCount(model: *mut QStandardItemModel) -> i32;
        unsafe fn QStandardItemModel_setRowCount(model: *mut QStandardItemModel, rows: i32);
        unsafe fn QStandardItemModel_setColumnCount(model: *mut QStandardItemModel, cols: i32);
        unsafe fn QStandardItemModel_setData(
            model: *mut QStandardItemModel, row: i32, col: i32, value: &CxxString,
        );
        unsafe fn QStandardItemModel_data(
            model: *mut QStandardItemModel, row: i32, col: i32,
        ) -> String;
        unsafe fn QStandardItemModel_setHeaderData(
            model: *mut QStandardItemModel, section: i32, orientation: i32,
            value: &CxxString,
        );
        unsafe fn QStandardItemModel_headerData(
            model: *mut QStandardItemModel, section: i32, orientation: i32,
        ) -> String;
        unsafe fn QStandardItemModel_insertRow(model: *mut QStandardItemModel, row: i32);
        unsafe fn QStandardItemModel_removeRow(model: *mut QStandardItemModel, row: i32);
        unsafe fn QStandardItemModel_insertColumn(model: *mut QStandardItemModel, column: i32);
        unsafe fn QStandardItemModel_removeColumn(model: *mut QStandardItemModel, column: i32);
        unsafe fn QStandardItemModel_clear(model: *mut QStandardItemModel);
        unsafe fn QStandardItemModel_appendRow(
            model: *mut QStandardItemModel, texts: Vec<String>,
        );
        unsafe fn QStandardItemModel_onModelReset(model: *mut QStandardItemModel, ctx: u64);
        unsafe fn QStandardItemModel_onDataChanged(model: *mut QStandardItemModel, ctx: u64);
        unsafe fn QStandardItemModel_onRowsInserted(model: *mut QStandardItemModel, ctx: u64);
        unsafe fn QStandardItemModel_onRowsRemoved(model: *mut QStandardItemModel, ctx: u64);

        // ============================================================
        // QTableView
        // ============================================================

        unsafe fn QTableView_new(parent: *mut QWidget) -> *mut QTableView;
        unsafe fn QTableView_delete(view: *mut QTableView);
        unsafe fn QTableView_setModel(view: *mut QTableView, model: *mut QStandardItemModel);
        unsafe fn QTableView_model(view: *mut QTableView) -> *mut QStandardItemModel;
        unsafe fn QTableView_setSelectionMode(view: *mut QTableView, mode: i32);
        unsafe fn QTableView_setSelectionBehavior(view: *mut QTableView, behavior: i32);
        unsafe fn QTableView_setShowGrid(view: *mut QTableView, show: bool);
        unsafe fn QTableView_setAlternatingRowColors(view: *mut QTableView, enable: bool);
        unsafe fn QTableView_setSortingEnabled(view: *mut QTableView, enable: bool);
        unsafe fn QTableView_resizeColumnsToContents(view: *mut QTableView);
        unsafe fn QTableView_resizeRowsToContents(view: *mut QTableView);
        unsafe fn QTableView_selectRow(view: *mut QTableView, row: i32);
        unsafe fn QTableView_clearSelection(view: *mut QTableView);
        unsafe fn QTableView_onClicked(view: *mut QTableView, ctx: u64);
        unsafe fn QTableView_onDoubleClicked(view: *mut QTableView, ctx: u64);
        unsafe fn toQWidget_QTableView(view: *mut QTableView) -> *mut QWidget;

        // ============================================================
        // QListView
        // ============================================================

        unsafe fn QListView_new(parent: *mut QWidget) -> *mut QListView;
        unsafe fn QListView_delete(view: *mut QListView);
        unsafe fn QListView_setModel(view: *mut QListView, model: *mut QStandardItemModel);
        unsafe fn QListView_model(view: *mut QListView) -> *mut QStandardItemModel;
        unsafe fn QListView_setSelectionMode(view: *mut QListView, mode: i32);
        unsafe fn QListView_setViewMode(view: *mut QListView, mode: i32);
        unsafe fn QListView_onClicked(view: *mut QListView, ctx: u64);
        unsafe fn QListView_onDoubleClicked(view: *mut QListView, ctx: u64);
        unsafe fn toQWidget_QListView(view: *mut QListView) -> *mut QWidget;

        // ============================================================
        // QTreeView
        // ============================================================

        unsafe fn QTreeView_new(parent: *mut QWidget) -> *mut QTreeView;
        unsafe fn QTreeView_delete(view: *mut QTreeView);
        unsafe fn QTreeView_setModel(view: *mut QTreeView, model: *mut QStandardItemModel);
        unsafe fn QTreeView_model(view: *mut QTreeView) -> *mut QStandardItemModel;
        unsafe fn QTreeView_setSelectionMode(view: *mut QTreeView, mode: i32);
        unsafe fn QTreeView_setHeaderHidden(view: *mut QTreeView, hidden: bool);
        unsafe fn QTreeView_setAnimated(view: *mut QTreeView, animated: bool);
        unsafe fn QTreeView_setIndentation(view: *mut QTreeView, indent: i32);
        unsafe fn QTreeView_setRootIsDecorated(view: *mut QTreeView, decorated: bool);
        unsafe fn QTreeView_setItemsExpandable(view: *mut QTreeView, expandable: bool);
        unsafe fn QTreeView_expandAll(view: *mut QTreeView);
        unsafe fn QTreeView_collapseAll(view: *mut QTreeView);
        unsafe fn QTreeView_onClicked(view: *mut QTreeView, ctx: u64);
        unsafe fn QTreeView_onDoubleClicked(view: *mut QTreeView, ctx: u64);
        unsafe fn QTreeView_onExpanded(view: *mut QTreeView, ctx: u64);
        unsafe fn QTreeView_onCollapsed(view: *mut QTreeView, ctx: u64);
        unsafe fn toQWidget_QTreeView(view: *mut QTreeView) -> *mut QWidget;

        // ============================================================
        // QItemSelectionModel
        // ============================================================

        unsafe fn QItemSelectionModel_hasSelection(sm: *mut QItemSelectionModel) -> bool;
        unsafe fn QItemSelectionModel_onSelectionChanged(sm: *mut QItemSelectionModel, ctx: u64);
        unsafe fn QItemSelectionModel_onCurrentChanged(sm: *mut QItemSelectionModel, ctx: u64);
        unsafe fn QUiLoader_new() -> *mut QUiLoader;
        unsafe fn QUiLoader_load(
            loader: *mut QUiLoader,
            ui_path: &CxxString,
            parent: *mut QWidget,
        ) -> *mut QWidget;
        unsafe fn QUiLoader_delete(loader: *mut QUiLoader);

        // ============================================================
        // QDialog
        // ============================================================
        unsafe fn QDialog_new(parent: *mut QWidget) -> *mut QDialog;
        unsafe fn QDialog_setModal(dialog: *mut QDialog, modal: bool);
        unsafe fn QDialog_setWindowTitle(dialog: *mut QDialog, title: &CxxString);
        unsafe fn QDialog_setMinimumSize(dialog: *mut QDialog, w: i32, h: i32);
        unsafe fn QDialog_resize(dialog: *mut QDialog, w: i32, h: i32);
        unsafe fn QDialog_show(dialog: *mut QDialog);
        unsafe fn QDialog_exec(dialog: *mut QDialog);
        unsafe fn QDialog_accept(dialog: *mut QDialog);
        unsafe fn QDialog_reject(dialog: *mut QDialog);
        unsafe fn QDialog_setLayout(dialog: *mut QDialog, layout: *mut QLayout);
        unsafe fn QDialog_delete(dialog: *mut QDialog);
    }
