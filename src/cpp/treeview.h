// src/cpp/treeview.h — QTreeView
#pragma once

#include <QtWidgets/QTreeView>
#include <QtGui/QStandardItemModel>
#include "signal.h"

inline QTreeView *QTreeView_new(QWidget *parent) {
    return new QTreeView(parent);
}

inline void QTreeView_delete(QTreeView *v) { delete v; }

inline void QTreeView_setModel(QTreeView *v, QStandardItemModel *m) {
    v->setModel(m);
}

inline QStandardItemModel *QTreeView_model(QTreeView *v) {
    return qobject_cast<QStandardItemModel *>(v->model());
}

inline void QTreeView_setSelectionMode(QTreeView *v, int mode) {
    v->setSelectionMode(static_cast<QAbstractItemView::SelectionMode>(mode));
}

inline void QTreeView_setHeaderHidden(QTreeView *v, bool hidden) {
    v->setHeaderHidden(hidden);
}

inline void QTreeView_setAnimated(QTreeView *v, bool animated) {
    v->setAnimated(animated);
}

inline void QTreeView_setIndentation(QTreeView *v, int indent) {
    v->setIndentation(indent);
}

inline void QTreeView_setRootIsDecorated(QTreeView *v, bool decorated) {
    v->setRootIsDecorated(decorated);
}

inline void QTreeView_setItemsExpandable(QTreeView *v, bool expandable) {
    v->setItemsExpandable(expandable);
}

inline void QTreeView_expandAll(QTreeView *v) { v->expandAll(); }
inline void QTreeView_collapseAll(QTreeView *v) { v->collapseAll(); }

// --- Signals ---
inline void QTreeView_onClicked(QTreeView *v, uint64_t ctx) {
    QObject::connect(v, &QTreeView::clicked,
                     [ctx](const QModelIndex &) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}

inline void QTreeView_onDoubleClicked(QTreeView *v, uint64_t ctx) {
    QObject::connect(v, &QTreeView::doubleClicked,
                     [ctx](const QModelIndex &) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}

inline void QTreeView_onExpanded(QTreeView *v, uint64_t ctx) {
    QObject::connect(v, &QTreeView::expanded,
                     [ctx](const QModelIndex &) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}

inline void QTreeView_onCollapsed(QTreeView *v, uint64_t ctx) {
    QObject::connect(v, &QTreeView::collapsed,
                     [ctx](const QModelIndex &) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}

// --- Upcast ---
inline QWidget *toQWidget_QTreeView(QTreeView *v) {
    return static_cast<QWidget *>(v);
}
