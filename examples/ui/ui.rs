//! `.ui` file example — load + find + connect signals
//!
//! ```bash
//! cargo run --example ui
//! ```

use qtrs::prelude::*;

fn main() {
    let app = Application::new();

    let loader = UiLoader::new();
    let window = loader.load("examples/ui/ui.ui", None);

    match window {
        Some(w) => {
            // Find the button by its objectName (set in Qt Designer)
            if let Some(FoundWidget::PushButton(mut btn)) =
                w.find(WidgetKind::PushButton, "pushButton") {
                btn.connect_clicked(|| {
                    println!("Hello, World!");
                });
            }

            w.show();
            app.exec();
        }
        None => {
            eprintln!("Failed to load examples/ui/ui.ui");
            std::process::exit(1);
        }
    }
}
