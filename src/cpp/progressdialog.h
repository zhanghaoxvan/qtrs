// src/cpp/progressdialog.h — QProgressDialog
#pragma once

#include <QtWidgets/QProgressDialog>
#include <QtCore/QString>
#include <string>
#include <cstdint>

// Constructor / Destructor
inline QProgressDialog *QProgressDialog_new(const std::string &label,
                                             const std::string &cancelText,
                                             int min,
                                             int max,
                                             QWidget *parent = nullptr) {
    return new QProgressDialog(
        QString::fromStdString(label),
        QString::fromStdString(cancelText),
        min, max, parent);
}

inline void QProgressDialog_delete(QProgressDialog *w) { delete w; }

// Range
inline void QProgressDialog_setMinimum(QProgressDialog *w, int min) {
    w->setMinimum(min);
}

inline void QProgressDialog_setMaximum(QProgressDialog *w, int max) {
    w->setMaximum(max);
}

inline void QProgressDialog_setRange(QProgressDialog *w, int min, int max) {
    w->setRange(min, max);
}

// Value
inline void QProgressDialog_setValue(QProgressDialog *w, int value) {
    w->setValue(value);
}

inline int QProgressDialog_value(QProgressDialog *w) {
    return w->value();
}

// Label / button text
inline void QProgressDialog_setLabelText(QProgressDialog *w, const std::string &text) {
    w->setLabelText(QString::fromStdString(text));
}

inline void QProgressDialog_setCancelButtonText(QProgressDialog *w, const std::string &text) {
    w->setCancelButtonText(QString::fromStdString(text));
}

// State
inline bool QProgressDialog_wasCanceled(QProgressDialog *w) {
    return w->wasCanceled();
}

inline void QProgressDialog_setMinimumDuration(QProgressDialog *w, int ms) {
    w->setMinimumDuration(ms);
}

// Auto-behavior
inline void QProgressDialog_setAutoClose(QProgressDialog *w, bool close) {
    w->setAutoClose(close);
}

inline void QProgressDialog_setAutoReset(QProgressDialog *w, bool reset) {
    w->setAutoReset(reset);
}

// Visibility (inherited from QWidget but exposed here for convenience)
inline void QProgressDialog_show(QProgressDialog *w) {
    w->show();
}

inline void QProgressDialog_hide(QProgressDialog *w) {
    w->hide();
}

inline void QProgressDialog_close(QProgressDialog *w) {
    w->close();
}

// Upcast
inline QWidget *toQWidget_QProgressDialog(QProgressDialog *w) {
    return static_cast<QWidget *>(w);
}
