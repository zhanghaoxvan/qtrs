unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
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
    }
