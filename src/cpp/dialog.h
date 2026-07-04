// src/cpp/dialog.h — QMessageBox convenience dialogs
#pragma once

#include <QMessageBox>
#include <QString>
#include <string>

inline void QMessageBox_information(QWidget *parent, const std::string &title,
                                     const std::string &text) {
    QMessageBox::information(parent, QString::fromStdString(title),
                              QString::fromStdString(text));
}
inline void QMessageBox_warning(QWidget *parent, const std::string &title,
                                 const std::string &text) {
    QMessageBox::warning(parent, QString::fromStdString(title),
                          QString::fromStdString(text));
}
inline void QMessageBox_critical(QWidget *parent, const std::string &title,
                                  const std::string &text) {
    QMessageBox::critical(parent, QString::fromStdString(title),
                           QString::fromStdString(text));
}
inline int QMessageBox_question(QWidget *parent, const std::string &title,
                                 const std::string &text) {
    return QMessageBox::question(parent, QString::fromStdString(title),
                                  QString::fromStdString(text));
}
