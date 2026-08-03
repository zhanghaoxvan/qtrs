// src/cpp/desktopservices.h — QDesktopServices
#pragma once

#include <QtGui/QDesktopServices>
#include <QtCore/QUrl>
#include <QtCore/QString>
#include <string>

inline bool QDesktopServices_openUrl(const std::string &url) {
    return QDesktopServices::openUrl(QUrl(QString::fromStdString(url)));
}
