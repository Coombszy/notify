# Notify
# This service is to send push notifications to a mobile phone using IFTTT.
#
########################################################################################################
import datetime
import time
import requests
import json
import configparser

# CONFIGS/LIBS
########################################################################################################
VERSION = 'v0.1'
HEADERS = {'content-type': 'application/json', 'Accept-Charset': 'UTF-8'}
CONFIG_LOCATION = './config.ini'

config = configparser.ConfigParser()
config.read(CONFIG_LOCATION)

URL = config['notify']['URL']
WAIT = int(config['notify']['WAIT'])
NOTIFICATION_LOCATION = config['notify']['NOTIFICATION_LOCATION']
########################################################################################################

# DATA
########################################################################################################
data = {}

#   monday
data[0] = json.load(open(NOTIFICATION_LOCATION + '0.json'))
#   tuesday
data[1] = json.load(open(NOTIFICATION_LOCATION + '1.json'))
#   wednesday
data[2] = json.load(open(NOTIFICATION_LOCATION + '2.json'))
#   thursday
data[3] = json.load(open(NOTIFICATION_LOCATION + '3.json'))
#   friday
data[4] = json.load(open(NOTIFICATION_LOCATION + '4.json'))
#   saturday
data[5] = json.load(open(NOTIFICATION_LOCATION + '5.json'))
#   sunday
data[6] = json.load(open(NOTIFICATION_LOCATION + '6.json'))
########################################################################################################

# MAIN
########################################################################################################
print('Starting Notify ' + VERSION)
RUNNING = True
DAY = datetime.datetime.today().weekday()
while(RUNNING):

    now = datetime.datetime.today()
    # If day has changed, roll over and reload config
    if( now.weekday() != DAY):
        DAY = now.weekday() 
        data[DAY] = json.load(open(CONFIG_LOCATION + str(DAY) +'.json'))

    # Get minutes passed in the day
    minutes = (now.hour*60) + now.minute

    for notification in data[DAY]['notifications']:
        # If active is missing, add it 
        if 'active' not in notification:
            notification['active'] = 'true'

        # If notification is ready to send, Send!
        if notification['active'] == 'true' and int(notification['trigger']) <= minutes:
            # Send notification
            data_json = '{"value1":"' + str(notification['title']) + '", "value2":"' + str(notification['content']) + '"}'
            r = requests.post(URL, data=data_json, headers=HEADERS)
            # If notification failed
            if not(r.ok):
                print("FAILED TO SEND NOTIFICATION:" + data)

            notification['active'] = 'false'

    time.sleep(WAIT)
