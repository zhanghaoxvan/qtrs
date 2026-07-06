// src/cpp/slider.h — QSlider
#pragma once

#include <QObject>
#include <QSlider>
#include <Qt>

#include "signal.h"

inline QSlider *QSlider_new(int orientation, QWidget *parent) {
    auto *s = new QSlider(static_cast<Qt::Orientation>(orientation), parent);
    s->setRange(0, 100);
    return s;
}
inline int QSlider_value(QSlider *s) { return s->value(); }
inline void QSlider_setValue(QSlider *s, int v) { s->setValue(v); }
inline void QSlider_setRange(QSlider *s, int min, int max) {
    s->setRange(min, max);
}
inline void QSlider_delete(QSlider *s) { delete s; }

inline void QSlider_onValueChanged(QSlider *s, uint64_t ctx) {
    QObject::connect(s, &QSlider::valueChanged,
                     [ctx](int value) {
                         if (g_hasIntTrampoline) g_intTrampoline(ctx, value);
                     });
}
