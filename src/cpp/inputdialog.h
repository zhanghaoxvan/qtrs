// src/cpp/inputdialog.h — QInputDialog static convenience functions
#pragma once

#include <QtWidgets/QInputDialog>
#include <QtCore/QString>
#include <QtCore/QStringList>
#include <string>
#include <vector>
#include <cstdint>

#include "rust/cxx.h"

// Static convenience: getText
// Returns empty string if cancelled.
inline rust::String QInputDialog_getText(QWidget *parent,
                                         const std::string &title,
                                         const std::string &label,
                                         const std::string &text) {
    bool ok = false;
    QString result = QInputDialog::getText(
        parent,
        QString::fromStdString(title),
        QString::fromStdString(label),
        QLineEdit::Normal,
        QString::fromStdString(text),
        &ok);
    return ok ? rust::String(result.toStdString()) : rust::String();
}

// Static convenience: getInt
// Returns 0 if cancelled.
inline int QInputDialog_getInt(QWidget *parent,
                                const std::string &title,
                                const std::string &label,
                                int value,
                                int min,
                                int max,
                                int step) {
    bool ok = false;
    int result = QInputDialog::getInt(
        parent,
        QString::fromStdString(title),
        QString::fromStdString(label),
        value,
        min,
        max,
        step,
        &ok);
    return ok ? result : 0;
}

// Static convenience: getDouble
inline double QInputDialog_getDouble(QWidget *parent,
                                      const std::string &title,
                                      const std::string &label,
                                      double value,
                                      double min,
                                      double max,
                                      int decimals) {
    bool ok = false;
    double result = QInputDialog::getDouble(
        parent,
        QString::fromStdString(title),
        QString::fromStdString(label),
        value,
        min,
        max,
        decimals,
        &ok);
    return ok ? result : 0.0;
}

// Static convenience: getItem
// Returns empty string if cancelled.
inline rust::String QInputDialog_getItem(QWidget *parent,
                                        const std::string &title,
                                        const std::string &label,
                                        rust::Vec<rust::String> items,
                                        int current,
                                        bool editable) {
    QStringList qitems;
    for (auto &item : items) {
        qitems << QString::fromStdString(std::string(item));
    }
    bool ok = false;
    QString result = QInputDialog::getItem(
        parent,
        QString::fromStdString(title),
        QString::fromStdString(label),
        qitems,
        current,
        editable,
        &ok);
    return ok ? rust::String(result.toStdString()) : rust::String();
}
