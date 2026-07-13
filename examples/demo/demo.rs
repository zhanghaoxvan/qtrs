// examples/demo.rs — qtrs widget gallery demo
//
// Run: cargo run --example demo

use qtrs::prelude::*;
use qtrs::dialog;
use qtrs::signals::slider_signals::VALUE_CHANGED;
use qtrs::signals::spin_box_slots::SET_VALUE;

fn main() {
    let app = Application::new();

    // ============================================================
    // Main window
    // ============================================================
    let window = Widget::new()
        .title("qtrs Widget Gallery")
        .icon("assets/icon.png")
        .size(900, 700)
        .build();

    // ============================================================
    // Main layout
    // ============================================================
    let mut main_layout = VBoxLayout::with_parent(&window);
    main_layout.set_spacing(10);
    main_layout.set_contents_margins(15, 15, 15, 15);

    // ============================================================
    // Title
    // ============================================================
    let title_label = Label::new("\nqtrs Widget Gallery")
        .parent(&window)
        .build();
    main_layout.add_widget(Box::new(title_label));
    
    // ============================================================
    // Section 1: Buttons
    // ============================================================
    let group1 = GroupBox::new("Buttons")
        .parent(&window)
        .build();
    let mut g1_layout = VBoxLayout::with_parent(&group1);
    g1_layout.set_spacing(8);

    // PushButton
    let btn = PushButton::new("Click Me")
        .on_clicked(|| println!("[LOG] Button clicked"))
        .parent(&group1)
        .build();
    g1_layout.add_widget(Box::new(btn));

    // CheckBox
    let cb = CheckBox::new("Check me")
        .checked(false)
        .on_toggled(|checked| println!("[LOG] CheckBox: {}", checked))
        .parent(&group1)
        .build();
    g1_layout.add_widget(Box::new(cb));

    // RadioButton group
    let rb1_label = Label::new("Radio Group (click to log):").parent(&group1).build();
    g1_layout.add_widget(Box::new(rb1_label));

    let rb1 = RadioButton::new("Option A")
        .checked(true)
        .on_toggled(|checked| {
            if checked {
                println!("[LOG] Radio selected: A");
            }
        })
        .parent(&group1)
        .build();
    g1_layout.add_widget(Box::new(rb1));

    let rb2 = RadioButton::new("Option B")
        .checked(false)
        .on_toggled(|checked| {
            if checked {
                println!("[LOG] Radio selected: B");
            }
        })
        .parent(&group1)
        .build();
    g1_layout.add_widget(Box::new(rb2));

    let rb3 = RadioButton::new("Option C")
        .checked(false)
        .on_toggled(|checked| {
            if checked {
                println!("[LOG] Radio selected: C");
            }
        })
        .parent(&group1)
        .build();
    g1_layout.add_widget(Box::new(rb3));

    main_layout.add_widget(Box::new(group1));

    // ============================================================
    // Section 2: Input widgets
    // ============================================================
    let group2 = GroupBox::new("Inputs")
        .parent(&window)
        .build();
    let mut g2_layout = VBoxLayout::with_parent(&group2);
    g2_layout.set_spacing(8);

    // LineEdit
    let edit = LineEdit::new("Type text here...")
        .on_return_pressed(|| println!("[LOG] LineEdit return pressed"))
        .parent(&group2)
        .build();
    g2_layout.add_widget(Box::new(edit));

    // ComboBox
    let mut combo = ComboBox::new()
        .items(&["Item 1", "Item 2", "Item 3", "Item 4"])
        .on_current_text_changed(|| println!("[LOG] ComboBox text changed"))
        .parent(&group2)
        .build();
    combo.connect_current_index_changed(|idx| {
        println!("[LOG] ComboBox index: {}", idx);
    });
    g2_layout.add_widget(Box::new(combo));

    // TextEdit
    let text_edit = TextEdit::new()
        .placeholder("Write multi-line text here...")
        .on_text_changed(|| println!("[LOG] TextEdit content changed"))
        .parent(&group2)
        .build();
    g2_layout.add_widget(Box::new(text_edit));

    main_layout.add_widget(Box::new(group2));

    // ============================================================
    // Section 3: Full Bidirectional Sync (Slider ↔ SpinBox → ProgressBar)
    // ============================================================
    let group3 = GroupBox::new("Values")
        .parent(&window)
        .build();
    let mut g3_layout = VBoxLayout::with_parent(&group3);
    g3_layout.set_spacing(8);

    // Build all widgets first (before adding to layout)
    let bar = ProgressBar::new()
        .range(0, 100)
        .value(50)
        .format("%p%")
        .parent(&group3)
        .build();

    let slider = Slider::horizontal()
        .range(0, 100)
        .parent(&group3)
        .build();
    slider.set_value(50);

    let spin = SpinBox::new()
        .range(0, 100)
        .value(50)
        .suffix(" units")
        .parent(&group3)
        .build();

    // ============================================================
    // ALL connections BEFORE adding to layout
    // ============================================================

    // Slider → SpinBox (bidirectional)
    slider.connect(
        VALUE_CHANGED,
        &spin,
        SET_VALUE,
        ConnType::Queued,
    );

    // SpinBox → Slider (bidirectional)
    spin.connect(
        VALUE_CHANGED,
        &slider,
        SET_VALUE,
        ConnType::Queued,
    );

    // Slider → ProgressBar
    slider.connect(
        VALUE_CHANGED,
        &bar,
        SET_VALUE,
        ConnType::Auto,
    );

    // SpinBox → ProgressBar
    spin.connect(
        VALUE_CHANGED,
        &bar,
        SET_VALUE,
        ConnType::Auto,
    );

    // ============================================================
    // NOW add widgets to layout (after all connections)
    // ============================================================
    g3_layout.add_widget(Box::new(bar));
    g3_layout.add_widget(Box::new(slider));
    g3_layout.add_widget(Box::new(spin));

    main_layout.add_widget(Box::new(group3));

    // ============================================================
    // Section 4: TabWidget
    // ============================================================
    let mut tabs = TabWidget::new()
        .on_current_changed(|idx| println!("[LOG] Tab changed to: {}", idx))
        .parent(&window)
        .build();

    // Tab 1: Simple text
    let tab1 = Widget::new().parent(&tabs).build();
    let mut tab1_layout = VBoxLayout::with_parent(&tab1);
    let tab1_label = Label::new("This is Tab 1").parent(&tab1).build();
    tab1_layout.add_widget(Box::new(tab1_label));
    tabs.add_tab(Box::new(tab1), "Tab 1");

    // Tab 2: CheckBoxes
    let tab2 = Widget::new().parent(&tabs).build();
    let mut tab2_layout = VBoxLayout::with_parent(&tab2);
    let cb1 = CheckBox::new("Option X").parent(&tab2).build();
    let cb2 = CheckBox::new("Option Y").parent(&tab2).build();
    tab2_layout.add_widget(Box::new(cb1));
    tab2_layout.add_widget(Box::new(cb2));
    tabs.add_tab(Box::new(tab2), "Tab 2");

    // Tab 3: Button
    let tab3 = Widget::new().parent(&tabs).build();
    let mut tab3_layout = VBoxLayout::with_parent(&tab3);
    let btn_tab3 = PushButton::new("Tab 3 Button")
        .on_clicked(|| println!("[LOG] Tab 3 button clicked"))
        .parent(&tab3)
        .build();
    tab3_layout.add_widget(Box::new(btn_tab3));
    tabs.add_tab(Box::new(tab3), "Tab 3");

    main_layout.add_widget(Box::new(tabs));

    // ============================================================
    // Section 5: Menu + MenuBar
    // ============================================================
    let file_menu = Menu::new("File")
        .action("New", || println!("[LOG] Menu: New"))
        .action("Open", || {
            println!("[LOG] Menu: Open");
            if let Some(path) = FileDialog::open_file(
                None,  // No parent needed
                "Select a file",
                "",
                "All Files (*.*);;Text Files (*.txt);;Images (*.png *.jpg *.bmp)"
            ) {
                println!("[LOG] File selected: {}", path);
            } else {
                println!("[LOG] File dialog cancelled");
            }
        })
        .action("Save", || {
            println!("[LOG] Menu: Save");
            if let Some(path) = FileDialog::select_directory(
                None, "Select a directory to save", 
                "",
            ) {
                println!("[LOG] Directory selected: {}", path);
            } else {
                println!("[LOG] File dialog cancelled");
            }
        })
        .action("Exit", || {
            println!("[LOG] Menu: Exit");
            std::process::exit(0);
        })
        .parent(&window)
        .build();

    let edit_menu = Menu::new("Edit")
        .action("Copy", || println!("[LOG] Menu: Copy"))
        .action("Paste", || println!("[LOG] Menu: Paste"))
        .action("Cut", || println!("[LOG] Menu: Cut"))
        .parent(&window)
        .build();

    let help_menu = Menu::new("Help")
        .action("About", || {
            println!("[LOG] Menu: About");
            dialog::information(
                None,  // No parent needed
                "About",
                "qtrs Widget Gallery\nVersion 0.3.0\n\nAll widgets test",
            );
        })
        .parent(&window)
        .build();

    let _menubar = MenuBar::new()
        .add_menu(file_menu)
        .add_menu(edit_menu)
        .add_menu(help_menu)
        .parent(&window)
        .build();

    // ============================================================
    // Bottom buttons: Dialogs
    // ============================================================
    let mut bottom_layout = HBoxLayout::new();
    bottom_layout.set_spacing(10);

    let info_btn = PushButton::new("Show Info")
        .on_clicked(|| {
            println!("[LOG] Info dialog shown");
            dialog::information(None, "Info", "This is an information dialog");
        })
        .parent(&window)
        .build();
    bottom_layout.add_widget(Box::new(info_btn));

    let warn_btn = PushButton::new("Show Warning")
        .on_clicked(|| {
            println!("[LOG] Warning dialog shown");
            dialog::warning(None, "Warning", "This is a warning dialog");
        })
        .parent(&window)
        .build();
    bottom_layout.add_widget(Box::new(warn_btn));

    let ask_btn = PushButton::new("Ask Question")
        .on_clicked(|| {
            println!("[LOG] Question dialog shown");
            let answer = dialog::question(None, "Question", "Are you sure?");
            println!("[LOG] Answer: {}", if answer { "Yes" } else { "No" });
        })
        .parent(&window)
        .build();
    bottom_layout.add_widget(Box::new(ask_btn));

    let mut bottom_widget = Widget::new().parent(&window).build();
    bottom_widget.set_hlayout(bottom_layout.layout_ptr());
    main_layout.add_widget(Box::new(bottom_widget));

    // ============================================================
    // Status bar
    // ============================================================
    let status_label = Label::new("Status: Ready - All widgets loaded")
        .parent(&window)
        .build();
    main_layout.add_widget(Box::new(status_label));

    // ============================================================
    // Show window and run
    // ============================================================
    window.show();
    println!("[LOG] Application started - all widgets loaded");
    println!("[LOG] Interact with widgets to see logs in terminal");

    app.exec();
}