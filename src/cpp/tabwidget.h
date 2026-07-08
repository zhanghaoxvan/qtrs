// src/cpp/tabwidget.h — QTabWidget
#pragma once

#include <QObject>
#include <QTabWidget>
#include <QWidget>
#include <QString>
#include <string>

#include "signal.h"

/// Create a new QTabWidget with optional parent.
inline QTabWidget *QTabWidget_new(QWidget *parent) {
    return new QTabWidget(parent);
}

/// Add a tab with the given page widget and label.
inline void QTabWidget_addTab(QTabWidget *tw, QWidget *page, const std::string &label) {
    tw->addTab(page, QString::fromStdString(label));
}

/// Get the currently selected tab index.
inline int QTabWidget_currentIndex(QTabWidget *tw) {
    return tw->currentIndex();
}

/// Set the currently selected tab by index.
inline void QTabWidget_setCurrentIndex(QTabWidget *tw, int index) {
    tw->setCurrentIndex(index);
}

/// Delete the tab widget.
inline void QTabWidget_delete(QTabWidget *tw) {
    delete tw;
}

/// Connect the currentChanged signal to a Rust callback.
inline void QTabWidget_onCurrentChanged(QTabWidget *tw, uint64_t ctx) {
    QObject::connect(tw, &QTabWidget::currentChanged,
                     [ctx](int index) {
                         if (g_hasIntTrampoline) g_intTrampoline(ctx, index);
                     });
}
