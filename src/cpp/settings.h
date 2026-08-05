// src/cpp/settings.h
#pragma once

#include <QtCore/QSettings>
#include <QtCore/QString>
#include <QtCore/QStringList>
#include <QtCore/QVariant>
#include "rust/cxx.h"
#include <string>
#include <vector>

// --- Constructors ---
inline QSettings* QSettings_new_user_app(const rust::String& org, const rust::String& app) {
    return new QSettings(QString::fromStdString(std::string(org)), QString::fromStdString(std::string(app)));
}

inline QSettings* QSettings_new_scope_app(int scope, const rust::String& org, const rust::String& app) {
    return new QSettings(QSettings::Scope(scope), QString::fromStdString(std::string(org)), QString::fromStdString(std::string(app)));
}

inline QSettings* QSettings_new_format_scope(int format, int scope, const rust::String& org, const rust::String& app) {
    return new QSettings(QSettings::Format(format), QSettings::Scope(scope), QString::fromStdString(std::string(org)), QString::fromStdString(std::string(app)));
}

inline QSettings* QSettings_new_file(const rust::String& fileName, int format) {
    return new QSettings(QString::fromStdString(std::string(fileName)), QSettings::Format(format));
}

inline void QSettings_delete(QSettings* s) {
    delete s;
}

// --- Group ---
inline void QSettings_beginGroup(QSettings* s, const rust::String& prefix) {
    s->beginGroup(QString::fromStdString(std::string(prefix)));
}

inline void QSettings_endGroup(QSettings* s) {
    s->endGroup();
}

inline rust::String QSettings_group(QSettings* s) {
    return rust::String(s->group().toStdString());
}

// --- Array ---
inline int QSettings_beginReadArray(QSettings* s, const rust::String& prefix) {
    return s->beginReadArray(QString::fromStdString(std::string(prefix)));
}

inline void QSettings_beginWriteArray(QSettings* s, const rust::String& prefix, int size) {
    s->beginWriteArray(QString::fromStdString(std::string(prefix)), size);
}

inline void QSettings_endArray(QSettings* s) {
    s->endArray();
}

inline void QSettings_setArrayIndex(QSettings* s, int i) {
    s->setArrayIndex(i);
}

// --- Read ---

inline QVariant* QSettings_value(QSettings* s, const rust::String& key, QVariant* default_value) {
    return new QVariant(s->value(QString::fromStdString(std::string(key)), *default_value));
}

// --- Write ---
inline void QSettings_setValue(QSettings* s, const rust::String& key, QVariant* value) {
    s->setValue(QString::fromStdString(std::string(key)), *value);
}

// --- Other ---
inline bool QSettings_contains(QSettings* s, const rust::String& key) {
    return s->contains(QString::fromStdString(std::string(key)));
}

inline void QSettings_remove(QSettings* s, const rust::String& key) {
    s->remove(QString::fromStdString(std::string(key)));
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

inline int QSettings_status(QSettings* s) {
    return static_cast<int>(s->status());
}

inline rust::Vec<rust::String> QSettings_allKeys(QSettings* s) {
    rust::Vec<rust::String> result;
    for (const auto& key : s->allKeys()) {
        result.push_back(rust::String(key.toStdString()));
    }
    return result;
}

inline rust::Vec<rust::String> QSettings_childKeys(QSettings* s) {
    rust::Vec<rust::String> result;
    for (const auto& key : s->childKeys()) {
        result.push_back(rust::String(key.toStdString()));
    }
    return result;
}

inline rust::Vec<rust::String> QSettings_childGroups(QSettings* s) {
    rust::Vec<rust::String> result;
    for (const auto& group : s->childGroups()) {
        result.push_back(rust::String(group.toStdString()));
    }
    return result;
}

inline rust::String QSettings_fileName(QSettings* s) {
    return rust::String(s->fileName().toStdString());
}

inline bool QSettings_fallbacksEnabled(QSettings* s) {
    return s->fallbacksEnabled();
}

inline void QSettings_setFallbacksEnabled(QSettings* s, bool enabled) {
    s->setFallbacksEnabled(enabled);
}