// src/cpp/itemselectionmodel.h — QItemSelectionModel
#pragma once

#include <QtCore/QItemSelectionModel>
#include "signal.h"

// QItemSelectionModel is owned by the view — no new/delete here.
// It is obtained from a view via the Rust wrapper.

inline bool QItemSelectionModel_hasSelection(QItemSelectionModel *sm) {
    return sm->hasSelection();
}

// --- Signals ---
inline void QItemSelectionModel_onSelectionChanged(QItemSelectionModel *sm,
                                                    uint64_t ctx) {
    QObject::connect(sm, &QItemSelectionModel::selectionChanged,
                     [ctx](const QItemSelection &, const QItemSelection &) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}

inline void QItemSelectionModel_onCurrentChanged(QItemSelectionModel *sm,
                                                  uint64_t ctx) {
    QObject::connect(sm, &QItemSelectionModel::currentChanged,
                     [ctx](const QModelIndex &, const QModelIndex &) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}
