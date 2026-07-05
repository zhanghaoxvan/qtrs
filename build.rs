// build.rs — probe Qt 6 and configure compilation
use std::env;

use system_deps::Config;

fn main() {
    // 1. Probe base Qt 6 (always required)
    let deps = Config::new()
        .probe()
        .expect("Qt6 base library is not installed! Install qt6-base-dev or visit https://qt.io/");

    // 2. Set up cxx bridge
    let mut build = cxx_build::bridge("src/ffi.rs");

    // 3. Add include paths and link libs
    for (_name, lib) in &deps {
        for include_path in &lib.include_paths {
            build.include(include_path);
        }
        for link_lib in &lib.libs {
            build.flag(&format!("-l{}", link_lib));
        }
    }

    // 4. Optional: Qt6 UiTools (feature = "ui")
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

    // 5. Compile C++ bridge
    build.file("src/cpp/qt_widget.cpp").compile("qtrs");

    // 6. Rerun triggers
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/cpp/qt_widget.cpp");
    for h in &["signal", "app", "widget", "button", "label", "input",
               "checkbox", "combobox", "textedit", "slider", "timer",
               "layout", "dialog", "uiloader"] {
        println!("cargo:rerun-if-changed=src/cpp/{}.h", h);
    }
}
