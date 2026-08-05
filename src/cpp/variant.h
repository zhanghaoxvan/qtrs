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
#if QT_IS_QT6
    #define QVARIANT_TYPE_ID(v) ((v)->typeId())
    #define QVARIANT_TYPE_ENUM(type) QMetaType::type
    #define QVARIANT_TYPE_ENUM_QSTRING QMetaType::QString
    #define QVARIANT_TYPE_ENUM_QSTRINGLIST QMetaType::QStringList
    #define QVARIANT_TYPE_ENUM_QBYTEARRAY QMetaType::QByteArray
#else
    #define QVARIANT_TYPE_ID(v) ((v)->type())
    #define QVARIANT_TYPE_ENUM(type) QVariant::type
    #define QVARIANT_TYPE_ENUM_QSTRING QVariant::String
    #define QVARIANT_TYPE_ENUM_QSTRINGLIST QVariant::StringList
    #define QVARIANT_TYPE_ENUM_QBYTEARRAY QVariant::ByteArray
#endif

// Generic type check macro
#define QVARIANT_IS_TYPE(v, type) \
    (QVARIANT_TYPE_ID(v) == QVARIANT_TYPE_ENUM(type))

// Special type check macro for Qt types with different names
#define QVARIANT_IS_QT_TYPE(v, type_enum) \
    (QVARIANT_TYPE_ID(v) == type_enum)

inline bool QVariant_is_int(QVariant* v) {
    return QVARIANT_IS_TYPE(v, Int);
}

inline bool QVariant_is_uint(QVariant* v) {
    return QVARIANT_IS_TYPE(v, UInt);
}

inline bool QVariant_is_long(QVariant* v) {
    return QVARIANT_IS_TYPE(v, LongLong);
}

inline bool QVariant_is_bool(QVariant* v) {
    return QVARIANT_IS_TYPE(v, Bool);
}

inline bool QVariant_is_double(QVariant* v) {
    return QVARIANT_IS_TYPE(v, Double);
}

inline bool QVariant_is_string(QVariant* v) {
    return QVARIANT_IS_QT_TYPE(v, QVARIANT_TYPE_ENUM_QSTRING);
}

inline bool QVariant_is_stringlist(QVariant* v) {
    return QVARIANT_IS_QT_TYPE(v, QVARIANT_TYPE_ENUM_QSTRINGLIST);
}

inline bool QVariant_is_bytearray(QVariant* v) {
    return QVARIANT_IS_QT_TYPE(v, QVARIANT_TYPE_ENUM_QBYTEARRAY);
}

// --- Getters (Qt -> Rust) ---
inline int QVariant_to_int(QVariant* v) {
    return v->toInt();
}

inline unsigned int QVariant_to_uint(QVariant* v) {
    return v->toUInt();
}

inline int64_t QVariant_to_long(QVariant* v) {
    return static_cast<int64_t>(v->toLongLong());
}

inline bool QVariant_to_bool(QVariant* v) {
    return v->toBool();
}

inline double QVariant_to_double(QVariant* v) {
    return v->toDouble();
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

