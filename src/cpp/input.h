// src/cpp/input.h — QLineEdit
#pragma once

#include <QtWidgets/QLineEdit>
#include <QtCore/QObject>
#include <QtCore/QString>
#include <string>

#include "rust/cxx.h"
#include "signal.h"

inline QLineEdit *QLineEdit_new(const std::string &text, QWidget *parent) {
    return new QLineEdit(QString::fromStdString(text), parent);
}
inline rust::String QLineEdit_text(QLineEdit *edit) {
    return edit->text().toStdString();
}
inline void QLineEdit_setText(QLineEdit *edit, const std::string &text) {
    edit->setText(QString::fromStdString(text));
}
inline void QLineEdit_clear(QLineEdit *e) { e->clear(); }
inline void QLineEdit_selectAll(QLineEdit *e) { e->selectAll(); }
inline void QLineEdit_copy(QLineEdit *e) { e->copy(); }
inline void QLineEdit_cut(QLineEdit *e) { e->cut(); }
inline void QLineEdit_paste(QLineEdit *e) { e->paste(); }
inline void QLineEdit_undo(QLineEdit *e) { e->undo(); }
inline void QLineEdit_redo(QLineEdit *e) { e->redo(); }
inline void QLineEdit_setReadOnly(QLineEdit *e, bool ro) { e->setReadOnly(ro); }
inline bool QLineEdit_isReadOnly(QLineEdit *e) { return e->isReadOnly(); }
inline void QLineEdit_setEchoMode(QLineEdit *e, int mode) { e->setEchoMode(static_cast<QLineEdit::EchoMode>(mode)); }
inline void QLineEdit_setMaxLength(QLineEdit *e, int len) { e->setMaxLength(len); }
inline int QLineEdit_maxLength(QLineEdit *e) { return e->maxLength(); }
inline int QLineEdit_cursorPosition(QLineEdit *e) { return e->cursorPosition(); }
inline void QLineEdit_setCursorPosition(QLineEdit *e, int pos) { e->setCursorPosition(pos); }
inline void QLineEdit_delete(QLineEdit *edit) { delete edit; }

inline void QLineEdit_onReturnPressed(QLineEdit *edit, uint64_t ctx) {
    QObject::connect(edit, &QLineEdit::returnPressed, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}
