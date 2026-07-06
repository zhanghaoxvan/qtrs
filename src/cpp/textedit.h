// src/cpp/textedit.h — QTextEdit (multi-line)
#pragma once

#include <QObject>
#include <QString>
#include <QTextEdit>
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
