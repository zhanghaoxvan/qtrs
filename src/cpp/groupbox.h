// src/cpp/groupbox.h — QGroupBox
#pragma once

#include <QtWidgets/QGroupBox>
#include <QtCore/QString>
#include <string>

/// Create a new QGroupBox with title and optional parent.
inline QGroupBox *QGroupBox_new(const std::string &title, QWidget *parent) {
    return new QGroupBox(QString::fromStdString(title), parent);
}

/// Set the group box title.
inline void QGroupBox_setTitle(QGroupBox *gb, const std::string &title) {
    gb->setTitle(QString::fromStdString(title));
}

/// Delete the group box.
inline void QGroupBox_delete(QGroupBox *gb) {
    delete gb;
}
