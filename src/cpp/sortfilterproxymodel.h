// src/cpp/sortfilterproxymodel.h — QSortFilterProxyModel
#pragma once

#include <QtCore/QSortFilterProxyModel>
#include <QtGui/QStandardItemModel>
#include "signal.h"

inline QSortFilterProxyModel *QSortFilterProxyModel_new(QObject *parent) {
    return new QSortFilterProxyModel(parent);
}
inline void QSortFilterProxyModel_delete(QSortFilterProxyModel *m) { delete m; }
inline void QSortFilterProxyModel_setSourceModel(QSortFilterProxyModel *m, QStandardItemModel *src) {
    m->setSourceModel(src);
}
inline void QSortFilterProxyModel_setFilterRole(QSortFilterProxyModel *m, int role) {
    m->setFilterRole(role);
}
inline void QSortFilterProxyModel_setFilterFixedString(QSortFilterProxyModel *m, const std::string &text) {
    m->setFilterFixedString(QString::fromStdString(text));
}
inline void QSortFilterProxyModel_setFilterCaseSensitivity(QSortFilterProxyModel *m, int cs) {
    m->setFilterCaseSensitivity(static_cast<Qt::CaseSensitivity>(cs));
}
inline void QSortFilterProxyModel_setSortRole(QSortFilterProxyModel *m, int role) {
    m->setSortRole(role);
}
inline void QSortFilterProxyModel_sort(QSortFilterProxyModel *m, int col, int order) {
    m->sort(col, static_cast<Qt::SortOrder>(order));
}
