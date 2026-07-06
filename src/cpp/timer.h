// src/cpp/timer.h — QTimer
#pragma once

#include <QObject>
#include <QTimer>

#include "signal.h"

inline QTimer *QTimer_new() { return new QTimer(); }
inline void QTimer_start(QTimer *t, int ms) { t->start(ms); }
inline void QTimer_stop(QTimer *t) { t->stop(); }
inline bool QTimer_isActive(QTimer *t) { return t->isActive(); }
inline void QTimer_delete(QTimer *t) { delete t; }

inline void QTimer_onTimeout(QTimer *t, uint64_t ctx) {
    QObject::connect(t, &QTimer::timeout, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}

/// Qt-style one-shot: fires once after `ms`, then self-destructs.
inline void QTimer_singleShot(int ms, uint64_t ctx) {
    QTimer::singleShot(ms, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}
