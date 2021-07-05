use gtk::{
    prelude::{ContainerExt, EntryExt},
    EditableSignals, Entry,
};

const MARGIN: i32 = 5;
pub struct EntryBox {
    pub _box: gtk::Box,
}

impl EntryBox {
    pub fn new() -> Self {
        let _box = gtk::Box::new(gtk::Orientation::Vertical, 5);

        let simple_entry = Entry::builder()
            .margin(MARGIN)
            .text("Initial Text")
            .tooltip_text("Simple entry")
            .build();

        let righ_align_entry = Entry::builder()
            .tooltip_text("Right aligned text entry")
            .margin(MARGIN)
            .xalign(1f32)
            .text("Text is right aligned")
            .build();

        let limited_entry = Entry::builder()
            .tooltip_text("Limited 3 chars entry")
            .margin(5)
            .max_length(3)
            .text("123")
            .build();

        let placeholder_entry = Entry::builder()
            .margin(MARGIN)
            .tooltip_text("Placeholder entry")
            .placeholder_text("Please fill with information")
            .build();

        let invisible_char_entry = Entry::builder()
            .margin(MARGIN)
            .text("Invisible text entry")
            .visibility(false)
            .tooltip_text("Invisible char entry")
            .invisible_char('\u{2022}' as u32)
            .build();

        let custom_action_entry = Entry::builder()
            .margin(MARGIN)
            .text("Press icon to find something")
            .secondary_icon_name("edit-find-symbolic")
            .build();

        custom_action_entry.connect_icon_release(move |_, _, _| {
            println!("icon find pressed");
        });

        let progress_entry = Entry::builder()
            .tooltip_text("A entry with 20 char max limit and a progress counter")
            .margin(MARGIN)
            .placeholder_text("Progress entry")
            .progress_fraction(0f64)
            .max_length(20)
            .build();

        progress_entry.connect_changed(move |e| {
            let progress = e.text().len() as f64 / e.max_length() as f64;
            e.set_progress_fraction(progress);
        });

        _box.add(&simple_entry);
        _box.add(&righ_align_entry);
        _box.add(&limited_entry);
        _box.add(&placeholder_entry);
        _box.add(&invisible_char_entry);
        _box.add(&custom_action_entry);
        _box.add(&progress_entry);

        EntryBox { _box }
    }
}
