use cascade::cascade;
use gtk::{
    prelude::{ContainerExt, StyleContextExt, WidgetExt},
    Button, Image,
};

pub struct ButtonBox {
    pub _box: gtk::Box,
    pub btn_counter: Button,
}

impl ButtonBox {
    pub fn new() -> Self {
        let _box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let hbox1 = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        let btn_destrictive_action = cascade! {
            Button::with_label("Destructive action");
            ..style_context()
            .add_class("destructive-action");
            ..set_margin(5);
        };

        let btn_suggested_action = cascade! {
            Button::with_label("Suggested action");
            ..style_context()
            .add_class("suggested-action");
            ..set_margin(5);
        };

        let btn_lock = cascade! {
             gtk::LockButtonBuilder::new()
                .text_lock("Lock")
                .text_unlock("Unlock")
                .build();
            ..set_margin(5);
        };

        let image = Image::from_icon_name(Some("document-new-symbolic"), gtk::IconSize::Button);
        let btn_image = Button::builder()
            .label("With image")
            .always_show_image(true)
            .image(&image)
            .margin(5)
            .build();

        let btn_counter = Button::builder().label("Counter: 0").margin(5).build();

        hbox.add(&btn_destrictive_action);
        hbox.add(&btn_suggested_action);
        hbox.add(&btn_lock);
        hbox.add(&btn_image);

        hbox1.add(&btn_counter);

        _box.add(&hbox);
        _box.add(&hbox1);
        ButtonBox { _box, btn_counter }
    }
}
