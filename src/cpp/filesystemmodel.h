// src/cpp/filesystemmodel.h — QFileSystemModel
#pragma once

#include <QtGui/QFileSystemModel>
#include <QtCore/QString>
#include <string>
#include "rust/cxx.h"

inline QFileSystemModel *QFileSystemModel_new(QObject *parent) {
    return new QFileSystemModel(parent);
}
inline void QFileSystemModel_delete(QFileSystemModel *m) { delete m; }
inline void QFileSystemModel_setRootPath(QFileSystemModel *m, const std::string &path) {
    m->setRootPath(QString::fromStdString(path));
}
inline rust::String QFileSystemModel_rootPath(QFileSystemModel *m) {
    return m->rootPath().toStdString();
}
inline rust::String QFileSystemModel_filePath(QFileSystemModel *m, int row, int col) {
    return m->filePath(m->index(row, col)).toStdString();
}
inline bool QFileSystemModel_isDir(QFileSystemModel *m, int row, int col) {
    return m->isDir(m->index(row, col));
}
