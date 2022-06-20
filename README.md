# Notify
Simple Rust app that will send POST requests to an IFTTT webhook.\
This will then can create other events/notifications on a smart phone.\
Notifications can be configured/scheduled from a JSON or can be manually triggered via an API request.

To use, open `notify.toml` and update the webhook URL ([See IFTTT config below](#IFTTT)). \
Then just run the binary/exe for your platform ([Or use docker-compose](#DockerCompose)).

# Scheduled Notifications
To create repeating scheduled notifications, update the JSON file `data/notifications.json` (See below for an example). Scheduled notifications can be disabled via the config in `notify.toml`.

```
[
  {
    "title": "Sample Notification 1",
    "content": "Some message",
    "cron": "*/1 * * * *",
    "enabled": false
  },
  {
    "title": "Sample Notification 2",
    "content": "Some message 2 with an image!",
    "image": "https://upload.wikimedia.org/wikipedia/commons/thumb/2/2f/Google_2015_logo.svg/368px-Google_2015_logo.svg.png",
    "cron": "*/7 10 * * *",
    "enabled": false
  }
]
```
### JSON attributes
- title = The title of the notification received on the IFTTT mobile app
- content = The content of the notification received on the IFTTT mobile app
- image = The url of an image to include in the notification received. This is optional
- cron = The cron expression for when the notification will run (must be 5 fields long, starting at minutes)
- enabled = Disable/enable a notification from being sent. requires restart if changed

# Web Server (API)
Notify will host a web server that can receive http requests. Web server can be configured (including disabled) within the config `notify.toml`.
| Endpoint        | Method | Description                                                                                       | Response                                                                         |
| --------------- | ------ | ------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `/notification` | POST   | Accepts incoming [notification JSON](#web-json-post-notification) and sends API request to IFTTT. | 204: No Content (Success)<br>400: JSON object containing `error` and `timestamp` |
| `/health`       | GET    | Returns health and uptime of the service.                                                         | 200: JSON object containing `uptime`                                             |

### Web JSON
Body of POST request should be like this:
```
{
    "title":"Title of notification",
    "content":"Content of notification"
}
```
- title = The title of the notification received on the IFTTT mobile app
- content = The content of the notification received on the IFTTT mobile app
- image = The url of an image to include in the notification received. This is optional

# Docker Compose
The service is best ran using a docker-compose file. Use the sample provided:
```
version: '3.3'

services:
  notify:
    container_name: notify
    image: coombszy/notify:2.0.0 # Latest currently refers to Legacy
    volumes:
      - ./config:/app/config
      - ./data:/app/data
```
### Volume mounts
- /app/config = Folder to store the `notify.toml` for the service
- /app/data = Folder to store notifications JSON

# IFTTT
Currently, you cannot publish an IFTTT applet if it uses the webhook functionality. So i've included instructions on how to make the services yourself:

1. Create a new applet with the following 'If This' and 'Then That'
<div>
  <img src="https://github.com/coombszy/notify/blob/master/docs/IFTTT-1.png?raw=true" width="400">
</div>

2. The 'If This' should be configured as so
<div>
  <img src="https://github.com/coombszy/notify/blob/master/docs/IFTTT-2.png?raw=true" width="400">
</div>

3. And the 'Then That' should be configured like this
<div>
    <img src="https://github.com/coombszy/notify/blob/master/docs/IFTTT-3.png?raw=true" width="400">
</div>

4. Finally you will need to get your Webhooks key from the [Webhooks Service FAQ](https://help.ifttt.com/hc/en-us/articles/115010230347-Webhooks-service-FAQ). Insert this API key into the `notify.toml` config file for the `ifttt_key`.

5. If you chose a different event name in step 2, change `notify.toml` variable `ifttt_event` to the value you set.

# Legacy
If you are looking for the legacy version, it can been [found here](https://github.com/Coombszy/notify/tree/legacy). And the update notes can be [found here](notify_v2.md).

# Links
Some useful links
- [Docker](https://hub.docker.com/r/coombszy/notify) ![Docker](https://img.shields.io/docker/pulls/coombszy/notify)
- [Github](https://github.com/Coombszy/notify)
- [IFTTT](https://ifttt.com)
