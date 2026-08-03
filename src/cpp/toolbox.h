// src/cpp/toolbox.h — QToolBox
#pragma once

#include <QtCore/QObject>
#include <QtWidgets/QToolBox>
#include <QtWidgets/QWidget>
#include <QtCore/QString>
#include <QtGui/QIcon>
#include <string>
#include "rust/cxx.h"

#include "signal.h"

inline QToolBox *QToolBox_new(QWidget *parent) {
    return new QToolBox(parent);
}
inline void QToolBox_delete(QToolBox *tb) { delete tb; }
inline void QToolBox_addItem(QToolBox *tb, QWidget *widget, const std::string &text) {
    tb->addItem(widget, QString::fromStdString(text));
}
inline void QToolBox_insertItem(QToolBox *tb, int index, QWidget *widget, const std::string &text) {
    tb->insertItem(index, widget, QString::fromStdString(text));
}
inline void QToolBox_removeItem(QToolBox *tb, int index) { tb->removeItem(index); }
inline void QToolBox_setItemText(QToolBox *tb, int index, const std::string &text) {
    tb->setItemText(index, QString::fromStdString(text));
}
inline rust::String QToolBox_itemText(QToolBox *tb, int index) {
    return rust::String(tb->itemText(index).toStdString());
}
inline void QToolBox_setItemIcon(QToolBox *tb, int index, const std::string &icon_path) {
    tb->setItemIcon(index, QIcon(QString::fromStdString(icon_path)));
}
inline void QToolBox_setItemEnabled(QToolBox *tb, int index, bool enabled) {
    tb->setItemEnabled(index, enabled);
}
inline bool QToolBox_isItemEnabled(QToolBox *tb, int index) {
    return tb->isItemEnabled(index);
}
inline int QToolBox_currentIndex(QToolBox *tb) { return tb->currentIndex(); }
inline void QToolBox_setCurrentIndex(QToolBox *tb, int index) { tb->setCurrentIndex(index); }
inline int QToolBox_count(QToolBox *tb) { return tb->count(); }
inline QWidget *QToolBox_widget(QToolBox *tb, int index) { return tb->widget(index); }
inline void QToolBox_onCurrentChanged(QToolBox *tb, uint64_t ctx) {
    QObject::connect(tb, &QToolBox::currentChanged, [ctx](int) {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}
inline QWidget *toQWidget_QToolBox(QToolBox *tb) {
    return static_cast<QWidget*>(tb);
}
