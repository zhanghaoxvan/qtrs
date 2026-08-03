// src/cpp/keysequenceedit.h — QKeySequenceEdit
#pragma once

#include <QtWidgets/QKeySequenceEdit>
#include <QtCore/QString>
#include <string>
#include "signal.h"

inline QKeySequenceEdit *QKeySequenceEdit_new(QWidget *parent) {
    return new QKeySequenceEdit(parent);
}
inline void QKeySequenceEdit_delete(QKeySequenceEdit *e) { delete e; }
inline void QKeySequenceEdit_clear(QKeySequenceEdit *e) { e->clear(); }
inline void QKeySequenceEdit_onEditingFinished(QKeySequenceEdit *e, uint64_t ctx) {
    QObject::connect(e, &QKeySequenceEdit::editingFinished, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}
inline QWidget *toQWidget_QKeySequenceEdit(QKeySequenceEdit *e) {
    return static_cast<QWidget *>(e);
}
