use gtk::prelude::{ComboBoxExtManual, ComboBoxTextExt, ContainerExt};

const WIDGET_PADDING: i32 = 5;

pub struct ComboBoxBox {
    pub _box: gtk::Box,
}

impl ComboBoxBox {
    pub fn new() -> Self {
        let _box = gtk::Box::new(gtk::Orientation::Vertical, WIDGET_PADDING);
        let simple_combo = gtk::ComboBoxText::builder()
            .tooltip_text("Simple combo boxt text")
            .margin(WIDGET_PADDING)
            .build();

        simple_combo.append_text("Item 1");
        simple_combo.append_text("Item 2");
        simple_combo.append_text("Item 3");
        simple_combo.set_active(Some(0));

        _box.add(&simple_combo);
        Self { _box }
    }
}
