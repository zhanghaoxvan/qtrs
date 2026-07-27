// src/cpp/tableview.h — QTableView
#pragma once

#include <QtWidgets/QTableView>
#include <QtGui/QStandardItemModel>
#include "signal.h"

inline QTableView *QTableView_new(QWidget *parent) {
    return new QTableView(parent);
}

inline void QTableView_delete(QTableView *v) { delete v; }

inline void QTableView_setModel(QTableView *v, QStandardItemModel *m) {
    v->setModel(m);
}

inline QStandardItemModel *QTableView_model(QTableView *v) {
    return qobject_cast<QStandardItemModel *>(v->model());
}

inline void QTableView_setSelectionMode(QTableView *v, int mode) {
    v->setSelectionMode(static_cast<QAbstractItemView::SelectionMode>(mode));
}

inline void QTableView_setSelectionBehavior(QTableView *v, int behavior) {
    v->setSelectionBehavior(
        static_cast<QAbstractItemView::SelectionBehavior>(behavior));
}

inline void QTableView_setShowGrid(QTableView *v, bool show) {
    v->setShowGrid(show);
}

inline void QTableView_setAlternatingRowColors(QTableView *v, bool enable) {
    v->setAlternatingRowColors(enable);
}

inline void QTableView_setSortingEnabled(QTableView *v, bool enable) {
    v->setSortingEnabled(enable);
}

inline void QTableView_resizeColumnsToContents(QTableView *v) {
    v->resizeColumnsToContents();
}

inline void QTableView_resizeRowsToContents(QTableView *v) {
    v->resizeRowsToContents();
}

inline void QTableView_selectRow(QTableView *v, int row) {
    v->selectRow(row);
}

inline void QTableView_clearSelection(QTableView *v) {
    v->clearSelection();
}

// --- Signals ---
inline void QTableView_onClicked(QTableView *v, uint64_t ctx) {
    QObject::connect(v, &QTableView::clicked,
                     [ctx](const QModelIndex &) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}

inline void QTableView_onDoubleClicked(QTableView *v, uint64_t ctx) {
    QObject::connect(v, &QTableView::doubleClicked,
                     [ctx](const QModelIndex &) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}

// --- Upcast ---
inline QWidget *toQWidget_QTableView(QTableView *v) {
    return static_cast<QWidget *>(v);
}
