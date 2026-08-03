// src/cpp/intvalidator.h — QIntValidator + QDoubleValidator
#pragma once

#include <QtGui/QIntValidator>
#include <QtGui/QDoubleValidator>

// QIntValidator
inline QIntValidator *QIntValidator_new(QObject *parent) { return new QIntValidator(parent); }
inline void QIntValidator_delete(QIntValidator *v) { delete v; }
inline void QIntValidator_setRange(QIntValidator *v, int min, int max) { v->setRange(min, max); }
inline void QIntValidator_setBottom(QIntValidator *v, int b) { v->setBottom(b); }
inline void QIntValidator_setTop(QIntValidator *v, int t) { v->setTop(t); }

// QDoubleValidator
inline QDoubleValidator *QDoubleValidator_new(QObject *parent) { return new QDoubleValidator(parent); }
inline void QDoubleValidator_delete(QDoubleValidator *v) { delete v; }
inline void QDoubleValidator_setRange(QDoubleValidator *v, double min, double max, int decimals) {
    v->setRange(min, max, decimals);
}
