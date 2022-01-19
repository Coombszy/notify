# Logger library to make code a bit neater and i'm lazy
#
#   Logging Level table:
#       0 - ERROR
#       1 - WARNING
#       2 - LOGGING
#       3 - DEBUG
#
########################################################################################################
class Logger:

    def __init__(self, logging_level):
        self.level = logging_level

    def log(content, level):
        print()
