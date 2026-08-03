// src/cpp/widget.h — QWidget and common properties
#pragma once

#include "qwidget.h"
#include <QtGui/QFont>
#include <QtGui/QIcon>
#include <QtCore/QString>
#include <QtWidgets/QWidget>
#include <string>
#include "rust/cxx.h"

inline QWidget *QWidget_new(QWidget *parent) { return new QWidget(parent); }
inline void QWidget_show(QWidget *widget) { widget->show(); }
inline void QWidget_hide(QWidget *widget) { widget->hide(); }
inline void QWidget_delete(QWidget *widget) { delete widget; }

inline void QWidget_setWindowTitle(QWidget *widget, const std::string &title) {
    widget->setWindowTitle(QString::fromStdString(title));
}
inline void QWidget_resize(QWidget *widget, int width, int height) {
    widget->resize(width, height);
}
inline void QWidget_setWindowIcon(QWidget *widget,
                                   const std::string &icon_path) {
    QIcon icon(QString::fromStdString(icon_path));
    if (!icon.isNull()) widget->setWindowIcon(icon);
}
inline void QWidget_setLayout(QWidget *widget, QLayout *layout) {
    widget->setLayout(layout);
}

// Common properties
inline void QWidget_setEnabled(QWidget *w, bool e) { w->setEnabled(e); }
inline void QWidget_setVisible(QWidget *w, bool v) { w->setVisible(v); }
inline void QWidget_setToolTip(QWidget *w, const std::string &tip) {
    w->setToolTip(QString::fromStdString(tip));
}
inline void QWidget_setMinimumSize(QWidget *w, int width, int height) {
    w->setMinimumSize(width, height);
}
inline void QWidget_setMaximumSize(QWidget *w, int width, int height) {
    w->setMaximumSize(width, height);
}
inline void QWidget_setFixedSize(QWidget *w, int width, int height) {
    w->setFixedSize(width, height);
}
inline void QWidget_setStyleSheet(QWidget *w, const std::string &css) {
    w->setStyleSheet(QString::fromStdString(css));
}
inline void QWidget_setFont(QWidget *w, QFont *f) {
    w->setFont(*f);
}

inline QFont *QWidget_font(QWidget *w) {
    return new QFont(w->font());
}

inline void QWidget_disconnectAll(QWidget *w) {
    QObject::disconnect(w, nullptr, nullptr, nullptr);
}
inline void QWidget_move(QWidget *w, int x, int y) {
    w->move(x, y);
}
inline void QWidget_moveToPoint(QWidget *w, QPoint *p) {
    w->move(*p);
}

// --- Size/Position getters ---
inline int QWidget_width(QWidget *w) { return w->width(); }
inline int QWidget_height(QWidget *w) { return w->height(); }
inline int QWidget_x(QWidget *w) { return w->x(); }
inline int QWidget_y(QWidget *w) { return w->y(); }

// --- Geometry ---
inline void QWidget_setGeometry(QWidget *w, int x, int y, int ww, int h) {
    w->setGeometry(x, y, ww, h);
}

// --- State queries ---
inline bool QWidget_isVisible(QWidget *w) { return w->isVisible(); }
inline bool QWidget_isEnabled(QWidget *w) { return w->isEnabled(); }
inline bool QWidget_isHidden(QWidget *w) { return w->isHidden(); }

// --- Window title getter ---
inline rust::String QWidget_windowTitle(QWidget *w) {
    return rust::String(w->windowTitle().toStdString());
}

// --- Focus ---
inline void QWidget_setFocus(QWidget *w) { w->setFocus(); }
inline bool QWidget_hasFocus(QWidget *w) { return w->hasFocus(); }
inline void QWidget_clearFocus(QWidget *w) { w->clearFocus(); }

// --- Object name ---
inline void QWidget_setObjectName(QWidget *w, const std::string &name) {
    w->setObjectName(QString::fromStdString(name));
}
inline rust::String QWidget_objectName(QWidget *w) {
    return rust::String(w->objectName().toStdString());
}

// --- Repaint ---
inline void QWidget_update(QWidget *w) { w->update(); }
inline void QWidget_repaint(QWidget *w) { w->repaint(); }

// --- Close ---
inline void QWidget_close(QWidget *w) { w->close(); }

// --- Parent ---
inline QWidget *QWidget_parentWidget(QWidget *w) {
    return w->parentWidget();
}

// --- Min/max size getters ---
inline int QWidget_minimumWidth(QWidget *w) { return w->minimumWidth(); }
inline int QWidget_minimumHeight(QWidget *w) { return w->minimumHeight(); }
inline int QWidget_maximumWidth(QWidget *w) { return w->maximumWidth(); }
inline int QWidget_maximumHeight(QWidget *w) { return w->maximumHeight(); }

// --- Window state ---
inline void QWidget_raiseWidget(QWidget *w) { w->raise(); }
inline void QWidget_lowerWidget(QWidget *w) { w->lower(); }
inline bool QWidget_isMinimized(QWidget *w) { return w->isMinimized(); }
inline bool QWidget_isMaximized(QWidget *w) { return w->isMaximized(); }

// --- More QWidget essentials ---
inline void QWidget_adjustSize(QWidget *w) { w->adjustSize(); }
inline bool QWidget_isActiveWindow(QWidget *w) { return w->isActiveWindow(); }
inline bool QWidget_underMouse(QWidget *w) { return w->underMouse(); }
inline bool QWidget_isWindow(QWidget *w) { return w->isWindow(); }
inline QWidget *QWidget_window(QWidget *w) { return w->window(); }
inline void QWidget_setWindowOpacity(QWidget *w, double opacity) { w->setWindowOpacity(opacity); }
inline void QWidget_setFixedWidth(QWidget *w, int width) { w->setFixedWidth(width); }
inline void QWidget_setFixedHeight(QWidget *w, int height) { w->setFixedHeight(height); }
inline void QWidget_setMouseTracking(QWidget *w, bool enable) { w->setMouseTracking(enable); }
inline bool QWidget_hasMouseTracking(QWidget *w) { return w->hasMouseTracking(); }
inline void QWidget_setAcceptDrops(QWidget *w, bool enable) { w->setAcceptDrops(enable); }
inline void QWidget_setAutoFillBackground(QWidget *w, bool enable) { w->setAutoFillBackground(enable); }
inline void QWidget_showFullScreen(QWidget *w) { w->showFullScreen(); }
inline void QWidget_showMaximized(QWidget *w) { w->showMaximized(); }
inline void QWidget_showMinimized(QWidget *w) { w->showMinimized(); }
inline void QWidget_showNormal(QWidget *w) { w->showNormal(); }
inline void QWidget_setSizePolicy(QWidget *w, int hPolicy, int vPolicy) {
    w->setSizePolicy(static_cast<QSizePolicy::Policy>(hPolicy), static_cast<QSizePolicy::Policy>(vPolicy));
}
inline bool QWidget_isVisibleTo(QWidget *w, QWidget *ancestor) { return w->isVisibleTo(ancestor); }