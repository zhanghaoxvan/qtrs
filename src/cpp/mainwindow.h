// src/cpp/mainwindow.h — QMainWindow
#pragma once

#include <QtCore/QObject>
#include <QtCore/QString>
#include <QtWidgets/QMainWindow>
#include <QtWidgets/QMenuBar>
#include <QtWidgets/QStatusBar>
#include <QtWidgets/QTabWidget>
#include <QtWidgets/QToolBar>
#include <QtWidgets/QWidget>
#include <string>

#include "signal.h"

// Constructor / Destructor
inline QMainWindow *QMainWindow_new(QWidget *parent) {
    return new QMainWindow(parent);
}

inline void QMainWindow_delete(QMainWindow *w) { delete w; }

// Child widgets
inline void QMainWindow_setMenuBar(QMainWindow *w, QMenuBar *menuBar) {
    w->setMenuBar(menuBar);
}

inline void QMainWindow_setCentralWidget(QMainWindow *w, QWidget *central) {
    w->setCentralWidget(central);
}

inline void QMainWindow_setStatusBar(QMainWindow *w, QStatusBar *statusBar) {
    w->setStatusBar(statusBar);
}

// Toolbars
inline QToolBar *QMainWindow_addToolBar(QMainWindow *w, const std::string &title) {
    return w->addToolBar(QString::fromStdString(title));
}

inline void QMainWindow_addToolBarBreak(QMainWindow *w) {
    w->addToolBarBreak();
}

// Window management
inline void QMainWindow_setWindowTitle(QMainWindow *w, const std::string &title) {
    w->setWindowTitle(QString::fromStdString(title));
}

inline void QMainWindow_resize(QMainWindow *w, int width, int height) {
    w->resize(width, height);
}

inline void QMainWindow_show(QMainWindow *w) { w->show(); }

inline void QMainWindow_hide(QMainWindow *w) { w->hide(); }

// Dock options
inline void QMainWindow_setDockOptions(QMainWindow *w, int options) {
    w->setDockOptions(static_cast<QMainWindow::DockOption>(options));
}

inline void QMainWindow_setTabPosition(QMainWindow *w, int areas, int tabPos) {
    w->setTabPosition(static_cast<Qt::DockWidgetAreas>(areas),
                      static_cast<QTabWidget::TabPosition>(tabPos));
}

// Upcast
inline QWidget *toQWidget_QMainWindow(QMainWindow *w) {
    return static_cast<QWidget *>(w);
}
