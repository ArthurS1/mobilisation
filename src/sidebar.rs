use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

use crate::core::category::Category;
use crate::sidebar_row;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/space/soulie/mobilisation/sidebar.ui")]
    pub struct MobilisationSidebar {
        #[template_child]
        pub content: TemplateChild<gtk::Box>,
        #[template_child]
        pub error: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub spinner: TemplateChild<adw::Spinner>,
        #[template_child]
        pub category_list: TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MobilisationSidebar {
        const NAME: &'static str = "MobilisationSidebar";
        type Type = super::MobilisationSidebar;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MobilisationSidebar {}
    impl WidgetImpl for MobilisationSidebar {}
    impl BoxImpl for MobilisationSidebar {}
}

glib::wrapper! {
    pub struct MobilisationSidebar(ObjectSubclass<imp::MobilisationSidebar>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Buildable;
}

impl MobilisationSidebar {
    pub fn show_error(&self) {
        self.imp().spinner.set_visible(false);
        self.imp().content.set_visible(false);
        self.imp().error.set_visible(true);
    }

    pub fn append_categories(&self, categories: &Vec<Category>) -> () {
        self.imp().spinner.set_visible(false);
        self.imp().content.set_visible(true);
        self.imp().error.set_visible(false);
        let _ = categories.into_iter().for_each(|category| {
            let row = glib::Object::builder::<sidebar_row::MobilisationSidebarRow>()
                    .property("label", category.label.clone())
                    .build();
            glib::g_log!(glib::LogLevel::Debug, "Creating a row with label {}", category.label);
            self.imp().category_list.append(&row);
        });
    }
}
