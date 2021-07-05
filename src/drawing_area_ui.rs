use std::f64::consts::PI;

use gtk::{DrawingArea, Inhibit, cairo::{FontSlant, FontWeight}, prelude::{ContainerExt, WidgetExt}};

const MARGIN: i32 = 5;
pub struct DrawingAreaBox {
    pub _box: gtk::Box,
}

impl DrawingAreaBox {
    pub fn new() -> Self {
        let _box = gtk::Box::new(gtk::Orientation::Vertical, MARGIN);
        let area = DrawingArea::builder()
            .height_request(500)
            .build();

        area.connect_draw(move |_, cr| {
            cr.scale(500f64, 500f64);

            cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
            cr.set_font_size(0.35);

            cr.move_to(0.04, 0.53);
            cr.show_text("Hello").expect("Invalid cairo surface state");

            cr.move_to(0.27, 0.65);
            cr.text_path("void");
            cr.set_source_rgb(0.5, 0.5, 1.0);
            cr.fill_preserve().expect("Invalid cairo surface state");
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.set_line_width(0.01);
            cr.stroke().expect("Invalid cairo surface state");

            cr.set_source_rgba(1.0, 0.2, 0.2, 0.6);
            cr.arc(0.04, 0.53, 0.02, 0.0, PI * 2.);
            cr.arc(0.27, 0.65, 0.02, 0.0, PI * 2.);
            cr.fill().expect("Invalid cairo surface state");

            Inhibit(false)
        });

        _box.add(&area);
        Self { _box }
    }
}
