//! `.ui` file loading example
//!
//! Loads a Qt Designer XML file at runtime via `UiLoader` and displays
//! the resulting widget tree.
//!
//! ```bash
//! cargo run --example ui --features ui
//! ```

use qtrs::prelude::*;

fn main() {
    let app = Application::new();

    // Create the loader and load the .ui file
    let loader = UiLoader::new();
    let window = loader.load("examples/ui/ui.ui", None);

    match window {
        Some(w) => {
            w.show();
            app.exec();
        }
        None => {
            eprintln!("Failed to load examples/ui/ui.ui");
            std::process::exit(1);
        }
    }
}
