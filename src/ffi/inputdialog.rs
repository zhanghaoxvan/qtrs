unsafe extern "C++" {
    include!("src/cpp/qt_widget.h");
    // --- QInputDialog ---
    unsafe fn QInputDialog_getText(
    parent: *mut QWidget,
    title: &CxxString,
    label: &CxxString,
    text: &CxxString,
    ) -> String;
    unsafe fn QInputDialog_getInt(
    parent: *mut QWidget,
    title: &CxxString,
    label: &CxxString,
    value: i32,
    min: i32,
    max: i32,
    step: i32,
    ) -> i32;
    unsafe fn QInputDialog_getDouble(
    parent: *mut QWidget,
    title: &CxxString,
    label: &CxxString,
    value: f64,
    min: f64,
    max: f64,
    decimals: i32,
    ) -> f64;
    unsafe fn QInputDialog_getItem(
    parent: *mut QWidget,
    title: &CxxString,
    label: &CxxString,
    items: Vec<String>,
    current: i32,
    editable: bool,
    ) -> String;
    }
