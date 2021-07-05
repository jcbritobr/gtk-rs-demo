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
                let new_widget = &button_box._box;
                ButtonUiController::setup(&button_box);
                paned.pack2(new_widget, true, false);
                new_widget.show_all();
            }
            ExampleType::CustomCellRenderer => {
                let custom_cell_renderer_box = CustomCellRendererBox::new();
                let new_widget = &custom_cell_renderer_box._box;
                paned.pack2(new_widget, true, false);
                new_widget.show_all();
            }
            ExampleType::ComboBox => {
                let combo_box = ComboBoxBox::new();
                let new_widget = &combo_box._box;
                paned.pack2(new_widget, true, false);
                new_widget.show_all();
            }
            ExampleType::DrawingArea => {
                let drawing_box = DrawingAreaBox::new();
                let new_widget = &drawing_box._box;
                paned.pack2(new_widget, true, false);
                new_widget.show_all();
            }
            ExampleType::Entry => {
                let entry_box = EntryBox::new();
                let new_widget = &entry_box._box;
                paned.pack2(new_widget, true, false);
                new_widget.show_all();
            }
        }
    });
}
