use qtrs::dialog;
use qtrs::prelude::*;
use qtrs::splitter::HORIZONTAL;
use std::cell::RefCell;
use std::rc::Rc;

const DEFAULT_MARKDOWN: &str = "# Markdown Editor

Welcome to the **Markdown Editor**!
Edit your markdown on the left and see a live preview on the right.

## Features

- **Bold** and *italic* text
- `inline code` and code blocks
- Ordered and unordered lists
- [Links](https://example.com) and images

### Code Block

```rust
fn main() {
    println!(\"Hello, Markdown!\");
}
```

### Blockquote

> Markdown is a lightweight markup language
> with plain-text formatting syntax.

---

Happy editing!
";

fn markdown_to_html(md: &str) -> String {
    let parser = pulldown_cmark::Parser::new(md);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}

fn main() {
    let app = Application::new();

    let window = Rc::new(
        MainWindow::new()
            .window_title("Markdown Editor")
            .size(900, 600)
            .build(),
    );

    let initial_html = markdown_to_html(DEFAULT_MARKDOWN);

    // Preview widget — will be set as the child of a ScrollArea.
    let preview = Rc::new(RefCell::new(
        TextBrowser::new()
            .html(&initial_html)
            .build(),
    ));

    // Editor widget — will be set as the child of a ScrollArea.
    let text_edit = Rc::new(RefCell::new(
        PlainTextEdit::new()
            .text(DEFAULT_MARKDOWN)
            .build(),
    ));

    // Live preview: every keystroke re-renders markdown into the preview pane.
    text_edit.borrow_mut().connect_text_changed({
        let text_edit = Rc::clone(&text_edit);
        let preview = Rc::clone(&preview);
        move || {
            let md = text_edit.borrow().plain_text();
            let html = markdown_to_html(&md);
            preview.borrow().set_html(&html);
        }
    });

    let file_path = Rc::new(RefCell::new(None::<String>));

    // ---- File menu ----
    let file_menu = Menu::new("File")
        .action("New", {
            let text_edit = Rc::clone(&text_edit);
            let file_path = Rc::clone(&file_path);
            move || {
                text_edit.borrow().set_plain_text("");
                *file_path.borrow_mut() = None;
            }
        })
        .action("Open...", {
            let text_edit = Rc::clone(&text_edit);
            let file_path = Rc::clone(&file_path);
            move || {
                if let Some(path) = FileDialog::open_file(
                    None,
                    "Open File",
                    "",
                    "Markdown Files (*.md);;Text Files (*.txt);;All Files (*.*)",
                ) {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        text_edit.borrow().set_plain_text(&content);
                        *file_path.borrow_mut() = Some(path);
                    }
                }
            }
        })
        .action("Save", {
            let text_edit = Rc::clone(&text_edit);
            let file_path = Rc::clone(&file_path);
            move || {
                let path = file_path.borrow().clone();
                match path {
                    Some(p) => {
                        let content = text_edit.borrow().plain_text();
                        let _ = std::fs::write(&p, content);
                    }
                    None => {
                        if let Some(new_path) = FileDialog::save_file(
                            None,
                            "Save File",
                            "",
                            "Markdown Files (*.md);;Text Files (*.txt);;All Files (*.*)",
                        ) {
                            let content = text_edit.borrow().plain_text();
                            let _ = std::fs::write(&new_path, content);
                            *file_path.borrow_mut() = Some(new_path);
                        }
                    }
                }
            }
        })
        .action("Save As...", {
            let text_edit = Rc::clone(&text_edit);
            let file_path = Rc::clone(&file_path);
            move || {
                if let Some(path) = FileDialog::save_file(
                    None,
                    "Save File As",
                    "",
                    "Markdown Files (*.md);;Text Files (*.txt);;All Files (*.*)",
                ) {
                    let content = text_edit.borrow().plain_text();
                    let _ = std::fs::write(&path, content);
                    *file_path.borrow_mut() = Some(path);
                }
            }
        })
        .action("About", || dialog::information(None, "About", "TextEditor v0.1.0"))
        .action("Exit", || std::process::exit(0))
        .parent(&*window)
        .build();

    let menu_bar = MenuBar::new()
        .add_menu(file_menu)
        .parent(&*window)
        .build();

    window.set_menu_bar(&menu_bar);

    // ---- Build the split-pane layout without unsafe ----
    //
    // Qt containers (Splitter, layouts) take ownership of widgets via Box.
    // That conflicts with keeping Rc handles for callbacks.  To work around
    // this we wrap each content widget in a ScrollArea, move only the
    // ScrollAreas into the Splitter, and let the content widgets stay in Rc.
    //
    // ScrollArea::set_widget takes &dyn AsWidget (a shared reference), so
    // the content widgets remain owned by their Rc handles.

    // Tell the Rust wrappers that Qt now manages the C++ lifetimes.
    text_edit.borrow_mut().set_has_parent();
    preview.borrow_mut().set_has_parent();

    // Left pane: scrollable editor.
    let left_scroll = ScrollArea::new()
        .set_widget_resizable(true)
        .build();
    left_scroll.set_widget(&*text_edit.borrow());

    // Right pane: scrollable preview.
    let right_scroll = ScrollArea::new()
        .set_widget_resizable(true)
        .build();
    right_scroll.set_widget(&*preview.borrow());

    let mut splitter = Splitter::new(HORIZONTAL).build();
    splitter.add_widget(Box::new(left_scroll));
    splitter.add_widget(Box::new(right_scroll));

    window.set_central_widget(&splitter);
    window.show();
    app.exec();
}
