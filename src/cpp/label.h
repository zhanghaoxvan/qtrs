// src/cpp/label.h — QLabel
#pragma once

#include "qnamespace.h"
#include <QtWidgets/QLabel>
#include <QtCore/QString>
#include <string>

inline QLabel *QLabel_new(const std::string &text, QWidget *parent) {
    return new QLabel(QString::fromStdString(text), parent);
}
inline void QLabel_setText(QLabel *label, const std::string &text) {
    label->setText(QString::fromStdString(text));
}
inline void QLabel_delete(QLabel *label) { delete label; }
inline void QLabel_setAlignment(QLabel *label, int alignment) {
    label->setAlignment(static_cast<Qt::Alignment>(alignment));
}