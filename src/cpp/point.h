// src/cpp/point.h — QPoint wrapper

#pragma once

#include <QtCore/QPoint>

inline QPoint *QPoint_new(int x, int y) {
    return new QPoint(x, y);
}

inline void QPoint_delete(QPoint *p) {
    delete p;
}