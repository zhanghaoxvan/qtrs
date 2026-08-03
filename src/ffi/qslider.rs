unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
// --- QSlider ---
        unsafe fn QSlider_new(orientation: i32, parent: *mut QWidget) -> *mut QSlider;
        unsafe fn QSlider_value(s: *mut QSlider) -> i32;
        unsafe fn QSlider_setValue(s: *mut QSlider, value: i32);
        unsafe fn QSlider_setRange(s: *mut QSlider, min: i32, max: i32);
        unsafe fn QSlider_setOrientation(s: *mut QSlider, o: i32);
        unsafe fn QSlider_orientation(s: *mut QSlider) -> i32;
        unsafe fn QSlider_setTickPosition(s: *mut QSlider, pos: i32);
        unsafe fn QSlider_setTickInterval(s: *mut QSlider, interval: i32);
        unsafe fn QSlider_invertedAppearance(s: *mut QSlider) -> bool;
        unsafe fn QSlider_delete(s: *mut QSlider);
        unsafe fn QSlider_onValueChanged(s: *mut QSlider, ctx: u64);
    }
