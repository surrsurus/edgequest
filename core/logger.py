# Logger.py

# Amazing logger Max made'

'''

Logger class with lots of cool features.

EdgeQuest uses a system that prints to the console and writes to a file.

The code might be hard to understand because it's certified MaxCode(tm)

'''

# Imports ----------------------------------------------------------------------

import sys
import time
import os
import errno
import traceback

# ------------------------------------------------------------------------------

# Logger class -----------------------------------------------------------------

class Logger:
    '''
        [True]       USE_STDIO:        Write to standard console output if enabled
        [True]       FILE_OUTPUT:      Write to a logfile if enabled. Formatted `log_%Y-%m-%d_%H-%M-%S`
        [True]       USE_TIMESTAMPS:   Whether or not to prefix timestamps
        ['%H:%M:%S'] TIMESTAMP_FORMAT: The format string supplied to `time.strftime()`
    '''
    def __init__(self, USE_STDIO=True, FILE_OUTPUT=True, USE_TIMESTAMPS=True,
                 TIMESTAMP_FORMAT='%H:%M:%S', STDERR_THRESHOLD=20, LOGFILE_DIR='../logs'):
        self.WRITE_TO_STDIO   = USE_STDIO
        self.WRITE_TO_FILE    = FILE_OUTPUT
        self.TIMESTAMP_FORMAT = TIMESTAMP_FORMAT
        self.USE_TIMESTAMPS   = USE_TIMESTAMPS
        self.STDERR_THRESHOLD = STDERR_THRESHOLD
        self.LOGFILE_DIR      = LOGFILE_DIR
        if self.WRITE_TO_FILE:
            try:
                if not os.path.exists(self.LOGFILE_DIR):
                    os.mkdir(self.LOGFILE_DIR)
                self.LOGFILE = open(self.LOGFILE_DIR + '/log_' + time.strftime('%Y-%m-%d_%H-%M-%S') + '.eqlg', 'a')
            except (IOError, OSError) as e:
                self.WRITE_TO_FILE = False

    def log(self, level=0, *args):
        ''' Log from any amounts of arguments'''
        self.write('[' + time.strftime(self.TIMESTAMP_FORMAT) + '] ', level)
        for v in args:
            self.write(v + ' ', level)
        self.write('\n', level)

    def write(self, msg, level=0):
        ''' Write to a log file and/or print to stdout '''
        if self.WRITE_TO_STDIO:
            if self.STDERR_THRESHOLD > level:
                sys.stdout.write(msg)
            else:
                sys.stderr.write(msg)
        if self.WRITE_TO_FILE: self.LOGFILE.write(msg)

    # Helper functions for pre-formatted thingymajigs
    def debug(self, *args):
        self.log(0, '{0: <4}'.format('[DEBUG]'), *args)
    def info(self, *args):
        self.log(1, '{0: <4}'.format('[INFO]'), *args)
    def important(self, *args):
        self.log(10, '{0: <4}'.format('[IMP]'), *args)
    def warn(self, *args):
        self.log(15, '{0: <4}'.format('[WARN]'), *args)
    def error(self, *args):
        self.log(50, '{0: <4}'.format('[ERROR]'), *args)
    def severe(self, *args):
        self.log(100, '{0: <4}'.format('[SEVERE]'), *args)

# ------------------------------------------------------------------------------

# ------------------------------------------------------------------------------

# Use same logger class across multiple files without having to re-initialize
# class

logger = Logger(
    USE_STDIO        = True,
    FILE_OUTPUT      = True,
    USE_TIMESTAMPS   = True,
    TIMESTAMP_FORMAT = '%H:%M:%S',
    STDERR_THRESHOLD = 15
)

# ------------------------------------------------------------------------------

# Tests ------------------------------------------------------------------------

'''
l = Logger(
    USE_STDIO        = True,
    FILE_OUTPUT      = True,
    USE_TIMESTAMPS   = True,
    TIMESTAMP_FORMAT = '%H:%M:%S',
    STDERR_THRESHOLD = 15
)

l.debug('This is a debug')
l.info('This is info')
l.important('This is important')
l.warn('This is a warning')
l.error('This is an error')
l.severe('This is severe')

'''

# ------------------------------------------------------------------------------
