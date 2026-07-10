// src/cpp/thread.h

#pragma once
#include <QtCore/QThread>
#include <QtCore/QCoreApplication>

/// Returns true if the current thread is the GUI (main) thread
inline bool QObject_isInGuiThread() {
    if (!QCoreApplication::instance()) {
        return true;  // No QApplication yet, assume main thread
    }
    return QThread::currentThread() == QCoreApplication::instance()->thread();
}