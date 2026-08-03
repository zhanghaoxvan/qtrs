// build.rs — supports both Qt5 and Qt6, using qmake only

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // 1. Find Qt installation prefix
    let (qt_prefix, qt_version) = find_qt().expect(
        concat!(
            "Qt not found!\n",
            "Please ensure Qt5 or Qt6 is installed and `qmake` is in your PATH.\n\n",
            "Installation guides:\n",
            "  - Debian/Ubuntu: sudo apt install qtbase5-dev  (Qt5)\n",
            "  - Debian/Ubuntu: sudo apt install qt6-base-dev  (Qt6)\n",
            "  - Fedora:        sudo dnf install qt5-qtbase-devel or qt6-qtbase-devel\n",
            "  - Arch:          sudo pacman -S qt5-base or qt6-base\n",
            "  - macOS:         brew install qt@5 or qt@6\n",
            "  - Windows:       Install from qt.io\n\n",
            "Verify with: qmake -query QT_VERSION"
        )
    );

    let major = if qt_version.starts_with("6.") { 6 } else { 5 };

    // 2. Get paths from qmake
    let lib_path = qmake_query("QT_INSTALL_LIBS")
        .unwrap_or_else(|| format!("{}/lib", qt_prefix));
    let include_path = qmake_query("QT_INSTALL_HEADERS")
        .unwrap_or_else(|| format!("{}/include", qt_prefix));
    
    // QtUiTools include path (always needed for C++ compilation)
    let uitools_include = qmake_query("QT_INSTALL_HEADERS")
        .map(|path| {
            let candidates = [
                format!("{}/QtUiTools", path),
                format!("{}/qt6/QtUiTools", path),
                format!("{}/Qt6/QtUiTools", path),
                format!("{}/qt5/QtUiTools", path),
                format!("{}/Qt5/QtUiTools", path),
            ];
            for candidate in candidates {
                if PathBuf::from(&candidate).exists() {
                    return candidate;
                }
            }
            format!("{}/QtUiTools", path)
        })
        .unwrap_or_else(|| format!("{}/QtUiTools", include_path));

    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:include={}", include_path);

    // 3. Link Qt libraries
    println!("cargo:rustc-link-lib=Qt{}Core", major);
    println!("cargo:rustc-link-lib=Qt{}Gui", major);
    println!("cargo:rustc-link-lib=Qt{}Widgets", major);
    println!("cargo:rustc-link-lib=Qt{}UiTools", major);

    // 4. Pass Qt version to Rust via cfg flags
    println!("cargo:rustc-cfg=qt_{}", major);
    println!("cargo:rustc-check-cfg=cfg(qt_5)");
    println!("cargo:rustc-check-cfg=cfg(qt_6)");

    // 5. Generate _all.rs from src/ffi/*.rs, then set up cxx bridge
    let all_rs = generate_ffi();
    // 6. Generate _all.rs from src/ffi/*.rs into $OUT_DIR, then set up cxx bridge
    
    let mut build = cxx_build::bridge(all_rs);

    // Add main include
    build.include(&include_path);
    
    // QtUiTools include and C++ define (always; Rust "ui" feature gates the Rust API)
    build.include(&uitools_include);
    build.define("QTRS_HAS_UI", None);

    // Add Qt module includes
    let modules = ["QtCore", "QtGui", "QtWidgets"];

    // Direct subdirectories
    for module in &modules {
        let path = format!("{}/{}", include_path, module);
        if PathBuf::from(&path).exists() {
            build.include(&path);
        }
    }

    // qt6/ or Qt6/ subdirectories (cross-platform)
    for sub in &["qt6", "Qt6", "qt5", "Qt5"] {
        let sub_path = format!("{}/{}", include_path, sub);
        if PathBuf::from(&sub_path).exists() {
            build.include(&sub_path);
            for module in &modules {
                let module_path = format!("{}/{}/{}", include_path, sub, module);
                if PathBuf::from(&module_path).exists() {
                    build.include(&module_path);
                }
            }
        }
    }

    // Project includes
    build.include(".");
    build.include("src/cpp");

    // Compile
    build.file("src/cpp/qt_widget.cpp").compile("qtrs");

    // Rerun triggers
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/cpp/qt_widget.cpp");

    for name in &[
        "signal", "app", "widget", "button", "label", "input",
        "checkbox", "combobox", "textedit", "slider", "timer",
        "layout", "dialog", "uiloader", "progressbar", "filedialog",
        "radiobutton", "groupbox", "tabwidget", "spinbox", "menu",
        "thread", "point", "listwidget",
        "action", "mainwindow", "toolbar", "statusbar",
        "messagebox", "inputdialog", "progressdialog",
        "tablewidget", "treewidget", "scrollarea",
        "stackedwidget", "splitter", "font",
        "dateedit", "timeedit", "datetimeedit",
        "plaintextedit", "textbrowser",
        "frame", "toolbutton", "calendarwidget", "shortcut", "systemtrayicon",
        "standarditemmodel", "tableview", "listview", "treeview",
        "itemselectionmodel",
    ] {
        println!("cargo:rerun-if-changed=src/cpp/{}.h", name);
    }
}

/// Concatenate src/ffi/*.rs into $OUT_DIR/_all.rs
fn generate_ffi() -> PathBuf {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("_all.rs");
    let ffi_dir = Path::new("src/ffi");

    let mut body = String::new();
    if ffi_dir.exists() {
        let mut files: Vec<_> = fs::read_dir(ffi_dir).unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
            .collect();
        files.sort_by_key(|e| e.file_name());
        if !files.is_empty() {
            for entry in &files {
                let content = fs::read_to_string(entry.path()).unwrap_or_default();
                body.push_str(&content);
                body.push('\n');
            }
            println!("cargo:rerun-if-changed=src/ffi");
        }
    }

    // Fallback: if no ffi files, extract from the monolithic backup
    if body.trim().is_empty() {
        if let Ok(content) = fs::read_to_string("src/ffi.rs.bak") {
            if let Some(idx) = content.find("pub mod ffi_inner {") {
                let s = idx + "pub mod ffi_inner {".len();
                let mut d = 1; let mut i = s;
                while i < content.len() && d > 0 {
                    if content.as_bytes()[i] == b'{' { d += 1; }
                    else if content.as_bytes()[i] == b'}' { d -= 1; }
                    i += 1;
                }
                body = content[s..i-1].to_string();
            }
        }
    }

    let out = format!(
        "// Auto-generated by build.rs — do not edit.\n\
         // Concatenated from src/ffi/*.rs\n\n\
         #[cxx::bridge]\n\
         pub mod ffi_inner {{\n{}\n}}\n\n\
         pub use ffi_inner::*;\n",
        body
    );
    fs::write(&out_path, out).unwrap();
    out_path
}

/// Find Qt installation and version
fn find_qt() -> Option<(String, String)> {
    for cmd in &["qmake6", "qmake", "qmake-qt5"] {
        let version_output = Command::new(cmd)
            .args(["-query", "QT_VERSION"])
            .output()
            .ok()?;

        if !version_output.status.success() {
            continue;
        }

        let version = String::from_utf8_lossy(&version_output.stdout).trim().to_string();
        if !version.starts_with("5.") && !version.starts_with("6.") {
            continue;
        }

        let path_output = Command::new(cmd)
            .args(["-query", "QT_INSTALL_PREFIX"])
            .output()
            .ok()?;

        if path_output.status.success() {
            let path = String::from_utf8_lossy(&path_output.stdout).trim().to_string();
            if !path.is_empty() && PathBuf::from(&path).exists() {
                return Some((path, version));
            }
        }
    }
    None
}

/// Run qmake -query
fn qmake_query(key: &str) -> Option<String> {
    for cmd in &["qmake6", "qmake", "qmake-qt5"] {
        let output = Command::new(cmd)
            .args(["-query", key])
            .output()
            .ok()?;

        if output.status.success() {
            let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    None
}