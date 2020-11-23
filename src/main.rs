//! # Dmenu Notify
//!
//! A dmenu interface for [notifyd](https://github.com/hugglesfox/notifyd).

use dmenu_facade::DMenu;
use std::fmt;
use zbus::dbus_proxy;
use zbus::fdo;

/// A notification from dbus
struct Notification {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}: {}", self.app_name, self.summary, self.body)
    }
}

/// Notifyd dbus interface
#[dbus_proxy(
    interface = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifyd {
    fn get_notification_queue(&self) -> fdo::Result<Vec<Notification>>;

    fn close_notification(&self, id: u32) -> fdo::Result<()>;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::new_session()?;
    let proxy = NotifydProxy::new(&connection)?;

    let notifications = proxy.get_notification_queue()?;
    let chosen = Dmenu::default().execute(&notifications);

    if let Ok(notification) = chosen {
        proxy.close_notification(notification.id);
    }
}