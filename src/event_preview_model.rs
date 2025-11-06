use adw::prelude::*;
use core::cell;
use gtk::glib::{self, GString, Object};
use gtk::subclass::prelude::*;

use crate::core::event::{DateTime, Event, HumanReadableDateTime};

mod imp {
    use super::*;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::MobilisationEventPreviewModel)]
    pub struct MobilisationEventPreviewModel {
        #[property(get, set)]
        pub title: cell::RefCell<String>,
        #[property(get, set)]
        pub picture_url: cell::RefCell<Option<String>>,
        #[property(get, set)]
        pub human_readable_time: cell::RefCell<String>,
        #[property(get, set)]
        pub description: cell::RefCell<String>,
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
    pub fn new(event: &Event) -> Self {
        Object::builder()
            .property("title", &event.title)
            .property(
                "picture_url",
                event.picture_url.as_ref().map(|e| e.to_string()),
            )
            .property("description", Self::make_date_string(event))
            .property(
                "human_readable_time",
                match event.compute_human_readable_begining(
                    None::<&fn() -> chrono::DateTime<chrono::Local>>,
                ) {
                    HumanReadableDateTime::Later(date) => date,
                    HumanReadableDateTime::Now => "Now".to_string(),
                },
            )
            .build()
    }

    fn make_date_string(event: &Event) -> String {
        if event.is_long() {
            format!(
                "{} to {}",
                Self::localize_date(&event.begins_on),
                Self::localize_date(&event.ends_on)
            )
        } else {
            format!(
                "{} - {}h",
                Self::localize_date(&event.begins_on).to_string(),
                event.compute_duration_in_hours()
            )
        }
    }

    fn localize_date(date_time: &DateTime) -> GString {
        /* We unwrap here because cases where a chrono::DateTime outputs an
         * rfc3339 compliant string and glib::DateTime::from_iso8601 fails to
         * read it seem improbable enough that we choose not to care for this
         * prototype */
        glib::DateTime::from_iso8601(&date_time.to_rfc3339(), None)
            .and_then(|v| v.format("%x %X"))
            .unwrap()
    }
}
