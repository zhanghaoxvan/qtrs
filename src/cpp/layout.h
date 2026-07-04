// src/cpp/layout.h — QVBoxLayout / QHBoxLayout / QGridLayout + toQWidget upcasts
#pragma once

#include <QGridLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QLineEdit>
#include <QPushButton>
#include <QCheckBox>
#include <QComboBox>
#include <QSlider>
#include <QTextEdit>
#include <QVBoxLayout>
#include <QWidget>

// --- QVBoxLayout ---
inline QVBoxLayout *QVBoxLayout_new(QWidget *parent) {
    return new QVBoxLayout(parent);
}
inline void QVBoxLayout_addWidget(QVBoxLayout *layout, QWidget *widget) {
    layout->addWidget(widget);
}
inline void QVBoxLayout_delete(QVBoxLayout *layout) { delete layout; }

// --- QHBoxLayout ---
inline QHBoxLayout *QHBoxLayout_new(QWidget *parent) {
    return new QHBoxLayout(parent);
}
inline void QHBoxLayout_addWidget(QHBoxLayout *layout, QWidget *widget) {
    layout->addWidget(widget);
}
inline void QHBoxLayout_delete(QHBoxLayout *layout) { delete layout; }

// --- QGridLayout ---
inline QGridLayout *QGridLayout_new(QWidget *parent) {
    return new QGridLayout(parent);
}
inline void QGridLayout_addWidget(QGridLayout *layout, QWidget *widget,
                                   int row, int col, int rowSpan,
                                   int colSpan) {
    layout->addWidget(widget, row, col, rowSpan, colSpan);
}
inline void QGridLayout_delete(QGridLayout *layout) { delete layout; }

// --- toQWidget upcasts (cxx doesn't understand C++ inheritance) ---
inline QWidget *toQWidget_QWidget(QWidget *w) { return w; }
inline QWidget *toQWidget_QPushButton(QPushButton *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QLabel(QLabel *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QLineEdit(QLineEdit *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QCheckBox(QCheckBox *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QComboBox(QComboBox *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QTextEdit(QTextEdit *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QSlider(QSlider *w) {
    return static_cast<QWidget *>(w);
}
