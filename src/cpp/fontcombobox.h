// src/cpp/fontcombobox.h — QFontComboBox
#pragma once

#include <QtWidgets/QFontComboBox>
#include <QtCore/QString>
#include <string>
#include "signal.h"

inline QFontComboBox *QFontComboBox_new(QWidget *parent) {
    return new QFontComboBox(parent);
}
inline void QFontComboBox_delete(QFontComboBox *cb) { delete cb; }
inline void QFontComboBox_setCurrentFont(QFontComboBox *cb, const std::string &family) {
    cb->setCurrentFont(QFont(QString::fromStdString(family)));
}
inline rust::String QFontComboBox_currentFont(QFontComboBox *cb) {
    return cb->currentFont().family().toStdString();
}
inline void QFontComboBox_setFontFilters(QFontComboBox *cb, int filters) {
    cb->setFontFilters(static_cast<QFontComboBox::FontFilters>(filters));
}
inline void QFontComboBox_onCurrentFontChanged(QFontComboBox *cb, uint64_t ctx) {
    QObject::connect(cb, &QFontComboBox::currentFontChanged, [ctx](const QFont &) {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}
inline QWidget *toQWidget_QFontComboBox(QFontComboBox *cb) {
    return static_cast<QWidget *>(cb);
}
