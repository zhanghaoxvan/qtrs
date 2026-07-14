// src/cpp/calendarwidget.h — QCalendarWidget
#pragma once

#include <QtWidgets/QCalendarWidget>
#include <QtWidgets/QWidget>
#include <QtCore/QDate>
#include <QtCore/QObject>
#include <QtCore/QString>
#include <string>

#include "rust/cxx.h"
#include "signal.h"

// Constructor / Destructor
inline QCalendarWidget *QCalendarWidget_new(QWidget *parent) {
    return new QCalendarWidget(parent);
}

inline void QCalendarWidget_delete(QCalendarWidget *cal) { delete cal; }

// Date helpers
static QDate parseDate(const std::string &date_str) {
    return QDate::fromString(QString::fromStdString(date_str), "yyyy-MM-dd");
}

static rust::String formatDate(const QDate &date) {
    if (!date.isValid()) return rust::String();
    return rust::String(date.toString("yyyy-MM-dd").toStdString());
}

// Properties
inline void QCalendarWidget_setSelectedDate(QCalendarWidget *cal, const std::string &date_str) {
    QDate date = parseDate(date_str);
    if (date.isValid()) cal->setSelectedDate(date);
}

inline rust::String QCalendarWidget_selectedDate(QCalendarWidget *cal) {
    return formatDate(cal->selectedDate());
}

inline void QCalendarWidget_setMinimumDate(QCalendarWidget *cal, const std::string &date_str) {
    QDate date = parseDate(date_str);
    if (date.isValid()) cal->setMinimumDate(date);
}

inline void QCalendarWidget_setMaximumDate(QCalendarWidget *cal, const std::string &date_str) {
    QDate date = parseDate(date_str);
    if (date.isValid()) cal->setMaximumDate(date);
}

inline void QCalendarWidget_setFirstDayOfWeek(QCalendarWidget *cal, int day) {
    cal->setFirstDayOfWeek(static_cast<Qt::DayOfWeek>(day));
}

inline void QCalendarWidget_setGridVisible(QCalendarWidget *cal, bool visible) {
    cal->setGridVisible(visible);
}

inline void QCalendarWidget_setNavigationBarVisible(QCalendarWidget *cal, bool visible) {
    cal->setNavigationBarVisible(visible);
}

// Signals
inline void QCalendarWidget_onSelectionChanged(QCalendarWidget *cal, uint64_t ctx) {
    QObject::connect(cal, &QCalendarWidget::selectionChanged, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}

inline void QCalendarWidget_onActivated(QCalendarWidget *cal, uint64_t ctx) {
    QObject::connect(cal, &QCalendarWidget::activated, [ctx](const QDate &date) {
        if (g_hasStringTrampoline) {
            rust::String rustDate = formatDate(date);
            g_stringTrampoline(ctx, std::move(rustDate));
        }
    });
}

// Upcast
inline QWidget *toQWidget_QCalendarWidget(QCalendarWidget *cal) {
    return static_cast<QWidget *>(cal);
}
