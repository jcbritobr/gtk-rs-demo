use std::{cell::Cell, rc::Rc};

use gtk::prelude::{ButtonExt};

use crate::button_ui::ButtonBox;

pub struct ButtonUiController {}

impl ButtonUiController {
    pub fn setup(ui: &ButtonBox) {
        on_btn_counter_clicked(ui);
    }
}

fn on_btn_counter_clicked(ui: &ButtonBox) {
    let counter = Rc::new(Cell::new(0));
    let button = ui.btn_counter.clone();

    button.connect_clicked(move |btn| {
        counter.set(counter.get() + 1);
        btn.set_label(&format!("Counter: {}", counter.get()));
    });
}
