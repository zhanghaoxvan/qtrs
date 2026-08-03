// src/cpp/doublespinbox.h — QDoubleSpinBox
#pragma once

#include <QtCore/QObject>
#include <QtWidgets/QDoubleSpinBox>
#include <QtCore/QString>
#include <string>

#include "signal.h"

inline QDoubleSpinBox *QDoubleSpinBox_new(QWidget *parent) {
    return new QDoubleSpinBox(parent);
}
inline void QDoubleSpinBox_delete(QDoubleSpinBox *sb) { delete sb; }
inline void QDoubleSpinBox_setValue(QDoubleSpinBox *sb, double value) {
    sb->setValue(value);
}
inline double QDoubleSpinBox_value(QDoubleSpinBox *sb) {
    return sb->value();
}
inline void QDoubleSpinBox_setRange(QDoubleSpinBox *sb, double min, double max) {
    sb->setRange(min, max);
}
inline void QDoubleSpinBox_setSingleStep(QDoubleSpinBox *sb, double step) {
    sb->setSingleStep(step);
}
inline double QDoubleSpinBox_singleStep(QDoubleSpinBox *sb) {
    return sb->singleStep();
}
inline void QDoubleSpinBox_setDecimals(QDoubleSpinBox *sb, int decimals) {
    sb->setDecimals(decimals);
}
inline int QDoubleSpinBox_decimals(QDoubleSpinBox *sb) {
    return sb->decimals();
}
inline void QDoubleSpinBox_setPrefix(QDoubleSpinBox *sb, const std::string &prefix) {
    sb->setPrefix(QString::fromStdString(prefix));
}
inline void QDoubleSpinBox_setSuffix(QDoubleSpinBox *sb, const std::string &suffix) {
    sb->setSuffix(QString::fromStdString(suffix));
}
inline void QDoubleSpinBox_setMinimum(QDoubleSpinBox *sb, double min) {
    sb->setMinimum(min);
}
inline void QDoubleSpinBox_setMaximum(QDoubleSpinBox *sb, double max) {
    sb->setMaximum(max);
}
inline double QDoubleSpinBox_minimum(QDoubleSpinBox *sb) {
    return sb->minimum();
}
inline double QDoubleSpinBox_maximum(QDoubleSpinBox *sb) {
    return sb->maximum();
}
inline void QDoubleSpinBox_setReadOnly(QDoubleSpinBox *sb, bool readOnly) {
    sb->setReadOnly(readOnly);
}
inline void QDoubleSpinBox_setGroupSeparatorShown(QDoubleSpinBox *sb, bool shown) {
    sb->setGroupSeparatorShown(shown);
}
inline void QDoubleSpinBox_onValueChanged(QDoubleSpinBox *sb, uint64_t ctx) {
    QObject::connect(sb, QOverload<double>::of(&QDoubleSpinBox::valueChanged),
                     [ctx](double value) {
                         if (g_hasStringTrampoline)
                             g_stringTrampoline(ctx, rust::String(QString::number(value, 'f').toStdString()));
                     });
}
inline QWidget *toQWidget_QDoubleSpinBox(QDoubleSpinBox *sb) {
    return static_cast<QWidget*>(sb);
}
