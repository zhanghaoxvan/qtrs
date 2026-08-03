// src/cpp/lcdnumber.h — QLCDNumber
#pragma once

#include <QtCore/QObject>
#include <QtWidgets/QLCDNumber>
#include <QtCore/QString>
#include <string>

#include "signal.h"

inline QLCDNumber *QLCDNumber_new(QWidget *parent) {
    return new QLCDNumber(parent);
}
inline void QLCDNumber_delete(QLCDNumber *lcd) { delete lcd; }
inline void QLCDNumber_displayInt(QLCDNumber *lcd, int value) { lcd->display(value); }
inline void QLCDNumber_displayStr(QLCDNumber *lcd, const std::string &text) {
    lcd->display(QString::fromStdString(text));
}
inline void QLCDNumber_setDigitCount(QLCDNumber *lcd, int n) { lcd->setDigitCount(n); }
inline int QLCDNumber_digitCount(QLCDNumber *lcd) { return lcd->digitCount(); }
inline void QLCDNumber_setMode(QLCDNumber *lcd, int mode) {
    lcd->setMode(static_cast<QLCDNumber::Mode>(mode));
}
inline void QLCDNumber_setSegmentStyle(QLCDNumber *lcd, int style) {
    lcd->setSegmentStyle(static_cast<QLCDNumber::SegmentStyle>(style));
}
inline void QLCDNumber_setSmallDecimalPoint(QLCDNumber *lcd, bool enabled) {
    lcd->setSmallDecimalPoint(enabled);
}
inline bool QLCDNumber_checkOverflow(QLCDNumber *lcd, int value) {
    return lcd->checkOverflow(value);
}
inline void QLCDNumber_onOverflow(QLCDNumber *lcd, uint64_t ctx) {
    QObject::connect(lcd, &QLCDNumber::overflow, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}
inline QWidget *toQWidget_QLCDNumber(QLCDNumber *lcd) {
    return static_cast<QWidget*>(lcd);
}
