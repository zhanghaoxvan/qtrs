// src/cpp/settings.h
#pragma once

#include <QtCore/QSettings>
#include <QtCore/QString>
#include <QtCore/QStringList>
#include <QtCore/QVariant>
#include <string>
#include <vector>

// --- Constructors ---
inline QSettings* QSettings_new_user_app(const std::string& org, const std::string& app) {
    return new QSettings(QString::fromStdString(org), QString::fromStdString(app));
}

inline QSettings* QSettings_new_scope_app(QSettings::Scope scope, const std::string& org, const std::string& app) {
    return new QSettings(scope, QString::fromStdString(org), QString::fromStdString(app));
}

inline QSettings* QSettings_new_format_scope(QSettings::Format format, QSettings::Scope scope,
                                              const std::string& org, const std::string& app) {
    return new QSettings(format, scope, QString::fromStdString(org), QString::fromStdString(app));
}

inline QSettings* QSettings_new_file(const std::string& fileName, QSettings::Format format) {
    return new QSettings(QString::fromStdString(fileName), format);
}

inline void QSettings_delete(QSettings* s) {
    delete s;
}

// --- Group ---
inline void QSettings_beginGroup(QSettings* s, const std::string& prefix) {
    s->beginGroup(QString::fromStdString(prefix));
}

inline void QSettings_endGroup(QSettings* s) {
    s->endGroup();
}

inline std::string QSettings_group(QSettings* s) {
    return s->group().toStdString();
}

// --- Array ---
inline int QSettings_beginReadArray(QSettings* s, const std::string& prefix) {
    return s->beginReadArray(QString::fromStdString(prefix));
}

inline void QSettings_beginWriteArray(QSettings* s, const std::string& prefix, int size) {
    s->beginWriteArray(QString::fromStdString(prefix), size);
}

inline void QSettings_endArray(QSettings* s) {
    s->endArray();
}

inline void QSettings_setArrayIndex(QSettings* s, int i) {
    s->setArrayIndex(i);
}

// --- Read ---
inline std::string QSettings_value_string(QSettings* s, const std::string& key, const std::string& default_value) {
    return s->value(QString::fromStdString(key), QString::fromStdString(default_value)).toString().toStdString();
}

inline int QSettings_value_int(QSettings* s, const std::string& key, int default_value) {
    return s->value(QString::fromStdString(key), default_value).toInt();
}

inline bool QSettings_value_bool(QSettings* s, const std::string& key, bool default_value) {
    return s->value(QString::fromStdString(key), default_value).toBool();
}

inline double QSettings_value_double(QSettings* s, const std::string& key, double default_value) {
    return s->value(QString::fromStdString(key), default_value).toDouble();
}

// --- Write ---
inline void QSettings_setValue_string(QSettings* s, const std::string& key, const std::string& value) {
    s->setValue(QString::fromStdString(key), QString::fromStdString(value));
}

inline void QSettings_setValue_int(QSettings* s, const std::string& key, int value) {
    s->setValue(QString::fromStdString(key), value);
}

inline void QSettings_setValue_bool(QSettings* s, const std::string& key, bool value) {
    s->setValue(QString::fromStdString(key), value);
}

inline void QSettings_setValue_double(QSettings* s, const std::string& key, double value) {
    s->setValue(QString::fromStdString(key), value);
}

inline void QSettings_setValue_variant(QSettings* s, const std::string& key, QVariant* value) {
    s->setValue(QString::fromStdString(key), *value);
}

// --- Other ---
inline bool QSettings_contains(QSettings* s, const std::string& key) {
    return s->contains(QString::fromStdString(key));
}

inline void QSettings_remove(QSettings* s, const std::string& key) {
    s->remove(QString::fromStdString(key));
}

inline void QSettings_sync(QSettings* s) {
    s->sync();
}

inline void QSettings_clear(QSettings* s) {
    s->clear();
}

inline bool QSettings_isWritable(QSettings* s) {
    return s->isWritable();
}

inline QSettings::Status QSettings_status(QSettings* s) {
    return s->status();
}

inline QStringList QSettings_allKeys(QSettings* s) {
    return s->allKeys();
}

inline QStringList QSettings_childKeys(QSettings* s) {
    return s->childKeys();
}

inline QStringList QSettings_childGroups(QSettings* s) {
    return s->childGroups();
}

inline std::string QSettings_fileName(QSettings* s) {
    return s->fileName().toStdString();
}

inline bool QSettings_fallbacksEnabled(QSettings* s) {
    return s->fallbacksEnabled();
}

inline void QSettings_setFallbacksEnabled(QSettings* s, bool enabled) {
    s->setFallbacksEnabled(enabled);
}