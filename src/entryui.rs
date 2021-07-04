use gtk::{Button, prelude::ContainerExt};

pub struct EntryBox {
    pub _box: gtk::Box,
}

impl EntryBox {
    pub fn new() -> Self {
        let _box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let simple_button = Button::with_label("Drag-me");
        _box.add(&simple_button);
        EntryBox{
            _box
        }
    }
}