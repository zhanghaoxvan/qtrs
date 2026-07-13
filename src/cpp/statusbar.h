// src/cpp/statusbar.h — QStatusBar
#pragma once

#include <QtCore/QString>
#include <QtWidgets/QStatusBar>
#include <QtWidgets/QWidget>
#include <string>

#include "signal.h"

// Constructor / Destructor
inline QStatusBar *QStatusBar_new(QWidget *parent) {
    return new QStatusBar(parent);
}

inline void QStatusBar_delete(QStatusBar *bar) { delete bar; }

// Message display
inline void QStatusBar_showMessage(QStatusBar *bar, const std::string &text, int timeout_ms) {
    bar->showMessage(QString::fromStdString(text), timeout_ms);
}

inline void QStatusBar_clearMessage(QStatusBar *bar) {
    bar->clearMessage();
}

// Widget management
inline void QStatusBar_addWidget(QStatusBar *bar, QWidget *widget) {
    bar->addWidget(widget);
}

inline void QStatusBar_addPermanentWidget(QStatusBar *bar, QWidget *widget) {
    bar->addPermanentWidget(widget);
}

inline void QStatusBar_removeWidget(QStatusBar *bar, QWidget *widget) {
    bar->removeWidget(widget);
}

// Size grip
inline void QStatusBar_setSizeGripEnabled(QStatusBar *bar, bool enabled) {
    bar->setSizeGripEnabled(enabled);
}

// Upcast
inline QWidget *toQWidget_QStatusBar(QStatusBar *bar) {
    return static_cast<QWidget *>(bar);
}
