// src/cpp/listwidget.h — QListWidget
#pragma once

#include <QtWidgets/QListWidget>
#include <QtCore/QString>
#include <string>
#include <vector>

#include "rust/cxx.h"
#include "signal.h"

// Constructor / Destructor
inline QListWidget *QListWidget_new(QWidget *parent = nullptr) {
    return new QListWidget(parent);
}

inline void QListWidget_delete(QListWidget *w) { delete w; }

// Item management
inline void QListWidget_addItem(QListWidget *w, const std::string &text) {
    w->addItem(QString::fromStdString(text));
}

inline void QListWidget_addItems(QListWidget *w, const std::vector<std::string> &items) {
    for (const auto &item : items) {
        w->addItem(QString::fromStdString(item));
    }
}

inline void QListWidget_insertItem(QListWidget *w, int row, const std::string &text) {
    w->insertItem(row, QString::fromStdString(text));
}

inline void QListWidget_clear(QListWidget *w) {
    w->clear();
}

inline void QListWidget_removeItem(QListWidget *w, int row) {
    delete w->takeItem(row);
}

// Getters
inline int QListWidget_count(QListWidget *w) {
    return w->count();
}

inline rust::String QListWidget_itemText(QListWidget *w, int row) {
    QListWidgetItem *item = w->item(row);
    return item ? item->text().toStdString() : rust::String();
}

inline int QListWidget_currentRow(QListWidget *w) {
    return w->currentRow();
}

inline rust::String QListWidget_currentText(QListWidget *w) {
    QListWidgetItem *item = w->currentItem();
    return item ? item->text().toStdString() : rust::String();
}

// Selection
inline void QListWidget_setCurrentRow(QListWidget *w, int row) {
    w->setCurrentRow(row);
}

inline void QListWidget_setSelectionMode(QListWidget *w, int mode) {
    w->setSelectionMode(static_cast<QAbstractItemView::SelectionMode>(mode));
}

// Signals
inline void QListWidget_onItemClicked(QListWidget *w, uint64_t ctx) {
    QObject::connect(w, &QListWidget::itemClicked, [ctx](QListWidgetItem *item) {
        if (g_hasStringTrampoline) {
            QString text = item ? item->text() : QString();
            rust::String rustText = rust::String(text.toStdString());
            g_stringTrampoline(ctx, std::move(rustText));
        }
    });
}

inline void QListWidget_onItemDoubleClicked(QListWidget *w, uint64_t ctx) {
    QObject::connect(w, &QListWidget::itemDoubleClicked, [ctx](QListWidgetItem *item) {
        if (g_hasStringTrampoline) {
            QString text = item ? item->text() : QString();
            rust::String rustText = rust::String(text.toStdString());
            g_stringTrampoline(ctx, std::move(rustText));
        }
    });
}

inline void QListWidget_onCurrentItemChanged(QListWidget *w, uint64_t ctx) {
    QObject::connect(w, &QListWidget::currentItemChanged, [ctx](QListWidgetItem *current, QListWidgetItem * /*previous*/) {
        if (g_hasStringTrampoline) {
            QString text = current ? current->text() : QString();
            rust::String rustText = rust::String(text.toStdString());
            g_stringTrampoline(ctx, std::move(rustText));
        }
    });
}

// Upcast
inline QWidget *toQWidget_QListWidget(QListWidget *w) {
    return static_cast<QWidget *>(w);
}
