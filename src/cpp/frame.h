// src/cpp/frame.h — QFrame
#pragma once

#include <QtWidgets/QFrame>
#include <QtWidgets/QWidget>
#include <string>

// Constructor / Destructor
inline QFrame *QFrame_new(QWidget *parent) {
    return new QFrame(parent);
}

inline void QFrame_delete(QFrame *frame) { delete frame; }

// Properties
inline void QFrame_setFrameShape(QFrame *frame, int shape) {
    frame->setFrameShape(static_cast<QFrame::Shape>(shape));
}

inline void QFrame_setFrameShadow(QFrame *frame, int shadow) {
    frame->setFrameShadow(static_cast<QFrame::Shadow>(shadow));
}

inline void QFrame_setLineWidth(QFrame *frame, int width) {
    frame->setLineWidth(width);
}

inline void QFrame_setMidLineWidth(QFrame *frame, int width) {
    frame->setMidLineWidth(width);
}

inline void QFrame_setFrameStyle(QFrame *frame, int style) {
    frame->setFrameStyle(style);
}

// Upcast
inline QWidget *toQWidget_QFrame(QFrame *frame) {
    return static_cast<QWidget *>(frame);
}
