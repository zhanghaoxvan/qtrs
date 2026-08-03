// src/cpp/clipboard.h — QClipboard
#pragma once

#include <QtGui/QClipboard>
#include <QtGui/QGuiApplication>
#include <QtCore/QString>
#include <string>
#include "rust/cxx.h"

inline void QClipboard_setText(const std::string &text) {
    QGuiApplication::clipboard()->setText(QString::fromStdString(text));
}
inline rust::String QClipboard_text() {
    return QGuiApplication::clipboard()->text().toStdString();
}
inline void QClipboard_clear() { QGuiApplication::clipboard()->clear(); }
