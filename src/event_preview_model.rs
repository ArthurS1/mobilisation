use crate::domain;
use adw::prelude::*;
use core::cell;
use gtk::glib::{self, Object};
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::MobilisationEventPreviewModel)]
    pub struct MobilisationEventPreviewModel {
        #[property(get, set)]
        pub title: cell::RefCell<String>,
        #[property(get, set)]
        pub picture_url: cell::RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MobilisationEventPreviewModel {
        const NAME: &'static str = "MobilisationEventPreviewModel";
        type Type = super::MobilisationEventPreviewModel;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for MobilisationEventPreviewModel {}
}

glib::wrapper! {
    pub struct MobilisationEventPreviewModel(ObjectSubclass<imp::MobilisationEventPreviewModel>);
}

impl MobilisationEventPreviewModel {
    pub fn new(event: &domain::Event) -> Self {
        Object::builder()
            .property("title", &event.title)
            .property(
                "picture_url",
                event.picture_url.as_ref().map(|e| e.to_string()),
            )
            .build()
    }
}
