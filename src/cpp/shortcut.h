// src/cpp/shortcut.h — QShortcut
#pragma once

#include <QtGui/QKeySequence>
#include <QtGui/QShortcut>
#include <QtWidgets/QWidget>
#include <QtCore/QObject>
#include <string>

#include "signal.h"

// Constructor / Destructor
inline QShortcut *QShortcut_new(const std::string &key, QWidget *parent) {
    return new QShortcut(QKeySequence(QString::fromStdString(key)), parent);
}

inline void QShortcut_delete(QShortcut *sc) { delete sc; }

// Properties
inline void QShortcut_setKey(QShortcut *sc, const std::string &key) {
    sc->setKey(QKeySequence(QString::fromStdString(key)));
}

inline void QShortcut_setEnabled(QShortcut *sc, bool enabled) {
    sc->setEnabled(enabled);
}

inline void QShortcut_setAutoRepeat(QShortcut *sc, bool repeat) {
    sc->setAutoRepeat(repeat);
}

// Signals
inline void QShortcut_onActivated(QShortcut *sc, uint64_t ctx) {
    QObject::connect(sc, &QShortcut::activated, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}

// Note: QShortcut::activatedAmbiguous was removed in Qt6.
// The on_activated_ambiguous signal is available on the Rust side
// but is a no-op on Qt6 builds.
