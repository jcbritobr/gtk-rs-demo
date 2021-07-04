use crate::model::ExampleType;
use cascade::cascade;
use glib::StaticType;
use gtk::{
    prelude::{
        CellLayoutExt, ContainerExt, GtkWindowExt, HeaderBarExt, PanedExt, TreeStoreExtManual,
        TreeViewExt, WidgetExt,
    },
    Application, ApplicationWindow, Button, CellRendererText, HeaderBar, Orientation, Paned,
    TreeStore, TreeView, TreeViewColumn,
};

pub struct Ui {
    pub window: ApplicationWindow,
    pub tree_content: TreeView,
    pub paned: Paned,
    pub current_widget: Option<gtk::Box>,
}

impl Ui {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(640)
            .default_height(480)
            .title("Gtk Demo")
            .build();

        let header_bar = cascade! {
            HeaderBar::default();
            ..set_title(Some("GTK Demo"));
            ..set_show_close_button(true);
        };

        let paned = cascade! {
            Paned::new(Orientation::Horizontal);
            ..set_position(100);
            ..set_wide_handle(false);
        };

        let tree_content = cascade!{
            TreeView::new();
            ..set_headers_visible(false);
        };

        let btn1 = Button::with_label("Clique me 2");

        let tree_store = TreeStore::new(&[String::static_type()]);
        tree_content.set_model(Some(&tree_store));
        append_text_column(&tree_content);

        for i in ExampleType::iterator() {
            tree_store.insert_with_values(None, None, &[(0, &format!("{}", i))]);
        }

        paned.pack1(&tree_content, true, false);
        paned.pack2(&btn1, true, true);

        window.set_titlebar(Some(&header_bar));
        window.add(&paned);

        window.show_all();

        Ui {
            window,
            tree_content,
            paned,
            current_widget: None,
        }
    }
}

fn append_text_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}
