// src/cpp/input.h — QLineEdit
#pragma once

#include <QLineEdit>
#include <QObject>
#include <QString>
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
inline void QLineEdit_delete(QLineEdit *edit) { delete edit; }

inline void QLineEdit_onReturnPressed(QLineEdit *edit, uint64_t ctx) {
    QObject::connect(edit, &QLineEdit::returnPressed, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}
