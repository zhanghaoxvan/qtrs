// src/cpp/cursor.h — QCursor shapes
#pragma once

#include <QtGui/QCursor>
#include <QtWidgets/QWidget>

// Used to set cursor shape on any QWidget
inline void QWidget_setCursor(QWidget *w, int shape) {
    w->setCursor(QCursor(static_cast<Qt::CursorShape>(shape)));
}
inline void QWidget_unsetCursor(QWidget *w) { w->unsetCursor(); }
