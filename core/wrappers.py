# Wrappers.py

# Libtcod wrappers to make things shorter

'''

libtcod calls get kind of long so we can abstract those with wrapper functions
that do the same thing with less space

You can read about each of these below.

This is purely for aesthetics, and not really required, so this can be removed
if one wanted to

'''

# Imports ----------------------------------------------------------------------

# Rendering library
from modules import libtcodpy as libtcod

# ------------------------------------------------------------------------------

# Libtcod Wrappers -------------------------------------------------------------

def tcod_print_ex(console, x, y, background, alignment, char):
    ''' Wrap console_print_ex '''
    libtcod.console_print_ex(console, x, y, background, alignment, char)

def tcod_set_char_bg(console, x, y, color, bg_set=None):
    ''' Wrap console_set_char_background '''
    if bg_set:
        libtcod.console_set_char_background(console, x, y, color, bg_set)
    else:
        libtcod.console_set_char_background(console, x, y, color)

def tcod_put_char_ex(console, x, y, char, fg_color, bg_color):
    ''' Wrap console_put_char_ex '''
    libtcod.console_put_char_ex(console, x, y, char, fg_color, bg_color)

def tcod_put_char(console, x, y, char, bg_set):
    ''' Wrap put char '''
    libtcod.console_put_char(console, x, y, char, bg_set)

def tcod_set_fg(console, color):
    ''' Wrap the foreground setter '''
    libtcod.console_set_default_foreground(console, color)

def tcod_set_bg(console, color):
    ''' Wrap background setter '''
    libtcod.console_set_default_background(console, color)

def tcod_clear(console):
    ''' Wrap console clearer '''
    libtcod.console_clear(console)

# ------------------------------------------------------------------------------
