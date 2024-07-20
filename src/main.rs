mod image_viewer;
use gtk::prelude::*;
use image_viewer::ImageViewerWindow;

fn main() {
    let app = gtk::Application::builder()
        .application_id("org.gtk-rs.image_viewer")
        .build();

    app.connect_activate(|app| {
        let win = ImageViewerWindow::new(app);
        win.present();
    });

    app.run();
}
