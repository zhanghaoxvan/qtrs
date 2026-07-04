// src/cpp/qml.h — QML / Qt Quick support (feature = "qml")
#pragma once

#ifdef QTRS_HAS_QML

#include <QMetaObject>
#include <QObject>
#include <QQmlApplicationEngine>
#include <QQmlComponent>
#include <QQmlContext>
#include <QString>
#include <string>

#include "rust/cxx.h"
#include "signal.h"

// --- QML engine ---
inline QQmlApplicationEngine *QQmlApplicationEngine_new() {
    return new QQmlApplicationEngine();
}
inline void QQmlApplicationEngine_load(QQmlApplicationEngine *engine,
                                        const std::string &qml_path) {
    engine->load(QString::fromStdString(qml_path));
}
inline void QQmlApplicationEngine_delete(QQmlApplicationEngine *engine) {
    delete engine;
}

// --- QML object access via uint64_t ---

inline uint64_t QQmlApplicationEngine_rootObject(
    QQmlApplicationEngine *engine) {
    auto roots = engine->rootObjects();
    return roots.isEmpty() ? 0
                           : reinterpret_cast<uint64_t>(roots.first());
}

inline uint64_t QObject_findChild(uint64_t parent, const std::string &name) {
    auto *obj = reinterpret_cast<QObject *>(parent);
    auto *child = obj->findChild<QObject *>(
        QString::fromStdString(name), Qt::FindChildrenRecursively);
    return reinterpret_cast<uint64_t>(child);
}

inline rust::String QObject_property_str(uint64_t obj,
                                          const std::string &prop) {
    return reinterpret_cast<QObject *>(obj)
        ->property(prop.c_str())
        .toString()
        .toStdString();
}

inline void QObject_setProperty_str(uint64_t obj, const std::string &prop,
                                     const std::string &value) {
    reinterpret_cast<QObject *>(obj)->setProperty(
        prop.c_str(), QString::fromStdString(value));
}

// --- SignalRelay — bridges QML signals to our trampoline ---
class SignalRelay : public QObject {
    Q_OBJECT
public:
    uint64_t ctx;
    SignalRelay(uint64_t c, QObject *parent) : QObject(parent), ctx(c) {}
public slots:
    void fire() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    }
};

inline void QObject_connectSignal(uint64_t sender_raw,
                                   const std::string &sig, uint64_t ctx) {
    auto *sender = reinterpret_cast<QObject *>(sender_raw);
    auto *relay = new SignalRelay(ctx, sender);
    std::string fullSig = "2" + sig;
    QObject::connect(sender, fullSig.c_str(), relay, SLOT("fire()"));
}

inline void QObject_invokeMethod1Bool1Str(
    uint64_t obj, const std::string &method, bool arg1,
    const std::string &arg2) {
    QMetaObject::invokeMethod(reinterpret_cast<QObject *>(obj),
                               method.c_str(), Q_ARG(bool, arg1),
                               Q_ARG(QString, QString::fromStdString(arg2)));
}

// ============================================================
// QmlBackend — QObject exposed as context property "backend"
// ============================================================

using QmlBackendCallback = void (*)(const char *username,
                                     const char *password);
extern QmlBackendCallback g_qmlBackendCb;
extern class QmlBackend *g_qmlBackend;

/// QObject with Q_INVOKABLE slots and signals — QML calls
/// `backend.login(user, pass)` and listens via
/// `Connections { target: backend; onLoginResult: ... }`.
class QmlBackend : public QObject {
    Q_OBJECT
public:
    QmlBackend(QObject *parent = nullptr) : QObject(parent) {}

    Q_INVOKABLE void login(const QString &username,
                            const QString &password) {
        if (g_qmlBackendCb) {
            QByteArray u = username.toUtf8();
            QByteArray p = password.toUtf8();
            g_qmlBackendCb(u.constData(), p.constData());
        }
    }

    Q_INVOKABLE void emitResult(bool success, const QString &message) {
        emit loginResult(success, message);
    }

signals:
    void loginResult(bool success, const QString &message);
};

// --- extern "C" bridge to Rust (declarations only — defs in qt_widget.cpp) ---

extern "C" {
    void qmlBackend_setCallback(QmlBackendCallback cb);
    void qmlBackend_register(void *enginePtr);
    void qmlBackend_emitResult(bool success, const char *msg);
}

#endif // QTRS_HAS_QML
