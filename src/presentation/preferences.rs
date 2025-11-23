use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/space/soulie/mobilisation/sidebar.ui")]
    pub struct MobilisationPreferences {}

    #[glib::object_subclass]
    impl ObjectSubclass for MobilisationPreferences {
        const NAME: &'static str = "MobilisationPreferences";
        type Type = super::MobilisationPreferences;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MobilisationPreferences {}

    impl WidgetImpl for MobilisationPreferences {}

    impl AdwDialogImpl for MobilisationPreferences {}

    impl PreferencesDialogImpl for MobilisationPreferences {}
}

glib::wrapper! {
  pub struct MobilisationPreferences(ObjectSubclass<imp::MobilisationPreferences>)
  @extends adw::PreferencesDialog, adw::Dialog, gtk::Widget,
  @implements gtk::Buildable;
}

impl MobilisationPreferences {
  pub fn new() -> Self {
    glib::Object::builder::<MobilisationPreferences>()
      .build()
  }
}
