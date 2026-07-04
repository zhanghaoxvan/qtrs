// src/cpp/checkbox.h — QCheckBox
#pragma once

#include <QCheckBox>
#include <QObject>
#include <QString>
#include <string>

#include "signal.h"

inline QCheckBox *QCheckBox_new(const std::string &text, QWidget *parent) {
    return new QCheckBox(QString::fromStdString(text), parent);
}
inline bool QCheckBox_isChecked(QCheckBox *cb) { return cb->isChecked(); }
inline void QCheckBox_setChecked(QCheckBox *cb, bool c) { cb->setChecked(c); }
inline void QCheckBox_delete(QCheckBox *cb) { delete cb; }

inline void QCheckBox_onToggled(QCheckBox *cb, uint64_t ctx) {
    QObject::connect(cb, &QCheckBox::toggled,
                     [ctx](bool checked) { g_boolTrampoline(ctx, checked); });
}
