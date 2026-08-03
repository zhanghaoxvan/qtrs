// src/cpp/combobox.h — QComboBox
#pragma once

#include <QtWidgets/QComboBox>
#include <QtCore/QObject>
#include <QtCore/QString>
#include <string>

#include "rust/cxx.h"
#include "signal.h"

inline QComboBox *QComboBox_new(QWidget *parent) {
    return new QComboBox(parent);
}
inline void QComboBox_addItem(QComboBox *cb, const std::string &text) {
    cb->addItem(QString::fromStdString(text));
}
inline rust::String QComboBox_currentText(QComboBox *cb) {
    return cb->currentText().toStdString();
}
inline void QComboBox_setCurrentIndex(QComboBox *cb, int i) {
    cb->setCurrentIndex(i);
}
inline void QComboBox_delete(QComboBox *cb) { delete cb; }

inline void QComboBox_onCurrentTextChanged(QComboBox *cb, uint64_t ctx) {
    QObject::connect(cb, &QComboBox::currentTextChanged,
                     [ctx](const QString &) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}

inline void QComboBox_onCurrentIndexChanged(QComboBox *cb, uint64_t ctx) {
    QObject::connect(cb, QOverload<int>::of(&QComboBox::currentIndexChanged),
                     [ctx](int index) {
                         if (g_hasIntTrampoline) g_intTrampoline(ctx, index);
                     });
}
inline int QComboBox_count(QComboBox *cb) { return cb->count(); }
inline void QComboBox_removeItem(QComboBox *cb, int index) { cb->removeItem(index); }
inline void QComboBox_clear(QComboBox *cb) { cb->clear(); }
inline void QComboBox_setEditable(QComboBox *cb, bool edit) { cb->setEditable(edit); }
inline bool QComboBox_isEditable(QComboBox *cb) { return cb->isEditable(); }
inline void QComboBox_setMaxCount(QComboBox *cb, int max) { cb->setMaxCount(max); }
