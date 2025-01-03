// "Hello World" from https://www.gtk.org/docs/language-bindings/rust/

use gtk4 as gtk;

use glib::clone;
// glib and other dependencies are re-exported by the gtk crate
use gtk::glib;
use gtk::prelude::*;

// When the application is launched…
fn on_activate(application: &gtk::Application) {
    // … create a new window …
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("UxPlay")
        .default_width(480)
        .default_height(480)
        .build();

    let header = gtk::Label::new(Some("UxPlay Status:"));

    let button = gtk::Button::with_label("Close");

    let the_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    the_box.append(&header);
    the_box.append(&button);

    // … which closes the window when clicked
    button.connect_clicked(clone!(@weak window => move |_| window.close()));
    window.set_child(Some(&the_box));
    window.present();
}

fn main() {
    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id("dev.brownian.UxPlay-gtk")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    app.run();
}
