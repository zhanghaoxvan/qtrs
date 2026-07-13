// src/cpp/splitter.h — QSplitter
#pragma once

#include <QtWidgets/QSplitter>
#include <string>
#include <vector>
#include <cstdint>

#include "rust/cxx.h"
#include "signal.h"

// Constructor / Destructor
inline QSplitter *QSplitter_new(int orientation, QWidget *parent = nullptr) {
    return new QSplitter(static_cast<Qt::Orientation>(orientation), parent);
}

inline void QSplitter_delete(QSplitter *w) { delete w; }

// Widget management
inline void QSplitter_addWidget(QSplitter *w, QWidget *widget) {
    w->addWidget(widget);
}

inline void QSplitter_insertWidget(QSplitter *w, int index, QWidget *widget) {
    w->insertWidget(index, widget);
}

inline void QSplitter_setStretchFactor(QSplitter *w, int index, int stretch) {
    w->setStretchFactor(index, stretch);
}

// Sizes
inline void QSplitter_setSizes(QSplitter *w, rust::Vec<int32_t> sizes) {
    QList<int> list;
    for (int s : sizes) {
        list.append(s);
    }
    w->setSizes(list);
}

inline rust::Vec<int32_t> QSplitter_sizes(QSplitter *w) {
    QList<int> s = w->sizes();
    rust::Vec<int32_t> result;
    for (int val : s) {
        result.push_back(val);
    }
    return result;
}

// Orientation
inline void QSplitter_setOrientation(QSplitter *w, int orientation) {
    w->setOrientation(static_cast<Qt::Orientation>(orientation));
}

// Count
inline int QSplitter_count(QSplitter *w) {
    return w->count();
}

// Handle
inline void QSplitter_setHandleWidth(QSplitter *w, int width) {
    w->setHandleWidth(width);
}

// Collapsible
inline void QSplitter_setChildrenCollapsible(QSplitter *w, bool collapsible) {
    w->setChildrenCollapsible(collapsible);
}

// Upcast
inline QWidget *toQWidget_QSplitter(QSplitter *w) {
    return static_cast<QWidget *>(w);
}
