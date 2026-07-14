// src/cpp/timeedit.h — QTimeEdit
#pragma once

#include <QtCore/QObject>
#include <QtCore/QString>
#include <QtCore/QTime>
#include <QtWidgets/QTimeEdit>
#include <string>

#include "rust/cxx.h"
#include "signal.h"

inline QTimeEdit *QTimeEdit_new(QWidget *parent) {
    return new QTimeEdit(parent);
}

inline void QTimeEdit_delete(QTimeEdit *w) { delete w; }

inline void QTimeEdit_setTime(QTimeEdit *w, const std::string &timeStr) {
    QTime t = QTime::fromString(QString::fromStdString(timeStr), Qt::ISODate);
    if (t.isValid()) w->setTime(t);
}

inline rust::String QTimeEdit_time(QTimeEdit *w) {
    return w->time().toString(Qt::ISODate).toStdString();
}

inline void QTimeEdit_setDisplayFormat(QTimeEdit *w, const std::string &format) {
    w->setDisplayFormat(QString::fromStdString(format));
}

inline void QTimeEdit_onTimeChanged(QTimeEdit *w, uint64_t ctx) {
    QObject::connect(w, &QTimeEdit::timeChanged, [ctx](const QTime &time) {
        if (g_hasStringTrampoline) {
            rust::String s = rust::String(time.toString(Qt::ISODate).toStdString());
            g_stringTrampoline(ctx, std::move(s));
        }
    });
}

inline QWidget *toQWidget_QTimeEdit(QTimeEdit *w) {
    return static_cast<QWidget *>(w);
}
