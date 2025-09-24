use adw::subclass::prelude::*;
use adw::prelude::*;
use gtk::glib;
use core::cell;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/space/soulie/mobilisation/sidebar_row.ui")]
    #[properties(wrapper_type = super::MobilisationSidebarRow)]
    pub struct MobilisationSidebarRow {
        #[property(get, set)]
        label: cell::RefCell<String>,
        #[template_child]
        pub row_label: TemplateChild<gtk::Label>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MobilisationSidebarRow {
        const NAME: &'static str = "MobilisationSidebarRow";
        type Type = super::MobilisationSidebarRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    #[glib::derived_properties]
    impl ObjectImpl for MobilisationSidebarRow {
        fn constructed(&self) {
            self.parent_constructed();
            let a = self.obj();
            a.bind_property("label", &self.row_label.get(), "label").build();
        }
    }
    impl WidgetImpl for MobilisationSidebarRow {}
    impl ListBoxRowImpl for MobilisationSidebarRow {}
}

glib::wrapper! {
    pub struct MobilisationSidebarRow(ObjectSubclass<imp::MobilisationSidebarRow>)
    @extends gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Buildable;
}
