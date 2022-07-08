use adw::prelude::*;
use adw::{ApplicationWindow};
use gtk::{gio};

use gtk::glib::signal::Inhibit;
use gio::Settings;

fn main() {
    gio::resources_register_include!("compiled.gresource").unwrap();

    let app = adw::Application::builder().build();
    app.connect_startup(|_| {
        adw::init();
    });
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(350)
        .build();
    window.show();
}
