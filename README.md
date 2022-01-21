# Notify
Simple python service that will send POST requests to an IFTTT webhook. This will then create other events/notifications on my smart phone.
To use, rename `configSample.ini` to `config.ini` and update the webhook URL.
To create notifications/entries, update the JSON files in the `notifications` directory (See below for an example). The number in the name of the JSON represents the day of the week, e.g monday is `0.json`, and friday is `4.json`.

```
{
	"notifications" :[
		{
			"title": "Testing Notification 1",
			"content": "Some message",
			"trigger": "1"
		},
		{
			"title": "Testing Notification 2",
			"content": "Some message 2 with an image!",
			"image": "https://upload.wikimedia.org/wikipedia/commons/thumb/2/2f/Google_2015_logo.svg/368px-Google_2015_logo.svg.png",
			"trigger": "3600"
		}
	]
}
```
### JSON attributes
- title = The title of the notification received on the IFTTT mobile app
- content = The content of the notification received on the IFTTT mobile app
- image = The url of an image to include in the notification received
- trigger = The minute in the day when the notification will be triggered

# Docker Compose
The service is best ran using a docker-compose file. Use the sample provided:
```
version: '3.3'

services:
  notify:
    container_name: notify
    image: coombszy/notify:latest
    volumes:
      - ./config:/app/config
      - ./notifications:/app/notifications
```
### Volume mounts
- /app/config = Folder to store the `config.ini` for the service
- /app/notifications = Folder to store notification JSONs for each day

## Links
- [Docker](https://hub.docker.com/r/coombszy/notify) (![Docker](https://img.shields.io/docker/pulls/coombszy/notify.svg))
- [Github](https://github.com/Coombszy/notify)
