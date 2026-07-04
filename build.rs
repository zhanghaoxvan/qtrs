// build.rs — probe Qt 6 and configure compilation
use std::env;
use std::path::Path;
use std::process::Command;

use system_deps::Config;

fn main() {
    // 1. Probe base Qt 6 (always required)
    let deps = Config::new()
        .probe()
        .expect("Qt6 base library is not installed! Install qt6-base-dev or visit https://qt.io/");

    // Collect all base include paths
    let mut all_includes: Vec<String> = Vec::new();
    for (_name, lib) in &deps {
        for include_path in &lib.include_paths {
            all_includes.push(include_path.to_string_lossy().to_string());
        }
    }

    // 2. Set up cxx bridge
    let mut build = cxx_build::bridge("src/ffi.rs");

    // 3. Add include paths and link libs
    for path in &all_includes {
        build.include(path);
    }
    for (_name, lib) in &deps {
        for link_lib in &lib.libs {
            build.flag(&format!("-l{}", link_lib));
        }
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // 4. Optional: Qt6 QML (feature = "qml")
    if env::var("CARGO_FEATURE_QML").is_ok() {
        let qml_lib = pkg_config::Config::new()
            .atleast_version("6.0")
            .probe("Qt6Qml")
            .expect("Qt6Qml not found. Install qt6-declarative-dev");

        for path in &qml_lib.include_paths {
            build.include(path);
            all_includes.push(path.to_string_lossy().to_string());
        }
        build.flag("-lQt6Qml");
        build.flag("-DQTRS_HAS_QML");

        // Run moc on qml.h for the SignalRelay Q_OBJECT helper
        let moc = find_moc();
        let header = format!("{}/src/cpp/qml.h", manifest_dir);
        let moc_out = format!("{}/moc_qml.cpp", out_dir);
        let mut moc_cmd = Command::new(&moc);
        moc_cmd.arg("-DQTRS_HAS_QML");
        moc_cmd.arg("-o").arg(&moc_out).arg(&header);
        for path in &all_includes {
            moc_cmd.arg("-I").arg(path);
        }
        let status = moc_cmd
            .status()
            .expect("Failed to run moc. Install qt6-base-dev-tools.");
        if !status.success() {
            panic!("moc failed on {}", header);
        }
        build.include(&out_dir);
    }

    // 5. Optional: Qt6 UiTools (feature = "ui")
    if env::var("CARGO_FEATURE_UI").is_ok() {
        let ui_lib = pkg_config::Config::new()
            .atleast_version("6.0")
            .probe("Qt6UiTools")
            .expect("Qt6UiTools not found. Install qt6-tools-dev");

        for path in &ui_lib.include_paths {
            build.include(path);
        }
        build.flag("-lQt6UiTools");
        build.flag("-DQTRS_HAS_UI");
    }

    build.include(".");
    build.include("src/cpp"); // for #include "signal.h" etc between modules

    // 6. Compile C++ bridge
    build.file("src/cpp/qt_widget.cpp").compile("qtrs");

    // 7. Rerun triggers
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/cpp/qml.h");
    println!("cargo:rerun-if-changed=src/cpp/qt_widget.cpp");
    for h in &["signal", "app", "widget", "button", "label", "input",
               "checkbox", "combobox", "textedit", "slider", "timer",
               "layout", "dialog", "qml", "uiloader"] {
        println!("cargo:rerun-if-changed=src/cpp/{}.h", h);
    }
}

fn find_moc() -> String {
    for path in &["/usr/lib/qt6/libexec/moc", "/usr/lib/qt6/bin/moc"] {
        if Path::new(path).exists() {
            return path.to_string();
        }
    }
    for name in &["moc-qt6", "moc6"] {
        if Command::new(name).arg("--version").output().is_ok() {
            return name.to_string();
        }
    }
    if let Ok(out) = Command::new("qmake6")
        .args(["-query", "QT_INSTALL_LIBEXECS"])
        .output()
    {
        let p = format!("{}/moc", String::from_utf8_lossy(&out.stdout).trim());
        if Path::new(&p).exists() {
            return p;
        }
    }
    if let Ok(out) = Command::new("qmake6")
        .args(["-query", "QT_INSTALL_BINS"])
        .output()
    {
        let p = format!("{}/moc", String::from_utf8_lossy(&out.stdout).trim());
        if Path::new(&p).exists() {
            return p;
        }
    }
    panic!("Qt6 moc not found — install qt6-base-dev-tools");
}
