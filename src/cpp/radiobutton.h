// src/cpp/radiobutton.h — QRadioButton
#pragma once

#include <QtCore/QObject>
#include <QtWidgets/QRadioButton>
#include <QtCore/QString>
#include <string>

#include "signal.h"

/// Create a new QRadioButton with text and optional parent.
inline QRadioButton *QRadioButton_new(const std::string &text, QWidget *parent) {
    return new QRadioButton(QString::fromStdString(text), parent);
}

/// Check if the radio button is selected.
inline bool QRadioButton_isChecked(QRadioButton *rb) {
    return rb->isChecked();
}

/// Set the checked state.
inline void QRadioButton_setChecked(QRadioButton *rb, bool checked) {
    rb->setChecked(checked);
}

/// Delete the radio button.
inline void QRadioButton_delete(QRadioButton *rb) {
    delete rb;
}

/// Connect the toggled signal to a Rust callback.
inline void QRadioButton_onToggled(QRadioButton *rb, uint64_t ctx) {
    QObject::connect(rb, &QRadioButton::toggled,
                     [ctx](bool checked) {
                         if (g_hasBoolTrampoline) g_boolTrampoline(ctx, checked);
                     });
}
