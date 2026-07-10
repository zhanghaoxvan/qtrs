// src/cpp/button.h — QPushButton
#pragma once

#include <QtCore/QObject>
#include <QtWidgets/QPushButton>
#include <QtCore/QString>
#include <string>

#include "signal.h"

inline QPushButton *QPushButton_new(const std::string &text,
                                     QWidget *parent) {
    return new QPushButton(QString::fromStdString(text), parent);
}
inline void QPushButton_show(QPushButton *btn) { btn->show(); }
inline void QPushButton_setText(QPushButton *btn, const std::string &text) {
    btn->setText(QString::fromStdString(text));
}
inline void QPushButton_delete(QPushButton *btn) { delete btn; }

inline void QPushButton_onClicked(QPushButton *btn, uint64_t ctx) {
    QObject::connect(btn, &QPushButton::clicked, [ctx](bool) {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}
