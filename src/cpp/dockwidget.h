// src/cpp/dockwidget.h — QDockWidget
#pragma once

#include <QtCore/QObject>
#include <QtWidgets/QDockWidget>
#include <QtCore/QString>
#include <string>

#include "signal.h"

inline QDockWidget *QDockWidget_new(QWidget *parent) {
    return new QDockWidget(parent);
}
inline void QDockWidget_delete(QDockWidget *dw) { delete dw; }
inline void QDockWidget_setWindowTitle(QDockWidget *dw, const std::string &title) {
    dw->setWindowTitle(QString::fromStdString(title));
}
inline void QDockWidget_setWidget(QDockWidget *dw, QWidget *widget) {
    dw->setWidget(widget);
}
inline QWidget *QDockWidget_widget(QDockWidget *dw) {
    return dw->widget();
}
inline void QDockWidget_setFeatures(QDockWidget *dw, int features) {
    dw->setFeatures(static_cast<QDockWidget::DockWidgetFeatures>(features));
}
inline int QDockWidget_features(QDockWidget *dw) {
    return static_cast<int>(dw->features());
}
inline void QDockWidget_setAllowedAreas(QDockWidget *dw, int areas) {
    dw->setAllowedAreas(static_cast<Qt::DockWidgetAreas>(areas));
}
inline int QDockWidget_allowedAreas(QDockWidget *dw) {
    return static_cast<int>(dw->allowedAreas());
}
inline void QDockWidget_setFloating(QDockWidget *dw, bool floating) {
    dw->setFloating(floating);
}
inline bool QDockWidget_isFloating(QDockWidget *dw) {
    return dw->isFloating();
}
inline void QDockWidget_setVisible(QDockWidget *dw, bool visible) {
    dw->setVisible(visible);
}
inline void QDockWidget_show(QDockWidget *dw) { dw->show(); }
inline void QDockWidget_hide(QDockWidget *dw) { dw->hide(); }
inline void QDockWidget_onVisibilityChanged(QDockWidget *dw, uint64_t ctx) {
    QObject::connect(dw, &QDockWidget::visibilityChanged, [ctx](bool) {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}
inline void QDockWidget_onFeaturesChanged(QDockWidget *dw, uint64_t ctx) {
    QObject::connect(dw, &QDockWidget::featuresChanged, [ctx](int) {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}
inline QWidget *toQWidget_QDockWidget(QDockWidget *dw) {
    return static_cast<QWidget*>(dw);
}
