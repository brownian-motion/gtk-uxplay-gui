use gtk4 as gtk;
use gtk::{gio, Application};
use glib::Object;
use crate::states::{*, UxPlayInstallStatus::*};
use std::cell::{Ref, RefCell};
use gtk4::subclass::prelude::ObjectSubclassIsExt;

glib::wrapper! {
	pub struct UxPlayStatusPane(ObjectSubclass<imp::UxPlayStatusPane>)
		@extends gtk::Box, gtk::Widget,
		@implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl UxPlayStatusPane {
	pub fn new() -> Self {
		Object::builder().build()
	}

	fn render_status(&self) {
		let status_text = match *self.imp().status.borrow() {
			None => "Loading...",
            Some(NotInstalled) => "Not installed or not in $PATH",
            Some(ErrorRunning) => "Installed but could not be run",
            Some(Installed) => "Installed",
        };
        self.imp().status_header.set_label(status_text);

		let version: Ref<Option<String>> = self.imp().version.borrow();
		let version: &Option<String> = &*version;
		let version: Option<&str> = version.as_ref().map(String::as_str);
		let version: &str = version.unwrap_or("unknown version");
		self.imp().status_content.set_label(version);
	}

	pub fn set_info(&self, info: Option<UxPlayInstallInfo>) {
		match info {
			Some(UxPlayInstallInfo{ status, version }) => {
				*self.imp().status.borrow_mut() = Some(status);
				*self.imp().version.borrow_mut() = version;
			},
			None => {
				*self.imp().status.borrow_mut() = None;
				*self.imp().version.borrow_mut() = None;
			}
		}
		
		self.render_status();
	}
}

impl Default for UxPlayStatusPane {
	fn default() -> Self {
		Self::new()
	}
}

mod imp {
	use std::cell::{Ref, RefCell};
	use glib::subclass::InitializingObject;
	use gtk4 as gtk;
	use gtk::prelude::*;
	use gtk::subclass::prelude::*;
	use gtk::{glib, Label, CompositeTemplate};
	use crate::states;

	// this guy holds the state
	#[derive(CompositeTemplate, Default)]
	#[template(resource = "/dev/brownian/UxPlayGui/status.ui")]
	pub struct UxPlayStatusPane {
		#[template_child]
		pub status_header: TemplateChild<Label>,

		#[template_child]
		pub status_content: TemplateChild<Label>,

		pub status: RefCell<Option<states::UxPlayInstallStatus>>,
		pub version: RefCell<Option<String>>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for UxPlayStatusPane {
		const NAME: &'static str = "UxPlayStatusPane";
		type Type = super::UxPlayStatusPane;
		type ParentType = gtk::Box;

		fn class_init(klass: &mut Self::Class) {
			klass.bind_template();
		}

		fn instance_init(obj: &InitializingObject<Self>) {
			obj.init_template();
		}
	}

	// Trait shared by all GObjects
	impl ObjectImpl for UxPlayStatusPane {
		fn constructed(&self) {
	        // Call "constructed" on parent
	        self.parent_constructed();

	        // Setup
	        let obj = self.obj();
        	obj.render_status();
		}
	}

	// Trait shared by all widgets
	impl WidgetImpl for UxPlayStatusPane {}

	// Trait shared by all boxes
	impl BoxImpl for UxPlayStatusPane {}
}