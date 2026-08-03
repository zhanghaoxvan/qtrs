// src/cpp/pixmap.h — QPixmap
#pragma once

#include <QtGui/QPixmap>
#include <QtCore/QString>
#include <string>
#include "rust/cxx.h"

inline void QPixmap_load(QPixmap *p, const std::string &path) { p->load(QString::fromStdString(path)); }
inline bool QPixmap_isNull(const QPixmap *p) { return p->isNull(); }
inline int QPixmap_width(const QPixmap *p) { return p->width(); }
inline int QPixmap_height(const QPixmap *p) { return p->height(); }
