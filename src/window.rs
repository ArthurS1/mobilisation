/* window.rs
 *
 * Copyright 2025
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::domain;
use crate::http_client;
use crate::runtime;
use crate::sidebar::MobilisationSidebar;
use crate::event_preview_model::MobilisationEventPreviewModel;
use crate::event_preview::MobilisationEventPreview;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/space/soulie/mobilisation/window.ui")]
    pub struct MobilisationWindow {
        #[template_child]
        pub sidebar: TemplateChild<MobilisationSidebar>,
        #[template_child]
        pub sidebar_show: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub split_view: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        pub event_previews: TemplateChild<gtk::ListView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MobilisationWindow {
        const NAME: &'static str = "MobilisationWindow";
        type Type = super::MobilisationWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    fn load_config(sidebar: &MobilisationSidebar) {
        let (sender, receiver) = async_channel::unbounded();
        runtime().spawn(async move {
            let _ = sender
                .send(domain::fetch_config("https://mobilizon.fr/api").await)
                .await;
        });
        glib::spawn_future_local(glib::clone!(
            #[strong]
            sidebar,
            async move {
                receiver
                    .recv()
                    .await
                    .map(|value| match value {
                        Ok(v) => sidebar.append_categories(&v.categories),
                        Err(err) => {
                            glib::g_log!(
                                glib::LogLevel::Error,
                                "Error fetching config : {:?}",
                                err
                            );
                            sidebar.show_error();
                        }
                    })
                    .map_err(|err| {
                        glib::g_log!(glib::LogLevel::Critical, "Channel error : {}", err);
                        sidebar.show_error();
                    })
            }
        ));
    }

    fn create_event_timeline(window: &MobilisationWindow, events: &Vec<domain::Event>) {
        let event_objects = events
          .into_iter()
          .map(|event| MobilisationEventPreviewModel::new(event))
          .collect::<Vec<MobilisationEventPreviewModel>>();
        let model = gio::ListStore::new::<MobilisationEventPreviewModel>();
        model.extend_from_slice(&event_objects);
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
          let preview = MobilisationEventPreview::new();
          list_item
            .downcast_ref::<gtk::ListItem>()
            .expect("Could not downcast Object to ListItem.")
            .set_child(Some(&preview));
        });
        factory.connect_bind(move |_, list_item| {
          let model = list_item
            .downcast_ref::<gtk::ListItem>()
            .expect("Could not downcast Object to ListItem.")
            .item()
            .and_downcast::<MobilisationEventPreviewModel>()
            .expect("Could not downcast EventPreviewModel from ListItem item.");
          let event_preview = list_item
            .downcast_ref::<gtk::ListItem>()
            .expect("Could not downcast Object to ListItem.")
            .child()
            .and_downcast::<MobilisationEventPreview>()
            .expect("Could not downcast EventPreviewModel from ListItem item.");
          event_preview.imp().event_name.get().set_label(model.title().as_str());
          event_preview.imp().event_description.get().set_label("test");
        });
        let selection_model = gtk::NoSelection::new(Some(model));
        window.event_previews.set_model(Some(&selection_model));
        window.event_previews.set_factory(Some(&factory));
    }

    fn load_events(obj: &super::MobilisationWindow) {
        let (sender, receiver) = async_channel::unbounded();
        runtime().spawn(async move {
            let _ = sender
                .send(domain::fetch_events(http_client(), "https://mobilizon.fr/api").await)
                .await;
        });
        glib::spawn_future_local(glib::clone!(
            #[strong]
            obj,
            async move {
                receiver
                    .recv()
                    .await
                    .map(|value| {
                      match value {
                        Ok((vector, _)) => create_event_timeline(&obj.imp(), &vector),
                        Err(err) => {
                            glib::g_log!(
                                glib::LogLevel::Error,
                                "Error fetching events : {:?}",
                                err
                            );
                        }
                      }
                    })
                    .map_err(|err| {
                        glib::g_log!(glib::LogLevel::Critical, "Channel error : {}", err);
                        obj.imp().sidebar.show_error();
                    })
            }
        ));
    }

    impl ObjectImpl for MobilisationWindow {
        fn constructed(&self) {
            let obj = self.obj();
            self.parent_constructed();
            load_config(obj.imp().sidebar.as_ref());
            load_events(obj.as_ref());
        }
    }
    impl WidgetImpl for MobilisationWindow {}
    impl WindowImpl for MobilisationWindow {}
    impl ApplicationWindowImpl for MobilisationWindow {}
    impl AdwApplicationWindowImpl for MobilisationWindow {}
}

glib::wrapper! {
    pub struct MobilisationWindow(ObjectSubclass<imp::MobilisationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl MobilisationWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        let window = glib::Object::builder::<MobilisationWindow>()
            .property("application", application)
            .build();
        let a = window.imp();
        a.sidebar_show
            .bind_property("active", &a.split_view.get(), "show_sidebar")
            .bidirectional()
            .build();
        window
    }
}
