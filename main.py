# Notify
# This service is to send push notifications to a mobile phone using IFTTT.
#
########################################################################################################
import datetime
import time
import requests
import json
import configparser
from lib.logger import Logger
from os.path import exists

# CONFIGS/LIBS
########################################################################################################
VERSION = 'v1.2'
HEADERS = {'content-type': 'application/json', 'Accept-Charset': 'UTF-8'}
CONFIG_LOCATION = './config/config.ini'

if not(exists(CONFIG_LOCATION)):
    Logger._write(content="MISSING CONFIG FILE", level=0)
    exit(1)

config = configparser.ConfigParser()
config.read(CONFIG_LOCATION)

URL = config['notify']['URL']
WAIT = int(config['notify']['WAIT'])
NOTIFICATION_LOCATION = config['notify']['NOTIFICATION_LOCATION']
LOGGING_LEVEL = int(config['notify']['LOGGING_LEVEL'])
WRITE_TO_LOG_FILE = bool(config['notify']['WRITE_TO_LOG_FILE'])
LOG_FILE_DIR = config['notify']['LOG_FILE_DIR']
########################################################################################################

# DATA
########################################################################################################
for day in range(0,7):
    if not(exists(NOTIFICATION_LOCATION + str(day) +'.json')):
        Logger._write(content="MISSING DAY JSON FILE: " + str(day) + '.json', level=0)
        exit(1)

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

# METHODS
########################################################################################################

# Produces notification json for IFTTT endpoint
def get_json(notification):

    if 'image' in notification:
        data_json = '{"value1":"' + str(notification['title']) + '", "value2":"' + str(notification['content']) + '", "value3":"' + str(notification['image']) + '"}'
    else:
        data_json = '{"value1":"' + str(notification['title']) + '", "value2":"' + str(notification['content']) + '"}'

    return data_json

########################################################################################################

# MAIN
########################################################################################################
logger = Logger(LOGGING_LEVEL, WRITE_TO_LOG_FILE, LOG_FILE_DIR)

logger.log('Starting Notify ' + VERSION)
RUNNING = True
DAY = datetime.datetime.today().weekday()
while(RUNNING):

    now = datetime.datetime.today()
    # If day has changed, roll over and reload config
    if( now.weekday() != DAY):
        DAY = now.weekday() 
        data[DAY] = json.load(open(NOTIFICATION_LOCATION + str(DAY) +'.json'))

    # Get minutes passed in the day
    minutes = (now.hour*60) + now.minute

    for notification in data[DAY]['notifications']:
        # If active is missing, add it 
        if 'active' not in notification:
            notification['active'] = 'true'

        # If notification is ready to send, Send!
        if notification['active'] == 'true' and int(notification['trigger']) <= minutes:
            
            # Send notification
            data_json = get_json(notification)
            r = requests.post(URL, data=data_json, headers=HEADERS)
            
            # If notification failed
            if not(r.ok):
                logger.warn("Failed to send notification: \n" + data_json)
            else:
                logger.log("Notification '" + str(notification['title']) + "' sent")

            notification['active'] = 'false'

    time.sleep(WAIT)

########################################################################################################
#   Copyright (C) 2022  Liam Coombs
#
#    This program is free software: you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation, either version 3 of the License, or
#    (at your option) any later version.
#
#    This program is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with this program.  If not, see <https://www.gnu.org/licenses/>.