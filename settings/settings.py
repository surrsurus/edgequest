# Settings.py

import modules.libtcodpy as libtcod

################################################################################
# Settings
################################################################################

# Store a lot of common values and libtcod objects needed

# Libtcod settings -------------------------------------------------------------

# Screen width and height
SCREEN_WIDTH = 80
SCREEN_HEIGHT = 37

# FPS limiter. Doesn't do much in a turn based game
# Allows for faster astar movement and typing
# 25 provides a reasonable pace for the game to be set at
LIMIT_FPS = 25

# FOV Algorithm for libtcod to use
FOV_ALGO = 0

# Resource Locations -----------------------------------------------------------

# The locations for the JSON files that define the game content
MONSTER_JSON_PATH = 'json/monster.json'
ITEM_JSON_PATH = 'json/items.json'
COLOR_JSON_PATH = 'json/colors.json'

# Main menu image
MENU_IMAGE = 'images/menu_background.png'

# ------------------------------------------------------------------------------

# Map settings -----------------------------------------------------------------

# Size of the map
# Rendering gets very wonky the lower these are due to the camera system
MAP_WIDTH  = 80
MAP_HEIGHT = 40

# Size of the map portion shown on-screen
# Currently this is used as the top-left portion that fits in between
#   the console panel and the status panel
CAMERA_WIDTH  = SCREEN_WIDTH - 20
CAMERA_HEIGHT = SCREEN_HEIGHT - 10

# Parameters for dungeon generator
ROOM_MAX_SIZE = 10
ROOM_MIN_SIZE = 6
MAX_ROOMS     = 8

# ------------------------------------------------------------------------------

# Interface settings -----------------------------------------------------------

# Set the default player color and character
PLAYER_COLOR     = libtcod.gold
PLAYER_CHARACTER = '@'

# Inventory Screen width
INVENTORY_WIDTH = 50

# Level Up Screen width
LEVEL_SCREEN_WIDTH = 40

# Character Details screen width
CHARACTER_SCREEN_WIDTH = 30

# Sizes and coordinates relevant for the GUI
# Universal width for all bars
BAR_WIDTH = 20

# Status Panel settings
PANEL_HEIGHT = SCREEN_HEIGHT
PANEL_WIDTH  = 20
PANEL_Y      = SCREEN_HEIGHT - PANEL_HEIGHT

# Console Panel settings
MSG_PANEL_HEIGHT = 10
MSG_PANEL_Y      = SCREEN_HEIGHT - MSG_PANEL_HEIGHT

# Message bar's position and size
MSG_X      = 1
MSG_WIDTH  = SCREEN_WIDTH - BAR_WIDTH - 2
MSG_HEIGHT = MSG_PANEL_HEIGHT - 1

# ------------------------------------------------------------------------------

# Item and Game settings -------------------------------------------------------

# Healing Potion
HEAL_AMOUNT = 40

# Mana Potion
MANA_AMOUNT = 20

# Siphon Spell cost
SIPHON_COST   = 1
SIPHON_AMOUNT = 10

# Magic Missile Spell cost
MISSILE_COST   = 10
MISSILE_DAMAGE = 10
MISSILE_RANGE  = 5

# Lightning Scroll
LIGHTNING_DAMAGE = 10
LIGHTNING_RANGE  = 16

# Confusion scroll
CONFUSE_RANGE     = 8
CONFUSE_NUM_TURNS = 10

# Fireball scroll
FIREBALL_RADIUS = 3
FIREBALL_DAMAGE = 25

# Experience and level-ups
# Experience Algorithm: LEVEL_UP_FACTOR * current level + base
LEVEL_UP_BASE   = 200
LEVEL_UP_FACTOR = 150

# Length of blindness
BLIND_LENGTH = 10

# Firearm range
FIREARM_RANGE = 8

# Regen speed
REGEN_SPEED = 10

# Default attack message
DEFAULT_ATTACK = 'attacks'

# Default player name
DEFAULT_NAME = 'Max'

# Recieve perks at this many uses
PERK_BASE = 10

# Radius of FOV
TORCH_RADIUS = 10

# Ranged monster distance
MONSTER_RANGE = FIREARM_RANGE

# CSGO floor settings
CSGO_FLOOR = 5

# ------------------------------------------------------------------------------

# Debug settings ---------------------------------------------------------------

# Determines whether the map is revealed or not
# Default: true
FOG_OF_WAR_ENABLED = True

# Turns FOV on and off
# Default: true
FOV_ENABLED = True

# Player cannot die
# Default: false
GOD_MODE = False

# Allows travel through walls
# Default: false
WALL_HACK = False

# Travel through floors anywhere
# Default: false
STAIR_HACK = False

# Invisible mode (enemies can't see player)
# Default: false
INVISIBLE = False

# See all entities on map
# Default: false
SEE_ALL = False

# Coordinates show all the time when the mouse is hovered above entity
# Default: false
COORDS_UNDER_MOUSE = False

# Determines wheter the walls will light up. Looks nice when true
# Default: true
FOV_LIGHT_WALLS = True

# ------------------------------------------------------------------------------

# Intro settings ---------------------------------------------------------------

# Title crawl text
# the '' signifies a space inbetween the lines, as each element is drawn to a
#   line on the screen
INTRO_TEXT = [
    'You are an edgelord.',
    '','','','','','','',
    'You have trained all your life',
    '','',
    'in the arts of fedora tipping,',
    '','',
    'katana wielding,',
    '','',
    'and no-scoping with the AWP.',
    '','','','','','','','','','','',
    'Today your diety, Carl Sagan, has called upon you.',
    '','','','','','','',
    'You, his chosen servant, have been tasked with the ultimate feat',
    '','','',
    'You must summon all of your edge and delve into',
    '',
    'the stygian catacombs of mount myr',
    '',
    'and retrieve the sacred artifact:',
    '','','','','','','','','','',
    'The StatTrak Fedora | Fade (Factory New)',
    '','','','','','','','','',
    'Carl Sagan informs you that the journey will not be easy',
    '','','',
    'It will be perilous,',
    '','','','',
    'Full of danger,',
    '','','','',
    'full of monstrous enemies,',
    '','','','',
    'and full of people who personally prefer ruby over python.',
    '','','','','','','','',
    'Go! Young hero!',
    '','','','',
    'Retrive the Fedora of Fade!',
    '','','','','','',
    'May enlightenment and edge be ever at your heels'
]

# Buffer so that text appears to crawl from the bottom
BUFFER = ['' for x in range(SCREEN_HEIGHT)]

# Full intro wall of text
INTRO_WALL = BUFFER + INTRO_TEXT

# ------------------------------------------------------------------------------

# Keymaps ----------------------------------------------------------------------

# Keys that trigger fullscreen mode
FULLSCREEN_KEYS = [
    libtcod.KEY_TAB,
    libtcod.KEY_F4
]

# Keymap for movement keys
MOVEMENT_KEYS = [
    # Num pad keys
    '1',
    '2',
    '3',
    '4',
    '6',
    '7',
    '8',
    '9',
    # Vim keys
    'y',
    'u',
    'h',
    'j',
    'k',
    'l',
    'b',
    'n'
]

# Movement keymap for the key.vk libtcod objects
MOVEMENT_KEYS_VK = [
    # Arrow Keys
    libtcod.KEY_UP,
    libtcod.KEY_DOWN,
    libtcod.KEY_LEFT,
    libtcod.KEY_RIGHT
]

# ------------------------------------------------------------------------------

# Lists ------------------------------------------------------------------------

# List of weapon slot_list
WEAPON_SLOTS = [
    'right hand',
    'left hand'
]

# List of color objects
COLORS = {
    'desaturated_green': libtcod.desaturated_green,
    'darker_green': libtcod.darker_green,
    'lime': libtcod.lime,
    'violet': libtcod.violet,
    'light_yellow': libtcod.light_yellow,
    'gold': libtcod.gold,
    'sky': libtcod.sky,
    'azure': libtcod.azure,
    'light_turquoise': libtcod.light_turquoise,
    'light_sea': libtcod.light_sea,
    'light_red': libtcod.light_red,
    'dark_gray': libtcod.dark_gray,
    'light_green': libtcod.light_green,
    'gray': libtcod.gray,
    'darker_orange': libtcod.darker_orange,
    'silver': libtcod.silver,
    'white': libtcod.white,
    'dark_crimson': libtcod.dark_crimson,
    'crimson': libtcod.crimson,
    'chartreuse': libtcod.chartreuse,
    'black': libtcod.black,
    'orange': libtcod.orange,
    'ct_color': libtcod.Color(0, 0, 153),
    't_color': libtcod.Color(204, 102, 0),
    'red': libtcod.red
}

# Theme settings ---------------------------------------------------------------

CURRENT_THEME = 'torchlight'

# ------------------------------------------------------------------------------
