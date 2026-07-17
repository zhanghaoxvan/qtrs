use qtrs::*;

fn main() {
    let app = Application::new();

    // Build a top-level window
    let mut window = Widget::new()
        .title("Hello, qtrs!")
        .size(400, 300)
        .icon("assets/icon.png")
        .build();

    // Put widgets in a vertical layout
    let mut layout = VBoxLayout::with_parent(&window);

    let btn = PushButton::new("Click me")
        .on_clicked(|| println!("clicked!"))
        .build();
    let label = Label::new("Welcome!").build();

    // Layout takes ownership of widgets
    layout.add(btn);
    layout.add(label);

    // Install layout, then show the window
    window.set_layout(&layout);
    window.show();

    // Enter the Qt event loop
    app.exec();
}
