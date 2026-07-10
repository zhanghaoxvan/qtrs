// src/cpp/progress.h — QProgressBar
#pragma once

#include <QtWidgets/QProgressBar>
#include <QtCore/QString>

inline QProgressBar *QProgressBar_new(QWidget *parent) {
    return new QProgressBar(parent);
}

inline void QProgressBar_setValue(QProgressBar *p, int value) {
    p->setValue(value);
}

inline int QProgressBar_value(QProgressBar *p) {
    return p->value();
}

inline void QProgressBar_setRange(QProgressBar *p, int min, int max) {
    p->setRange(min, max);
}

inline void QProgressBar_setMinimum(QProgressBar *p, int min) {
    p->setMinimum(min);
}

inline void QProgressBar_setMaximum(QProgressBar *p, int max) {
    p->setMaximum(max);
}

inline void QProgressBar_setFormat(QProgressBar *p, const std::string &format) {
    p->setFormat(QString::fromStdString(format));
}

inline void QProgressBar_delete(QProgressBar *p) {
    delete p;
}