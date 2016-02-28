# Colors.py

from modules import libtcodpy as libtcod

################################################################################
# Colors
################################################################################

# Store a lot of colors for libtcod to use when rendering the dungeon

''' Format:

color_lighting_tile_[rgb]_color

* Always starts with 'color'
* Underline between each identifier
* Second item is the lighting, dark or light
* Third item is the tile piece, wall or floor
* Fourth is an optional rgb value that specifies whether libtcod.Color is used
* Last should always be a color descriptor

'''

# Dark colors ------------------------------------------------------------------
# Mainly used for 'unlit' floors and walls where the player cannot see

# Absolute azure colors
color_dark_wall_azure         = libtcod.darkest_azure
color_dark_ground_azure       = libtcod.azure

# RGB azure colors
color_dark_wall_rgb_azure     = libtcod.Color(0, 0, 100)
color_dark_ground_rgb_azure   = libtcod.Color(50, 50, 150)

# RGB Lighter gray colors
color_dark_wall_rgb_gray      = libtcod.Color(98, 98, 98)
color_dark_ground_rgb_gray    = libtcod.Color(128,128, 128)

# Default colors
# Grayish wall colors made by Max
color_dark_wall               = libtcod.Color(51, 51, 51)
color_dark_ground             = libtcod.Color(33, 33, 33)

# ------------------------------------------------------------------------------

# Light colors -----------------------------------------------------------------
# Used where the player can see. These colors are lighter and maybe yellowish

# RGB off-yellow-ish colors
color_light_wall_rgb_yellow   = libtcod.Color(130, 110, 50)
color_light_ground_rgb_yellow = libtcod.Color(200, 180, 50)

# Default colors
# RGB Earthy yellow and brown colors made by Max
color_light_ground            = libtcod.Color(110, 109, 91)
color_light_wall              = libtcod.Color(128, 127, 98)

# ------------------------------------------------------------------------------
