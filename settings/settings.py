# Settings.py

import modules.libtcodpy as libtcod

################################################################################
# Settings
################################################################################

# Store a lot of common values and libtcod objects needed

'''

These should be clear, but if you have problems, you can ask a question on the
GitHub issue tracker

'''

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

# Fortune cookie heal ammount
FORTUNE_HEAL = 10

# Dog range
DOG_RANGE = 10

# Monster Sense range
SENSE_RANGE = 7

# Mana Potion
MANA_AMOUNT = 40

# Siphon Spell cost
SIPHON_COST   = 1
SIPHON_AMOUNT = 10

# Magic Missile Spell cost
MISSILE_COST   = 10
MISSILE_DAMAGE = 7
MISSILE_RANGE  = 5

# Lightning Scroll
LIGHTNING_DAMAGE = 20
LIGHTNING_RANGE  = 16

# Confusion scroll
CONFUSE_RANGE     = 8
CONFUSE_NUM_TURNS = 10

# Fireball scroll
FIREBALL_RADIUS = 3
FIREBALL_DAMAGE = 25

# Experience and level-ups
# Experience Algorithm: LEVEL_UP_FACTOR * current level + base
LEVEL_UP_BASE   = 150
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

# Megadeath

MEGADEATH = 90000000

# Chance of strucutures spawning
STRUCT_CHANCE = 80

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

# Lists ------------------------------------------------------------------------

# List of weapon slot_list
WEAPON_SLOTS = [
    'right hand',
    'left hand'
]

# List of all slots
SLOT_LIST = [
    'right hand',
    'left hand',
    'head',
    'face',
    'neck',
    'torso',
    'hands',
    'legs',
    'feet',
    'accessory'
]

# List of color objects
COLORS = {
    'desaturated_green': libtcod.desaturated_green,
    'darker_green': libtcod.darker_green,
    'lime': libtcod.lime,
    'green': libtcod.green,
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
    'navy': libtcod.dark_blue,
    'yellow': libtcod.yellow,
    'red': libtcod.red
}

# List of colors for console
TEXT_COLORS = {
    'default'  : libtcod.white,
    'good'     : libtcod.light_green,
    'bad'      : libtcod.light_red,
    'very_bad' : libtcod.red,
    'level_up' : libtcod.yellow,
    'fail'     : libtcod.orange,
    'magic'    : libtcod.light_sea,
    'debug'    : libtcod.fuchsia,
    'neutral'  : libtcod.gray,
    'edge'     : libtcod.fuchsia,
    'crit'     : libtcod.gold,
    'title'    : libtcod.light_yellow
}

# List of consumables
CONSUMABLES = [
    'mountain dew',
    'coke zero'
]

# Fallback item / monster ------------------------------------------------------

FALLBACK_MONSTER = {
    "thegoof": {
        "name": "the goof",
        "id":"thegoof",
        "char": "G",
        "color": "lime",
        "chance":[[30, 1], [0, 3]],
      	"hp":5,
      	"defense":0,
      	"power":0,
      	"xp":1,
        "mana":0,
   	    "death_func":"normal",
        "attack_msg":"RAISES ERRORS at",
        "ai": "normal"
    }
}

# Theme settings ---------------------------------------------------------------

CURRENT_THEME = 'torchlight'

RAINBOW_MODE = False

# ------------------------------------------------------------------------------
