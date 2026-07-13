// src/cpp/tablewidget.h — QTableWidget
#pragma once

#include <QtWidgets/QTableWidget>
#include <QtCore/QString>
#include <string>
#include <vector>
#include <cstdint>

#include "rust/cxx.h"
#include "signal.h"

// Constructor / Destructor
inline QTableWidget *QTableWidget_new(int rows, int cols, QWidget *parent = nullptr) {
    return new QTableWidget(rows, cols, parent);
}

inline void QTableWidget_delete(QTableWidget *w) { delete w; }

// Row/Column count
inline void QTableWidget_setRowCount(QTableWidget *w, int rows) {
    w->setRowCount(rows);
}

inline void QTableWidget_setColumnCount(QTableWidget *w, int cols) {
    w->setColumnCount(cols);
}

// Item management
inline void QTableWidget_setItem(QTableWidget *w, int row, int col, const std::string &text) {
    w->setItem(row, col, new QTableWidgetItem(QString::fromStdString(text)));
}

inline rust::String QTableWidget_itemText(QTableWidget *w, int row, int col) {
    QTableWidgetItem *item = w->item(row, col);
    return item ? item->text().toStdString() : rust::String();
}

// Header labels
inline void QTableWidget_setHorizontalHeaderLabels(QTableWidget *w, rust::Vec<rust::String> labels) {
    QStringList list;
    for (const auto &label : labels) {
        list << QString::fromStdString(std::string(label));
    }
    w->setHorizontalHeaderLabels(list);
}

inline void QTableWidget_setVerticalHeaderLabels(QTableWidget *w, rust::Vec<rust::String> labels) {
    QStringList list;
    for (const auto &label : labels) {
        list << QString::fromStdString(std::string(label));
    }
    w->setVerticalHeaderLabels(list);
}

// Current cell
inline void QTableWidget_setCurrentCell(QTableWidget *w, int row, int col) {
    w->setCurrentCell(row, col);
}

inline int QTableWidget_currentRow(QTableWidget *w) {
    return w->currentRow();
}

inline int QTableWidget_currentColumn(QTableWidget *w) {
    return w->currentColumn();
}

// Selected rows
inline rust::Vec<int32_t> QTableWidget_selectedRows(QTableWidget *w) {
    rust::Vec<int32_t> rows;
    auto selected = w->selectionModel()->selectedRows();
    for (const auto &index : selected) {
        rows.push_back(index.row());
    }
    return rows;
}

// Clear
inline void QTableWidget_clear(QTableWidget *w) {
    w->clear();
}

inline void QTableWidget_clearContents(QTableWidget *w) {
    w->clearContents();
}

// Selection
inline void QTableWidget_setSelectionMode(QTableWidget *w, int mode) {
    w->setSelectionMode(static_cast<QAbstractItemView::SelectionMode>(mode));
}

inline void QTableWidget_setSelectionBehavior(QTableWidget *w, int behavior) {
    w->setSelectionBehavior(static_cast<QAbstractItemView::SelectionBehavior>(behavior));
}

// Row operations
inline void QTableWidget_removeRow(QTableWidget *w, int row) {
    w->removeRow(row);
}

inline void QTableWidget_insertRow(QTableWidget *w, int row) {
    w->insertRow(row);
}

// Column/row sizing
inline void QTableWidget_setColumnWidth(QTableWidget *w, int col, int width) {
    w->setColumnWidth(col, width);
}

inline void QTableWidget_setRowHeight(QTableWidget *w, int row, int height) {
    w->setRowHeight(row, height);
}

// Signals
inline void QTableWidget_onCellClicked(QTableWidget *w, uint64_t ctx) {
    QObject::connect(w, &QTableWidget::cellClicked, [ctx](int /*row*/, int /*col*/) {
        if (g_hasVoidTrampoline) {
            g_voidTrampoline(ctx);
        }
    });
}

inline void QTableWidget_onCellDoubleClicked(QTableWidget *w, uint64_t ctx) {
    QObject::connect(w, &QTableWidget::cellDoubleClicked, [ctx](int /*row*/, int /*col*/) {
        if (g_hasVoidTrampoline) {
            g_voidTrampoline(ctx);
        }
    });
}

inline void QTableWidget_onCurrentCellChanged(QTableWidget *w, uint64_t ctx) {
    QObject::connect(w, &QTableWidget::currentCellChanged, [ctx](int /*row*/, int /*col*/, int /*prevRow*/, int /*prevCol*/) {
        if (g_hasVoidTrampoline) {
            g_voidTrampoline(ctx);
        }
    });
}

// Upcast
inline QWidget *toQWidget_QTableWidget(QTableWidget *w) {
    return static_cast<QWidget *>(w);
}
