// src/cpp/toolbar.h — QToolBar
#pragma once

#include <QtCore/QObject>
#include <QtCore/QString>
#include <QtGui/QAction>
#include <QtWidgets/QToolBar>
#include <QtWidgets/QWidget>
#include <string>

#include "signal.h"

// Constructor / Destructor
inline QToolBar *QToolBar_new(const std::string &title, QWidget *parent) {
    return new QToolBar(QString::fromStdString(title), parent);
}

inline void QToolBar_delete(QToolBar *toolbar) { delete toolbar; }

// Action management
inline void QToolBar_addAction(QToolBar *toolbar, const std::string &text, uint64_t ctx) {
    QAction *action = toolbar->addAction(QString::fromStdString(text));
    QObject::connect(action, &QAction::triggered, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}

inline void QToolBar_addSeparator(QToolBar *toolbar) {
    toolbar->addSeparator();
}

inline void QToolBar_addWidget(QToolBar *toolbar, QWidget *widget) {
    toolbar->addWidget(widget);
}

// Appearance
inline void QToolBar_setMovable(QToolBar *toolbar, bool movable) {
    toolbar->setMovable(movable);
}

inline void QToolBar_setFloatable(QToolBar *toolbar, bool floatable) {
    toolbar->setFloatable(floatable);
}

inline void QToolBar_setIconSize(QToolBar *toolbar, int w, int h) {
    toolbar->setIconSize(QSize(w, h));
}

inline void QToolBar_setAllowedAreas(QToolBar *toolbar, int areas) {
    toolbar->setAllowedAreas(static_cast<Qt::ToolBarAreas>(areas));
}

inline void QToolBar_setToolButtonStyle(QToolBar *toolbar, int style) {
    toolbar->setToolButtonStyle(static_cast<Qt::ToolButtonStyle>(style));
}

// Clear
inline void QToolBar_clear(QToolBar *toolbar) {
    toolbar->clear();
}

// Upcast
inline QWidget *toQWidget_QToolBar(QToolBar *toolbar) {
    return static_cast<QWidget *>(toolbar);
}
