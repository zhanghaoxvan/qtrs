// This C++ file includes the umbrella header, the moc-generated
// code for Q_OBJECT classes, and out-of-line definitions for
// extern "C" functions that are called from Rust via raw FFI.
#include "qt_widget.h"

#ifdef QTRS_HAS_QML
#include "moc_qml.cpp"

// --- extern "C" definitions for QmlBackend bridge ---

QmlBackendCallback g_qmlBackendCb = nullptr;
QmlBackend *g_qmlBackend = nullptr;

extern "C" {

void qmlBackend_setCallback(QmlBackendCallback cb) {
    g_qmlBackendCb = cb;
}

void qmlBackend_register(void *enginePtr) {
    auto *engine = static_cast<QQmlApplicationEngine *>(enginePtr);
    g_qmlBackend = new QmlBackend();
    engine->rootContext()->setContextProperty("backend", g_qmlBackend);
}

void qmlBackend_emitResult(bool success, const char *msg) {
    if (g_qmlBackend)
        g_qmlBackend->emitResult(success, QString::fromUtf8(msg));
}

} // extern "C"
#endif // QTRS_HAS_QML
