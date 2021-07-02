mod ui;
mod uicontroller;

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
