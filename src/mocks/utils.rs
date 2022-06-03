// Some utils to make life easier
use crate::libs::structs::Notification;
use log::info;

// Send a notification to IFTTT using notification object
pub async fn send_notification(notification: Notification) {
    info!(
        "Nofitication for \"{}\" was supposed to be sent [MOCKED]",
        notification.title
    );
}
