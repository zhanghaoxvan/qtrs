// src/cpp/spinbox.h — QSpinBox
#pragma once

#include <QObject>
#include <QSpinBox>
#include <QString>
#include <string>

#include "signal.h"

/// Create a new QSpinBox with optional parent.
inline QSpinBox *QSpinBox_new(QWidget *parent) {
    return new QSpinBox(parent);
}

/// Set the current value.
inline void QSpinBox_setValue(QSpinBox *sb, int value) {
    sb->setValue(value);
}

/// Get the current value.
inline int QSpinBox_value(QSpinBox *sb) {
    return sb->value();
}

/// Set the value range (min, max).
inline void QSpinBox_setRange(QSpinBox *sb, int min, int max) {
    sb->setRange(min, max);
}

/// Set the suffix text (e.g. " cm", " kg").
inline void QSpinBox_setSuffix(QSpinBox *sb, const std::string &suffix) {
    sb->setSuffix(QString::fromStdString(suffix));
}

/// Delete the spin box.
inline void QSpinBox_delete(QSpinBox *sb) {
    delete sb;
}

/// Connect the valueChanged signal to a Rust callback.
inline void QSpinBox_onValueChanged(QSpinBox *sb, uint64_t ctx) {
    QObject::connect(sb, QOverload<int>::of(&QSpinBox::valueChanged),
                     [ctx](int value) {
                         if (g_hasIntTrampoline) g_intTrampoline(ctx, value);
                     });
}
