use std::{cell::Cell, rc::Rc};

use gtk::{
    prelude::{ContainerExt, ProgressBarExt, StyleContextExt, WidgetExt},
    Adjustment, Application, ApplicationWindow, Button, Label, Orientation, ProgressBar, Scale,
};

pub struct Ui {
    pub window: ApplicationWindow,
    pub button_increase: Button,
    pub button_decrease: Button,
    pub label: Label,
    pub progress: ProgressBar,
    pub scale: Scale,
    pub button_file_chooser: Button,
    pub button_color_chooser: Button,
    pub button_drag_me: Button,
    pub label_drop: Label,
    pub counter: Rc<Cell<i32>>,
}

impl Ui {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(300)
            .default_height(200)
            .title("Gtk Demo")
            .build();

        let button_increase = Button::with_label("Increase");
        button_increase.set_margin_start(10);
        button_increase.set_margin_top(10);
        button_increase.set_margin_end(10);
        let button_decrease = Button::with_label("Decrease");
        button_decrease.set_margin_start(10);
        button_decrease.set_margin_end(10);

        let label = Label::new(Some("Count: 0"));

        let progress = ProgressBar::default();
        progress.set_fraction(0.8);
        progress.set_margin_end(10);
        progress.set_margin_start(10);

        let adjustment = Adjustment::builder()
            .value(0f64)
            .lower(0f64)
            .upper(1f64)
            .build();
        let scale = Scale::builder()
            .orientation(Orientation::Horizontal)
            .adjustment(&adjustment)
            .build();

        let gtk_box = gtk::Box::new(Orientation::Vertical, 10);
        let box_dialogs = gtk::Box::new(Orientation::Horizontal, 10);

        let button_file_chooser = Button::with_label("Open File Chooser");
        button_file_chooser.set_margin_start(10);
        button_file_chooser.set_margin_end(10);
        button_file_chooser.set_margin_bottom(10);

        let button_color_chooser = Button::with_label("Open Color Chooser");
        button_color_chooser.set_margin_bottom(10);
        button_color_chooser.set_margin_end(10);

        let button_destructive_action = Button::with_label("Destructive action");
        button_destructive_action.set_margin_bottom(10);
        button_destructive_action.set_margin_end(10);

        let button_suggested_action = Button::with_label("Informative action");
        button_suggested_action.set_margin_bottom(10);
        button_suggested_action.set_margin_end(10);

        button_destructive_action
            .style_context()
            .add_class("destructive-action");
        button_suggested_action
            .style_context()
            .add_class("suggested-action");

        box_dialogs.add(&button_file_chooser);
        box_dialogs.add(&button_color_chooser);
        box_dialogs.add(&button_destructive_action);
        box_dialogs.add(&button_suggested_action);

        let box_drag_drop = gtk::Box::new(Orientation::Horizontal, 10);
        let button_drag_me = Button::with_label("Drag me");
        button_drag_me.set_margin(10);

        let label_drop = Label::new(Some("Drop here"));
        label_drop.set_margin(10);
        
        box_drag_drop.add(&button_drag_me);
        box_drag_drop.add(&label_drop);

        window.add(&gtk_box);
        gtk_box.add(&button_increase);
        gtk_box.add(&button_decrease);
        gtk_box.add(&label);
        gtk_box.add(&progress);
        gtk_box.add(&scale);
        gtk_box.add(&box_dialogs);
        gtk_box.add(&box_drag_drop);

        window.show_all();

        Ui {
            window,
            button_increase,
            button_decrease,
            label,
            progress,
            scale,
            button_file_chooser,
            button_color_chooser,
            button_drag_me,
            label_drop,
            counter: Rc::new(Cell::new(0)),
        }
    }
}
