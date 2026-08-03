// src/cpp/dialog.h — QDialog support
#pragma once

#include <QtWidgets/QDialog>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QPushButton>
#include <QtCore/QString>
#include <string>

inline QDialog* QDialog_new(QWidget* parent) {
    return new QDialog(parent);
}

inline void QDialog_setModal(QDialog* dialog, bool modal) {
    dialog->setModal(modal);
}

inline void QDialog_setWindowTitle(QDialog* dialog, const std::string& title) {
    dialog->setWindowTitle(QString::fromStdString(title));
}

inline void QDialog_setMinimumSize(QDialog* dialog, int w, int h) {
    dialog->setMinimumSize(w, h);
}

inline void QDialog_resize(QDialog* dialog, int w, int h) {
    dialog->resize(w, h);
}

inline void QDialog_show(QDialog* dialog) {
    dialog->show();
}

inline void QDialog_exec(QDialog* dialog) {
    dialog->exec();
}

inline void QDialog_accept(QDialog* dialog) {
    dialog->accept();
}

inline void QDialog_reject(QDialog* dialog) {
    dialog->reject();
}

inline void QDialog_setLayout(QDialog* dialog, QLayout* layout) {
    dialog->setLayout(layout);
}

inline void QDialog_delete(QDialog* dialog) {
    delete dialog;
}
