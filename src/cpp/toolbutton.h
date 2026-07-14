// src/cpp/toolbutton.h — QToolButton
#pragma once

#include <QtGui/QIcon>
#include <QtGui/QKeySequence>
#include <QtWidgets/QToolButton>
#include <QtWidgets/QWidget>
#include <QtCore/QObject>
#include <QtCore/QString>
#include <string>

#include "signal.h"

// Constructor / Destructor
inline QToolButton *QToolButton_new(QWidget *parent) {
    return new QToolButton(parent);
}

inline void QToolButton_delete(QToolButton *btn) { delete btn; }

// Properties
inline void QToolButton_setText(QToolButton *btn, const std::string &text) {
    btn->setText(QString::fromStdString(text));
}

inline void QToolButton_setIcon(QToolButton *btn, const std::string &icon_path) {
    QIcon icon(QString::fromStdString(icon_path));
    if (!icon.isNull()) btn->setIcon(icon);
}

inline void QToolButton_setToolButtonStyle(QToolButton *btn, int style) {
    btn->setToolButtonStyle(static_cast<Qt::ToolButtonStyle>(style));
}

inline void QToolButton_setPopupMode(QToolButton *btn, int mode) {
    btn->setPopupMode(static_cast<QToolButton::ToolButtonPopupMode>(mode));
}

inline void QToolButton_setAutoRaise(QToolButton *btn, bool enabled) {
    btn->setAutoRaise(enabled);
}

inline void QToolButton_setCheckable(QToolButton *btn, bool checkable) {
    btn->setCheckable(checkable);
}

inline void QToolButton_setChecked(QToolButton *btn, bool checked) {
    btn->setChecked(checked);
}

inline void QToolButton_setShortcut(QToolButton *btn, const std::string &key) {
    btn->setShortcut(QKeySequence(QString::fromStdString(key)));
}

// Signals
inline void QToolButton_onClicked(QToolButton *btn, uint64_t ctx) {
    QObject::connect(btn, &QToolButton::clicked, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}

inline void QToolButton_onToggled(QToolButton *btn, uint64_t ctx) {
    QObject::connect(btn, &QToolButton::toggled, [ctx](bool checked) {
        if (g_hasBoolTrampoline) g_boolTrampoline(ctx, checked);
    });
}

// Upcast
inline QWidget *toQWidget_QToolButton(QToolButton *btn) {
    return static_cast<QWidget *>(btn);
}
