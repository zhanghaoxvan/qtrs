// src/cpp/stackedwidget.h — QStackedWidget
#pragma once

#include <QtWidgets/QStackedWidget>
#include <string>
#include <cstdint>

#include "rust/cxx.h"
#include "signal.h"

// Constructor / Destructor
inline QStackedWidget *QStackedWidget_new(QWidget *parent = nullptr) {
    return new QStackedWidget(parent);
}

inline void QStackedWidget_delete(QStackedWidget *w) { delete w; }

// Page management
inline int QStackedWidget_addWidget(QStackedWidget *w, QWidget *page) {
    return w->addWidget(page);
}

inline int QStackedWidget_insertWidget(QStackedWidget *w, int index, QWidget *page) {
    return w->insertWidget(index, page);
}

inline void QStackedWidget_removeWidget(QStackedWidget *w, QWidget *page) {
    w->removeWidget(page);
}

// Current page
inline void QStackedWidget_setCurrentIndex(QStackedWidget *w, int index) {
    w->setCurrentIndex(index);
}

inline int QStackedWidget_currentIndex(QStackedWidget *w) {
    return w->currentIndex();
}

// Count
inline int QStackedWidget_count(QStackedWidget *w) {
    return w->count();
}

// Get widget at index
inline QWidget *QStackedWidget_widget(QStackedWidget *w, int index) {
    return w->widget(index);
}

// Signals
inline void QStackedWidget_onCurrentChanged(QStackedWidget *w, uint64_t ctx) {
    QObject::connect(w, &QStackedWidget::currentChanged, [ctx](int index) {
        if (g_hasIntTrampoline) {
            g_intTrampoline(ctx, index);
        }
    });
}

// Upcast
inline QWidget *toQWidget_QStackedWidget(QStackedWidget *w) {
    return static_cast<QWidget *>(w);
}
