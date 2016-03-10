# Logger.py

# Amazing logger Max made

'''

Logger class with lots of cool features.

EdgeQuest uses a system that prints to the console and writes to a file.

The code might be hard to understand because it's certified MaxCode(tm)

'''

# Imports ----------------------------------------------------------------------

import sys
import time
import os
import re
import errno
import traceback

# ------------------------------------------------------------------------------

# Logger class -----------------------------------------------------------------

class Logger:
    def __init__(self,
            # Defaults.
            use_stdio        = True,
            file_output      = True,
            use_timestamps   = True,
            timestamp_format = '%H:%M:%S',
            stderr_threshold = 20,
            logfile_dir      = '../logs'):
        # Set the internals...
        self.write_to_stdio   = use_stdio
        self.write_to_file    = file_output
        self.timestamp_format = timestamp_format
        self.use_timestamps   = use_timestamps
        self.stderr_threshold = stderr_threshold
        self.logfile_dir      = logfile_dir
        if self.write_to_file:
            # Try to open the file
            try:
                # Make the directory if it does not exist
                if not os.path.exists(self.logfile_dir):
                    os.mkdir(self.logfile_dir)
                self.LOGFILE = open(self.logfile_dir + '/log_' + time.strftime('%Y-%m-%d_%H-%M-%S') + '.eqlg', 'a')
            except (IOError, OSError) as e:
                self.write_to_file = False
                print 'LOGGER INIT ERROR: CANNOT WRITE TO FILE'
                print 'NONFATAL'
                print 'This is likely because edgequest does not have write permissions'
                traceback.print_exc()

    def log(self, level=0, *args):
        ''' Log from any amounts of arguments'''
        # NOTE: This function DOES NOT generate a [LOG] stamp.
        # NOTE: It will only print a timestamp and the args passed to it.
        self.write('[' + time.strftime(self.timestamp_format) + '] ', level)
        for v in args:
            self.write(str(v) + ' ', level)
        self.write('\n', level)

    def write(self, msg, level=0):
        ''' Write to a log file and/or print to stdout '''
        if self.write_to_stdio:
            if self.stderr_threshold > level:
                sys.stdout.write(msg)
            else:
                sys.stderr.write(msg)
        if self.write_to_file: self.LOGFILE.write(msg)

    # Helper functions for pre-formatted thingymajigs
    # Guide --------------------------------------------------------------------
    #
    #   Debug:
    #       Use this if you are printing generally unhelpful but
    #       very internal information.
    #   Info:
    #       Use this if you need something printed. Basically the default.
    #   Important:
    #       Use this if you want higher priority than info but not
    #       conveyed as a warning.
    #       May be used for noting something.
    #   Warn:
    #       Use this to log conflicts or unstable things
    #       This may also be helpful when logging things that are mildly wrong
    #       or should not be used in the future.
    #   Error:
    #       Use this if you have had a non-game-breaking error
    #   Severe:
    #       Use this to convey that something went horribly wrong
    #       Usually a game breaking issue or system issue
    #
    # --------------------------------------------------------------------------
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

    def print_exception(self):
        etype, evalue, etb = sys.exc_info()
        fetype  = re.compile(r'.*\.(.*)\'').match(str(etype)).group(1)
        # evalue is already nice and readable!
        self.error(str(fetype) + ': ' + str(evalue))

        # Get a list of the call stack at the time
        # Each entry is a tuple: (filename, line number, function name, text)
        tbl = traceback.extract_tb(etb)
        for t in tbl:
            filename = t[0]
            linenum  = t[1]
            funcname = t[2]
            text     = t[3]

            if text is None: text = ''

            v = filename + ':' + str(linenum) + '\tat ' + funcname
            self.error(v)

# ------------------------------------------------------------------------------

# ------------------------------------------------------------------------------

# Use same logger class across multiple files without having to re-initialize
# class

logger = Logger(
    use_stdio        = True,
    file_output      = True,
    use_timestamps   = True,
    timestamp_format = '%H:%M:%S',
    stderr_threshold = 15
)

# ------------------------------------------------------------------------------

# Tests ------------------------------------------------------------------------

'''
l = Logger(
    use_stdio        = True,
    file_output      = True,
    use_timestamps   = True,
    timestamp_format = '%H:%M:%S',
    stderr_threshold = 15
)

l.debug('This is a debug')
l.info('This is info')
l.important('This is important')
l.warn('This is a warning')
l.error('This is an error')
l.severe('This is severe')

'''

# ------------------------------------------------------------------------------
