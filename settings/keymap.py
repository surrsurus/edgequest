# Keymap.py

''' You can set your custom keymaps here while playing EdgeQuest

Please note that all of your changes will be overwritten with each consecutive
release of the game

'''

# Imports ----------------------------------------------------------------------

from modules import libtcodpy as libtcod

# ------------------------------------------------------------------------------

# Keymap -----------------------------------------------------------------------

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

WAIT_KEYS = [
    '5',
    '.'
]

# Keys that trigger fullscreen mode
FULLSCREEN_KEYS = [
    libtcod.KEY_TAB,
    libtcod.KEY_F4
]

QUIT_KEY = libtcod.KEY_ESCAPE

# Keys based on the actual characters
GET_ITEM_KEY = 'g'
DROP_ITEM_KEY = 'd'
INVENTORY_KEY = 'i'
EQUIPMENT_MENU_KEY = 'e'
FOOD_MENU_KEY = 'E'
GO_DOWN_KEY = '>'
GO_UP_KEY = '<'
STATS_KEY = 'c'
TOGGLE_SIPHON_KEY = 'q'
TAUNT_KEY = 't'
ACTIVATE_WEAPON_KEY = 'f'
CAST_MAGIC_MISSLE_KEY = 'm'
SHOW_HELP_KEY = '?'

# Debug
SPAWN_DEBUG_CONSOLE_M_KEY = 'z'
SPAWN_DEBUG_CONSOLE_I_KEY = 'x'
KILL_ALL_KEY = 'v'
SPAWN_MONSTER_KEY = 'o'
RELOAD_MAP_KEY = 'r'
