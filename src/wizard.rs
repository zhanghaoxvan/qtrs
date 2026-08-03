//! Wizard dialog for multi-step workflows.
//! Wraps [`QWizard`](https://doc.qt.io/qt-6/qwizard.html)
//! and [`QWizardPage`](https://doc.qt.io/qt-6/qwizardpage.html).

use cxx::let_cxx_string;
use crate::ffi;
use crate::widget::AsWidget;

// --- QWizardPage ---
pub struct WizardPage { ptr: *mut ffi::QWizardPage, has_parent: bool }
impl WizardPage {
    pub fn new() -> WizardPageBuilder { WizardPageBuilder::new() }
    pub fn set_title(&self, title: &str) {
        let_cxx_string!(c = title);
        unsafe { ffi::QWizardPage_setTitle(self.ptr, &c); }
    }
    pub fn set_sub_title(&self, sub: &str) {
        let_cxx_string!(c = sub);
        unsafe { ffi::QWizardPage_setSubTitle(self.ptr, &c); }
    }
}
impl AsWidget for WizardPage {
    fn widget_ptr(&self) -> *mut ffi::QWidget { unsafe { ffi::toQWidget_QWizardPage(self.ptr) } }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}
impl Drop for WizardPage {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if !self.has_parent { unsafe { ffi::QWizardPage_delete(self.ptr); } }
        self.ptr = std::ptr::null_mut();
    }
}
pub struct WizardPageBuilder {
    title: Option<String>, sub_title: Option<String>, parent: Option<*mut ffi::QWidget>,
}
impl WizardPageBuilder {
    fn new() -> Self { Self { title: None, sub_title: None, parent: None } }
    pub fn title(mut self, t: impl Into<String>) -> Self { self.title = Some(t.into()); self }
    pub fn sub_title(mut self, t: impl Into<String>) -> Self { self.sub_title = Some(t.into()); self }
    pub fn parent(mut self, p: &dyn AsWidget) -> Self { self.parent = Some(p.widget_ptr()); self }
    pub fn build(self) -> WizardPage {
        let ptr = unsafe { ffi::QWizardPage_new(self.parent.unwrap_or(std::ptr::null_mut())) };
        assert!(!ptr.is_null());
        let wp = WizardPage { ptr, has_parent: self.parent.is_some() };
        if let Some(ref t) = self.title { wp.set_title(t); }
        if let Some(ref t) = self.sub_title { wp.set_sub_title(t); }
        wp
    }
}

// --- QWizard ---
pub struct Wizard { ptr: *mut ffi::QWizard, has_parent: bool }
impl Wizard {
    pub fn new() -> WizardBuilder { WizardBuilder::new() }
    pub fn add_page(&self, page: &WizardPage) {
        unsafe { ffi::QWizard_addPage(self.ptr, page.ptr); }
    }
    pub fn set_window_title(&self, title: &str) {
        let_cxx_string!(c = title);
        unsafe { ffi::QWizard_setWindowTitle(self.ptr, &c); }
    }
    pub fn next(&self) { unsafe { ffi::QWizard_next(self.ptr); } }
    pub fn back(&self) { unsafe { ffi::QWizard_back(self.ptr); } }
    pub fn restart(&self) { unsafe { ffi::QWizard_restart(self.ptr); } }
    pub fn current_id(&self) -> i32 { unsafe { ffi::QWizard_currentId(self.ptr) } }
}
impl AsWidget for Wizard {
    fn widget_ptr(&self) -> *mut ffi::QWidget { unsafe { ffi::toQWidget_QWizard(self.ptr) } }
    fn set_has_parent(&mut self) { self.has_parent = true; }
}
impl Drop for Wizard {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        if !self.has_parent { unsafe { ffi::QWizard_delete(self.ptr); } }
        self.ptr = std::ptr::null_mut();
    }
}
pub struct WizardBuilder { title: Option<String>, parent: Option<*mut ffi::QWidget> }
impl WizardBuilder {
    fn new() -> Self { Self { title: None, parent: None } }
    pub fn window_title(mut self, t: impl Into<String>) -> Self { self.title = Some(t.into()); self }
    pub fn parent(mut self, p: &dyn AsWidget) -> Self { self.parent = Some(p.widget_ptr()); self }
    pub fn build(self) -> Wizard {
        let ptr = unsafe { ffi::QWizard_new(self.parent.unwrap_or(std::ptr::null_mut())) };
        assert!(!ptr.is_null());
        let w = Wizard { ptr, has_parent: self.parent.is_some() };
        if let Some(ref t) = self.title { w.set_window_title(t); }
        w
    }
}
