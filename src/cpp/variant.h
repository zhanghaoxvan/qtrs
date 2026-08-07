// ============================================================
// src/cpp/variant.h — QVariant FFI bindings
// ============================================================

#pragma once

#include <rust/cxx.h>
#include <QtCore/QVariant>
#include <QtCore/QString>
#include <QtCore/QByteArray>
#include <QtCore/QStringList>
#include <string>
#include <vector>

// --- Constructors (Rust -> Qt) ---
inline QVariant* QVariant_from_int(int v) {
    return new QVariant(v);
}

inline QVariant* QVariant_from_uint(unsigned int v) {
    return new QVariant(v);
}

inline QVariant* QVariant_from_long(int64_t v) {
    return new QVariant(static_cast<qlonglong>(v));
}

inline QVariant* QVariant_from_bool(bool v) {
    return new QVariant(v);
}

inline QVariant* QVariant_from_double(double v) {
    return new QVariant(v);
}

inline QVariant* QVariant_from_string(rust::String s) {
    return new QVariant(QString::fromStdString(std::string(s)));
}

inline QVariant* QVariant_from_stringlist(rust::Vec<rust::String> v) {
    QStringList list;
    for (auto& s : v) list.append(QString::fromStdString(std::string(s)));
    return new QVariant(list);
}

inline QVariant* QVariant_from_bytearray(rust::Slice<const uint8_t> slice) {
    return new QVariant(QByteArray(
        reinterpret_cast<const char*>(slice.data()),
        slice.size()
    ));
}

// --- Destructor ---
inline void QVariant_delete(QVariant* v) {
    delete v;
}

// --- Type check ---

#if QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
    #define QT_IS_QT6 1
#else
    #define QT_IS_QT6 0
#endif

// Qt6: typeId() + QMetaType, Qt5: type() + QVariant
inline bool QVariant_is_int(QVariant* v) {
    return v->canConvert<int>();
}

inline bool QVariant_is_uint(QVariant* v) {
    return v->canConvert<unsigned int>();
}

inline bool QVariant_is_long(QVariant* v) {
    return v->canConvert<qint64>();
}

inline bool QVariant_is_bool(QVariant* v) {
    return v->canConvert<bool>();
}

inline bool QVariant_is_double(QVariant* v) {
    return v->canConvert<double>();
}

inline bool QVariant_is_string(QVariant* v) {
    return v->canConvert<QString>();
}

inline bool QVariant_is_stringlist(QVariant* v) {
    return v->canConvert<QStringList>();
}

inline bool QVariant_is_bytearray(QVariant* v) {
    return v->canConvert<QByteArray>();
}

// --- Getters (Qt -> Rust) ---
inline int QVariant_to_int(QVariant* v, bool* ok) {
    return v->toInt(ok);
}

inline unsigned int QVariant_to_uint(QVariant* v, bool* ok) {
    return v->toUInt(ok);
}

inline int64_t QVariant_to_long(QVariant* v, bool* ok) {
    return v->toLongLong(ok);
}

inline bool QVariant_to_bool(QVariant* v, bool* ok) {
    if (!v->canConvert<bool>()) {
        *ok = false;
        return false;
    }
    *ok = true;
    return v->toBool();
}

inline double QVariant_to_double(QVariant* v, bool* ok) {
    return v->toDouble(ok);
}

inline rust::String QVariant_to_string(QVariant* v, bool* ok) {
    if (!v->canConvert<QString>()) {
        *ok = false;
        return rust::String("");
    }
    *ok = true;
    return rust::String(v->toString().toStdString());
}

inline rust::String QVariant_to_string(QVariant* v) {
    return rust::String(v->toString().toStdString());
}

inline rust::Vec<rust::String> QVariant_to_stringlist(QVariant* v) {
    rust::Vec<rust::String> result;
    for (const auto& s : v->toStringList())
        result.push_back(rust::String(s.toStdString()));
    return result;
}

inline rust::Vec<uint8_t> QVariant_to_bytearray(QVariant* v) {
    QByteArray arr = v->toByteArray();
    rust::Vec<uint8_t> result;
    result.reserve(arr.size());
    for (int i = 0; i < arr.size(); ++i)
        result.push_back(static_cast<uint8_t>(arr[i]));
    return result;
}

