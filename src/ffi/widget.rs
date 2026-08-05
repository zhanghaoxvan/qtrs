unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
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

    // --- QWidget getters / state queries ---
    unsafe fn QWidget_width(widget: *mut QWidget) -> i32;
    unsafe fn QWidget_height(widget: *mut QWidget) -> i32;
    unsafe fn QWidget_x(widget: *mut QWidget) -> i32;
    unsafe fn QWidget_y(widget: *mut QWidget) -> i32;
    unsafe fn QWidget_setGeometry(widget: *mut QWidget, x: i32, y: i32, w: i32, h: i32);
    unsafe fn QWidget_isVisible(widget: *mut QWidget) -> bool;
    unsafe fn QWidget_isEnabled(widget: *mut QWidget) -> bool;
    unsafe fn QWidget_isHidden(widget: *mut QWidget) -> bool;
    unsafe fn QWidget_windowTitle(widget: *mut QWidget) -> String;
    unsafe fn QWidget_setFocus(widget: *mut QWidget);
    unsafe fn QWidget_hasFocus(widget: *mut QWidget) -> bool;
    unsafe fn QWidget_clearFocus(widget: *mut QWidget);
    unsafe fn QWidget_setObjectName(widget: *mut QWidget, name: &CxxString);
    unsafe fn QWidget_objectName(widget: *mut QWidget) -> String;
    unsafe fn QWidget_update(widget: *mut QWidget);
    unsafe fn QWidget_repaint(widget: *mut QWidget);
    unsafe fn QWidget_close(widget: *mut QWidget);
    unsafe fn QWidget_parentWidget(widget: *mut QWidget) -> *mut QWidget;
    unsafe fn QWidget_minimumWidth(widget: *mut QWidget) -> i32;
    unsafe fn QWidget_minimumHeight(widget: *mut QWidget) -> i32;
    unsafe fn QWidget_maximumWidth(widget: *mut QWidget) -> i32;
    unsafe fn QWidget_maximumHeight(widget: *mut QWidget) -> i32;
    unsafe fn QWidget_raiseWidget(widget: *mut QWidget);
    unsafe fn QWidget_lowerWidget(widget: *mut QWidget);
    unsafe fn QWidget_isMinimized(widget: *mut QWidget) -> bool;
    unsafe fn QWidget_isMaximized(widget: *mut QWidget) -> bool;
    unsafe fn QWidget_adjustSize(widget: *mut QWidget);
    unsafe fn QWidget_isActiveWindow(widget: *mut QWidget) -> bool;
    unsafe fn QWidget_underMouse(widget: *mut QWidget) -> bool;
    unsafe fn QWidget_isWindow(widget: *mut QWidget) -> bool;
    unsafe fn QWidget_window(widget: *mut QWidget) -> *mut QWidget;
    unsafe fn QWidget_setWindowOpacity(widget: *mut QWidget, opacity: f64);
    unsafe fn QWidget_setFixedWidth(widget: *mut QWidget, width: i32);
    unsafe fn QWidget_setFixedHeight(widget: *mut QWidget, height: i32);
    unsafe fn QWidget_setMouseTracking(widget: *mut QWidget, enable: bool);
    unsafe fn QWidget_hasMouseTracking(widget: *mut QWidget) -> bool;
    unsafe fn QWidget_setAcceptDrops(widget: *mut QWidget, enable: bool);
    unsafe fn QWidget_setAutoFillBackground(widget: *mut QWidget, enable: bool);
    unsafe fn QWidget_showFullScreen(widget: *mut QWidget);
    unsafe fn QWidget_showMaximized(widget: *mut QWidget);
    unsafe fn QWidget_showMinimized(widget: *mut QWidget);
    unsafe fn QWidget_showNormal(widget: *mut QWidget);
    unsafe fn QWidget_setSizePolicy(widget: *mut QWidget, hPolicy: i32, vPolicy: i32);
    unsafe fn QWidget_isVisibleTo(widget: *mut QWidget, ancestor: *mut QWidget) -> bool;
}
