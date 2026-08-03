// src/cpp/stringlistmodel.h — QStringListModel
#pragma once

#include <QtCore/QStringListModel>
#include <QtCore/QString>
#include <string>
#include "rust/cxx.h"
#include "signal.h"

inline QStringListModel *QStringListModel_new(QObject *parent) {
    return new QStringListModel(parent);
}
inline void QStringListModel_delete(QStringListModel *m) { delete m; }
inline void QStringListModel_setStringList(QStringListModel *m, rust::Vec<rust::String> list) {
    QStringList qlist;
    for (const auto &s : list) qlist.append(QString::fromStdString(std::string(s)));
    m->setStringList(qlist);
}
inline rust::String QStringListModel_data(QStringListModel *m, int row) {
    return m->stringList().at(row).toStdString();
}
inline int QStringListModel_rowCount(QStringListModel *m) { return m->rowCount(); }
