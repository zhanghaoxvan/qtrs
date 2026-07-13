// src/cpp/action.h — QAction
#pragma once

#include <QtGui/QAction>
#include <QtGui/QIcon>
#include <QtGui/QKeySequence>
#include <QtCore/QObject>
#include <QtCore/QString>
#include <string>

#include "rust/cxx.h"
#include "signal.h"

// Constructor / Destructor
inline QAction *QAction_new(const std::string &text, QWidget *parent) {
    return new QAction(QString::fromStdString(text), parent);
}

inline void QAction_delete(QAction *action) { delete action; }

// Properties
inline void QAction_setText(QAction *action, const std::string &text) {
    action->setText(QString::fromStdString(text));
}

inline void QAction_setIcon(QAction *action, const std::string &icon_path) {
    QIcon icon(QString::fromStdString(icon_path));
    if (!icon.isNull()) action->setIcon(icon);
}

inline void QAction_setCheckable(QAction *action, bool checkable) {
    action->setCheckable(checkable);
}

inline void QAction_setChecked(QAction *action, bool checked) {
    action->setChecked(checked);
}

inline void QAction_setShortcut(QAction *action, const std::string &key) {
    action->setShortcut(QKeySequence(QString::fromStdString(key)));
}

inline void QAction_setEnabled(QAction *action, bool enabled) {
    action->setEnabled(enabled);
}

inline void QAction_setToolTip(QAction *action, const std::string &tip) {
    action->setToolTip(QString::fromStdString(tip));
}

// Getters
inline rust::String QAction_text(QAction *action) {
    return rust::String(action->text().toStdString());
}

inline bool QAction_isChecked(QAction *action) {
    return action->isChecked();
}

inline bool QAction_isEnabled(QAction *action) {
    return action->isEnabled();
}

// Signals
inline void QAction_onTriggered(QAction *action, uint64_t ctx) {
    QObject::connect(action, QOverload<bool>::of(&QAction::triggered),
                     [ctx](bool checked) {
                         if (g_hasBoolTrampoline) g_boolTrampoline(ctx, checked);
                     });
}

inline void QAction_onToggled(QAction *action, uint64_t ctx) {
    QObject::connect(action, &QAction::toggled,
                     [ctx](bool checked) {
                         if (g_hasBoolTrampoline) g_boolTrampoline(ctx, checked);
                     });
}
