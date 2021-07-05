const WIDGET_PADDING: i32 = 5;

pub struct CustomCellRendererBox {
    pub _box: gtk::Box
}

impl CustomCellRendererBox {
    pub fn new() -> Self {
        let _box = gtk::Box::new(gtk::Orientation::Vertical, WIDGET_PADDING);
        Self {
            _box
        }
    }
}