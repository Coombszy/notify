# Notify
Simple python service that will send POST requests to an IFTTT webhook. This will then create other events/notifications on my smart phone.
To use, rename `configSample.ini` to `config.ini` and update the webhook URL.
To create notifications/entries, update the json files in the `notifications` directory (See below for an example). The number in the name of the json represents the day of the week, e.g monday is `0.json`, and friday is `4.json`.

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
			"content": "Some message 2",
			"trigger": "3600"
		}
	]
}
```

- title = The title of the notification received on the IFTTT mobile app
- content = The content of the notification received on the IFTTT mobile app
- trigger = The minute in the day when the notification will be triggered

## Links
- [Docker](https://hub.docker.com/r/coombszy/notify) (![Docker](https://img.shields.io/docker/pulls/coombszy/notify.svg))
- [Github](https://github.com/Coombszy/notify)
