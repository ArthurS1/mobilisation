use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/space/soulie/mobilisation/event_preview.ui")]
    pub struct MobilisationEventPreview {
        #[template_child]
        pub event_name: TemplateChild<gtk::Label>,
        #[template_child]
        pub event_description: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MobilisationEventPreview {
        const NAME: &'static str = "MobilisationEventPreview";
        type Type = super::MobilisationEventPreview;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MobilisationEventPreview {}
    impl WidgetImpl for MobilisationEventPreview {}
    impl BoxImpl for MobilisationEventPreview {}
}

glib::wrapper! {
    pub struct MobilisationEventPreview(ObjectSubclass<imp::MobilisationEventPreview>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Buildable;
}

impl MobilisationEventPreview {
    pub fn new() -> Self {
        glib::Object::builder::<MobilisationEventPreview>().build()
    }
}
