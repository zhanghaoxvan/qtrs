// src/cpp/messagebox.h — QMessageBox
#pragma once

#include <QtWidgets/QMessageBox>
#include <QtCore/QString>
#include <string>
#include <cstdint>

// Constructor / Destructor
inline QMessageBox *QMessageBox_new(QWidget *parent = nullptr) {
    return new QMessageBox(parent);
}

inline void QMessageBox_delete(QMessageBox *w) { delete w; }

// Configuration
inline void QMessageBox_setIcon(QMessageBox *w, int icon) {
    w->setIcon(static_cast<QMessageBox::Icon>(icon));
}

inline void QMessageBox_setText(QMessageBox *w, const std::string &text) {
    w->setText(QString::fromStdString(text));
}

inline void QMessageBox_setInformativeText(QMessageBox *w, const std::string &text) {
    w->setInformativeText(QString::fromStdString(text));
}

inline void QMessageBox_setWindowTitle(QMessageBox *w, const std::string &title) {
    w->setWindowTitle(QString::fromStdString(title));
}

inline void QMessageBox_setStandardButtons(QMessageBox *w, int buttons) {
    w->setStandardButtons(static_cast<QMessageBox::StandardButtons>(buttons));
}

inline void QMessageBox_setDefaultButton(QMessageBox *w, int button) {
    w->setDefaultButton(static_cast<QMessageBox::StandardButton>(button));
}

inline void QMessageBox_setDetailedText(QMessageBox *w, const std::string &text) {
    w->setDetailedText(QString::fromStdString(text));
}

inline int QMessageBox_exec(QMessageBox *w) {
    return static_cast<int>(w->exec());
}

// Static convenience
inline void QMessageBox_about(QWidget *parent, const std::string &title, const std::string &text) {
    QMessageBox::about(parent, QString::fromStdString(title), QString::fromStdString(text));
}

// Upcast
inline QWidget *toQWidget_QMessageBox(QMessageBox *w) {
    return static_cast<QWidget *>(w);
}
