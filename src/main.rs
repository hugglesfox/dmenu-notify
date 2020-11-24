//! # Dmenu Notify
//!
//! A dmenu interface for [notifyd](https://github.com/hugglesfox/notifyd).
//!
//! Selecting an item from the menu is the equivilant to closing the notification.
//! Notifications are displayed sorted by urgency.

#[macro_use]
extern crate clap;

use dmenu_facade::{Color, DMenu};
use serde::Deserialize;
use std::fmt;
use zbus::dbus_proxy;
use zbus::fdo;
use zvariant::derive::Type;

/// A notification from dbus
#[derive(Debug, Deserialize, Type)]
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
    pub urgency: u32,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} - {}", self.app_name, self.summary, self.body)
    }
}

/// Notifyd dbus interface
#[dbus_proxy(
    interface = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifyd {
    /// Get notifications
    fn get_notification_queue(&self) -> fdo::Result<Vec<Notification>>;

    /// Close a notification
    fn close_notification(&self, id: u32) -> fdo::Result<()>;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app_from_crate!().get_matches();

    let connection = zbus::Connection::new_session()?;
    let proxy = NotifydProxy::new(&connection)?;

    let mut notifications = proxy.get_notification_queue()?;
    notifications.sort_by(|a, b| b.urgency.partial_cmp(&a.urgency).unwrap());

    let chosen = DMenu::default()
        .case_insensitive()
        .with_font("monospace:size=10")
        .with_colors(
            Some(Color("#3a3c4e")),
            Some(Color("#e9e9f4")),
            Some(Color("#b45bcf")),
            Some(Color("#e9e9f4")),
        )
        .execute(&notifications);

    if let Ok(notification) = chosen {
        proxy.close_notification(notification.id)?;
    }

    Ok(())
}
