use adw::subclass::prelude::*;
use adw::prelude::*;
use gtk::glib;
use core::cell;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/space/soulie/mobilisation/sidebar_header.ui")]
    #[properties(wrapper_type = super::MobilisationSidebarHeader)]
    pub struct MobilisationSidebarHeader {
        #[property(get, set)]
        pub title: cell::RefCell<String>,
        #[property(get, set)]
        pub icon_resource: cell::RefCell<String>,
        #[template_child]
        pub icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MobilisationSidebarHeader {
        const NAME: &'static str = "MobilisationSidebarHeader";
        type Type = super::MobilisationSidebarHeader;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    #[glib::derived_properties]
    impl ObjectImpl for MobilisationSidebarHeader {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.bind_property("title", &self.label.get(), "label").build();
            obj.bind_property("icon_resource", &self.icon.get(), "resource").build();
        }
    }
    impl WidgetImpl for MobilisationSidebarHeader {}
    impl ListBoxRowImpl for MobilisationSidebarHeader {}
}

glib::wrapper! {
    pub struct MobilisationSidebarHeader(ObjectSubclass<imp::MobilisationSidebarHeader>)
    @extends gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Buildable;
}
