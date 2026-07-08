// src/cpp/menu.h — QMenu + QMenuBar
#pragma once

#include <QAction>
#include <QMenu>
#include <QMenuBar>
#include <QObject>
#include <QString>
#include <string>

#include "signal.h"

// ============================================================
// QMenu
// ============================================================

/// Create a new QMenu with title and optional parent.
inline QMenu *QMenu_new(const std::string &title, QWidget *parent) {
    return new QMenu(QString::fromStdString(title), parent);
}

/// Add a menu action that triggers a Rust callback.
inline void QMenu_addAction(QMenu *menu, const std::string &text, uint64_t ctx) {
    QAction *action = menu->addAction(QString::fromStdString(text));
    QObject::connect(action, &QAction::triggered, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}

/// Delete the menu.
inline void QMenu_delete(QMenu *menu) {
    delete menu;
}

// ============================================================
// QMenuBar
// ============================================================

/// Create a new QMenuBar with optional parent.
inline QMenuBar *QMenuBar_new(QWidget *parent) {
    return new QMenuBar(parent);
}

/// Add a menu to the menu bar.
inline void QMenuBar_addMenu(QMenuBar *mb, QMenu *menu) {
    mb->addMenu(menu);
}

/// Delete the menu bar.
inline void QMenuBar_delete(QMenuBar *mb) {
    delete mb;
}
