use crate::libs::structs::*;

#[test]
fn notification_to_string() {
    let _title = "ATitle 123 abc".to_string();
    let _content = "Some Content123 awd!!!???".to_string();
    let _image = "www.image.com".to_string();
    let _cron = "* * * * *".to_string();

    let notification: Notification = Notification {
        title: _title.clone(),
        content: _content.clone(),
        image: Some(_image.clone()),
        cron: _cron.clone(),
        key: None,
        event: None,
    };
    let notification_string: String = format!("{{\"title\":\"{_title}\",\"content\":\"{_content}\",\"image\":\"{_image}\",\"cron\":\"{_cron}\",\"event\":null,\"key\":null}}");

    assert_eq!(notification.display(), notification_string);

    let notification_imageless: Notification = Notification {
        title: _title.clone(),
        content: _content.clone(),
        image: None,
        cron: _cron.clone(),
        key: None,
        event: None,
    };
    let notification_imageless_string: String = format!("{{\"title\":\"{_title}\",\"content\":\"{_content}\",\"image\":null,\"cron\":\"{_cron}\",\"event\":null,\"key\":null}}");

    assert_eq!(
        notification_imageless.display(),
        notification_imageless_string
    );
}

#[test]
fn notification_to_string_pretty() {
    let _title = "ATitle 123 abc".to_string();
    let _content = "Some Content123 awd!!!???".to_string();
    let _image = "www.image.com".to_string();
    let _cron = "* * * * *".to_string();

    let notification: Notification = Notification {
        title: _title.clone(),
        content: _content.clone(),
        image: Some(_image.clone()),
        cron: _cron.clone(),
        key: None,
        event: None,
    };
    let notification_string: String = format!("{{\n  \"title\": \"{_title}\",\n  \"content\": \"{_content}\",\n  \"image\": \"{_image}\",\n  \"cron\": \"{_cron}\",\n  \"event\": null,\n  \"key\": null\n}}");

    assert_eq!(notification.display_pretty(), notification_string);

    let notification_imageless: Notification = Notification {
        title: _title.clone(),
        content: _content.clone(),
        image: None,
        cron: _cron.clone(),
        key: None,
        event: None,
    };
    let notification_imageless_string: String = format!("{{\n  \"title\": \"{_title}\",\n  \"content\": \"{_content}\",\n  \"image\": null,\n  \"cron\": \"{_cron}\",\n  \"event\": null,\n  \"key\": null\n}}");

    assert_eq!(
        notification_imageless.display_pretty(),
        notification_imageless_string
    );
}
