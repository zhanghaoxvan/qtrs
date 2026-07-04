// src/cpp/app.h — QApplication singleton
#pragma once

#include <QApplication>
#include <QIcon>
#include <QString>
#include <string>

inline QApplication *QApplication_new() {
    static QApplication *app = nullptr;
    static int argc = 0;
    static char *argv[] = {nullptr};
    if (!app) {
        app = new QApplication(argc, argv);
    }
    return app;
}

inline int QApplication_exec(QApplication *app) { return app->exec(); }

inline void QApplication_setWindowIcon(QApplication *app,
                                        const std::string &icon_path) {
    QIcon icon(QString::fromStdString(icon_path));
    if (!icon.isNull()) {
        app->setWindowIcon(icon);
    }
}

inline void QApplication_setDesktopFileName(QApplication *app,
                                             const std::string &name) {
    app->setDesktopFileName(QString::fromStdString(name));
}
