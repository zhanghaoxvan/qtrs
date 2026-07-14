// src/cpp/datetimeedit.h — QDateTimeEdit
#pragma once

#include <QtCore/QDateTime>
#include <QtCore/QObject>
#include <QtCore/QString>
#include <QtWidgets/QDateTimeEdit>
#include <string>

#include "rust/cxx.h"
#include "signal.h"

inline QDateTimeEdit *QDateTimeEdit_new(QWidget *parent) {
    return new QDateTimeEdit(parent);
}

inline void QDateTimeEdit_delete(QDateTimeEdit *w) { delete w; }

inline void QDateTimeEdit_setDateTime(QDateTimeEdit *w, const std::string &dtStr) {
    QDateTime dt = QDateTime::fromString(QString::fromStdString(dtStr), Qt::ISODate);
    if (dt.isValid()) w->setDateTime(dt);
}

inline rust::String QDateTimeEdit_dateTime(QDateTimeEdit *w) {
    return w->dateTime().toString(Qt::ISODate).toStdString();
}

inline void QDateTimeEdit_setDisplayFormat(QDateTimeEdit *w, const std::string &format) {
    w->setDisplayFormat(QString::fromStdString(format));
}

inline void QDateTimeEdit_setCalendarPopup(QDateTimeEdit *w, bool enabled) {
    w->setCalendarPopup(enabled);
}

inline void QDateTimeEdit_onDateTimeChanged(QDateTimeEdit *w, uint64_t ctx) {
    QObject::connect(w, &QDateTimeEdit::dateTimeChanged, [ctx](const QDateTime &dt) {
        if (g_hasStringTrampoline) {
            rust::String s = rust::String(dt.toString(Qt::ISODate).toStdString());
            g_stringTrampoline(ctx, std::move(s));
        }
    });
}

inline QWidget *toQWidget_QDateTimeEdit(QDateTimeEdit *w) {
    return static_cast<QWidget *>(w);
}
