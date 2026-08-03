// src/cpp/completer.h — QCompleter
#pragma once

#include <QtWidgets/QCompleter>
#include <QtCore/QStringListModel>
#include <QtCore/QString>
#include <string>
#include "rust/cxx.h"
#include "signal.h"

inline QCompleter *QCompleter_new(QStringListModel *model, QObject *parent) {
    return new QCompleter(model, parent);
}
inline void QCompleter_delete(QCompleter *c) { delete c; }
inline void QCompleter_setCompletionMode(QCompleter *c, int mode) {
    c->setCompletionMode(static_cast<QCompleter::CompletionMode>(mode));
}
inline void QCompleter_setCaseSensitivity(QCompleter *c, int cs) {
    c->setCaseSensitivity(static_cast<Qt::CaseSensitivity>(cs));
}
inline void QCompleter_setFilterMode(QCompleter *c, int mode) {
    c->setFilterMode(static_cast<Qt::MatchFlags>(mode));
}
inline void QCompleter_onActivated(QCompleter *c, uint64_t ctx) {
    QObject::connect(c, QOverload<const QString &>::of(&QCompleter::activated),
                     [ctx](const QString &text) {
                         if (g_hasStringTrampoline)
                             g_stringTrampoline(ctx, rust::String(text.toStdString()));
                     });
}
