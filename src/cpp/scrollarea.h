// src/cpp/scrollarea.h — QScrollArea
#pragma once

#include <QtWidgets/QScrollArea>
#include <QtCore/QString>
#include <cstdint>

// Constructor / Destructor
inline QScrollArea *QScrollArea_new(QWidget *parent = nullptr) {
    return new QScrollArea(parent);
}

inline void QScrollArea_delete(QScrollArea *w) { delete w; }

// Widget management
inline void QScrollArea_setWidget(QScrollArea *w, QWidget *widget) {
    w->setWidget(widget);
}

inline QWidget *QScrollArea_takeWidget(QScrollArea *w) {
    return w->takeWidget();
}

inline void QScrollArea_setWidgetResizable(QScrollArea *w, bool resizable) {
    w->setWidgetResizable(resizable);
}

// Scroll bar policies
inline void QScrollArea_setHorizontalScrollBarPolicy(QScrollArea *w, int policy) {
    w->setHorizontalScrollBarPolicy(static_cast<Qt::ScrollBarPolicy>(policy));
}

inline void QScrollArea_setVerticalScrollBarPolicy(QScrollArea *w, int policy) {
    w->setVerticalScrollBarPolicy(static_cast<Qt::ScrollBarPolicy>(policy));
}

// Scrolling
inline void QScrollArea_ensureVisible(QScrollArea *w, int x, int y) {
    w->ensureVisible(x, y);
}

inline void QScrollArea_ensureWidgetVisible(QScrollArea *w, QWidget *widget) {
    w->ensureWidgetVisible(widget);
}

// Upcast
inline QWidget *toQWidget_QScrollArea(QScrollArea *w) {
    return static_cast<QWidget *>(w);
}
