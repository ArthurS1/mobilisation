use adw::prelude::*;
use adw::subclass::prelude::*;
use core::cell;
use gtk::gdk;
use gtk::glib;
use url::Url;

use crate::infra::events::fetch_event_picture;
use crate::{http_client, runtime};

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/space/soulie/mobilisation/event_preview.ui")]
    #[properties(wrapper_type = super::MobilisationEventPreview)]
    pub struct MobilisationEventPreview {
        #[template_child]
        pub event_name: TemplateChild<gtk::Label>,
        #[template_child]
        pub event_description: TemplateChild<gtk::Label>,
        #[template_child]
        pub time: TemplateChild<gtk::Label>,
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub spinner: TemplateChild<adw::Spinner>,
        #[property(get, set)]
        pub picture_url: cell::RefCell<Option<String>>,
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

    #[glib::derived_properties]
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
        let a = glib::Object::builder::<MobilisationEventPreview>().build();
                                    println!("object built");
        a.connect_picture_url_notify(|e| {
                                    println!("picture property modified");
            let x = e.imp();
            match x.picture_url.take() {
                None => return,
                Some(picture_url) => {
                    let (sender, receiver) = async_channel::unbounded();
                    let parsed_picture_url = Url::parse(picture_url.as_str())
                        .expect("Failed to parse a normally parsable url.");
                    runtime().spawn(async move {
                                    println!("fetching");
                        let _ = sender
                            .send(fetch_event_picture(http_client(), &parsed_picture_url).await)
                            .await;
                    });
                    glib::spawn_future_local(glib::clone!(
                        #[weak(rename_to=obj)]
                        x,
                        async move {
                            let _ = receiver.recv().await.map(|v| {
                                println!("answer received");
                                let _ = v.map(move |bytes| {
                                    let bytes = glib::Bytes::from(bytes.as_ref());
                                    let texture = gdk::Texture::from_bytes(&bytes).unwrap();
                                    obj.picture.set_paintable(Some(&texture));
                                    obj.picture.set_visible(true);
                                    obj.spinner.set_visible(false);
                                });
                            });
                        }
                    ))
                }
            };
        });
        a
    }
}
