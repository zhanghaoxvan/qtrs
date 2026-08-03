// src/cpp/scrollbar.h — QScrollBar
#pragma once

#include <QtCore/QObject>
#include <QtWidgets/QScrollBar>
#include <QtCore/QString>
#include <string>

#include "signal.h"

inline QScrollBar *QScrollBar_new(int orientation, QWidget *parent) {
    return new QScrollBar(static_cast<Qt::Orientation>(orientation), parent);
}
inline void QScrollBar_delete(QScrollBar *sb) { delete sb; }
inline int QScrollBar_value(QScrollBar *sb) { return sb->value(); }
inline void QScrollBar_setValue(QScrollBar *sb, int value) { sb->setValue(value); }
inline void QScrollBar_setRange(QScrollBar *sb, int min, int max) {
    sb->setRange(min, max);
}
inline void QScrollBar_setSingleStep(QScrollBar *sb, int step) {
    sb->setSingleStep(step);
}
inline void QScrollBar_setPageStep(QScrollBar *sb, int step) {
    sb->setPageStep(step);
}
inline int QScrollBar_minimum(QScrollBar *sb) { return sb->minimum(); }
inline int QScrollBar_maximum(QScrollBar *sb) { return sb->maximum(); }
inline void QScrollBar_setMinimum(QScrollBar *sb, int min) { sb->setMinimum(min); }
inline void QScrollBar_setMaximum(QScrollBar *sb, int max) { sb->setMaximum(max); }
inline void QScrollBar_setOrientation(QScrollBar *sb, int orientation) {
    sb->setOrientation(static_cast<Qt::Orientation>(orientation));
}
inline void QScrollBar_setInvertedAppearance(QScrollBar *sb, bool inverted) {
    sb->setInvertedAppearance(inverted);
}
inline void QScrollBar_setInvertedControls(QScrollBar *sb, bool inverted) {
    sb->setInvertedControls(inverted);
}
inline void QScrollBar_setSliderPosition(QScrollBar *sb, int pos) {
    sb->setSliderPosition(pos);
}
inline int QScrollBar_sliderPosition(QScrollBar *sb) {
    return sb->sliderPosition();
}
inline void QScrollBar_onValueChanged(QScrollBar *sb, uint64_t ctx) {
    QObject::connect(sb, &QScrollBar::valueChanged, [ctx](int value) {
        if (g_hasIntTrampoline) g_intTrampoline(ctx, value);
    });
}
inline QWidget *toQWidget_QScrollBar(QScrollBar *sb) {
    return static_cast<QWidget*>(sb);
}
