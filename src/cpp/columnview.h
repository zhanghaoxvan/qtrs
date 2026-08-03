// src/cpp/columnview.h — QColumnView
#pragma once

#include <QtWidgets/QColumnView>
#include <QtGui/QStandardItemModel>
#include "signal.h"

inline QColumnView *QColumnView_new(QWidget *parent) { return new QColumnView(parent); }
inline void QColumnView_delete(QColumnView *v) { delete v; }
inline void QColumnView_setModel(QColumnView *v, QStandardItemModel *m) { v->setModel(m); }
inline QWidget *toQWidget_QColumnView(QColumnView *v) {
    return static_cast<QWidget *>(v);
}
