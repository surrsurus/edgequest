# Colors.py

# Imports ----------------------------------------------------------------------

import traceback
from modules import libtcodpy as libtcod
from modules import simplejson as json
from settings import CURRENT_THEME
from settings import COLOR_JSON_PATH

# ------------------------------------------------------------------------------

# Colors -----------------------------------------------------------------------

# Global Colors

color_light_ground     = libtcod.Color(110, 109, 91)
color_light_wall       = libtcod.Color(128, 127, 98)
color_dark_wall        = libtcod.Color(51, 51, 51)
color_dark_ground      = libtcod.Color(33, 33, 33)
color_wall_highlight   = libtcod.Color(130, 110, 50)
color_ground_highlight = libtcod.Color(200, 180, 50)

# ------------------------------------------------------------------------------

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

        print 'Color: (' + str(red_bit) + ', ' + str(green_bit) + ', ' + \
            str(blue_bit) + ')'

        return libtcod.Color(red_bit, green_bit, blue_bit);

    # If the color sting is mal-formatted
    except ValueError as e:
        print 'Error converting hex to color! Is it `rrggbb` formatted?'
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

    color_light_ground     = libtcod.Color(110, 109, 91)
    color_light_wall       = libtcod.Color(128, 127, 98)
    color_dark_wall        = libtcod.Color(51, 51, 51)
    color_dark_ground      = libtcod.Color(33, 33, 33)
    color_wall_highlight   = libtcod.Color(130, 110, 50)
    color_ground_highlight = libtcod.Color(200, 180, 50)

with open(COLOR_JSON_PATH) as json_data:
    color_data = json.load(json_data)

# Setting colors to the absence of colors is kind of hard. Let's not do that.
if CURRENT_THEME is not None:
    print 'Theme set: ' + CURRENT_THEME

    # Set all the colors here
    # Yes, I know. It goes over the proper line wrapping length.
    # The other way is 10x less readable, so I am not wrapping it.
    try:
        color_light_ground     = get_color(color_data[CURRENT_THEME]['ground']['light'])
        color_dark_ground      = get_color(color_data[CURRENT_THEME]['ground']['dark'])
        color_light_wall       = get_color(color_data[CURRENT_THEME]['wall']['light'])
        color_dark_wall        = get_color(color_data[CURRENT_THEME]['wall']['dark'])
        color_wall_highlight   = get_color(color_data[CURRENT_THEME]['highlight']['wall'])
        color_ground_highlight = get_color(color_data[CURRENT_THEME]['highlight']['ground'])
    except KeyError as e:
        print 'Something went terribly wrong while loading... Using defaults...'
        default_colors()
        print '----- STACK TRACE: -----'
        traceback.print_exc()
        print '------------------------'

    # Simple assertion testing to make sure all the colors loaded properly
    try:
        assert color_light_ground     is not None
        assert color_dark_ground      is not None
        assert color_light_wall       is not None
        assert color_dark_wall        is not None
        assert color_wall_highlight   is not None
        assert color_ground_highlight is not None
    except AssertionError as e:
        print 'Looks like there is invalid or missing data in the `' +\
            CURRENT_THEME + '` theme! Using defaults...'
        default_colors()
        print '----- STACK TRACE: -----'
        traceback.print_exc()
        print '------------------------'
else:
    print 'Warning: Theme is not set in settings.py! Using defaults...'
    default_colors()

# ------------------------------------------------------------------------------
