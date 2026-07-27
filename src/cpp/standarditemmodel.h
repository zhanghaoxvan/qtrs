// src/cpp/standarditemmodel.h — QStandardItemModel
#pragma once

#include <QtGui/QStandardItemModel>
#include <QtCore/QString>
#include <string>
#include "rust/cxx.h"
#include "signal.h"

inline QStandardItemModel *QStandardItemModel_new(QObject *parent) {
    return new QStandardItemModel(parent);
}

inline void QStandardItemModel_delete(QStandardItemModel *m) { delete m; }

inline int QStandardItemModel_rowCount(QStandardItemModel *m) {
    return m->rowCount();
}

inline int QStandardItemModel_columnCount(QStandardItemModel *m) {
    return m->columnCount();
}

inline void QStandardItemModel_setRowCount(QStandardItemModel *m, int rows) {
    m->setRowCount(rows);
}

inline void QStandardItemModel_setColumnCount(QStandardItemModel *m, int cols) {
    m->setColumnCount(cols);
}

inline void QStandardItemModel_setData(QStandardItemModel *m, int row, int col,
                                        const std::string &value) {
    QModelIndex idx = m->index(row, col);
    m->setData(idx, QString::fromStdString(value));
}

inline rust::String QStandardItemModel_data(QStandardItemModel *m, int row, int col) {
    QModelIndex idx = m->index(row, col);
    return m->data(idx).toString().toStdString();
}

inline void QStandardItemModel_setHeaderData(QStandardItemModel *m, int section,
                                              int orientation, const std::string &value) {
    m->setHeaderData(section, static_cast<Qt::Orientation>(orientation),
                     QString::fromStdString(value));
}

inline rust::String QStandardItemModel_headerData(QStandardItemModel *m, int section,
                                                    int orientation) {
    return m->headerData(section, static_cast<Qt::Orientation>(orientation))
        .toString()
        .toStdString();
}

inline void QStandardItemModel_insertRow(QStandardItemModel *m, int row) {
    m->insertRow(row);
}

inline void QStandardItemModel_removeRow(QStandardItemModel *m, int row) {
    m->removeRow(row);
}

inline void QStandardItemModel_insertColumn(QStandardItemModel *m, int column) {
    m->insertColumn(column);
}

inline void QStandardItemModel_removeColumn(QStandardItemModel *m, int column) {
    m->removeColumn(column);
}

inline void QStandardItemModel_clear(QStandardItemModel *m) { m->clear(); }

inline void QStandardItemModel_appendRow(QStandardItemModel *m,
                                          rust::Vec<rust::String> texts) {
    QList<QStandardItem *> items;
    for (const auto &text : texts) {
        items.append(new QStandardItem(
            QString::fromStdString(std::string(text))));
    }
    m->appendRow(items);
}

// --- Signals ---
inline void QStandardItemModel_onModelReset(QStandardItemModel *m, uint64_t ctx) {
    QObject::connect(m, &QStandardItemModel::modelReset, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}

inline void QStandardItemModel_onDataChanged(QStandardItemModel *m, uint64_t ctx) {
    QObject::connect(
        m, &QStandardItemModel::dataChanged,
        [ctx](const QModelIndex &, const QModelIndex &, const QVector<int> &) {
            if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
        });
}

inline void QStandardItemModel_onRowsInserted(QStandardItemModel *m, uint64_t ctx) {
    QObject::connect(m, &QStandardItemModel::rowsInserted,
                     [ctx](const QModelIndex &, int, int) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}

inline void QStandardItemModel_onRowsRemoved(QStandardItemModel *m, uint64_t ctx) {
    QObject::connect(m, &QStandardItemModel::rowsRemoved,
                     [ctx](const QModelIndex &, int, int) {
                         if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
                     });
}
