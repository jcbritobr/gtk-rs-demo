mod button_ui;
mod button_ui_controller;
mod entry_ui;
mod model;
mod ui;
mod uicontroller;
mod drawing_area_ui;

use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual},
    Application,
};
use ui::Ui;
use uicontroller::UiController;

fn main() {
    let app = Application::new(Some("org.gard.training"), Default::default());
    app.connect_activate(|app| {
        let ui = Ui::new(&app);
        UiController::setup(&ui);
    });
    app.run();
}
