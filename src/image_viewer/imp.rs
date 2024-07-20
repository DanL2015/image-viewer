use gtk::{gio, glib, subclass::prelude::*};

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "./image_viewer_window.ui")]

pub struct ImageViewerWindow {
    #[template_child(id = "picture")]
    pub picture: TemplateChild<gtk::Picture>,
}

#[glib::object_subclass]
impl ObjectSubclass for ImageViewerWindow {
    const NAME: &'static str = "ImageViewerWindow";
    type Type = super::ImageViewerWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.install_action_async("img.open", None, |win, _action_name, _parameter_type| async move {
            let filters = gio::ListStore::new::<gtk::FileFilter>();
            let image_filter = gtk::FileFilter::new();
            image_filter.add_mime_type("image/*");
            filters.append(&image_filter);

            let dialog = gtk::FileDialog::builder()
                .title("Open File")
                .accept_label("Open")
                .modal(true)
                .filters(&filters)
                .build();

            if let Ok(file) = dialog.open_future(Some(&win)).await {
                win.set_image(file);
            }
        });
    }

    fn instance_init(obj: &gtk::glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ImageViewerWindow {}
impl WidgetImpl for ImageViewerWindow {}
impl WindowImpl for ImageViewerWindow {}
impl ApplicationWindowImpl for ImageViewerWindow {}
