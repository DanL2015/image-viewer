mod imp;

use gtk::{
    gio,
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
    Application,
};

glib::wrapper! {
    pub struct ImageViewerWindow(ObjectSubclass<imp::ImageViewerWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl ImageViewerWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn set_image(&self, image: gio::File) {
        self.imp().picture.set_file(Some(&image));
    }
}
