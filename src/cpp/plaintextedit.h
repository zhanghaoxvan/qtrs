// src/cpp/plaintextedit.h — QPlainTextEdit
#pragma once

#include <QtCore/QObject>
#include <QtCore/QString>
#include <QtWidgets/QPlainTextEdit>
#include <string>

#include "rust/cxx.h"
#include "signal.h"

inline QPlainTextEdit *QPlainTextEdit_new(QWidget *parent) {
    return new QPlainTextEdit(parent);
}

inline void QPlainTextEdit_delete(QPlainTextEdit *w) { delete w; }

inline void QPlainTextEdit_setPlainText(QPlainTextEdit *w, const std::string &text) {
    w->setPlainText(QString::fromStdString(text));
}

inline rust::String QPlainTextEdit_plainText(QPlainTextEdit *w) {
    return w->toPlainText().toStdString();
}

inline void QPlainTextEdit_setPlaceholderText(QPlainTextEdit *w, const std::string &text) {
    w->setPlaceholderText(QString::fromStdString(text));
}

inline void QPlainTextEdit_setReadOnly(QPlainTextEdit *w, bool readOnly) {
    w->setReadOnly(readOnly);
}

inline void QPlainTextEdit_setLineWrapMode(QPlainTextEdit *w, int mode) {
    w->setLineWrapMode(static_cast<QPlainTextEdit::LineWrapMode>(mode));
}

inline void QPlainTextEdit_appendPlainText(QPlainTextEdit *w, const std::string &text) {
    w->appendPlainText(QString::fromStdString(text));
}

inline void QPlainTextEdit_clear(QPlainTextEdit *w) {
    w->clear();
}

inline void QPlainTextEdit_onTextChanged(QPlainTextEdit *w, uint64_t ctx) {
    QObject::connect(w, &QPlainTextEdit::textChanged, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}

inline void QPlainTextEdit_onCursorPositionChanged(QPlainTextEdit *w, uint64_t ctx) {
    QObject::connect(w, &QPlainTextEdit::cursorPositionChanged, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}

inline QWidget *toQWidget_QPlainTextEdit(QPlainTextEdit *w) {
    return static_cast<QWidget *>(w);
}
