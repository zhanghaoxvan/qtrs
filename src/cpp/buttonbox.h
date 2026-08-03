// src/cpp/buttonbox.h — QButtonGroup
#pragma once

#include <QtWidgets/QButtonGroup>
#include <QtWidgets/QAbstractButton>
#include "signal.h"

inline QButtonGroup *QButtonGroup_new(QObject *parent) {
    return new QButtonGroup(parent);
}
inline void QButtonGroup_delete(QButtonGroup *bg) { delete bg; }
inline void QButtonGroup_addButton(QButtonGroup *bg, QAbstractButton *btn, int id) {
    bg->addButton(btn, id);
}
inline void QButtonGroup_setExclusive(QButtonGroup *bg, bool exclusive) {
    bg->setExclusive(exclusive);
}
inline void QButtonGroup_onButtonClicked(QButtonGroup *bg, uint64_t ctx) {
    QObject::connect(bg, &QButtonGroup::idClicked, [ctx](int id) {
        if (g_hasIntTrampoline) g_intTrampoline(ctx, id);
    });
}
