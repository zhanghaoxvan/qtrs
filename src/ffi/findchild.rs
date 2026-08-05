unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
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
}
