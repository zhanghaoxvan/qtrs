// src/cpp/widget.h — QWidget and common properties
#pragma once

#include "qwidget.h"
#include <QtGui/QIcon>
#include <QtCore/QString>
#include <QtWidgets/QWidget>
#include <string>

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
inline void QWidget_disconnectAll(QWidget *w) {
    QObject::disconnect(w, nullptr, nullptr, nullptr);
}
inline void QWidget_move(QWidget *w, int x, int y) {
    w->move(x, y);
}
inline void QWidget_moveToPoint(QWidget *w, QPoint *p) {
    w->move(*p);
}