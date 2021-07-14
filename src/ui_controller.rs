use glib::IsA;
use gtk::{Paned, Widget};
use gtk::prelude::{
    ContainerExt, PanedExt, TreeModelExt, TreeSelectionExt, TreeViewExt, WidgetExt,
};

use crate::button_ui::*;
use crate::button_ui_controller::ButtonUiController;
use crate::combo_box_ui::ComboBoxBox;
use crate::custom_cell_renderer_ui::CustomCellRendererBox;
use crate::drawing_area_ui::DrawingAreaBox;
use crate::entry_ui::*;
use crate::{model::ExampleType, ui::Ui};

pub struct UiController {}

impl UiController {
    pub fn setup(ui: &Ui) {
        on_tree_clicked(ui);
    }
}

fn on_tree_clicked(ui: &Ui) {
    let tree = ui.tree_content.clone();
    let selection = tree.selection();
    let paned = ui.paned.clone();

    selection.connect_changed(move |s| {
        let (model, iter) = s.selected().expect("cant get selection");
        let path = model.path(&iter).expect("cant get path");
        let widget = paned
            .child2()
            .expect("can't extract the widget from paned component");
        paned.remove(&widget);

        match ExampleType::from_number(
            path.to_string()
                .parse::<usize>()
                .expect("cant convert usize for enum"),
        ) {
            ExampleType::Button => {
                let button_box = ButtonBox::new();
                ButtonUiController::setup(&button_box);
                update_panel(&paned, &button_box._box);
            }
            ExampleType::CustomCellRenderer => {
                let custom_cell_renderer_box = CustomCellRendererBox::new();
                update_panel(&paned, &custom_cell_renderer_box._box);
            }
            ExampleType::ComboBox => {
                let combo_box = ComboBoxBox::new();
                update_panel(&paned, &combo_box._box);
            }
            ExampleType::DrawingArea => {
                let drawing_box = DrawingAreaBox::new();
                update_panel(&paned, &drawing_box._box);
            }
            ExampleType::Entry => {
                let entry_box = EntryBox::new();
                update_panel(&paned, &entry_box._box);
            }
        }
    });
}

fn update_panel<W: IsA<Widget>>(paned: &Paned, cbox: &W) {
    paned.pack2(cbox, true, false);
    cbox.show_all();
}
