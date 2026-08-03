// src/cpp/headerview.h — QHeaderView
#pragma once

#include <QtWidgets/QHeaderView>
#include "signal.h"

inline QHeaderView *QHeaderView_new(int orientation, QWidget *parent) {
    return new QHeaderView(static_cast<Qt::Orientation>(orientation), parent);
}
inline void QHeaderView_delete(QHeaderView *h) { delete h; }
inline void QHeaderView_setStretchLastSection(QHeaderView *h, bool stretch) {
    h->setStretchLastSection(stretch);
}
inline void QHeaderView_resizeSection(QHeaderView *h, int section, int size) {
    h->resizeSection(section, size);
}
inline void QHeaderView_hideSection(QHeaderView *h, int section) {
    h->hideSection(section);
}
inline void QHeaderView_showSection(QHeaderView *h, int section) {
    h->showSection(section);
}
inline void QHeaderView_setSectionResizeMode(QHeaderView *h, int mode) {
    h->setSectionResizeMode(static_cast<QHeaderView::ResizeMode>(mode));
}
inline QWidget *toQWidget_QHeaderView(QHeaderView *h) {
    return static_cast<QWidget *>(h);
}
