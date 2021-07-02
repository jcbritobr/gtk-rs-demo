use glib::clone;
use gtk::{
    gdk,
    prelude::{
        ButtonExt, ColorChooserExt, DialogExt, DialogExtManual, FileChooserExt, GtkWindowExt,
        LabelExt, ProgressBarExt, RangeExt, WidgetExt, WidgetExtManual,
    },
    ColorChooserDialog, FileChooserDialog, ResponseType, TargetFlags,
};

use crate::ui::Ui;

pub struct UiController {}

impl UiController {
    pub fn setup(ui: &Ui) {
        on_button_increase_click(ui);
        on_button_decrease_click(ui);
        on_scale_value_changed(ui);
        on_button_open_file_chooser_click(ui);
        on_button_open_color_chooser_click(ui);
        on_setup_drag_and_drop(ui);
    }
}

fn on_setup_drag_and_drop(ui: &Ui) {
    let button = ui.button_drag_me.clone();
    let label = ui.label_drop.clone();

    let targets = vec![
        gtk::TargetEntry::new("STRING", TargetFlags::SAME_APP, 0),
        gtk::TargetEntry::new("text/plain", TargetFlags::SAME_APP, 0),
    ];

    button.drag_source_set(
        gdk::ModifierType::MODIFIER_MASK,
        &targets,
        gdk::DragAction::COPY,
    );

    button.connect_drag_data_get(|_, _, s, _, _| {
        let data = "I'm data!";
        s.set_text(data);
    });

    button.connect_clicked(clone!(@weak label => move |_|{
        label.set_text("Drop here");
    }));

    label.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);
    label.connect_drag_data_received(|w, _, _, _, s, _, _| {
        w.set_text(&s.text().expect("Couldn't get text"));
    });
}

fn on_button_open_color_chooser_click(ui: &Ui) {
    let button = ui.button_color_chooser.clone();
    let win = ui.window.clone();

    button.connect_clicked(move |_| {
        let chooser = ColorChooserDialog::new(Some("Color Chooser"), Some(&win));
        chooser.connect_response(move |cc, r| {
            if r == ResponseType::Ok {
                println!("get choosed color ...");
                let value = cc.rgba();
                println!("{}", value);
            }
            cc.close();
        });
        chooser.show_all();
    });
}

fn on_button_open_file_chooser_click(ui: &Ui) {
    let button = ui.button_file_chooser.clone();
    let win = ui.window.clone();
    button.connect_clicked(move |_| {
        let file_chooser =
            FileChooserDialog::new(Some("Open File"), Some(&win), gtk::FileChooserAction::Open);
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);
        file_chooser.connect_response(move |fc, r| {
            if r == ResponseType::Ok {
                if let Some(filename) = fc.filename() {
                    println!("{}", filename.to_string_lossy());
                }
            }
            fc.close();
        });
        file_chooser.show_all();
    });
}

fn on_scale_value_changed(ui: &Ui) {
    let scale = ui.scale.clone();
    let progress = ui.progress.clone();

    scale.connect_value_changed(move |s| {
        progress.set_fraction(s.value());
    });
}

fn on_button_increase_click(ui: &Ui) {
    let number = ui.counter.clone();
    let label = ui.label.clone();

    ui.button_increase
        .connect_clicked(clone!(@strong number => move |_|{
            number.set(number.get() + 1);
            label.set_text(&format!("Count: {}", number.get()));
        }));
}

fn on_button_decrease_click(ui: &Ui) {
    let number = ui.counter.clone();
    let label = ui.label.clone();

    ui.button_decrease
        .connect_clicked(clone!(@strong number => move |_|{
            number.set(number.get() - 1);
            label.set_text(&format!("Count: {}", number.get()));
        }));
}
