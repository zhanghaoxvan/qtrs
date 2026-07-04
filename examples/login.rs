// Login example — Rust backend for login.qml using the standard
// QML `Connections` pattern:
//
//   QML :  backend.login(user, pass)
//   QML :  Connections { target: backend; onLoginResult: ... }
//
// The `backend` object is a C++ `QmlBackend` (QObject with Q_OBJECT)
// exposed as a QML context property.  Rust connects to it via plain
// `extern "C"` FFI — no extra C++ files needed.

use std::ffi::{c_char, c_void, CString};

use qtrs::*;

// ============================================================
// FFI to QmlBackend (defined in src/cpp/qml.h, compiled via moc)
// ============================================================

unsafe extern "C" {
    /// Register the Rust login handler: fn(username, password).
    unsafe fn qmlBackend_setCallback(
        cb: Option<unsafe extern "C" fn(*const c_char, *const c_char)>,
    );

    /// Create the QmlBackend QObject and expose it as QML context
    /// property `"backend"` on the given QQmlApplicationEngine.
    unsafe fn qmlBackend_register(engine: *mut c_void);

    /// Emit `loginResult(success, message)` signal — received by
    /// QML's `Connections { onLoginResult: ... }`.
    unsafe fn qmlBackend_emitResult(success: bool, message: *const c_char);
}

// ============================================================
// Credentials
// ============================================================

const VALID_USER: &str = "admin";
const VALID_PASS: &str = "123456";

// ============================================================
// Login callback — invoked by C++ when QML calls backend.login()
// ============================================================

unsafe extern "C" fn on_login(username: *const c_char, password: *const c_char) {
    let user = unsafe { std::ffi::CStr::from_ptr(username) }
        .to_str()
        .unwrap_or("");
    let pass = unsafe { std::ffi::CStr::from_ptr(password) }
        .to_str()
        .unwrap_or("");

    let (success, msg) = if user == VALID_USER && pass == VALID_PASS {
        (true, format!("Welcome, {}!", user))
    } else {
        (false, "Invalid username or password".into())
    };

    let c_msg = CString::new(msg).unwrap();
    unsafe { qmlBackend_emitResult(success, c_msg.as_ptr()) };
}

// ============================================================
// Main
// ============================================================

fn main() {
    let app = Application::new();
    let engine = QmlEngine::new();

    // Register the Rust callback and create the backend QObject
    // BEFORE loading the QML, so QML can reference `backend` immediately.
    unsafe {
        qmlBackend_setCallback(Some(on_login));
        qmlBackend_register(engine.raw_ptr());
    }

    engine.load("examples/login.qml");
    app.exec();
}
