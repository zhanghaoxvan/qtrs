// src/cpp/textedit.h — QTextEdit (multi-line)
#pragma once

#include <QtCore/QObject>
#include <QtCore/QString>
#include <QtWidgets/QTextEdit>
#include <string>

#include "rust/cxx.h"
#include "signal.h"

inline QTextEdit *QTextEdit_new(QWidget *parent) {
    return new QTextEdit(parent);
}
inline rust::String QTextEdit_toPlainText(QTextEdit *edit) {
    return edit->toPlainText().toStdString();
}
inline void QTextEdit_setPlainText(QTextEdit *edit, const std::string &text) {
    edit->setPlainText(QString::fromStdString(text));
}
inline void QTextEdit_setPlaceholderText(QTextEdit *edit,
                                          const std::string &text) {
    edit->setPlaceholderText(QString::fromStdString(text));
}
inline void QTextEdit_delete(QTextEdit *edit) { delete edit; }

inline void QTextEdit_onTextChanged(QTextEdit *edit, uint64_t ctx) {
    QObject::connect(edit, &QTextEdit::textChanged, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}
inline void QTextEdit_setReadOnly(QTextEdit *e, bool ro) { e->setReadOnly(ro); }
inline bool QTextEdit_isReadOnly(QTextEdit *e) { return e->isReadOnly(); }
inline void QTextEdit_append(QTextEdit *e, const std::string &text) { e->append(QString::fromStdString(text)); }
inline void QTextEdit_copy(QTextEdit *e) { e->copy(); }
inline void QTextEdit_cut(QTextEdit *e) { e->cut(); }
inline void QTextEdit_paste(QTextEdit *e) { e->paste(); }
inline void QTextEdit_undo(QTextEdit *e) { e->undo(); }
inline void QTextEdit_redo(QTextEdit *e) { e->redo(); }
inline void QTextEdit_selectAll(QTextEdit *e) { e->selectAll(); }
