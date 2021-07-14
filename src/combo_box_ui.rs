use gio::ListStore;
use glib::StaticType;
use gtk::prelude::{ComboBoxExt, ComboBoxExtManual, ComboBoxTextExt, ContainerExt, GtkListStoreExt, GtkListStoreExtManual, WidgetExt};

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

        let label_combo = gtk::ComboBox::builder()
        .tooltip_text("Combo box built with label widgets")
        .margin(WIDGET_PADDING)
        .build();

        let store = gtk::ListStore::new(&[String::static_type()]);

        for _ in 0..10 {
            store.set(&store.append(), &[(0, &"I'm a child item")]);
            //store.insert_with_values( Some(i), &[(0, &"I'm a item of the list")]);
        }

        label_combo.set_model(Some(&store));

        _box.add(&simple_combo);
        _box.add(&label_combo);
        Self { _box }
    }
}
