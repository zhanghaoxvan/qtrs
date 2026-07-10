// src/cpp/filedialog.h — QFileDialog wrapper
#pragma once

#include <QtWidgets/QFileDialog>
#include <QtCore/QString>
#include <QtWidgets/QWidget>
#include <string>
#include <vector>
#include "rust/cxx.h"

// ============================================================
// Static convenience functions
// ============================================================

/// Open file dialog — returns selected file path or empty string
inline rust::String QFileDialog_getOpenFileName(
    QWidget* parent,
    const std::string& caption,
    const std::string& dir,
    const std::string& filter
) {
    QString result = QFileDialog::getOpenFileName(
        parent,
        QString::fromStdString(caption),
        QString::fromStdString(dir),
        QString::fromStdString(filter)
    );
    return rust::String(result.toStdString());
}

/// Open files dialog — returns list of selected file paths
inline rust::Vec<rust::String> QFileDialog_getOpenFileNames(
    QWidget* parent,
    const std::string& caption,
    const std::string& dir,
    const std::string& filter
) {
    QStringList result = QFileDialog::getOpenFileNames(
        parent,
        QString::fromStdString(caption),
        QString::fromStdString(dir),
        QString::fromStdString(filter)
    );
    rust::Vec<rust::String> paths;
    for (const QString& s : result) {
        paths.push_back(rust::String(s.toStdString()));
    }
    return paths;
}

/// Save file dialog — returns selected file path or empty string
inline rust::String QFileDialog_getSaveFileName(
    QWidget* parent,
    const std::string& caption,
    const std::string& dir,
    const std::string& filter
) {
    QString result = QFileDialog::getSaveFileName(
        parent,
        QString::fromStdString(caption),
        QString::fromStdString(dir),
        QString::fromStdString(filter)
    );
    return rust::String(result.toStdString());
}

/// Existing directory dialog — returns selected directory path or empty string
inline rust::String QFileDialog_getExistingDirectory(
    QWidget* parent,
    const std::string& caption,
    const std::string& dir
) {
    QString result = QFileDialog::getExistingDirectory(
        parent,
        QString::fromStdString(caption),
        QString::fromStdString(dir)
    );
    return rust::String(result.toStdString());
}