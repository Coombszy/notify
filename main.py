# Notify
# This service is to send push notifications to a mobile phone using IFTTT.
#
########################################################################################################
import datetime
import time
import requests
import json

# CONSTANTS
########################################################################################################
R_TEXT1 = '347192837'
R_TEXT2 = '912372135'
########################################################################################################

# CONFIGS
########################################################################################################
URL = 'https://maker.ifttt.com/trigger/notification_phone/with/key/dAwxxBLcB3qNYgaNR1XL2U'
HEADERS = {'content-type': 'application/json', 'Accept-Charset': 'UTF-8'}
JSON = '{"value1":"' + R_TEXT1 + '", "value2":"' + R_TEXT2 + '"}'
WAIT = 60
VERSION = 'v0.1'
CONFIG_LOCATION = './configs/'
########################################################################################################

# DATA
########################################################################################################
data = {}

#   monday
data[0] = json.load(open(CONFIG_LOCATION + '0.json'))
#   tuesday
data[1] = json.load(open(CONFIG_LOCATION + '1.json'))
#   wednesday
data[2] = json.load(open(CONFIG_LOCATION + '2.json'))
#   thursday
data[3] = json.load(open(CONFIG_LOCATION + '3.json'))
#   friday
data[4] = json.load(open(CONFIG_LOCATION + '4.json'))
#   saturday
data[5] = json.load(open(CONFIG_LOCATION + '5.json'))
#   sunday
data[6] = json.load(open(CONFIG_LOCATION + '6.json'))
########################################################################################################

# MAIN
########################################################################################################
print('Starting Notify ' + VERSION)
RUNNING = True
DAY = 0 # datetime.datetime.today().weekday()
while(RUNNING):

    now = datetime.datetime.today()
    # If day has changed, roll over and reload config
    if( now.weekday() != DAY):
        # DAY = now.weekday() #####################################################UN COMMENT ME!
        data[DAY] = json.load(open(CONFIG_LOCATION + str(DAY) +'.json'))

    # Get minutes passed in the day
    minutes = (now.hour*60) + now.minute

    for notification in data[DAY]['notifications']:
        if 'active' in notification:
            notification['active'] = 'true'

    time.sleep(WAIT)


# r = requests.post(URL, data=JSON, headers=HEADERS)
