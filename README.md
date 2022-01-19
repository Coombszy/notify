# Notify
Simple python service that will send POST requests to an IFTTT webhook. This will then create other events/notifications on my smart phone.
To use, rename `configSample.ini` to `config.ini` and update the webhook URL.
To create notifications/entries, update the json files in the `notifications` directory (See below for an example)

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