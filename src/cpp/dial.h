// src/cpp/dial.h — QDial
#pragma once

#include <QtCore/QObject>
#include <QtWidgets/QDial>
#include <QtCore/QString>
#include <string>

#include "signal.h"

inline QDial *QDial_new(QWidget *parent) { return new QDial(parent); }
inline void QDial_delete(QDial *dial) { delete dial; }
inline int QDial_value(QDial *dial) { return dial->value(); }
inline void QDial_setValue(QDial *dial, int value) { dial->setValue(value); }
inline void QDial_setRange(QDial *dial, int min, int max) { dial->setRange(min, max); }
inline void QDial_setSingleStep(QDial *dial, int step) { dial->setSingleStep(step); }
inline void QDial_setPageStep(QDial *dial, int step) { dial->setPageStep(step); }
inline int QDial_minimum(QDial *dial) { return dial->minimum(); }
inline int QDial_maximum(QDial *dial) { return dial->maximum(); }
inline void QDial_setMinimum(QDial *dial, int min) { dial->setMinimum(min); }
inline void QDial_setMaximum(QDial *dial, int max) { dial->setMaximum(max); }
inline void QDial_setNotchesVisible(QDial *dial, bool visible) { dial->setNotchesVisible(visible); }
inline bool QDial_notchesVisible(QDial *dial) { return dial->notchesVisible(); }
inline void QDial_setWrapping(QDial *dial, bool wrapping) { dial->setWrapping(wrapping); }
inline bool QDial_wrapping(QDial *dial) { return dial->wrapping(); }
inline void QDial_onValueChanged(QDial *dial, uint64_t ctx) {
    QObject::connect(dial, &QDial::valueChanged, [ctx](int value) {
        if (g_hasIntTrampoline) g_intTrampoline(ctx, value);
    });
}
inline QWidget *toQWidget_QDial(QDial *dial) {
    return static_cast<QWidget*>(dial);
}
