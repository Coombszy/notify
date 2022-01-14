from operator import truediv
from unittest import runner
import requests

# CONFIGS
########################################################################################################
URL = 'https://maker.ifttt.com/trigger/notification_phone/with/key/dAwxxBLcB3qNYgaNR1XL2U'
HEADERS = {'content-type': 'application/json', 'Accept-Charset': 'UTF-8'}
JSON = '{"value1":"REPLACEME1", "value2":"REPLACEME2"}'
VERSION = 'v0.1'
########################################################################################################

# DATA
########################################################################################################
data = []

data

########################################################################################################

print('Starting Notify ' + VERSION)
RUNNING = True
# while(RUNNING):
    



r = requests.post(URL, data=JSON, headers=HEADERS)




