// src/cpp/uiloader.h — .ui file loading
#pragma once

#ifdef QTRS_HAS_UI

#include <QtCore/QFile>
#include <QtCore/QString>
#include <QtUiTools/QUiLoader>
#include <QtWidgets/QWidget>
#include <string>

inline QUiLoader *QUiLoader_new() { return new QUiLoader(); }

inline QWidget *QUiLoader_load(QUiLoader *loader,
                                const std::string &ui_path,
                                QWidget *parent) {
    QFile file(QString::fromStdString(ui_path));
    if (!file.open(QIODevice::ReadOnly)) return nullptr;
    QWidget *widget = loader->load(&file, parent);
    file.close();
    return widget;
}

inline void QUiLoader_delete(QUiLoader *loader) { delete loader; }

#endif // QTRS_HAS_UI
