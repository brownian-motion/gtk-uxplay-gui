use glib::prelude::*;
use glib::subclass::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "UxPlayInstallStatus")]
pub enum UxPlayInstallStatus {
    NotInstalled,
    ErrorRunning,
    Installed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UxPlayInstallInfo {
    pub status: UxPlayInstallStatus,
    pub version: Option<String>,
}

impl UxPlayInstallInfo {
    pub fn installed(version: String) -> Self {
        UxPlayInstallInfo{ status: UxPlayInstallStatus::Installed, version: Some(version) }
    }

    pub fn error() -> Self {
        UxPlayInstallInfo{ status: UxPlayInstallStatus::ErrorRunning, version: None }
    }

    pub fn not_installed() -> Self {
        UxPlayInstallInfo{ status: UxPlayInstallStatus::NotInstalled, version: None }
    }
}
