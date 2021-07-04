use gtk::prelude::{
    ContainerExt, PanedExt, TreeModelExt, TreeSelectionExt, TreeViewExt, WidgetExt,
};

use crate::buttonui::*;
use crate::button_ui_controller::ButtonUiController;
use crate::entryui::*;
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
            ExampleType::CustomCellRenderer => {}
            ExampleType::ComboBox => {}
            ExampleType::DrawingArea => {}
            ExampleType::Entry => {
                let entry_box = EntryBox::new();
                let new_widget = &entry_box._box;
                paned.pack2(new_widget, true, false);
                new_widget.show_all();
            }
        }
    });
}
