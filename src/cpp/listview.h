// src/cpp/listview.h — QListView
#pragma once

#include <QtWidgets/QListView>
#include <QtGui/QStandardItemModel>
#include "signal.h"

inline QListView *QListView_new(QWidget *parent) {
    return new QListView(parent);
}

inline void QListView_delete(QListView *v) { delete v; }

inline void QListView_setModel(QListView *v, QStandardItemModel *m) {
    v->setModel(m);
}

inline QStandardItemModel *QListView_model(QListView *v) {
    return qobject_cast<QStandardItemModel *>(v->model());
}

inline void QListView_setSelectionMode(QListView *v, int mode) {
    v->setSelectionMode(static_cast<QAbstractItemView::SelectionMode>(mode));
}

inline void QListView_setViewMode(QListView *v, int mode) {
    v->setViewMode(static_cast<QListView::ViewMode>(mode));
}

// --- Signals ---
inline void QListView_onClicked(QListView *v, uint64_t ctx) {
    QObject::connect(v, &QListView::clicked,
                     [ctx](const QModelIndex &) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}

inline void QListView_onDoubleClicked(QListView *v, uint64_t ctx) {
    QObject::connect(v, &QListView::doubleClicked,
                     [ctx](const QModelIndex &) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}

// --- Upcast ---
inline QWidget *toQWidget_QListView(QListView *v) {
    return static_cast<QWidget *>(v);
}
