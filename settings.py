import modules.libtcodpy as libtcod

######################################
# Settings
######################################

######### Full Screen Key

FULLSCREEN_KEYS = [
    libtcod.KEY_TAB,
    libtcod.KEY_F4
    ]

######### Map

SCREEN_WIDTH = 80
SCREEN_HEIGHT = 37

# Size of the map portion shown on-screen
# Currently this is used as the top-left portion that fits in between
#   the console panel and the status panel
CAMERA_WIDTH = SCREEN_WIDTH - 20
CAMERA_HEIGHT = SCREEN_HEIGHT - 10

# Size of the map
MAP_WIDTH = 80
MAP_HEIGHT = 40

# FPS limiter. Doesn't do much in a turn based game
LIMIT_FPS = 10

# Parameters for dungeon generator
ROOM_MAX_SIZE = 10
ROOM_MIN_SIZE = 6
MAX_ROOMS = 8

# FOV Parameters
FOV_ALGO = 0
# Determines wheter the walls will light up. Looks nice when true
FOV_LIGHT_WALLS = True
TORCH_RADIUS = 10

######### UI

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
PANEL_WIDTH = 20
PANEL_Y = SCREEN_HEIGHT - PANEL_HEIGHT

# Console Panel settings
MSG_PANEL_HEIGHT = 10
MSG_PANEL_Y = SCREEN_HEIGHT - MSG_PANEL_HEIGHT

# Message bar's position and size
MSG_X = 1
MSG_WIDTH = SCREEN_WIDTH - BAR_WIDTH - 2
MSG_HEIGHT = MSG_PANEL_HEIGHT - 1

######### Item

# Healing Potion
HEAL_AMOUNT = 40

# Mana Potion
MANA_AMOUNT = 20

# Siphon Spell cost
SIPHON_COST = 5
SIPHON_AMOUNT = 5

# Magic Missile Spell cost
MISSILE_COST = 10
MISSILE_DAMAGE = 10
MISSILE_RANGE = 5

# Lightning Scroll
LIGHTNING_DAMAGE = 40
LIGHTNING_RANGE = 5

# Confusion scroll
CONFUSE_RANGE = 8
CONFUSE_NUM_TURNS = 10

# Fireball scroll
FIREBALL_RADIUS = 3
FIREBALL_DAMAGE = 25

# Experience and level-ups
LEVEL_UP_BASE = 200
LEVEL_UP_FACTOR = 150

# Blindness
BLIND_LENGTH = 10

# Firearms
FIREARM_RANGE = 8

######################################
# Debug Options
######################################

# FOW Bool. Default true
FOG_OF_WAR_ENABLED = True

# FOV Bool. Default true
FOV_ENABLED = True

# Invincibility. Default False
GOD_MODE = False

# Allows travel through walls. Default False
WALL_HACK = False

# Travel through floors anywhere
STAIR_HACK = False

# Coordinates show all the time
COORDS_UNDER_MOUSE = False

######################################
# Lists
######################################

movement_keys = [
    '1',
    '2',
    '3',
    '4',
    '6',
    '7',
    '8',
    '9',
    'y',
    'u',
    'h',
    'j',
    'k',
    'l',
    'b',
    'n'
]
