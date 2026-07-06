// src/cpp/signal.h — trampoline globals and SignalRelay QObject
#pragma once

#include <cstdint>
#include <string>

// cxx wraps extern "C" fn pointers in rust::Fn<>
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
