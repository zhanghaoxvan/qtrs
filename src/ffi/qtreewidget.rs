unsafe extern "C++" {
        include!("src/cpp/qt_widget.h");
// --- QTreeWidget ---
        unsafe fn QTreeWidget_new(parent: *mut QWidget) -> *mut QTreeWidget;
        unsafe fn QTreeWidget_delete(w: *mut QTreeWidget);
        unsafe fn QTreeWidget_addTopLevelItem(w: *mut QTreeWidget, text: &CxxString);
        unsafe fn QTreeWidget_clear(w: *mut QTreeWidget);
        unsafe fn QTreeWidget_currentItemText(w: *mut QTreeWidget) -> String;
        unsafe fn QTreeWidget_setHeaderLabel(w: *mut QTreeWidget, text: &CxxString);
        unsafe fn QTreeWidget_setHeaderLabels(w: *mut QTreeWidget, labels: Vec<String>);
        unsafe fn QTreeWidget_expandAll(w: *mut QTreeWidget);
        unsafe fn QTreeWidget_collapseAll(w: *mut QTreeWidget);
        unsafe fn QTreeWidget_expandItem(w: *mut QTreeWidget, text: &CxxString);
        unsafe fn QTreeWidget_setCurrentItem(w: *mut QTreeWidget, text: &CxxString);
        unsafe fn QTreeWidget_topLevelItemCount(w: *mut QTreeWidget) -> i32;
        unsafe fn QTreeWidget_onItemClicked(w: *mut QTreeWidget, ctx: u64);
        unsafe fn QTreeWidget_onItemDoubleClicked(w: *mut QTreeWidget, ctx: u64);
        unsafe fn QTreeWidget_onItemExpanded(w: *mut QTreeWidget, ctx: u64);
        unsafe fn QTreeWidget_onItemCollapsed(w: *mut QTreeWidget, ctx: u64);
        unsafe fn QTreeWidget_onCurrentItemChanged(w: *mut QTreeWidget, ctx: u64);
    }
