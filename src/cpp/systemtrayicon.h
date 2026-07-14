// src/cpp/systemtrayicon.h — QSystemTrayIcon
#pragma once

#include <QtWidgets/QSystemTrayIcon>
#include <QtWidgets/QMenu>
#include <QtGui/QIcon>
#include <QtCore/QObject>
#include <QtCore/QString>
#include <string>

#include "signal.h"

// Constructor / Destructor
inline QSystemTrayIcon *QSystemTrayIcon_new(const std::string &icon_path, QObject *parent) {
    QIcon icon(QString::fromStdString(icon_path));
    return new QSystemTrayIcon(icon, parent);
}

inline void QSystemTrayIcon_delete(QSystemTrayIcon *tray) { delete tray; }

// Properties
inline void QSystemTrayIcon_setIcon(QSystemTrayIcon *tray, const std::string &icon_path) {
    QIcon icon(QString::fromStdString(icon_path));
    if (!icon.isNull()) tray->setIcon(icon);
}

inline void QSystemTrayIcon_setToolTip(QSystemTrayIcon *tray, const std::string &tip) {
    tray->setToolTip(QString::fromStdString(tip));
}

inline void QSystemTrayIcon_show(QSystemTrayIcon *tray) {
    tray->show();
}

inline void QSystemTrayIcon_hide(QSystemTrayIcon *tray) {
    tray->hide();
}

inline bool QSystemTrayIcon_isVisible(QSystemTrayIcon *tray) {
    return tray->isVisible();
}

inline void QSystemTrayIcon_setContextMenu(QSystemTrayIcon *tray, QMenu *menu) {
    tray->setContextMenu(menu);
}

// Signal
inline void QSystemTrayIcon_onActivated(QSystemTrayIcon *tray, uint64_t ctx) {
    QObject::connect(tray, &QSystemTrayIcon::activated, [ctx](QSystemTrayIcon::ActivationReason reason) {
        if (g_hasIntTrampoline) g_intTrampoline(ctx, static_cast<int32_t>(reason));
    });
}
