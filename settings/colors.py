# Colors.py

# Generate color themes from colors.json

'''

This file is kind of weird because it's a mix between a core and settings module

Really this file is great for setting the color themes of EdgeQuest in a crazy easy
way and it has a lot of fallbacks and error handling that allows modders to know
that they suck at making JSON, which isn't even a programming language.

Max made this, therefore it might be completely confusing.

'''

# Imports ----------------------------------------------------------------------

import traceback

from modules import libtcodpy as libtcod
from modules import simplejson as json
from core.logger import logger
from settings import COLOR_JSON_PATH, CURRENT_THEME
import random

# ------------------------------------------------------------------------------

# Colors -----------------------------------------------------------------------

# Global Colors
color_light_ground     = libtcod.Color(110, 109, 91)
color_light_wall       = libtcod.Color(128, 127, 98)
color_dark_wall        = libtcod.Color(51, 51, 51)
color_dark_ground      = libtcod.Color(33, 33, 33)
color_wall_highlight   = libtcod.Color(130, 110, 50)
color_ground_highlight = libtcod.Color(200, 180, 50)
color_accent           = libtcod.Color(255, 255, 255)

# ------------------------------------------------------------------------------

# Open the colors.json file
with open(COLOR_JSON_PATH) as json_data:
    color_data = json.load(json_data)

# Functions --------------------------------------------------------------------

def hex_to_color(inp):
    ''' Converts a hex string into a libtcod Color object '''
    # Format the string for use
    fhex = inp.replace('#', '').strip()

    try:
        # Get string slices for each hex bit and convert them to numbers
        # NOTE: String slices are in base 16
        red_bit   = int(fhex[0:2], 16)
        green_bit = int(fhex[2:4], 16)
        blue_bit  = int(fhex[4:6], 16)

        logger.info('Color: (' + str(red_bit) + ', ' + str(green_bit) + ', ' + \
            str(blue_bit) + ')')

        return libtcod.Color(red_bit, green_bit, blue_bit);

    # If the color sting is mal-formatted
    except ValueError as e:
        logger.error('Problem converting hex to color! Is it `rrggbb` formatted?')
        return None

def get_color(col):
    ''' returns the color based on whether it's
    unicode or a list of rgb values '''
    if type(col) is unicode:
        return hex_to_color(col)
    elif type(col) is list:
        return libtcod.Color(*col)

def default_colors():
    ''' Sets the values used in the main file to hard-coded defaults
    Should be used as a fallback in case something goes wrong and we cannot
    load the colors from our theme file
    '''
    global color_light_ground, color_light_wall, \
        color_dark_wall, color_dark_ground, \
        color_wall_highlight, color_ground_highlight, \
        color_accent

    logger.info('Setting to default color scheme...')

    color_light_ground     = libtcod.Color(110, 109, 91)
    color_light_wall       = libtcod.Color(128, 127, 98)
    color_dark_wall        = libtcod.Color(51, 51, 51)
    color_dark_ground      = libtcod.Color(33, 33, 33)
    color_wall_highlight   = libtcod.Color(130, 110, 50)
    color_ground_highlight = libtcod.Color(200, 180, 50)
    color_accent           = libtcod.Color(255, 255, 255)

def rand_color():
    ''' Get a random RGB color '''
    r = random.randint(0, 255)
    g = random.randint(0, 255)
    b = random.randint(0, 255)
    return libtcod.Color(r, g, b)

def set_theme(theme):
    ''' Set theme '''
    global color_light_ground, color_light_wall, \
        color_dark_wall, color_dark_ground, \
        color_wall_highlight, color_ground_highlight, \
        color_accent

    # Setting colors to the absence of colors is kind of hard. Let's not do that.
    if theme is not None:
        logger.info('Theme set: ' + theme)

        # Set all the colors here
        # Yes, I know. It goes over the proper line wrapping length.
        # The other way is 10x less readable, so I am not wrapping it.
        try:
            color_light_ground     = get_color(color_data[theme]['ground']['light'])
            color_dark_ground      = get_color(color_data[theme]['ground']['dark'])
            color_light_wall       = get_color(color_data[theme]['wall']['light'])
            color_dark_wall        = get_color(color_data[theme]['wall']['dark'])
            color_wall_highlight   = get_color(color_data[theme]['highlight']['wall'])
            color_ground_highlight = get_color(color_data[theme]['highlight']['ground'])
            color_accent           = get_color(color_data[theme]['accent']['light'])
        except KeyError as e:
            logger.severe("KeyError({0}): {1}".format(e.errno, e.strerror))
            default_colors()
            logger.write('----- STACK TRACE: -----')
            logger.write(traceback.extract_stack())
            logger.write('------------------------')

        # Simple assertion testing to make sure all the colors loaded properly
        try:
            assert color_light_ground     is not None
            assert color_dark_ground      is not None
            assert color_light_wall       is not None
            assert color_dark_wall        is not None
            assert color_wall_highlight   is not None
            assert color_ground_highlight is not None
            assert color_accent           is not None
        except AssertionError as e:
            logger.severe("AssertionError ({0}): {1}".format(e.errno, e.strerror))
            default_colors()
            logger.write('----- STACK TRACE: -----')
            logger.write(traceback.extract_stack())
            logger.write('------------------------')
    else:
        logger.warn('No theme passed to set_theme()!')
        logger.info('Defaulting theme...')
        default_colors()

# ------------------------------------------------------------------------------
