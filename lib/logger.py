# Logger library to make code a bit neater and i'm lazy
#
#   Logging Level table:
#       0 - ERROR
#       1 - WARN
#       2 - LOG
#       3 - DEBUG
#
########################################################################################################
from datetime import datetime

class Logger:

    def __init__(self, logging_level):
        self.level = logging_level

    @staticmethod
    def timestamp():
        return datetime.now().strftime("[%H:%M:%S]")

    def write(self, content, level):

        prefix = ""
        
        if(level == 0):
            prefix = '[ERROR]'
        elif (level == 1):
            prefix = '[ WARN]'
        elif (level == 2):
            prefix = '[  LOG]'
        elif (level == 3):
            prefix = '[DEBUG]'
        else:
            prefix = '[ -' + str(level) + '- ]'
        
        print(Logger.timestamp() + prefix + content)

    def error(self, content):
        self.write(content, 0)

    def warn(self, content):
        self.write(content, 1)

    def log(self, content):
        self.write(content, 2)

    def debug(self, content):
        self.write(content, 3)
