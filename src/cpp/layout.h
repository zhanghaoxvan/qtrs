// src/cpp/layout.h — QVBoxLayout / QHBoxLayout / QGridLayout + toQWidget upcasts
#pragma once

#include "qwidget.h"
#include <QtWidgets/QGridLayout>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QLabel>
#include <QtWidgets/QLineEdit>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QCheckBox>
#include <QtWidgets/QComboBox>
#include <QtWidgets/QSlider>
#include <QtWidgets/QTextEdit>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QWidget>
#include <QtWidgets/QProgressBar>
#include <QtWidgets/QRadioButton>
#include <QtWidgets/QGroupBox>
#include <QtWidgets/QSpinBox>
#include <QtWidgets/QMenu>
#include <QtWidgets/QMenuBar>

// --- QVBoxLayout ---
inline QVBoxLayout *QVBoxLayout_new(QWidget *parent) {
    return new QVBoxLayout(parent);
}
inline void QVBoxLayout_addWidget(QVBoxLayout *layout, QWidget *widget) {
    layout->addWidget(widget);
}
inline void QVBoxLayout_delete(QVBoxLayout *layout) { delete layout; }
inline void QVBoxLayout_setSpacing(QVBoxLayout *l, int s) { l->setSpacing(s); }
inline void QVBoxLayout_setContentsMargins(QVBoxLayout *l, int lft, int top,
                                            int rgt, int bot) {
    l->setContentsMargins(lft, top, rgt, bot);
}

// --- QHBoxLayout ---
inline QHBoxLayout *QHBoxLayout_new(QWidget *parent) {
    return new QHBoxLayout(parent);
}
inline void QHBoxLayout_addWidget(QHBoxLayout *layout, QWidget *widget) {
    layout->addWidget(widget);
}
inline void QHBoxLayout_delete(QHBoxLayout *layout) { delete layout; }
inline void QHBoxLayout_setSpacing(QHBoxLayout *l, int s) { l->setSpacing(s); }
inline void QHBoxLayout_setContentsMargins(QHBoxLayout *l, int lft, int top,
                                            int rgt, int bot) {
    l->setContentsMargins(lft, top, rgt, bot);
}

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
inline QWidget *toQWidget_QProgressBar(QProgressBar *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QRadioButton(QRadioButton *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QGroupBox(QGroupBox *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QTabWidget(QTabWidget *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QSpinBox(QSpinBox *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QMenu(QMenu *w) {
    return static_cast<QWidget *>(w);
}
inline QWidget *toQWidget_QMenuBar(QMenuBar *w) {
    return static_cast<QWidget *>(w);
}

// --- findChild by objectName (for widgets loaded from .ui files) ---
inline QPushButton *QWidget_findPushButton(QWidget *parent, const std::string &name) {
    return parent->findChild<QPushButton *>(QString::fromStdString(name));
}
inline QLineEdit *QWidget_findLineEdit(QWidget *parent, const std::string &name) {
    return parent->findChild<QLineEdit *>(QString::fromStdString(name));
}
inline QCheckBox *QWidget_findCheckBox(QWidget *parent, const std::string &name) {
    return parent->findChild<QCheckBox *>(QString::fromStdString(name));
}
inline QLabel *QWidget_findLabel(QWidget *parent, const std::string &name) {
    return parent->findChild<QLabel *>(QString::fromStdString(name));
}
inline QWidget *QWidget_findWidget(QWidget *parent, const std::string &name) {
    return parent->findChild<QWidget *>(QString::fromStdString(name));
}
inline QComboBox *QWidget_findComboBox(QWidget *parent, const std::string &name) {
    return parent->findChild<QComboBox *>(QString::fromStdString(name));
}
inline QSlider *QWidget_findSlider(QWidget *parent, const std::string &name) {
    return parent->findChild<QSlider *>(QString::fromStdString(name));
}
inline QTextEdit *QWidget_findTextEdit(QWidget *parent, const std::string &name) {
    return parent->findChild<QTextEdit *>(QString::fromStdString(name));
}
inline QProgressBar *QWidget_findProgressBar(QWidget *parent, const std::string &name) {
    return parent->findChild<QProgressBar *>(QString::fromStdString(name));
}
inline QRadioButton *QWidget_findRadioButton(QWidget *parent, const std::string &name) {
    return parent->findChild<QRadioButton *>(QString::fromStdString(name));
}
inline QGroupBox *QWidget_findGroupBox(QWidget *parent, const std::string &name) {
    return parent->findChild<QGroupBox *>(QString::fromStdString(name));
}
inline QTabWidget *QWidget_findTabWidget(QWidget *parent, const std::string &name) {
    return parent->findChild<QTabWidget *>(QString::fromStdString(name));
}
inline QSpinBox *QWidget_findSpinBox(QWidget *parent, const std::string &name) {
    return parent->findChild<QSpinBox *>(QString::fromStdString(name));
}
