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