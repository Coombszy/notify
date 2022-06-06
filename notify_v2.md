# Notify Version 2.0.0 [WIP DOC, 2.0.0 NOT ACTUALLY *FULLY* READY YET! I'M A DRAFT/PLAN]
It's finally here!

I've been slowly tinkering away at a complete rewrite of notify for some time now. It's been a fun learning experience as it was primarily an excuse to learn Rust. This new version also includes some new features that I think are useful additions.

Part of this rewrite includes new configuration as well as changes to the notification json(s). This means the new version is NOT compatable with the old version. See [Legacy Support](##-legacy-support)

## What's New?
  - Goodbye Python, Hello Rust! 🦀 <br> This allows for cross platform binaries, and hopefully improve resource consumption (Not that it was particular intensive in the first place).
  - Improved Config: <br> Configs have been updated to have better names and include descriptions. As well as add support/controls for new features.
  - Improved Logging: <br> Logging level is now controlled via an environment var, and has improved style/output.
  - Replaced days of the week Notification jsons: <br> Instead of multiple json files for each day of the week, they are all rolled into one `notifications.json` now.
  - Added Cron Expression Scheduling: <br> Notifications are now scheduled using cron expressions... finally, as it should of been like this from day 1.
  - Added Disable/Enable Notifications: <br> Can disable and enable a notification within the `notifications.json`. Does require application restart to apply.
  - Added API/Web Server: <br> Application now runs a web server that can receive POST requests containing notifications that will be pushed through to IFTTT. Can be disabled and enabled in configs. See [ADD LINK HERE]() for more info.

## Legacy Support

Something something, legacy will continue basic bug support but no new features and will stop updating on TBD date.

# THIS IS A WIP DOC, V2.0.0 IS NOT RELEASED YET. THIS IS DOC/RELEASE IS SUBJECT TO CHANGE
