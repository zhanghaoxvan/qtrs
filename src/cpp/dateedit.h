// src/cpp/dateedit.h — QDateEdit
#pragma once

#include <QtCore/QDate>
#include <QtCore/QObject>
#include <QtCore/QString>
#include <QtWidgets/QDateEdit>
#include <string>

#include "rust/cxx.h"
#include "signal.h"

inline QDateEdit *QDateEdit_new(QWidget *parent) {
    return new QDateEdit(parent);
}

inline void QDateEdit_delete(QDateEdit *w) { delete w; }

inline void QDateEdit_setDate(QDateEdit *w, const std::string &dateStr) {
    QDate d = QDate::fromString(QString::fromStdString(dateStr), Qt::ISODate);
    if (d.isValid()) w->setDate(d);
}

inline rust::String QDateEdit_date(QDateEdit *w) {
    return w->date().toString(Qt::ISODate).toStdString();
}

inline void QDateEdit_setMinimumDate(QDateEdit *w, const std::string &dateStr) {
    QDate d = QDate::fromString(QString::fromStdString(dateStr), Qt::ISODate);
    if (d.isValid()) w->setMinimumDate(d);
}

inline void QDateEdit_setMaximumDate(QDateEdit *w, const std::string &dateStr) {
    QDate d = QDate::fromString(QString::fromStdString(dateStr), Qt::ISODate);
    if (d.isValid()) w->setMaximumDate(d);
}

inline void QDateEdit_clearMinimumDate(QDateEdit *w) {
    w->clearMinimumDate();
}

inline void QDateEdit_clearMaximumDate(QDateEdit *w) {
    w->clearMaximumDate();
}

inline void QDateEdit_setDisplayFormat(QDateEdit *w, const std::string &format) {
    w->setDisplayFormat(QString::fromStdString(format));
}

inline void QDateEdit_setCalendarPopup(QDateEdit *w, bool enabled) {
    w->setCalendarPopup(enabled);
}

inline void QDateEdit_onDateChanged(QDateEdit *w, uint64_t ctx) {
    QObject::connect(w, &QDateEdit::dateChanged, [ctx](const QDate &date) {
        if (g_hasStringTrampoline) {
            rust::String s = rust::String(date.toString(Qt::ISODate).toStdString());
            g_stringTrampoline(ctx, std::move(s));
        }
    });
}

inline QWidget *toQWidget_QDateEdit(QDateEdit *w) {
    return static_cast<QWidget *>(w);
}
