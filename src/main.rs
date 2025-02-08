// "Hello World" from https://www.gtk.org/docs/language-bindings/rust/
mod status;
mod states;
use status::*;
use states::*;

use gtk4 as gtk;
use gtk::{gio, Application};

use glib::clone;
// glib and other dependencies are re-exported by the gtk crate
use gtk::prelude::*;

use which::which;

use std::process::Command;


const APP_ID: &str = "dev.brownian.UxPlayGui";

// When the application is launched…
fn on_activate(application: &gtk::Application) {
    // … create a new window …
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("UxPlay panel")
        .default_width(480)
        .default_height(480)
        .build();

    let status_pane = UxPlayStatusPane::new();
    window.connect_show(clone!(@weak status_pane => move |_| {
        let version = get_uxplay_version();
        status_pane.set_info(Some(version));
    }));

    let button = gtk::Button::with_label("Close");

    let the_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    the_box.append(&status_pane);
    the_box.append(&button);

    // … which closes the window when clicked
    button.connect_clicked(clone!(@weak window => move |_| window.close()));
    window.set_child(Some(&the_box));
    window.present();
}

fn main() {
    // Register and include resources produced by build.rs
    gio::resources_register_include!("compiled.gresource")
        .expect("Failed to register resources.");

    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(on_activate);
    // Run the application
    app.run();
}

use states::UxPlayInstallStatus::*;

fn get_uxplay_version() -> UxPlayInstallInfo {
    let uxplay_path = match which("uxplay") {
        Ok(p) => p,
        Err(_) => return UxPlayInstallInfo::not_installed(),
    };

    let output = Command::new(&uxplay_path)
        .arg("-v")
        .output();

    let output = match output {
        Ok(out) => String::from_utf8(out.stdout),
        Err(_) => return UxPlayInstallInfo::error(),
    };

    match output {
        Ok(version_text) => UxPlayInstallInfo{ status: Installed, version: Some(version_text) },
        Err(_) => return UxPlayInstallInfo::error(),
    }
}


