// src/cpp/treewidget.h — QTreeWidget
#pragma once

#include <QtWidgets/QTreeWidget>
#include <QtCore/QString>
#include <string>
#include <vector>
#include <cstdint>

#include "rust/cxx.h"
#include "signal.h"

// Constructor / Destructor
inline QTreeWidget *QTreeWidget_new(QWidget *parent = nullptr) {
    return new QTreeWidget(parent);
}

inline void QTreeWidget_delete(QTreeWidget *w) { delete w; }

// Item management
inline void QTreeWidget_addTopLevelItem(QTreeWidget *w, const std::string &text) {
    w->addTopLevelItem(new QTreeWidgetItem(QStringList(QString::fromStdString(text))));
}

inline void QTreeWidget_clear(QTreeWidget *w) {
    w->clear();
}

inline rust::String QTreeWidget_currentItemText(QTreeWidget *w) {
    QTreeWidgetItem *item = w->currentItem();
    return item ? item->text(0).toStdString() : rust::String();
}

// Header
inline void QTreeWidget_setHeaderLabel(QTreeWidget *w, const std::string &text) {
    w->setHeaderLabel(QString::fromStdString(text));
}

inline void QTreeWidget_setHeaderLabels(QTreeWidget *w, rust::Vec<rust::String> labels) {
    QStringList list;
    for (auto &label : labels) {
        list << QString::fromStdString(std::string(label));
    }
    w->setHeaderLabels(list);
}

// Expand/Collapse
inline void QTreeWidget_expandAll(QTreeWidget *w) {
    w->expandAll();
}

inline void QTreeWidget_collapseAll(QTreeWidget *w) {
    w->collapseAll();
}

inline void QTreeWidget_expandItem(QTreeWidget *w, const std::string &text) {
    QList<QTreeWidgetItem *> items = w->findItems(QString::fromStdString(text), Qt::MatchExactly | Qt::MatchRecursive);
    if (!items.isEmpty()) {
        w->expandItem(items.first());
    }
}

// Current item
inline void QTreeWidget_setCurrentItem(QTreeWidget *w, const std::string &text) {
    QList<QTreeWidgetItem *> items = w->findItems(QString::fromStdString(text), Qt::MatchExactly | Qt::MatchRecursive);
    if (!items.isEmpty()) {
        w->setCurrentItem(items.first());
    }
}

// Count
inline int QTreeWidget_topLevelItemCount(QTreeWidget *w) {
    return w->topLevelItemCount();
}

// Signals
inline void QTreeWidget_onItemClicked(QTreeWidget *w, uint64_t ctx) {
    QObject::connect(w, &QTreeWidget::itemClicked, [ctx](QTreeWidgetItem *item, int /*column*/) {
        if (g_hasStringTrampoline) {
            QString text = item ? item->text(0) : QString();
            rust::String rustText = rust::String(text.toStdString());
            g_stringTrampoline(ctx, std::move(rustText));
        }
    });
}

inline void QTreeWidget_onItemDoubleClicked(QTreeWidget *w, uint64_t ctx) {
    QObject::connect(w, &QTreeWidget::itemDoubleClicked, [ctx](QTreeWidgetItem *item, int /*column*/) {
        if (g_hasStringTrampoline) {
            QString text = item ? item->text(0) : QString();
            rust::String rustText = rust::String(text.toStdString());
            g_stringTrampoline(ctx, std::move(rustText));
        }
    });
}

inline void QTreeWidget_onItemExpanded(QTreeWidget *w, uint64_t ctx) {
    QObject::connect(w, &QTreeWidget::itemExpanded, [ctx](QTreeWidgetItem *item) {
        if (g_hasStringTrampoline) {
            QString text = item ? item->text(0) : QString();
            rust::String rustText = rust::String(text.toStdString());
            g_stringTrampoline(ctx, std::move(rustText));
        }
    });
}

inline void QTreeWidget_onItemCollapsed(QTreeWidget *w, uint64_t ctx) {
    QObject::connect(w, &QTreeWidget::itemCollapsed, [ctx](QTreeWidgetItem *item) {
        if (g_hasStringTrampoline) {
            QString text = item ? item->text(0) : QString();
            rust::String rustText = rust::String(text.toStdString());
            g_stringTrampoline(ctx, std::move(rustText));
        }
    });
}

inline void QTreeWidget_onCurrentItemChanged(QTreeWidget *w, uint64_t ctx) {
    QObject::connect(w, &QTreeWidget::currentItemChanged, [ctx](QTreeWidgetItem *current, QTreeWidgetItem * /*previous*/) {
        if (g_hasStringTrampoline) {
            QString text = current ? current->text(0) : QString();
            rust::String rustText = rust::String(text.toStdString());
            g_stringTrampoline(ctx, std::move(rustText));
        }
    });
}

// Upcast
inline QWidget *toQWidget_QTreeWidget(QTreeWidget *w) {
    return static_cast<QWidget *>(w);
}
