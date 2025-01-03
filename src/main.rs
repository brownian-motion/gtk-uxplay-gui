// "Hello World" from https://www.gtk.org/docs/language-bindings/rust/

use gtk4 as gtk;

use glib::clone;
// glib and other dependencies are re-exported by the gtk crate
use gtk::glib;
use gtk::prelude::*;

use which::which;

use std::process::Command;

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
    let status_label = gtk::Label::new(Some(" Loading..."));
    window.connect_show(clone!(@weak status_label => move |_| {
        let version = get_uxplay_version();
        let version_str: String = match version {
            NotInstalled => String::from("Not installed or not in $PATH"),
            ErrorRunning => String::from("Installed but could not be run"),
            Installed{ version: v } => format!("Installed: {}", v),
        };
        status_label.set_label(&version_str);
    }));

    let button = gtk::Button::with_label("Close");

    let the_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    the_box.append(&header);
    the_box.append(&status_label);
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

enum UxPlayInstallStatus {
    NotInstalled,
    ErrorRunning,
    Installed { version: String },
}

use UxPlayInstallStatus::*;

fn get_uxplay_version() -> UxPlayInstallStatus {
    let uxplay_path = match which("uxplay") {
        Ok(p) => p,
        Err(_) => return NotInstalled,
    };

    let output = Command::new(&uxplay_path)
        .arg("-v")
        .output();

    let output = match output {
        Ok(out) => String::from_utf8(out.stdout),
        Err(_) => return ErrorRunning,
    };

    match output {
        Ok(string) => Installed{version: string},
        Err(_) => return ErrorRunning,
    }
}


