// src/cpp/signal.h — Trampoline globals + generic signal-slot connection

#pragma once

#include <cstdint>
#include <string>
#include <QObject>
#include <Qt>

#include "rust/cxx.h"

// ============================================================
// Trampoline globals — registered once from Rust at startup
// ============================================================

static bool g_hasVoidTrampoline = false;
static bool g_hasBoolTrampoline = false;
static bool g_hasIntTrampoline = false;
static rust::Fn<void(uint64_t)>       g_voidTrampoline;
static rust::Fn<void(uint64_t, bool)> g_boolTrampoline;
static rust::Fn<void(uint64_t, int32_t)> g_intTrampoline;

inline void qtrs_setVoidTrampoline(rust::Fn<void(uint64_t)> t) {
    g_voidTrampoline = t;
    g_hasVoidTrampoline = true;
}
inline void qtrs_setBoolTrampoline(rust::Fn<void(uint64_t, bool)> t) {
    g_boolTrampoline = t;
    g_hasBoolTrampoline = true;
}
inline void qtrs_setIntTrampoline(rust::Fn<void(uint64_t, int32_t)> t) {
    g_intTrampoline = t;
    g_hasIntTrampoline = true;
}

// ============================================================
// Generic signal-slot connection (string signature)
// ============================================================

/// Connect signal to slot using Qt6 string signature format
/// sig: "2signalName(args)" (signal)
/// slt: "1slotName(args)"   (slot)
inline bool QObject_connect(
    QObject* sender,
    const std::string& sig,
    QObject* receiver,
    const std::string& slt,
    int connType
) {
    Qt::ConnectionType type = static_cast<Qt::ConnectionType>(connType);
    return QObject::connect(sender, sig.c_str(), receiver, slt.c_str(), type);
}

/// Disconnect signal-slot connection
inline bool QObject_disconnect(
    QObject* sender,
    const std::string& sig,
    QObject* receiver,
    const std::string& slt
) {
    return QObject::disconnect(sender, sig.c_str(), receiver, slt.c_str());
}