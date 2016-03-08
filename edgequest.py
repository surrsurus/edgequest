# Edgequest.py

# Imports ----------------------------------------------------------------------

# Import necessary python modules
import math
import random
import shelve
import sys
import textwrap
import time
import traceback

# Camera controls
import core.camera as camera

# This import is different because the theme can change
# So we want to use the latest variables, not what the defaults are
import settings.colors as colors

# Animation tools
import core.animtools as anim

# Fortune generator
import core.fortune as fortune

# Map generation
from core.dmap import dMap

# Logger
from core.logger import logger

# Wall decorations
from core.wallselect import wallselect

# Libtcod wrappers
from core.wrappers import *

# Rendering library
from modules import libtcodpy as libtcod

# JSON parsing library
from modules import simplejson as json

# Settings
from settings.keymap import *
from settings.settings import *

# ------------------------------------------------------------------------------

# JSON loading -----------------------------------------------------------------
# Paths are defined in settings.py

# Load monsters
with open(MONSTER_JSON_PATH) as json_data:
    monster_data = json.load(json_data)

# Load items
with open(ITEM_JSON_PATH) as json_data:
    items_data = json.load(json_data)

# ------------------------------------------------------------------------------

# Game object placeholders -----------------------------------------------------

# Player object
player = None

# Dog object
dog = None

# We need to set this to prevent a segfault because py2.7 nested functions do weird stuff
player_name = DEFAULT_NAME

# Object List
objects = []

# Create the list of game messages and their colors, starts empty
game_msgs = []

# Inventory
inventory = []

# Map
world = None

# FOV Map
fov_map = None

# Stairs objects
dstairs = None
ustairs = None

# ------------------------------------------------------------------------------

# GUI objects ------------------------------------------------------------------

# Right panel for showing player stats
panel = libtcod.console_new(PANEL_WIDTH, SCREEN_HEIGHT)

# Bottom panel for showing console messages
msg_panel = libtcod.console_new(SCREEN_WIDTH, PANEL_HEIGHT)

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

# libtcod objects --------------------------------------------------------------

# Font
libtcod.console_set_custom_font('images/terminal10x16_gs_tc.png',
    libtcod.FONT_TYPE_GREYSCALE | libtcod.FONT_LAYOUT_TCOD)

# Initialize root console
libtcod.console_init_root(SCREEN_WIDTH, SCREEN_HEIGHT,
                            'Edgequest Pre-Alpha', False)

# And another
con = libtcod.console_new(MAP_WIDTH, MAP_HEIGHT)

# And one for a player-centered focus
dcon = libtcod.console_new(SCREEN_WIDTH, SCREEN_HEIGHT)

# And one for animations
anicon = libtcod.console_new(MAP_WIDTH, MAP_HEIGHT)

# FPS Limit
libtcod.sys_set_fps(LIMIT_FPS)

# Mouse and Keyboard detection
mouse = libtcod.Mouse()
key = libtcod.Key()

# ------------------------------------------------------------------------------

# Game variables ---------------------------------------------------------------

# Game State
game_state = 'playing'

# Player action
player_action = None

# Blindness tracking
blind         = False  # State
blind_counter = 0      # Turns since blinded

# Siphon
activate_siphon = True

# Message store
old_msg     = None
msg_counter = 1

# Dungeon level
dungeon_level = 1

# Timer
timer = 0

# Killstreak
kill_count = 0

# Stairs direction
stairs_up = True

# Perk tracking
perk_mtndew    = 0
perk_cokezero  = 0
perk_tazer     = 0
perk_incengren = 0
perk_fbang     = 0

# An array with all unblocked coords
unblocked_world = []

# ------------------------------------------------------------------------------

################################################################################
# Classes
################################################################################

# AI Classes -------------------------------------------------------------------

'''

These classes are assigned to monsters that should act autonomously.
Monsters usually have a fighter and ai class inside of the object class

You can see how this is implemented in the generate_monster function

'''

class BasicMonster:
    ''' AI for a basic monster.

    Has the functionality to track the player, attack it, and move to random
    locations around the map

    All monsters are based around this AI, and you can see the variations
    very easily by comparing it to this class

    '''
    def __init__(self):
        # Create a coordinate the monster travels to if it
        #   doesn't see the player
        self.backup_coord = get_rand_unblocked_coord()
        # Tamed variable
        self.tamed = False

    def take_turn(self):
        '''Monster takes its turn. If you can see it, it can see you '''

        # Always check the owner
        monster = self.owner

        # Get data
        # Make sure that the monster prioritizes between tamed monsters
        # and the player if it's in range
        # Get the closest tamed monster
        tamed_monster = closest_tamed_monster(SENSE_RANGE)
        # If it exists, store the distance
        if tamed_monster is not None:
            distance_to_tamed = monster.distance_to(tamed_monster)
        # Otherwise we don't need it so we set it to a HUGE int
        else:
            distance_to_tamed = MEGADEATH
        # Get distance to player
        distance_to_player = monster.distance_to(player)

        # Is the player in the fov?
        sees_player = libtcod.map_is_in_fov(fov_map, monster.x, monster.y)

        # If it's in the player's fov then it approaches them
        if distance_to_tamed < SENSE_RANGE and not sees_player:
            # Move towards player if far away
            if monster.distance_to(tamed_monster) >= 2:
                monster.move_astar(tamed_monster.x, tamed_monster.y, False)

            # Close enough, attack! (if the player is still alive.)
            elif player.fighter.hp > 0:
                monster.fighter.attack(tamed_monster)

        if sees_player and not INVISIBLE:

            # Select what to attack
            # Prioritize the closer one
            if distance_to_tamed < distance_to_player:
                target = tamed_monster
            # If they're equal, prioritize the player
            elif distance_to_tamed == distance_to_player:
                target = player
            # Otherwise the distance to the player must be shorter,
            # Therefore, prioritize the player
            else:
                target = player

            # Move towards target if far away
            if monster.distance_to(target) >= 2:
                monster.move_astar(target.x, target.y, False)

            # Close enough, attack! (if the target is still alive.)
            elif target.fighter.hp > 0:
                monster.fighter.attack(target)

        # Otherwise it moves to a random map location
        else:
            x, y = self.backup_coord
            monster.move_astar(x, y, False)

        # If the monster has reached the coord position, make a new one
        if (monster.x, monster.y) == self.backup_coord:
            self.backup_coord = get_rand_unblocked_coord()

class CenaMonster:
    ''' AI for John Cena.

    This is a fun joke monster that has some properties that are better suited
    for it's own class rather than hardwired if-elses somewhere in the code

    '''
    def __init__(self):
        # Create a coordinate the monster travels to if it
        #   doesn't see the player
        self.backup_coord = get_rand_unblocked_coord()
        self.saw_player = False
        # Tamed variable
        self.tamed = False

    def take_turn(self):
        '''Monster takes its turn. If you can see it, it can see you '''

        # Always check the owner
        monster = self.owner

        # Get data
        # Make sure that the monster prioritizes between tamed monsters
        # and the player if it's in range
        # Get the closest tamed monster
        tamed_monster = closest_tamed_monster(SENSE_RANGE)
        # If it exists, store the distance
        if tamed_monster is not None:
            distance_to_tamed = monster.distance_to(tamed_monster)
        # Otherwise we don't need it so we set it to a HUGE int
        else:
            distance_to_tamed = MEGADEATH
        # Get distance to player
        distance_to_player = monster.distance_to(player)

        # Is the player in the fov?
        sees_player = libtcod.map_is_in_fov(fov_map, monster.x, monster.y)

        # If it's in the player's fov then it approaches them
        if distance_to_tamed < SENSE_RANGE and not sees_player:
            # Move towards player if far away
            if monster.distance_to(tamed_monster) >= 2:
                monster.move_astar(tamed_monster.x, tamed_monster.y, False)

            # Close enough, attack! (if the player is still alive.)
            elif player.fighter.hp > 0:
                monster.fighter.attack(tamed_monster)

        if sees_player and not INVISIBLE:

            # Select what to attack
            # Prioritize the closer one
            if distance_to_tamed < distance_to_player:
                target = tamed_monster
            # If they're equal, prioritize the player
            elif distance_to_tamed == distance_to_player:
                target = player
            # Otherwise the distance to the player must be shorter,
            # Therefore, prioritize the player
            else:
                target = player

            # JOOOOOHN CENA
            # If John cena is in the fov, say his name once
            if not self.saw_player:
                message("AND HIS NAME IS... JOHN CENA!", self.owner.color)
                self.saw_player = True

            # Move towards target if far away
            if monster.distance_to(target) >= 2:
                monster.move_astar(target.x, target.y, False)

            # Close enough, attack! (if the target is still alive.)
            elif target.fighter.hp > 0:
                monster.fighter.attack(target)

        # Otherwise it moves to a random map location
        else:
            x, y = self.backup_coord
            monster.move_astar(x, y, False)

        # If the monster has reached the coord position, make a new one
        if (monster.x, monster.y) == self.backup_coord:
            self.backup_coord = get_rand_unblocked_coord()

class ConfusedMonster:
    ''' AI for a temporarily confused monster

    This makes the confuse spell actually do things. The monster just bumbles
    around randomly.

    This will revert to previous AI after a while.

    '''
    def __init__(self, old_ai, num_turns=CONFUSE_NUM_TURNS):
        ''' Get some important data on init '''
        # You defintely want to store the old ai, or else the monster will
        # not be able to revert
        self.old_ai = old_ai
        # Not really needed, as the number of turns are defined in settings.py
        # But we use it as a count-down timer instead
        self.num_turns = num_turns
        # Tamed variable
        self.tamed = False

    def take_turn(self):
        ''' Monster takes a turn, but moves randomly '''
        # Still confused...
        if self.num_turns > 0:
            # Move in a random direction, and decrease the number of
            # turns confused
            self.owner.move(libtcod.random_get_int(0, -1, 1),
                            libtcod.random_get_int(0, -1, 1))
            self.num_turns -= 1

        # Restore the previous AI
        # (this one will be deleted because it's not referenced anymore)
        else:
            self.owner.ai = self.old_ai
            message('The ' + self.owner.name + ' is no longer confused!',
                TEXT_COLORS['bad'])

class TalkingMonster:
    ''' An AI that says things

    Basically a basic monster but with a bit more character. Kind of fun
    for 'unique' monsters with rare spawn rates

    '''
    def __init__(self, speech, rate):
        ''' Initialize the speech and rate (as well as the backup_coord) '''
        # Must be a list
        self.speech = speech
        # Rate out of 100. Basically an integer percent chance
        self.rate = rate
        # backup_coord. You should have seen this previously
        self.backup_coord = get_rand_unblocked_coord()
        # Tamed variable
        self.tamed = False

    def take_turn(self):
        ''' Monster takes a normal turn, but says something '''
        # A basic monster takes its turn. If you can see it, it can see you
        monster = self.owner

        # Get data
        # Make sure that the monster prioritizes between tamed monsters
        # and the player if it's in range
        # Get the closest tamed monster
        tamed_monster = closest_tamed_monster(SENSE_RANGE)
        # If it exists, store the distance
        if tamed_monster is not None:
            distance_to_tamed = monster.distance_to(tamed_monster)
        # Otherwise we don't need it so we set it to a HUGE int
        else:
            distance_to_tamed = MEGADEATH
        # Get distance to player
        distance_to_player = monster.distance_to(player)

        # Is the player in the fov?
        sees_player = libtcod.map_is_in_fov(fov_map, monster.x, monster.y)

        # If it's in the player's fov then it approaches them
        if distance_to_tamed < SENSE_RANGE and not sees_player:
            # Move towards player if far away
            if monster.distance_to(tamed_monster) >= 2:
                monster.move_astar(tamed_monster.x, tamed_monster.y, False)

            # Close enough, attack! (if the player is still alive.)
            elif player.fighter.hp > 0:
                monster.fighter.attack(tamed_monster)

        if sees_player and not INVISIBLE:

            # Select what to attack
            # Prioritize the closer one
            if distance_to_tamed < distance_to_player:
                target = tamed_monster
            # If they're equal, prioritize the player
            elif distance_to_tamed == distance_to_player:
                target = player
            # Otherwise the distance to the player must be shorter,
            # Therefore, prioritize the player
            else:
                target = player

            # Move towards target if far away
            if monster.distance_to(target) >= 2:
                monster.move_astar(target.x, target.y, False)

            # Close enough, attack! (if the target is still alive.)
            elif target.fighter.hp > 0:
                monster.fighter.attack(target)

            # Depending on the rate of speech set in the json,
            #   the monster may talk
            # Rate must be a value from 0 - 99
            # The higher rate is, the less frequent the monster will talk
            if libtcod.random_get_int(0, 0, 100) > self.rate:
                # Say a random line
                msg = random.choice(self.speech)
                message(''.join([monster.name.capitalize(), ' says \'',
                    msg, '\'']), monster.color)

        # Otherwise it moves to a random map location
        else:
            x, y = self.backup_coord
            monster.move_astar(x, y, False)

        # If the monster reaches the backup_coord, make a new one
        if (monster.x, monster.y) == self.backup_coord:
            self.backup_coord = get_rand_unblocked_coord()

class TamedMonster:
    ''' AI for a basic monster.

    Tamed monsters have special privleges compared to other AI types.
    They can:
        * Follow player
        * Attack enemies
        * Have enemies attack them
        * Level up!

    Basically it's a whole lot of fun.

    '''
    def __init__(self):
        # Create a coordinate the monster travels to if it
        #   doesn't see the player
        self.backup_coord = get_rand_unblocked_coord()

        # Tamed is true! This monster is special.
        self.tamed = True

    def take_turn(self):
        '''Monster takes its turn. If you can see it, it can see you '''

        # Always check the owner
        monster = self.owner

        # Is the monster in the player fov
        sees_player = libtcod.map_is_in_fov(fov_map, monster.x, monster.y)

        # If it's in the player's fov then it approaches them
        if sees_player and not INVISIBLE:

            # See if there's any monsters near
            mon = closest_monster(DOG_RANGE)

            # Move towards player if far away and not displaced
            if self.owner.displaced:
                # flip variable. Prevents moving for one turn
                self.owner.displaced = False

            # If there's a monster, attack it!
            elif mon != None:

                # Move towards monster
                if monster.distance_to(mon) >= 2:
                    monster.move_astar(mon.x, mon.y, False)
                # Close enough, attack! (if the player is still alive.)
                elif player.fighter.hp > 0:
                    monster.fighter.attack(mon)

            # Move towards player
            elif monster.distance_to(player) >= 2:
                monster.move_astar(player.x, player.y, False)

        # Otherwise it moves to a random map location
        else:
            x, y = self.backup_coord
            monster.move_astar(x, y, False)

        # If the monster has reached the coord position, make a new one
        if (monster.x, monster.y) == self.backup_coord:
            self.backup_coord = get_rand_unblocked_coord()

class RangedTalkerMonster:
    ''' An AI that says things and shoots

    Exact same code as the TalkingMonster, but the attack range is larger AND
    there's an animation

    TODO: Implement firarm damage algorithm

    '''
    def __init__(self, speech, rate):
        ''' Initialize the speech and rate (as well as the backup_coord) '''
        self.speech = speech
        self.rate = rate
        self.backup_coord = get_rand_unblocked_coord()
        # Tamed variable
        self.tamed = False

    def take_turn(self):
        ''' Monster takes a normal turn, but says something '''
        # A basic monster takes its turn. If you can see it, it can see you
        monster = self.owner

        # Get data
        # Make sure that the monster prioritizes between tamed monsters
        # and the player if it's in range
        # Get the closest tamed monster
        tamed_monster = closest_tamed_monster(SENSE_RANGE)
        # If it exists, store the distance
        if tamed_monster is not None:
            distance_to_tamed = monster.distance_to(tamed_monster)
        # Otherwise we don't need it so we set it to a HUGE int
        else:
            distance_to_tamed = MEGADEATH
        # Get distance to player
        distance_to_player = monster.distance_to(player)

        # Is the player in the fov?
        sees_player = libtcod.map_is_in_fov(fov_map, monster.x, monster.y)

        # If it's in the player's fov then it approaches them
        if distance_to_tamed < SENSE_RANGE and not sees_player:
            # Move towards player if far away
            if monster.distance_to(tamed_monster) >= MONSTER_RANGE:
                monster.move_astar(tamed_monster.x, tamed_monster.y, False)

            # Close enough, attack! (if the player is still alive.)
            elif player.fighter.hp > 0:
                monster.fighter.attack(tamed_monster)

        if sees_player and not INVISIBLE:

            # Select what to attack
            # Prioritize the closer one
            if distance_to_tamed < distance_to_player:
                target = tamed_monster
            # If they're equal, prioritize the player
            elif distance_to_tamed == distance_to_player:
                target = player
            # Otherwise the distance to the player must be shorter,
            # Therefore, prioritize the player
            else:
                target = player

            # Move towards target if far away
            if monster.distance_to(target) >= MONSTER_RANGE:
                monster.move_astar(target.x, target.y, False)

            # Close enough, attack! (if the target is still alive.)
            elif target.fighter.hp > 0:
                monster.fighter.attack(target)
                animate_bolt(libtcod.yellow, self.owner.x, self.owner.y,
                    player.x, player.y)

            # Depending on the rate of speech set in the json,
            #   the monster may talk
            # Rate must be a value from 0 - 99
            # The higher rate is, the less frequent the monster will talk
            if libtcod.random_get_int(0, 0, 100) > self.rate:
                # Say a random line
                msg = random.choice(self.speech)
                message(''.join([monster.name.capitalize(), ' says \'',
                    msg, '\'']), monster.color)

        # Otherwise it moves to a random map location
        else:
            x, y = self.backup_coord
            monster.move_astar(x, y, False)

        # Reset backup coord
        if (monster.x, monster.y) == self.backup_coord:
            self.backup_coord = get_rand_unblocked_coord()

# ------------------------------------------------------------------------------

# Object classes ---------------------------------------------------------------

'''

Classes for all the game objects!

Here's an example hierarchy

Object:
    Fighter:
        AI

Object:
    Item:
        Equipment

You can see how this is implemented in generate_item

'''

class Equipment:
    ''' An object that can be equipped, yielding bonuses.
    automatically adds the Item component.

    This is for items that are not usable, namely armor, weapons, and firearms

    '''
    def __init__(self, slot, power_bonus=0, defense_bonus=0, max_hp_bonus=0,
        max_mana_bonus=0, max_accuracy_bonus=0, attack_msg=None, weapon_func=None,
        ranged_bonus=0, short_name=None):
        # Bonuses to stats
        self.power_bonus        = power_bonus
        self.defense_bonus      = defense_bonus
        self.max_hp_bonus       = max_hp_bonus
        self.max_mana_bonus     = max_mana_bonus
        self.ranged_bonus       = ranged_bonus
        self.max_accuracy_bonus = max_accuracy_bonus

        # Attack message changes how the player attacks (fluff)
        self.attack_msg         = attack_msg
        # Function called when weapon is used
        self.weapon_func        = weapon_func

        # Slot
        self.slot = slot
        # Equipped status. Always starts as false
        self.is_equipped        = False

        # Short name for rendering in the render_equips function
        self.short_name         = short_name

    def toggle_equip(self):
        ''' Toggle equip/dequip status. Used when selected from inventory '''
        if self.is_equipped:
            self.dequip()
        else:
            self.equip()

    def equip(self):
        ''' If the slot is already being used do nothing,
        except for dual weilding '''

        # If equipped, do nothing
        if self.is_equipped: return

        # See if there is a equipment existing in the slot this equipment wants
        #   to occupy
        old_equipment = get_equipped_in_slot(self.slot)

        # If there is something there...
        if old_equipment is not None:

            # If the item is to be equiped in the hands, find a
            #   free hand and equip it there
            # Essentially, this is dual weilding
            if self.slot in WEAPON_SLOTS:

                # Find and get the equipment in the other hand
                other_hand_equip = None
                if self.slot == 'left hand':
                    other_hand_equip = get_equipped_in_slot('right hand')
                elif self.slot == 'right hand':
                    other_hand_equip = get_equipped_in_slot('left hand')

                # If there is one there...
                if not other_hand_equip:

                    # Switch hands on the new equipment
                    if self.slot == 'left hand':
                        self.slot = 'right hand'
                    elif self.slot == 'right hand':
                        self.slot = 'left hand'

                    # Display a status message
                    message('You use your free hand to equip the ' +
                        self.owner.name)

                    # Finalize equip
                    self.update_attack_message()
                    self.is_equipped = True
                    message(''.join(['Equipped the ', self.owner.name, ' on your ',
                        self.slot, '.']), TEXT_COLORS['good'])

                # Otherwise let the player know that something needs to be
                # dequipped
                else:
                    message(''.join(['There is already a ',
                        other_hand_equip.owner.name, ' on your ',
                        self.slot, '!']),
                        TEXT_COLORS['fail'])

        # Otherwise, equip object and show a message about it
        else:
            # Finalize equip
            self.update_attack_message()
            self.is_equipped = True
            message(''.join(['Equipped the ', self.owner.name, ' on your ',
                self.slot, '.']), TEXT_COLORS['good'])

    def dequip(self):
        ''' Dequip object and show a message about it '''

        # If not equipped, make sure to do nothing!
        if not self.is_equipped: return

        # Change attack message to that of the item in the other hand
        if self.slot == 'left hand':
            item = get_equipped_in_slot('right hand')
            # Don't do it if there's no weapons
            if item != None:
                player.fighter.attack_msg = item.attack_msg
            else:
                player.fighter.attack_msg = DEFAULT_ATTACK

        # And again...
        elif self.slot == 'right hand':
            item = get_equipped_in_slot('left hand')
            # Don't do it if there's no weapons
            if item != None:
                player.fighter.attack_msg = item.attack_msg
            else:
                player.fighter.attack_msg = DEFAULT_ATTACK

        # Finalize dequip
        self.is_equipped = False
        message('Dequipped ' + self.owner.name + ' from ' + self.slot + '.',
                TEXT_COLORS['fail'])

        # Check stats
        '''
        if player.max_mana < player.mana:
            player.mana = player.max_mana

        if player.max_hp < player.hp:
            player.hp = player.max_hp
        '''

    def update_attack_message(self):
        ''' Update the player's attack message based on the equipment's '''
        if self.attack_msg:
            player.fighter.attack_msg = self.attack_msg
        else:
            player.fighter.attack_msg = DEFAULT_ATTACK

    def weapon_function(self):
        ''' Activate the special weapon function '''
        function = self.weapon_func
        if function is not None:
            function(self.owner)

class Fighter:
    ''' Combat-related properties and methods (monster, player, NPC)

    If you want to kill it, it needs to be a fighter

    '''
    def __init__(self, hp, defense, power, xp, mana, accuracy, death_function=None,
        attack_msg=None):
        # Store the base hp
        self.base_max_hp     = hp
        # Current hp
        self.hp              = hp

        self.base_defense    = defense

        self.base_power      = power

        self.xp              = xp

        self.base_accuracy   = accuracy

        # Called on death
        self.death_function  = death_function

        self.mana            = mana
        self.base_max_mana   = mana

        self.attack_msg      = attack_msg

    '''

    Properties are weird but amazing. Basically it's just an extension of the class.
    We use them to get the current stats based on all the equipment bonuses

    Basically, it's really handy

    '''

    @property
    def power(self):
        # Return actual power, by summing up the bonuses from all equipped items
        bonus = sum(equipment.power_bonus for equipment in \
            get_all_equipped(self.owner))
        return self.base_power + bonus

    @property
    def defense(self):
        # Return actual defense, by summing up the bonuses from
        #   all equipped items
        bonus = sum(equipment.defense_bonus for equipment in \
            get_all_equipped(self.owner))
        return self.base_defense + bonus

    @property
    def max_hp(self):
        # Return actual max_hp, by summing up the bonuses from
        #   all equipped items
        bonus = sum(equipment.max_hp_bonus for equipment in \
            get_all_equipped(self.owner))
        return self.base_max_hp + bonus

    @property
    def max_mana(self):
        # Return actual mana, by summing up the bonuses from all equipped items
        bonus = sum(equipment.max_mana_bonus for equipment in \
            get_all_equipped(self.owner))
        return self.base_max_mana + bonus

    @property
    def accuracy(self):
        # Return actual accuracy, by summing up the bonuses from all equipped items
        bonus = sum(equipment.max_accuracy_bonus for equipment in \
            get_all_equipped(self.owner))
        return self.base_accuracy + bonus

    @property
    def level_up_xp(self):
        # Return the experience needed to level up
        return LEVEL_UP_BASE + self.owner.level * LEVEL_UP_FACTOR

    def take_damage(self, damage):
        ''' Harm self by certain amount of damage '''
        global kill_count

        # Apply damage if possible
        if damage > 0:
            self.hp -= damage

        # Check for death. if there's a death function, call it
        if self.hp <= 0:
            self.hp = 0

            # Execute death function
            function = self.death_function
            if function is not None:
                function(self.owner)

            # Yield experience to the player, take some mana
            # and give some health

            # Don't award experience to player if player dies
            if self.owner != player:
                # Don't award for killing tamed animals :(
                if self.owner.ai:
                    if self.owner.ai.tamed:
                        # You might want to do something here later
                        # Don't remove this
                        pass
                # If a regular monster dies, xp is given to tamed monsters
                # and player, in the exact same amount as defined in the monster.json
                else:
                    player.fighter.xp += self.xp
                    for obj in objects:
                        if obj.ai:
                            if obj.ai.tamed:
                                obj.fighter.xp += self.xp

                    # Did anything level up
                    check_level_up()

                    # Try to siphon life
                    if activate_siphon:
                        player.fighter.siphon()

                    # Increment kill count
                    kill_count += 1

    def attack(self, target):
        ''' A simple formula for attack damage '''

        # NOTE: This never gets called. I'm considering removing it.
        ans = 'yes'
        if self.owner == player and target.ai.tamed:
            ans = console_input('Are you sure you want to attack the ' + target.name.capitalize() + '?')
            if ans is None:
                ans = 'no'
            elif ans.lower() in ['n', 'no']:
                ans = 'no'
            elif ans.lower() in ['y', 'yes']:
                ans = 'yes'
            else:
                ans = 'no'

        # Random factor for damage calculation
        random_fac = libtcod.random_get_int(0, 0, 5) - libtcod.random_get_int(0, 0, 6)
        damage = self.power - target.fighter.defense + random_fac

        if damage > 0 and ans == 'yes':
            # Make the target take some damage
            # Player lands a crit
            if random_fac == 5 and target != player:
                message(' '.join(['Critical hit!', self.owner.name.capitalize(), self.attack_msg,
                    target.name.capitalize(), 'for', str(damage),
                    'hit points.']),TEXT_COLORS['good'])
            # Monster lands a crit
            elif random_fac == 5 and target == player:
                message(' '.join(['Critical hit!', self.owner.name.capitalize(), self.attack_msg,
                    target.name.capitalize(), 'for', str(damage),
                    'hit points.']),TEXT_COLORS['very_bad'])
            # No one lands a crit
            else:
                message(' '.join([self.owner.name.capitalize(), self.attack_msg,
                    target.name.capitalize(), 'for', str(damage),
                    'hit points.']),TEXT_COLORS['bad'])

            target.fighter.take_damage(damage)

        # Attack the tamed monster
        # NOTE: Not used.
        elif ans == 'yes':
            message(' '.join([self.owner.name.capitalize(), self.attack_msg,
                target.name.capitalize(), 'but it has no effect!']),
                    TEXT_COLORS['fail'])

    def heal(self, amount):
        ''' Heal by the given amount, without going over the maximum '''
        self.hp += amount
        if self.hp > self.max_hp:
            self.hp = self.max_hp

    def cast(self, cost):
        ''' Not used. Not sure what this can be used for in the future '''
        if self.mana - cost < 0:
            message('You don\'t have enough mana to cast this!', TEXT_COLORS['fail'])
        else:
            self.mana -= cost

    def siphon(self):
        ''' Steal life. Sort of like a regeneration system '''
        if self.mana - SIPHON_COST < 0:
            message('You try to siphon any life away, but you aren\'t edgy enough',
                    TEXT_COLORS['fail'])
            return 'cancelled'

        self.mana -= SIPHON_COST
        self.heal(SIPHON_AMOUNT)

        message('You siphon life from the deceased', TEXT_COLORS['magic'])

    def magic_missile(self):
        ''' Fire a magic missile '''
        # Find closest monster
        monster = closest_monster(MISSILE_RANGE)
        if monster is None:  # No enemy found within maximum range
            message('No enemy is close enough to strike with your edge missile',
                    TEXT_COLORS['fail'])
            return 'cancelled'

        # Fire a magic missile
        if self.mana - MISSILE_COST < 0:
            message('You try to fire an edge missile, but you aren\'t edgy enough',
                    TEXT_COLORS['fail'])
            return 'cancelled'

        self.mana -= MISSILE_COST
        cast_magic_missile(self.owner)

    def restore(self, ammount):
        ''' Give some mana back to the player '''
        self.mana += ammount
        if self.mana > self.max_mana:
            self.mana = self.max_mana

class Item:
    ''' An item that can be picked up and used. '''
    def __init__(self, use_function=None):
        self.use_function = use_function

    def pick_up(self):
        ''' Add to the player's inventory and remove from the map '''

        # Inventory has a cap of 26 items
        if len(inventory) >= 26:
            message('Your inventory is full, cannot pick up ' +
                self.owner.name + '.', TEXT_COLORS['fail'])

        else:
            inventory.append(self.owner)
            objects.remove(self.owner)
            message('You picked up a ' + self.owner.name + '!',
                    TEXT_COLORS['good'])

            # Special case: automatically equip, if the corresponding equipment
            #   slot is unused, or if the item is a weapon
            equipment = self.owner.equipment
            if equipment is not None:
                is_weapon = equipment.slot in WEAPON_SLOTS
                if not get_equipped_in_slot(equipment.slot) or is_weapon:
                    equipment.equip()

    def drop(self):
        ''' Drops an item '''
        # Special case: if the object has the Equipment component,
        #   dequip it before dropping
        if self.owner.equipment:
            self.owner.equipment.dequip()

        # Add to the map and remove from the player's inventory. also, place it
        # at the player's coordinates
        objects.append(self.owner)
        inventory.remove(self.owner)
        self.owner.x = player.x
        self.owner.y = player.y

        if self.owner.name == 'bomb':
            message('You planted a ' + self.owner.name + '.', TEXT_COLORS['debug'])
        else:
            message('You dropped a ' + self.owner.name + '.', TEXT_COLORS['neutral'])

        # Easter egg!
        if self.owner.name == 'bomb':
            owner = self.owner
            objects.remove(owner)
            for obj in objects:
                if obj.x == player.x and obj.y == player.y and obj != player:
                    if obj.name in ['Bomb site A', 'Bomb site B']:
                        message('Terrorists win!')

                        (x, y) = camera.to_coords(player.x, player.y)

                        animate_blast(libtcod.red, x, y, FIREBALL_RADIUS*2)

                        for obj in objects:
                            if obj.name in ('Counter-Terrorist', 'Terrorist'):
                                obj.fighter.take_damage(MEGADEATH)

    def use(self):
        ''' Use an item '''
        global perk_fbang, perk_tazer, perk_mtndew, perk_confuse, \
            perk_cokezero, perk_incengren

        # Special case: if the object has the Equipment component, the 'use'
        #   action is to equip/dequip
        if self.owner.equipment:
            self.owner.equipment.toggle_equip()
            return

        # Just call the 'use_function' if it is defined
        if self.use_function is None:
            message('The ' + self.owner.name + ' cannot be used.', TEXT_COLORS['neutral'])
        else:
            if self.use_function() != 'cancelled':

                # Increment perks
                if self.owner.name == 'mountain dew':
                    perk_mtndew += 1
                elif self.owner.name == 'coke zero':
                    perk_cokezero += 1
                elif self.owner.name == 'tazer':
                    perk_tazer += 1
                elif self.owner.name == 'incendiary grenade':
                    perk_incengren += 1
                elif self.owner.name == 'flashbang':
                    perk_fbang += 1

                # Destroy after use, unless it was cancelled for some reason
                inventory.remove(self.owner)

class Object:
    '''
    This is a generic object: the player, a monster, an item, the stairs...
    It's always represented by a character on screen

    The foundation and root of basically everything except tiles, and screen
    rendering magic

    '''
    def __init__(self, x, y, char, name, color, blocks=False,
        always_visible=False, fighter=None, ai=None, item=None,
        gold=None, equipment=None):
        # These should be self-explanatory
        self.always_visible  = always_visible
        self.char            = char
        self.name            = name
        self.color           = color
        self.blocks          = blocks
        self.x               = x
        self.y               = y

        self.fighter = fighter
        if self.fighter:  # Let the fighter component know who owns it
            self.fighter.owner = self

        self.ai = ai
        if self.ai:  # Let the AI component know who owns it
            self.ai.owner = self

        self.item = item
        if self.item:  # Let the Item component know who owns it
            self.item.owner = self

        self.gold = gold
        if self.gold: # Let the gold know who owns it
            self.item.owner = self

        self.equipment = equipment
        if self.equipment:  # Let the Equipment component know who owns it
            self.equipment.owner = self
            # There must be an Item component for the
            #   Equipment component to work properly
            self.item = Item()
            self.item.owner = self

        # Store value if displaced
        self.displaced = False

        # Level
        self.level = 1

    def clear(self):
        ''' Erase the character that represents this object '''
        (x, y) = camera.to_coords(self.x, self.y)
        if x is None:
            tcod_put_char(con, x, y, ' ', libtcod.BKGND_NONE)

    def displace(self, dx, dy):
        ''' Displace a monster '''
        message('You displace the ' + self.name, TEXT_COLORS['neutral'])
        self.move_towards(self.x + dx, self.y + dy)
        self.displaced = True

    def distance(self, x, y):
        ''' Return the distance to some coordinates '''
        return math.sqrt((x - self.x) ** 2 + (y - self.y) ** 2)

    def distance_to(self, other):
        ''' Return the distance to another object '''
        dx = other.x - self.x
        dy = other.y - self.y
        return math.sqrt(dx ** 2 + dy ** 2)

    def draw(self):
        ''' Draw object. Only show if it's visible to the player; or it's set to
        'always visible' and on an explored tile '''

        # Draw if the player can see it
        player_can_see = libtcod.map_is_in_fov(fov_map, self.x, self.y)
        # Or if the player always can see it
        persistent = (self.always_visible and world[self.x][self.y].explored)

        # SEE_ALL is a debug thing that lets you see all objects.
        if player_can_see or persistent or SEE_ALL:
            (x, y) = camera.to_coords(self.x, self.y)

            if x is not None:
                # Set the color and then draw the character that
                # represents this object at its position
                tcod_set_fg(con, self.color)
                tcod_print_ex(con, x, y,
                                        libtcod.BKGND_NONE, libtcod.CENTER,
                                        self.char)

    def drop(self):
        ''' Add to the map and remove from the player's inventory.
        also, place it at the player's coordinates '''
        objects.append(self.owner)
        inventory.remove(self.owner)
        self.owner.x = player.x
        self.owner.y = player.y
        message('You dropped a ' + self.owner.name + '.', TEXT_COLORS['neutral'])

    def move(self, dx, dy):
        ''' Move by a given amount '''
        try:
            # Have the ability to hack through walls if it's the player
            # You don't want monsters wallhacking!
            monster_there = monster_occupy_check(self.x+dx, self.y+dy)
            if self.name == player_name and WALL_HACK:
                self.x += dx
                self.y += dy
            elif not is_blocked(self.x+dx, self.y+dy) and not monster_there:
                self.x += dx
                self.y += dy
        except IndexError:
            pass

    def move_astar(self, tx, ty, player_move):
        ''' A* Algorithm for pathfinding towards target

        libtcod has this built in... Thank goodness

        '''
        # Create a FOV map that has the dimensions of the map
        fov = libtcod.map_new(MAP_WIDTH, MAP_HEIGHT)

        # Scan the current map each turn and set all the walls as unwalkable
        for y1 in range(MAP_HEIGHT):
            for x1 in range(MAP_WIDTH):
                libtcod.map_set_properties(fov, x1, y1, not \
                                            world[x1][y1].block_sight, not \
                                            world[x1][y1].blocked)

        # Scan all the objects to see if there are objects that must be
        #   navigated around
        # Check also that the object isn't self or the target
        #   (so that the start and the end points are free)
        # The AI class handles the situation if self is next to the target
        #   so it will not use this A* function anyway
        for obj in objects:
            if obj.blocks and obj != self and (obj.x, obj.y) != (tx, ty):
                # Set the tile as a wall so it must be navigated around
                libtcod.map_set_properties(fov, obj.x, obj.y, True, False)

        # Allocate a A* path
        # The 1.41 is the normal diagonal cost of moving,
        #   it can be set as 0.0 if diagonal moves are prohibited
        my_path = libtcod.path_new_using_map(fov, 1.41)

        # Compute the path between self's coordinates and the
        # target's coordinates
        libtcod.path_compute(my_path, self.x, self.y, tx, ty)

        # Check if the path exists, and in this case, also the path is
        #   shorter than 25 tiles
        # The path size matters if you want the monster to use alternative
        #   longer paths (for example through other rooms) if for example
        #   the player is in a corridor
        # It makes sense to keep path size relatively low to keep the monsters
        #   from running around the map if there's an alternative path really
        #   far away
        # This breaks constantly so just set it to be really large
        if not libtcod.path_is_empty(my_path) and \
        libtcod.path_size(my_path) < 100 \
        or player_move:
            #Find the next coordinates in the computed full path
            x, y = libtcod.path_walk(my_path, True)
            if x or y:
                #Set self's coordinates to the next path tile
                self.x = x
                self.y = y
        else:
            # Keep the old move function as a backup so that if there are no
            #   paths (for example another monster blocks a corridor)
            # it will still try to move towards the
            # player (closer to the corridor opening)
            self.move_towards(tx, ty)

    def move_towards(self, target_x, target_y):
        ''' Move towards a target '''
        dx = 0
        dy = 0

        # First, try to move towards player by row
        if target_x == self.x:
            pass
        elif target_x < self.x:
            dx = -1
        elif target_x > self.x:
            dx = 1

        # Second, try to move towards player by column
        if target_y == self.y:
            pass
        elif target_y < self.y:
            dy = -1
        elif target_y > self.y:
            dy = 1

        # If the space the monster wants to go is open go there

        if not is_blocked(self.x + dx, self.y + dy) and not \
        monster_occupy_check(self.x+dx, self.y+dy):
            self.move(dx, dy)
        # Otherwise if the space adjacent to the monster on the y axis is open
        #   go there
        elif not is_blocked(self.x, self.y + dy) and \
        not monster_occupy_check(self.x, self.y + dy):
            self.move(0, dy)
        # Otherwise if the space adjacent to the monster on the x axis is open
        #   go there
        elif not is_blocked(self.x + dx, self.y) and \
        not monster_occupy_check(self.x + dx, self.y):
            self.move(dx, 0)
        # Otherwise do nothing
        else:
            pass

    def send_to_back(self):
        ''' Send object to back of render list '''
        # Make this object be drawn first, so all others appear
        #   above it if they're in the same tile.
        global objects
        objects.remove(self)
        objects.insert(0, self)

    def set_corpse(self):
        ''' Set the corpse of a monster '''
        self.blocks = False
        self.fighter = None
        self.ai = None
        self.send_to_back()
        self.char = '&'
        self.color = libtcod.dark_red
        self.name = ' '.join(['remains of', self.name])

    def set_player_corpse(self):
        ''' Set the corpse of the player '''
        self.char = '@'
        self.color = libtcod.dark_red

class Tile:
    ''' A tile of the map and its properties

    The foundation of every good world

    '''
    def __init__(self, blocked, block_sight=None):
        # Does it block players and monsters?
        self.blocked = blocked

        # Debug switch, all tiles are unexplored unless you're a debugger
        if FOG_OF_WAR_ENABLED:
            self.explored = False
        else:
            self.explored = True

        # By default, if a tile is blocked, it also blocks sight. Makes sense.
        if block_sight is None:
            block_sight = blocked
        self.block_sight = block_sight

# ------------------------------------------------------------------------------

# Misc -------------------------------------------------------------------------

# The namespace class is a blank class used to organize values.
# Used in check_args
class Namespace(object): pass

# ------------------------------------------------------------------------------

################################################################################
# Functions
################################################################################

def animate_bolt(color, dx, dy, tx, ty):
    ''' Animate a lightning bolt from the player to an enemy '''
    if not blind:
        # While, the distance to the monster is greater than 2
        # Aka go towards it until it's one space away
        while  math.sqrt((tx-dx) ** 2 + (ty-dy) ** 2) >= 2:
            # First, try to move towards monster by row
            if tx == dx:
                pass
            elif tx < dx:
                dx += -1
            elif tx > dx:
                dx += 1

            # Second, try to move towards player by column
            if ty == dy:
                pass
            elif ty < dy:
                dy += -1
            elif ty > dy:
                dy += 1

            (x, y) = camera.to_coords(dx, dy)

            fov_recompute()

            for obj in objects:
                if obj.name != player.name:
                    obj.draw()

            player.draw()

            tcod_set_fg(con, color)

            # Get the lightning bolt
            char = anim.lightning_direction(dx, dy, tx, ty)

            tcod_print_ex(con, x, y,
                                    libtcod.BKGND_NONE, libtcod.CENTER,
                                    char)
            libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)
            render_gui()
            libtcod.console_flush()

def animate_blast(color, tx, ty, radius):
    ''' Animate an explosion '''

    for i in range(radius):

        fov_recompute()

        for obj in objects:
            if obj.name != player.name:
                obj.draw()

        player.draw()

        tcod_set_fg(con, libtcod.red)

        tcod_put_char(con, tx,   ty,   '4', libtcod.BKGND_NONE)
        tcod_put_char(con, tx+i, ty,   '4', libtcod.BKGND_NONE)
        tcod_put_char(con, tx-i, ty,   '4', libtcod.BKGND_NONE)
        tcod_put_char(con, tx,   ty+i, '4', libtcod.BKGND_NONE)
        tcod_put_char(con, tx,   ty-i, '4', libtcod.BKGND_NONE)
        tcod_put_char(con, tx+i, ty+i, '4', libtcod.BKGND_NONE)
        tcod_put_char(con, tx-i, ty-i, '4', libtcod.BKGND_NONE)
        tcod_put_char(con, tx+i, ty-i, '4', libtcod.BKGND_NONE)
        tcod_put_char(con, tx-i, ty+i, '4', libtcod.BKGND_NONE)

        libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)
        render_gui()
        libtcod.console_flush()

def cast_confuse():
    ''' Ask the player for a target to confuse '''

    message('Left-click an enemy to confuse it, or right-click to cancel.',
        TEXT_COLORS['magic'])

    monster = target_monster(CONFUSE_RANGE)

    if monster is None: return 'cancelled'

    # Replace the monster's AI with a 'confused' one; after some turns it will
    # restore the old AI
    old_ai = monster.ai
    monster.ai = ConfusedMonster(old_ai)
    monster.ai.owner = monster  # Tell the new component who owns it
    message('The eyes of the ' + monster.name +
        ' look vacant, as he starts to stumble around!', TEXT_COLORS['magic'])

    render_all()

    # Present the root console
    libtcod.console_flush()

def cast_death():
    ''' Ask the player for a target tile to kill '''

    message('Left-click a target monster to report, or right-click to cancel.',
            TEXT_COLORS['magic'])

    mon = target_monster()

    # Handle possible errors
    if mon is None:
        message('Cancelled', TEXT_COLORS['fail'])
        return 'cancelled'

    else:
        message('The ' + mon.name + ' gets reported to HEART!',
            TEXT_COLORS['very_bad'])
        mon.fighter.take_damage(MEGADEATH)

def cast_explode():
    ''' Detonate a bomb '''

    message('The bomb explodes, burning everything within ' +
        str(FIREBALL_RADIUS*2) + ' tiles!', TEXT_COLORS['very_bad'])

    # Damage every fighter in range, including the player
    for obj in objects:
        if obj.distance(player.x, player.y) <= FIREBALL_RADIUS*2 and obj.fighter:
            message('The ' + obj.name + ' gets burned for ' +
                str(FIREBALL_DAMAGE*5) + ' hit points.', TEXT_COLORS['bad'])
            obj.fighter.take_damage(FIREBALL_DAMAGE*5)

    # Get the coordinates relative to the camera position
    (x, y) = camera.to_coords(player.x, player.y)

    # Then animate it
    animate_blast(libtcod.red, x, y, FIREBALL_RADIUS*2)

def cast_fireball():
    ''' Ask the player for a target tile to throw a fireball at '''

    message('Left-click a target tile for the fireball, or \
        right-click to cancel.', TEXT_COLORS['magic'])

    (x, y) = target_tile()
    if x is None: return 'cancelled'

    message('The fireball explodes, burning everything within ' +
        str(FIREBALL_RADIUS) + ' tiles!', TEXT_COLORS['magic'])

    # Damage every fighter in range, including the player
    for obj in objects:
        if obj.distance(x, y) <= FIREBALL_RADIUS and obj.fighter:
            message('The ' + obj.name + ' gets burned for ' +
                str(FIREBALL_DAMAGE) + ' hit points.', TEXT_COLORS['bad'])
            obj.fighter.take_damage(FIREBALL_DAMAGE)

    # Get the coordinates relative to the camera position
    (x, y) = camera.to_coords(x, y)

    # Then animate it
    animate_blast(libtcod.red, x, y, FIREBALL_RADIUS)

def cast_fortune():
    ''' Eat a fortune cookie. Not really a spell, more of a usable effect
    specific to the fortune cookie item '''

    message('You break open the fortune cookie and eat the shell.', TEXT_COLORS['neutral'])

    # Do nothing if full health
    if player.fighter.hp == player.fighter.max_hp:
        pass
    else:
        message('Your wounds start to feel better!', TEXT_COLORS['good'])
        player.fighter.heal(FORTUNE_HEAL)

    message('Oh! A fortune!', TEXT_COLORS['level_up'])
    message(fortune.get_fortune(), TEXT_COLORS['level_up'])

def cast_heal():
    ''' Heal the player '''

    if player.fighter.hp == player.fighter.max_hp:
        message('You are already at full health.', TEXT_COLORS['neutral'])
        return 'cancelled'

    message('Your wounds start to feel better!', TEXT_COLORS['good'])
    player.fighter.heal(HEAL_AMOUNT)

def cast_inflict_blindness():
    ''' Inflict blindness. Basically just limit what gets rendered '''

    global blind, blind_counter
    blind = True
    blind_counter = 0
    message('You are blinded!', TEXT_COLORS['very_bad'])

def cast_mana():
    ''' Give some mana back '''

    if player.fighter.mana == player.fighter.max_mana:
        message('You already have enough edge.', TEXT_COLORS['neutral'])
        return 'cancelled'

    message('You begin to feel edgy!', TEXT_COLORS['edge'])
    player.fighter.restore(MANA_AMOUNT)

def cast_magic_missile(fighter_owner):
    ''' Find closest enemy (inside a maximum range) and damage it
    assumes that you already have a monster in range '''

    obj = fighter_owner.fighter

    monster = closest_monster(MISSILE_RANGE)

    # Most complex damage algorithm you've screen
    # Scale based on max mana
    mana_scale = int(math.floor(obj.max_mana/(75)))
    # Random factor
    random_fac = libtcod.random_get_int(0, 0, 5) - libtcod.random_get_int(0, 0, 6)
    # Scale based on level
    # This is not math.floored because we don't want this to be come too
    # underpowered, so we math.floor it last in the final damage variable
    level_scale = obj.owner.level*.2
    # Add base missile damage to the scaled damage (damage*mana_scale)
    damage = MISSILE_DAMAGE + random_fac + int(math.floor((MISSILE_DAMAGE * mana_scale)*level_scale))

    # Zap it!
    if random_fac == 5:
        message('Critical hit! A missile of pure edge strikes the ' + monster.name +
            ' with a loud airhorn! The damage is ' + str(damage) +
            ' hit points.', TEXT_COLORS['crit'])
    else:
        message('A missile of pure edge strikes the ' + monster.name +
            ' with a loud airhorn! The damage is ' + str(damage) +
            ' hit points.', TEXT_COLORS['edge'])

    monster.fighter.take_damage(damage)

    # Animate the lightning bolt
    animate_bolt(libtcod.light_purple, player.x, player.y, monster.x, monster.y)

def cast_lightning():
    ''' Find closest enemy (inside a maximum range) and damage it '''

    monster = closest_monster(LIGHTNING_RANGE)
    if monster is None:  # No enemy found within maximum range
        message('No enemy is close enough to strike.', TEXT_COLORS['fail'])
        return 'cancelled'

    # Zap it!
    message('A lighting bolt strikes the ' + monster.name +
            ' with a loud thunder! The damage is ' + str(LIGHTNING_DAMAGE) +
            ' hit points.', TEXT_COLORS['magic'])
    monster.fighter.take_damage(LIGHTNING_DAMAGE)

    # Animate the lightning bolt
    animate_bolt(libtcod.light_azure, player.x, player.y, monster.x, monster.y)

def check_args():
    ''' Check the arguments the game is ran with

    Q: This looks confusing. Like really confusing.

    A: Once you know what's happening it makes a lot of sense. Here's the break-
    down.

    1. We run linux-run and pass arguments (-q, -h, --quickstart, etc)
    2. This function gets called
    3. Run through all args and find if they need to be expanded (-q, -h) or
        if they're already big --
    3a. Expand miniargs (-q) into big args (--quickstart) based on a dictionary
    4. Run method associated with argument (quickstart: ARG_QUICKSTART)
    5. Method changes flags or values in the game itself
    6. If flags are on or off, run the associated function (FLAGS.MENU will run
        main_menu())

    If you have questions, make sure to ask max what the heck is going on because
    as of right now he seems to know how to operate this black box in a black box

    '''

    global player_name, GOD_MODE, FOG_OF_WAR_ENABLED, STAIR_HACK, SEE_ALL, \
        COORDS_UNDER_MOUSE

    # Create a local namespace to store flags
    FLAGS = Namespace()

    FLAGS.FOUND    = False, # True if any argument was found
    FLAGS.MENU     = True,  # If this flag is on, the main menu will be displayed
    FLAGS.NEWGAME  = False, # Flag to initialize new game
    FLAGS.PLAYGAME = False  # Flag to play the game

    def ARG_QUICKSTART():
        ''' Enable quickstart mode '''
        # define globals used in system
        global player_name, GOD_MODE, FOG_OF_WAR_ENABLED, STAIR_HACK, SEE_ALL, \
            COORDS_UNDER_MOUSE
        player_name = DEFAULT_NAME
        # You connot modify variables in the outer-funcion scope, but you CAN
        # update a dictionary!
        FLAGS.MENU = False
        FLAGS.NEWGAME = True
        FLAGS.PLAYGAME = True

    def ARG_DEBUG():
        ''' Enable debug mode '''
        # define globals used in system
        global player_name, GOD_MODE, FOG_OF_WAR_ENABLED, STAIR_HACK, SEE_ALL, \
            COORDS_UNDER_MOUSE
        GOD_MODE = True
        FOG_OF_WAR_ENABLED = False
        STAIR_HACK = True
        SEE_ALL = True
        COORDS_UNDER_MOUSE = True

    try:
        # Function lookup for sysargs
        ARG_LOOKUP = {
            'quickstart': ARG_QUICKSTART,
            'debug': ARG_DEBUG
        }

        # Single-character table that links to the sysarg table
        ARG_EXPANSION = {
            'q': 'quickstart',
            'h': 'debug'
        }

        # assumes that the program is run with python2.7 -B edgequest.py
        # Scan over all input arguments
        for arg in sys.argv:
            # Look for small args (like `al` in `ls -al`)
            # In each argument, we look for a starting `-` that is not followed by another `-`
            if arg[0:1] == '-' and arg[1:2] != '-':
                for minarg in list(arg[1:]):
                    try:
                        # Expand argument and do a lookup with the expanded argument
                        expanded_arg = ARG_EXPANSION[minarg]
                        # Look up function and call it
                        ARG_LOOKUP[expanded_arg]()
                        FLAGS.FOUND = True
                    except (IndexError, KeyError) as e:
                        logger.error('Arg: invalid argument `' + minarg + '`, continuing as normal')
            # Look for large arguments like `--help`
            if arg[0:2] == '--':
                # We already have the expanded argument
                expanded_arg = arg[2:]
                try:
                    # Look up function and call it
                    ARG_LOOKUP[expanded_arg]()
                    FLAGS.FOUND = True
                except (IndexError, KeyError) as e:
                    logger.error('Arg: invalid argument `' + expanded_arg + '`, continuing as normal')
        # If there are no args found, run this
        if not FLAGS.FOUND:
            logger.info('No args found')
            main_menu()

        # Do stuff based on flags
        if FLAGS.MENU:
            main_menu()
        if FLAGS.NEWGAME:
            new_game()
        if FLAGS.PLAYGAME:
            play_game()
    except IndexError:
        logger.error('IndexError: Problem with flags. Defaulting to main menu...')
        main_menu()

def check_ground():
    ''' Look for an item in the player's tile '''
    for obj in objects:
        if obj.x == player.x and obj.y == player.y and obj != player:
            message(' '.join(['You see a', obj.name, 'here.']), TEXT_COLORS['neutral'])

def check_level_up():
    ''' See if the player's experience is enough to level-up '''

    # Check the to see if the player has enough xp
    if player.fighter.xp >= player.fighter.level_up_xp:

        # Initial render
        render_all()
        libtcod.console_flush()

        # Level up
        player.level += 1
        player.fighter.xp = 0
        message('Your battle skills grow stronger! You reached level ' +
            str(player.level) + '!', TEXT_COLORS['level_up'])

        choice = None
        while choice == None:  # Keep asking until a choice is made
            choice = menu('Level up! Choose a stat to raise:\n',
                ['Constitution (+20 HP, from ' + str(player.fighter.max_hp) +
                ')',
                'Strength (+1 attack, from ' + str(player.fighter.power) +
                ')',
                'Euphoria (+10 mana, from ' + str(player.fighter.max_mana) +
                ')',
                'Adderall (+1 accuracy, from ' + str(player.fighter.accuracy) +
                ')'], LEVEL_SCREEN_WIDTH)

        if choice == 0:
            player.fighter.base_max_hp += 20
            player.fighter.hp += 20
        elif choice == 1:
            player.fighter.base_power += 1
        elif choice == 2:
            player.fighter.base_max_mana += 10
        elif choice == 3:
            player.fighter.base_accuracy += 1

        # Pause
        libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS, key, mouse)

    for obj in objects:
        if obj.ai:
            if obj.ai.tamed:
                if obj.fighter.xp >= obj.fighter.level_up_xp:
                    message('Your ' + obj.name + ' has grown!')
                    obj.fighter.base_power += 2
                    obj.fighter.base_defense += 2
                    obj.fighter.base_max_hp += 20

def check_timer():
    ''' Check the timer periodically '''
    global timer

    # Regenerate health
    for obj in objects:
        if obj.fighter:
            if obj.fighter.hp != obj.fighter.max_hp:
                if timer % REGEN_SPEED == 0:
                    obj.fighter.heal(1)
                    timer += 1

def choose_name():
    ''' Choose a name for the hero '''

    global player_name

    name = console_input('Enter a name')

    # In case if the name isn't anything
    if name == '':
        name = DEFAULT_NAME

    player_name = name.capitalize()

def choose_theme():
    ''' Choose a theme '''

    # Set the screen to black
    tcod_set_bg(con, libtcod.black)

    # Clear screen
    tcod_clear(con)

    # Blit to screen
    libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)

    options = []

    # Store all color themes to list
    for key, value in colors.color_data.iteritems():
        if key != 'erroneous':
            options.append(key)

    # Preset list for player to select theme
    theme = menu('Choose a theme. Default: ' + CURRENT_THEME, options, INVENTORY_WIDTH)

    if theme is None:
        logger.warn('No theme selected')
        return None
    else:
        logger.info('Selecting theme...')
        # Set the theme
        for ind, val in enumerate(options):
            if ind == theme:
                initialize_theme(val)

def closest_monster(max_range):
    ''' Find closest enemy, up to a maximum range, and in the player's FOV '''

    closest_enemy = None

    # Start with (slightly more than) maximum range
    closest_dist = max_range + 1

    for obj in objects:
        # If it's a fighter and in the fov and has an ai
        if obj.fighter and not obj == player and \
        libtcod.map_is_in_fov(fov_map, obj.x, obj.y) and obj.ai:
            # Don't hurt tamed monsters!
            if not obj.ai.tamed:
                # Calculate distance between this obj and the player
                dist = player.distance_to(obj)
                if dist < closest_dist:  # It's closer, so remember it
                    closest_enemy = obj
                    closest_dist = dist

    return closest_enemy

def closest_tamed_monster(max_range):
    ''' Find closest tamed monster, up to a maximum range, and in the player's FOV '''

    closest_enemy = None

    # Start with (slightly more than) maximum range
    closest_dist = max_range + 1

    # Same as the closest_monster function, with one difference
    for obj in objects:
        if obj.fighter and not obj == player and \
        libtcod.map_is_in_fov(fov_map, obj.x, obj.y) and obj.ai:
            if obj.ai.tamed:
                # Calculate distance between this obj and the player
                dist = player.distance_to(obj)
                if dist < closest_dist:  # It's closer, so remember it
                    closest_enemy = obj
                    closest_dist = dist

    return closest_enemy

def console_input(title):
    ''' Display a black screen with a console and get input sent to it '''

    key = libtcod.Key()
    string = ''

    # Set the screen to black
    tcod_set_bg(con, libtcod.black)

    # Set text color to title color
    tcod_set_fg(con, TEXT_COLORS['title'])

    # Dispbox style key getting
    while not libtcod.console_is_window_closed():

        # This loop has a tendency to eat all the cpu
        time.sleep(1/LIMIT_FPS*2)

        # Check for keypresses
        if libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS, key, mouse):

            # Get key
            key_char = chr(key.c)

            # Set fullscreen
            if key.vk in FULLSCREEN_KEYS:
                toggle_fullscreen()

            # Enter submits string
            elif key.vk == libtcod.KEY_ENTER:
                break
            # Backspace deletes line
            elif key.vk == libtcod.KEY_BACKSPACE:
                if len(string) == 1:
                    string = ''
                else:
                    string = string[:-1]
            # Shift causes a problem in libtcod so make sure nothing happens if
            #   pressed
            elif key.vk == libtcod.KEY_SHIFT:
                pass
            # Add char to string
            elif key_char:
                string = ''.join([string, key_char])

        # Clear screen
        tcod_clear(con)

        # Prompt for string
        tcod_print_ex(con, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 4,
                                libtcod.BKGND_NONE, libtcod.CENTER,
                                title)

        # Blit to screen
        libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)

        # Show string
        dispbox('\n' + string + '\n', len(string))

        # Present the root console
        libtcod.console_flush()

    return string

def console_input_small():
    ''' Same as above but isn't as intrusive '''

    string = ''

    # Loop to show input from player
    while not libtcod.console_is_window_closed():

        # This loop has a tendency to eat all the cpu
        time.sleep(1/LIMIT_FPS*2)

        # Render before drawing a new dispbox
        render_all()

        # Check for keypresses
        if libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS, key, mouse):
            # Enter submits string
            key_char = chr(key.c)
            if key.vk == libtcod.KEY_ENTER:
                break
            elif key.vk in FULLSCREEN_KEYS:
                toggle_fullscreen()

            # Backspace deletes character
            elif key.vk == libtcod.KEY_BACKSPACE:
                if len(string) == 1:
                    string = ''
                else:
                    string = string[:-1]
            # Esc quits
            elif key.vk == libtcod.KEY_ESCAPE:
                check = False
                return None
                break
            elif key.vk == libtcod.KEY_SHIFT:
                pass
            elif key_char != '':
                string = ''.join([string, key_char])

            dispbox('\n' + string + '\n', len(string))

    return string

def consumables_menu(header):
    ''' Show a menu with each edible item as an option '''

    consum_inven = []

    if len(inventory) != 0:
        options = []
        sort_inventory()
        for item in inventory:
            # Only get consumable items
            if item.name in CONSUMABLES:
                options.append(item.name)
                consum_inven.append(item)
    else:
        options = ['No food']

    index = menu(header, options, INVENTORY_WIDTH)

    # If an item was chosen, return it
    if index is None or len(consum_inven) == 0:
        return None
    return consum_inven[index].item

def credits_screen():
    ''' Show a quick credits screen '''

    # Clear screen
    tcod_clear(con)

    # Set the screen to black
    tcod_set_bg(con, libtcod.black)

    tcod_set_fg(con, TEXT_COLORS['edge'])
    tcod_print_ex(con, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 7,
                            libtcod.BKGND_NONE, libtcod.CENTER,
                            'Thank you for playing EdgeQuest!\n')

    # Blit to screen
    libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)

    msgbox(
    '\n\n\nCredits:\n\n' +
    'Author: Gray (surrsurus)\n' +
    'Big thanks to:\n' +
    'Max for all the contributions! (XavilPergis)\n' +
    'and Fleck and Squirrel for playtesting\n\n' +
    'Press any key to continue',
    40)

def debug_spawn_console(json_list):
    ''' Spawn a mini-console to spawn-in monsters or items '''

    # Needs to have JSON data
    if json_list not in ['monster', 'item']:
        raise Exception('NoDataForObject')

    # Message displaying what will be spawned
    if json_list == 'monster':
        message('Enter a monster name', TEXT_COLORS['debug'])
    elif json_list == 'item':
        message('Enter an item name', TEXT_COLORS['debug'])

    # Show new message
    render_all()
    libtcod.console_flush()

    key = libtcod.Key()
    check = True

    # Get a name from a console input
    name = console_input_small()

    # Names have the ability to not exist, considering player is giving input
    found = False
    # If we're spawning a monster...
    if json_list == 'monster' and check:
        # For monser in the json list to check
        for mon in monster_data:
            # If the name corresponds to a name or id...
            if monster_data[mon]['name'] == name or \
            monster_data[mon]['id'] == name:
                # Generate a monster!
                obj = generate_monster(monster_data[mon]['id'], player.x+2,
                                        player.y)
                # Add monster to object list
                objects.append(obj)
                message('Spawned a ' + name)
                # We found one, so don't display an error message
                found = True
    # If we're spawning an item...
    elif json_list == 'item' and check:
        # For item in the json list...
        for item in items_data:
            # If the name corresponds to a name or id...
            if items_data[item]['name'] == name or \
            items_data[item]['id'] == name:
                # Generate an item
                obj = generate_item(items_data[item]['id'], player.x,
                                    player.y)

                # Add item to object list
                objects.append(obj)
                message('Spawned a ' + name)
                logger.debug('Spawning a ' + name)
                found = True

    if not found and check:
        message('Failed to find a ' + name)
        logger.debug('Failed to find a ' + name)

def debug_kill_all():
    ''' Kill everything with an ai that's not tamed '''

    for obj in objects:
        if obj.ai:
            if not obj.ai.tamed:
                obj.fighter.take_damage(MEGADEATH)

def de_dust():
    ''' Place objects on level. Easter egg '''
    # Maximum number of monsters per level
    max_monsters = from_dungeon_level([[4, 1], [7, 2], [13, 4],
        [20, 6], [30, 12]])

    # Chance of each monster
    monster_chances = {}
    # Monster name then chance
    for item in monster_data:
        monster_chances[str(monster_data[item]['id'])] = \
            from_dungeon_level(monster_data[item]['chance'])

    # Maximum number of items per level
    max_items = from_dungeon_level([[4, 1], [10, 3], [18, 6], [21, 7], [30, 9],
        [35, 10], [40, 12]])

    # Chance of each item (by default they have a chance of 0 at level 1,
    #   which then goes up)
    item_chances = {}
    # Item name then chance.
    for item in items_data:
        item_chances[str(items_data[item]['id'])] = \
            from_dungeon_level(items_data[item]['chance'])

    # Choose random number of monsters
    num_monsters = libtcod.random_get_int(0, 0, max_monsters+dungeon_level)

    for i in range(num_monsters):
        x, y = get_rand_unblocked_coord()

        while is_blocked(x, y):
            x, y = get_rand_unblocked_coord()

        choice = random_choice(monster_chances)

        if random.randint(0, 1) == 1:
            monster = generate_monster('t', x, y)
        else:
            monster = generate_monster('ct', x, y)

        # Add monster to object list
        objects.append(monster)

    # Choose random number of items
    num_items = libtcod.random_get_int(0, 0, max_items+dungeon_level)

    for i in range(num_items):
        x, y = get_rand_unblocked_coord()

        while is_blocked(x, y):
            x, y = get_rand_unblocked_coord()

        # Only place it if the tile is not blocked
        choice = random_choice(item_chances)

        item = generate_item(choice, x, y)

        objects.append(item)
        # Items appear below other objects
        # item.send_to_back()
        # Items are visible even out-of-FOV, if in an explored area
        item.always_visible = True

    x, y = get_rand_unblocked_coord()
    a = Object(x, y, 'A', 'Bomb site A', libtcod.white,
        always_visible=True)
    objects.append(a)
    a.send_to_back()

    x, y = get_rand_unblocked_coord()
    b = Object(x, y, 'B', 'Bomb site B', libtcod.white,
        always_visible=True)
    objects.append(b)
    b.send_to_back()

    x, y = get_rand_unblocked_coord()
    bomb = generate_item('bomb', x, y)
    objects.append(bomb)
    bomb.send_to_back()

def dispbox(header, width=50):
    ''' Like a msgbox but only lasts for one frame '''

    # Calculate total height for the header
    height = libtcod.console_get_height_rect(con, 0, 0, width, SCREEN_HEIGHT,
                                            header)

    # Create an off-screen console that represents the menu's window
    window = libtcod.console_new(width, height)

    # Print the header, with auto-wrap
    tcod_set_fg(window, libtcod.white)
    libtcod.console_print_rect_ex(window, 0, 0, width, height,
                                    libtcod.BKGND_NONE, libtcod.LEFT, header)

    # Blit the contents of 'window' to the root console
    x = SCREEN_WIDTH / 2 - width / 2
    y = SCREEN_HEIGHT / 2 - height / 2
    libtcod.console_blit(window, 0, 0, width, height, 0, x, y, 1.0, 0.7)

    # Present the root console
    libtcod.console_flush()

def equipment_menu(header):
    ''' Show a menu with each equipment item as an option '''

    equip_inven = []

    # If there's items in the inventory...
    if len(inventory) != 0:
        options = []
        sort_inventory()
        # Then for each item...
        for item in inventory:
            # Only get equipment...
            if item.equipment:
                text = item.name
                # And optionally show where it got equipped to
                if item.equipment.is_equipped:
                    text = text + ' (on ' + item.equipment.slot + ')'
                # Then append it to the options list
                options.append(text)
                equip_inven.append(item)
    else:
        options = ['No equipment']

    # Get a selection
    index = menu(header, options, INVENTORY_WIDTH)

    # If an item was chosen, return it
    if index is None or len(equip_inven) == 0:
        return None
    return equip_inven[index].item

def fire_weapon(equipment):
    ''' Find closest enemy and shoot it '''

    # You can't fire guns blind because that's dangerous!
    if not blind:
        # Get closest monster
        monster = closest_monster(FIREARM_RANGE)

        # No enemy found within maximum range
        if monster is None:
            message('No enemy is close enough to shoot.', TEXT_COLORS['fail'])
            return 'cancelled'

        # Super long damage algorithm
        # Make it harder to do a lot of damage
        challenge = monster.fighter.defense + monster.distance_to(player) - player.fighter.accuracy
        # Add a random factor
        random_fac = libtcod.random_get_int(0, 0, 5) - libtcod.random_get_int(0, 0, 6)
        # Beneficially with level
        level_scale_easy = (equipment.ranged_bonus/4) * math.floor(player.level/(4))
        # Negatively scale with level
        level_scale_hard = player.level*.2
        # Combine them all into a damage algorithm
        # This makes guns super weak unless their ranged damage is very high
        damage = int((equipment.ranged_bonus + random_fac + int(math.floor(level_scale_easy)*level_scale_hard)) - challenge)

        if damage > 0:

            # Shoot it!
            # Critical hit
            if random_fac == 5:
                message('Critical hit! ' + player_name + ' shoots the ' + monster.name +
                        ' with the ' + equipment.owner.name + '! The damage is ' +
                        str(damage) + ' hit points.', TEXT_COLORS['crit'])
            # Normal
            else:
                message(player_name + ' shoots the ' + monster.name +
                        ' with the ' + equipment.owner.name + '! The damage is ' +
                        str(damage) + ' hit points.', TEXT_COLORS['bad'])

            monster.fighter.take_damage(damage)

        else:
            # No damage
            message(player_name + ' shoots the ' + monster.name +
                ' with the ' + equipment.owner.name +
                'but the shot reflects off the armor!', TEXT_COLORS['bad'])

        # Animate a bullet
        animate_bolt(libtcod.yellow, player.x, player.y, monster.x, monster.y)
    else:
        # Failure message if blind
        message('You can\'t shoot while blind!', TEXT_COLORS['fail'])

def fov_recompute():
    ''' Recompute fov '''

    global world

    # Move the camera
    camera.move(player.x, player.y)

    # Recompute FOV if needed (the player moved or something)
    libtcod.map_compute_fov(fov_map, player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO)

    tcod_clear(con)

    # Go through all tiles, and set their background color according to the FOV
    for y in range(CAMERA_HEIGHT):
        for x in range(CAMERA_WIDTH):

            (map_x, map_y) = (camera.x + x, camera.y + y)
            visible = libtcod.map_is_in_fov(fov_map, map_x, map_y)

            wall = world[map_x][map_y].block_sight

            if not visible:
                # if it's not visible right now, the player can only see it
                #   if it's explored
                if world[map_x][map_y].explored:
                    # It's out of the player's FOV
                    # Still decorate walls
                    if wall:
                        c = wallselect(world, map_x, map_y)
                        tcod_put_char_ex(con, x, y, c, colors.color_light_ground, colors.color_dark_wall)
                    else:
                        tcod_set_char_bg(con, x, y, colors.color_dark_ground,
                            bg_set=libtcod.BKGND_SET)
            else:
                # It's visible
                # Decorate walls
                if wall:
                    c = wallselect(world, map_x, map_y)
                    tcod_put_char_ex(con, x, y, c, colors.color_accent, colors.color_light_wall)
                else:
                    tcod_set_char_bg(con, x, y, colors.color_light_ground,
                        bg_set=libtcod.BKGND_SET)
                # Since it's visible, explore it
                world[map_x][map_y].explored = True

def from_dungeon_level(table):
    ''' Returns a value that depends on level. the table specifies what
    value occurs after each level, default is 0.

    In case if you couldn't figure out what that means here's an example:
    input ->        [[25, 6]]
           chance ----/    \----- beyond this dungeon level
    All of the chances are totalled and then it's that chance out of that total
    It sort of made sense when I first heard about it but now it's basically
    just magic '''

    for (value, level) in reversed(table):
        if dungeon_level >= level:
            return value
    return 0

def game_over():
    ''' Lose the game '''

    msgbox_stats('You lose!')

    main_menu()

def game_win():
    ''' Win the game! '''

    msgbox_stats('You win!')

    credits_screen()

    exit()

def generate_monster(monster_id, x, y):
    ''' Generate monster from json '''

    # Dictionary of death functions
    dict_death_func = {
        'normal'    : monster_death,
        'slock'     : monster_death_slock,
        'talk'      : monster_death_talk
    }

    # Dictionary of AIs
    dict_ais = {
        'normal'    : BasicMonster(),
        'talk'      : TalkingMonster(0, 0),
        'rangedtalk': RangedTalkerMonster(0, 0),
        'cena'      : CenaMonster(),
        'tamed'     : TamedMonster()
    }

    # Test values
    try:
        assert monster_data[monster_id]['name']       is not None
        assert monster_data[monster_id]['char']       is not None
        assert monster_data[monster_id]['color']      is not None
        assert monster_data[monster_id]['chance']     is not None
        assert monster_data[monster_id]['hp']         is not None
        assert monster_data[monster_id]['defense']    is not None
        assert monster_data[monster_id]['power']      is not None
        assert monster_data[monster_id]['xp']         is not None
        assert monster_data[monster_id]['mana']       is not None
        assert monster_data[monster_id]['accuracy']   is not None
        assert monster_data[monster_id]['death_func'] is not None
        assert monster_data[monster_id]['attack_msg'] is not None
        assert monster_data[monster_id]['ai']         is not None
    except AssertionError as e:
        logger.severe('AssertionError at monster distinguishing')
        id_err(monster_id)

    # Set default values. These should always be present
    mon_name       = monster_data[monster_id]['name']
    mon_char       = monster_data[monster_id]['char']
    mon_color      = monster_data[monster_id]['color']
    mon_chance     = monster_data[monster_id]['chance']
    mon_hp         = monster_data[monster_id]['hp']
    mon_defense    = monster_data[monster_id]['defense']
    mon_power      = monster_data[monster_id]['power']
    mon_xp         = monster_data[monster_id]['xp']
    mon_mana       = monster_data[monster_id]['mana']
    mon_accuracy   = monster_data[monster_id]['accuracy']
    mon_death_func = monster_data[monster_id]['death_func']
    mon_attack_msg = monster_data[monster_id]['attack_msg']
    mon_ai         = monster_data[monster_id]['ai']

    # Select a death function
    death = None
    for key in dict_death_func:
        if mon_death_func == key:
            death = dict_death_func[key]

    # Fallback
    if death is None:
        logger.error('Monster ' + monster_id + 'has an invalid death_func')
        logger.info('Defaulting the monster death function...')
        death = monster_death

    # Select an AI
    ai = None
    for key in dict_ais:
        if mon_ai == key:
            ai = dict_ais[key]

    # Fallback
    if ai is None:
        logger.error('Monster ' + monster_id + 'has an invalid ai')
        logger.info('Defaulting the monster ai...')
        ai = BasicMonster()

    # These values might actually fail
    # Set values if applicable
    try:
        ai.speech = monster_data[monster_id]['speech']
        ai.rate   = monster_data[monster_id]['rate']
    except:
        pass

    '''
    Example:
    # Create an orc
    fighter_component = Fighter(hp=20, defense=0, power=4, xp=35,
                                death_function=monster_death)
    ai_component = BasicMonster()
    monster = Object(x, y, 'o', 'orc', libtcod.desaturated_green,
        blocks=True, fighter=fighter_component, ai=ai_component)
    '''

    # Read color
    color = json_get_color(mon_color)

    # Create component
    fighter_component = Fighter(
        hp             = int(mon_hp),
        defense        = int(mon_defense),
        power          = int(mon_power),
        xp             = int(mon_xp),
        mana           = int(mon_mana),
        accuracy       = int(mon_accuracy),
        death_function = death,
        attack_msg     = mon_attack_msg)

    monster = Object(x, y, mon_char,
        mon_name, color,
        blocks         = True,
        fighter        = fighter_component,
        ai             = ai)

    return monster

def generate_item(item_id, x, y):
    ''' Generate items from json '''

    '''
    Example:
    # Create a sword
    equipment_component = Equipment(slot='right hand', power_bonus=1)
    item = Object(x, y, '/', 'katana', libtcod.sky,
                    equipment=equipment_component)

    * Items MUST use Item class and item_component
    * Equipmnt MUST use Equipment class and equip_component

    Please look at the json for more info on properties of both
    '''

    # Dictionary of all effects of items
    dict_effects = {
        'heal'      : Item(use_function=cast_heal),
        'fireball'  : Item(use_function=cast_fireball),
        'death'     : Item(use_function=cast_death),
        'confuse'   : Item(use_function=cast_confuse),
        'lightning' : Item(use_function=cast_lightning),
        'mana'      : Item(use_function=cast_mana),
        'bomb'      : Item(use_function=cast_explode),
        'fortune'   : Item(use_function=cast_fortune)
    }

    try:
        assert items_data[item_id]['name']   is not None
        assert items_data[item_id]['id']     is not None
        assert items_data[item_id]['char']   is not None
        assert items_data[item_id]['color']  is not None
        assert items_data[item_id]['chance'] is not None
        assert items_data[item_id]['type']   is not None
    except AssertionError as e:
        logger.severe('[!] AssertionError at usable/equipment distinguishing')
        id_err(item_id)

    item_name = items_data[item_id]['name']
    item_id = items_data[item_id]['id']
    item_char = items_data[item_id]['char']
    item_color = items_data[item_id]['color']
    item_chance = items_data[item_id]['chance']
    item_type = items_data[item_id]['type']

    color = json_get_color(item_color)

    # If it's a usable item, get it's effect
    if item_type == 'item':

        try:
            assert items_data[item_id]['effect'] is not None
        except AssertionError as e:
            id_err(item_id)

        item_effect = items_data[item_id]['effect']

        # Select an effect
        effect = None
        for key in dict_effects:
            if item_effect == key:
                effect = dict_effects[key]

        # Fallback
        if effect is None:
            logger.error(effect + ' Not recognized as an item effect.')
            effect = Item(use_function=cast_heal)

        # Create a basic item
        item = Object(x, y, item_char, item_name, color, item=effect)

    elif item_type in ('equipment', 'firearm'):

        try:
            assert items_data[item_id]['subtype']  is not None
            assert items_data[item_id]['slot']     is not None
            assert items_data[item_id]['power']    is not None
            assert items_data[item_id]['defense']  is not None
            assert items_data[item_id]['hp']       is not None
            assert items_data[item_id]['mana']     is not None
            assert items_data[item_id]['accuracy'] is not None
        except AssertionError as e:
            logger.severe('AssertionError at equipment distinguishing')
            id_err(item_id)

        item_subtype  = items_data[item_id]['subtype']
        item_slot     = items_data[item_id]['slot']
        item_power    = items_data[item_id]['power']
        item_defense  = items_data[item_id]['defense']
        item_hp       = items_data[item_id]['hp']
        item_mana     = items_data[item_id]['mana']
        item_accuracy = items_data[item_id]['accuracy']

        # Dictionary of weapon actions
        dict_actions = {
            'knife'  : weapon_action_knife,
            'katana' : weapon_action_katana,
            'awp'    : weapon_action_awp
        }

        if item_subtype == 'weapon':

            try:
                assert items_data[item_id]['attack_msg']  is not None
                assert items_data[item_id]['weapon_func'] is not None
                assert items_data[item_id]['short_name']  is not None
            except AssertionError as e:
                logger.severe('AssertionError at weapon/firearm distinguishing')
                id_err(item_id)

            item_attack_msg  = items_data[item_id]['attack_msg']
            item_weapon_func = items_data[item_id]['weapon_func']
            item_short_name  = items_data[item_id]['short_name']

            # Select a weapon action
            func = None
            for key in dict_actions:
                if item_weapon_func == key:
                    func = dict_actions[key]

            # Fallback
            if func is None:
                func = weapon_action_else

            # Create the component
            equip_component = Equipment(
                slot               = item_slot,
                power_bonus        = item_power,
                defense_bonus      = item_defense,
                max_hp_bonus       = item_hp,
                max_mana_bonus     = item_mana,
                max_accuracy_bonus = item_accuracy,
                attack_msg         = item_attack_msg,
                weapon_func        = func,
                short_name         = item_short_name)

        elif item_subtype == 'firearm':

            try:
                assert items_data[item_id]['attack_msg']  is not None
                assert items_data[item_id]['weapon_func'] is not None
                assert items_data[item_id]['short_name']  is not None
                assert items_data[item_id]['ranged']      is not None
            except AssertionError as e:
                logger.severe('AssertionError at firearm distinguishing')
                id_err(item_id)

            item_attack_msg  = items_data[item_id]['attack_msg']
            item_weapon_func = items_data[item_id]['weapon_func']
            item_short_name  = items_data[item_id]['short_name']
            item_ranged      = items_data[item_id]['ranged']

            # Set the firearm action
            if item_weapon_func == 'firearm':
                func = weapon_action_firearm
            else:
                func = weapon_action_else

            # Create the component
            equip_component = Equipment(
                slot           = item_slot,
                power_bonus    = item_power,
                defense_bonus  = item_defense,
                max_hp_bonus   = item_hp,
                max_mana_bonus = item_mana,
                attack_msg     = item_attack_msg,
                weapon_func    = func,
                ranged_bonus   = item_ranged,
                short_name     = item_short_name)

        elif item_subtype == 'armor':

            try:
                assert items_data[item_id]['short_name']  is not None
            except AssertionError as e:
                id_err(item_id)

            item_short_name = items_data[item_id]['short_name']

            # Create the component
            equip_component = Equipment(
                slot           = item_slot,
                power_bonus    = item_power,
                defense_bonus  = item_defense,
                max_hp_bonus   = item_hp,
                max_mana_bonus = item_mana,
                short_name     = item_short_name)

        item = Object(x, y, item_char, item_name, color,
            equipment=equip_component)

    elif type == 'gold':
        item = Object(x, y, item_char, item_name, color)

    return item

def get_equipped_in_slot(slot):
    ''' Returns the equipment in a slot, or None if it's empty '''

    for obj in inventory:
        # If it's an equipped equipment in the slot return it
        if obj.equipment and obj.equipment.slot == \
        slot and obj.equipment.is_equipped:
            return obj.equipment
    return None

def get_all_equipped(obj):
    ''' Returns a list of equipped items '''

    # Maybe sort of possible to have monsters with equipments?
    if obj == player:
        equipped_list = []
        for item in inventory:
            # Append equipment if it's an equipment that's equipped
            # Bit of a tongue twister
            if item.equipment and item.equipment.is_equipped:
                equipped_list.append(item.equipment)
        return equipped_list
    else:
        return []  # Other objects have no equipment

def get_rand_unblocked_coord():
    ''' Get a random, unblocked coordinate on the map '''
    # If you don't understand what this does, then maybe you should learn some
    # Python before diving into the source code
    return random.choice(unblocked_world)

def git_screen():
    ''' Show a screen reminding the player to check for updates '''

    # Clear screen
    tcod_clear(con)

    # Set the screen to black
    tcod_set_bg(con, libtcod.black)

    tcod_set_fg(con, TEXT_COLORS['title'])
    tcod_print_ex(con, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 4,
        libtcod.BKGND_NONE, libtcod.CENTER, 'Thank you for playing EdgeQuest')

    # Blit to screen
    libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)

    msgbox(
    'Make sure to check the latest master version on github periodically!\n\n' +
    'Press any key to continue...',
    40)

def handle_keys():
    ''' Handle keypresses sent to the console. Executes other things,
    makes game playable

    Q: Why not use a switch statement?
    A: They don't exist in python.

    '''

    global game_state, objects, player_action, key, timer

    # F4 for Fullscreen
    if key.vk in FULLSCREEN_KEYS:
        toggle_fullscreen()

    if game_state == 'playing':
        # End game with escape
        if key.vk == QUIT_KEY:
            player_action = 'didnt-take-turn'
            save_game()

        key_char = chr(key.c)

        # Movement keys
        if key_char in MOVEMENT_KEYS or key.vk in MOVEMENT_KEYS_VK:
            if key_char in ('8', 'k') or key.vk == libtcod.KEY_UP: # N
                player_move(0, -1)
            elif key_char in ('2', 'j') or key.vk == libtcod.KEY_DOWN: # S
                player_move(0, 1)
            elif key_char in ('4', 'h') or key.vk == libtcod.KEY_LEFT: # W
                player_move(-1, 0)
            elif key_char in ('6', 'l') or key.vk == libtcod.KEY_RIGHT: # E
                player_move(1, 0)
            elif key_char in ('7', 'y'): # NW
                player_move(-1, -1)
            elif key_char in ('9', 'u'): # NE
                player_move(1, -1)
            elif key_char in ('1', 'b'): # SW
                player_move(-1, 1)
            elif key_char in ('3', 'n'): # SE
                player_move(1, 1)

            # Recompute the fov if moved
            camera.check_fov = True

            check_ground()

            player_action = 'move'

        # Wait
        elif key_char in WAIT_KEYS:
            camera.check_fov = True
            message('You wait', TEXT_COLORS['neutral'])
            player_action = 'wait'

        # Pick up an item
        elif key_char == GET_ITEM_KEY:
            for obj in objects:  # Look for an item in the player's tile
                if obj.x == player.x and obj.y == player.y and obj.item:
                    obj.item.pick_up()
                    player_action = 'pickup'
                    break
            else:
                message('There is nothing there to pick up', TEXT_COLORS['neutral'])
                player_action = 'didnt-take-turn'

        # Show the inventory
        elif key_char == INVENTORY_KEY:
            chosen_item = inventory_menu(
            'Press the key next to an item to use it, or any other to cancel.\
            \n')
            if chosen_item is not None:
                chosen_item.use()
                player_action = 'use'

        # Show equipment
        elif key_char == EQUIPMENT_MENU_KEY:
            chosen_item = equipment_menu(
            'Press the key next to an item to equip/dequip it, or any other to cancel.\
            \n')
            if chosen_item is not None:
                chosen_item.use()
                player_action = 'use'

        # Show food
        elif key_char == FOOD_MENU_KEY:
            chosen_item = consumables_menu(
            'Press the key next to an item to eat it, or any other to cancel.\
            \n')
            if chosen_item is not None:
                chosen_item.use()
                player_action = 'use'

        # Show the inventory; if an item is selected, drop it
        elif key_char == DROP_ITEM_KEY:
            chosen_item = inventory_menu(
            'Press the key next to an item to drop it, or any other to cancel.\
            \n')
            if chosen_item is not None:
                chosen_item.drop()
                player_action = 'drop'

        # Go down stairs, if the player is on them
        elif key_char == GO_DOWN_KEY:
            if (dstairs.x == player.x and dstairs.y == player.y) or STAIR_HACK:
                next_level()

        # Go up stairs, if the player is on them
        elif key_char == GO_UP_KEY:
            if (ustairs.x == player.x and ustairs.y == player.y) or STAIR_HACK:
                previous_level()

        # Show character information
        elif key_char == STATS_KEY:
            msgbox_stats('Character Information')

        elif key_char == TOGGLE_SIPHON_KEY:
            # Toggle the siphon ability
            toggle_siphon()
            player_action = 'didnt-take-turn'

        # Taunt
        elif key_char == TAUNT_KEY:
            taunt()
            player_action = 'taunt'

        # Activate weapon
        elif key_char == ACTIVATE_WEAPON_KEY:
            right = get_equipped_in_slot('right hand')
            left = get_equipped_in_slot('left hand')
            if left:
                left.weapon_function()
                player_action = 'activating'

            if right:
                right.weapon_function()
                player_action = 'activating'

        # Cast magic missile
        elif key_char == CAST_MAGIC_MISSLE_KEY:
            status = player.fighter.magic_missile()
            if status != 'cancelled':
                player_action = 'casting'
            else:
                player_action = 'didnt-take-turn'

        # Show help
        elif key_char == SHOW_HELP_KEY:
            how_to_play()
            player_action = 'didnt-take-turn'

        # Debug commands

        elif key_char == SPAWN_DEBUG_CONSOLE_M_KEY:
            debug_spawn_console('monster')
            player_action = 'didnt-take-turn'

        elif key_char == SPAWN_DEBUG_CONSOLE_I_KEY:
            debug_spawn_console('item')
            player_action = 'didnt-take-turn'

        elif key_char == KILL_ALL_KEY:
            debug_kill_all()
            player_action = 'didnt-take-turn'

        elif key_char == SPAWN_MONSTER_KEY:
            objects.append(generate_monster('silver2', player.x, player.y + 2))
            pass

        # Reset the map (DEBUG)
        elif key_char == RELOAD_MAP_KEY:
            # Clear screen
            for x in range(SCREEN_WIDTH):
                for y in range(SCREEN_HEIGHT):
                    tcod_put_char(con, x, y, ' ', libtcod.BKGND_BURN)

            # Make a new map
            make_map()
            fov_recompute()
            player_action = 'didnt-take-turn'

        else:
            player_action = 'didnt-take-turn'

def how_to_play():
    ''' Show a how to play menu '''

    msgbox(
    'How To Play\n\n \
    Numpad/Arrowkeys/Vim keys: Move \n \
    Click: Move to spot \n \
    . - Wait \n \
    i - Open Inventory \n \
    e - Open Equipment\n \
    E - Eat Food\n \
    g - Grab item below you\n \
    d - Drop item\n \
    > - Go Down Stairs\n \
    < - Go Up Stairs\n \
    c - View Stats\n \
    m - Fire Edge missile\n \
    q - Toggle siphon spell\n \
    ? - Open this menu\n\n \
    Debug Commands\n\n \
    r - Reload a new map\n \
    z - Open spawn monster console\n \
    x - Open spawn item console\n \
    v - Kill all on level\n\n \
    Press any key to continue...',
    CHARACTER_SCREEN_WIDTH)

def id_err(id):
    ''' Log a critical error with monster/item genereation '''
    logger.severe('Error: ' + id + ' is missing data!')
    logger.write('----- STACK TRACE: -----')
    logger.write(traceback.print_exc())
    logger.write('------------------------')
    exit()

def initialize_fov():
    ''' Initialize the fov '''

    global fov_map
    camera.check_fov = True

    # Create the FOV map, according to the generated map
    fov_map = libtcod.map_new(MAP_WIDTH, MAP_HEIGHT)
    for y in range(MAP_HEIGHT):
        for x in range(MAP_WIDTH):
            libtcod.map_set_properties(fov_map, x, y, not world[x][y].block_sight, not world[x][y].blocked)

def initialize_theme(theme):
    ''' Change theme on the fly. Basically a wrapper function'''
    colors.set_theme(theme)

def intro_cutscene():
    ''' Show a cutscene '''

    # Set Colors
    tcod_set_bg(con, libtcod.black)
    tcod_set_fg(con, TEXT_COLORS['title'])

    key = libtcod.Key()
    # We take the y and subtract it from the y val so that the text moves up
    #   the screen
    for y in range(len(INTRO_WALL)+1):
        # Able to break in the middle of the cutscene
        if libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS, key, mouse):
            if key.vk in FULLSCREEN_KEYS:
                toggle_fullscreen()
            elif key.vk == libtcod.KEY_ENTER:
                break

        if libtcod.console_is_window_closed():
            exit()

        tcod_clear(con)
        # Draw the wall at the y coord
        for i, line in enumerate(INTRO_WALL):
            tcod_print_ex(con, SCREEN_WIDTH / 2, i-y,
                libtcod.BKGND_NONE, libtcod.CENTER, line)

        libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)
        libtcod.console_flush()
        time.sleep(.75)

def inventory_menu(header):
    ''' Show a menu with each item of the inventory as an option '''

    # If inventory is empty...
    if len(inventory) == 0:
        options = ['Inventory is empty.']
    # Otherwise...
    else:
        options = []
        sort_inventory()
        # For item in inventory...
        for item in inventory:
            text = item.name
            # Show additional information, in case it's equipped
            if item.equipment and item.equipment.is_equipped:
                text = text + ' (on ' + item.equipment.slot + ')'
            # Append item to list
            options.append(text)

    # Get selection
    index = menu(header, options, INVENTORY_WIDTH)

    # If an item was chosen, return it
    if index is None or len(inventory) == 0:
        return None
    return inventory[index].item

def is_blocked(x, y):
    ''' Check if x, y on the map is blocked '''

    # First test the map tile
    if world[x][y].blocked:
        return True

    # Now check for any blocking objects
    for obj in objects:
        if obj.blocks and obj.x == x and obj.y == y:
            return True

    return False

def json_get_color(color_str):
    ''' Translate json color string into libtcod colors. Wrapper function '''
    return COLORS[color_str]

def load_game():
    ''' Open the previously saved shelve and load the game data '''

    # I have no idea how shelve works but it's magic
    global world, objects, player, inventory, game_msgs, game_state, \
            dungeon_level, dstairs, ustairs, blind, blind_counter

    file = shelve.open('savegame', 'r')
    world = file['world']
    objects = file['objects']
    player = objects[file['player_index']]
    inventory = file['inventory']
    game_msgs = file['game_msgs']
    game_state = file['game_state']
    dstairs = objects[file['dstairs_index']]
    ustairs = objects[file['ustairs_index']]
    dungeon_level = file['dungeon_level']
    kill_count = file['kill_count']
    blind = file['blind']
    blind_counter = file['blind_counter']
    file.close()

    if not blind:
        initialize_fov()
    else:
        player.draw()

def main_menu():
    ''' Show the main menu '''

    img = libtcod.image_load(MENU_IMAGE)

    git_screen()

    while not libtcod.console_is_window_closed():
        # Show the background image, at twice the regular console resolution
        libtcod.image_blit_2x(img, 0, 0, 0)

        # Show the game's title, and some credits!
        tcod_set_fg(0, TEXT_COLORS['title'])
        tcod_print_ex(0, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 4,
            libtcod.BKGND_NONE, libtcod.CENTER, 'Edgequest')
        tcod_print_ex(0, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 + 4,
            libtcod.BKGND_NONE, libtcod.CENTER, 'What hath God wrought?')

        tcod_set_fg(0, libtcod.black)
        tcod_print_ex(0, SCREEN_WIDTH / 2, SCREEN_HEIGHT - 2,
            libtcod.BKGND_NONE, libtcod.CENTER, 'By Gray')

        # Show options and wait for the player's choice
        choice = menu('Options', ['Play a new game', 'Continue last game',
                        'How to play', 'Credits', 'Quit'], 24)

        if choice == 0:  # New game
            intro_cutscene()
            choose_name()
            choose_theme()
            new_game()
            play_game()
        if choice == 1:  # Load last game
            try:
                load_game()
            except:
                msgbox('\n No saved game to load.\n', 24)
                continue
            play_game()
        if choice == 2:  # How to play
            how_to_play()
        if choice == 3:
            credits_screen()
        elif choice == 4:  # Quit
            exit()

def make_map():
    ''' Make a map '''

    global world, fov_map, objects, dstairs, ustairs, unblocked_world

    # Tamed monsters follow player up and down stairs
    saved_monsters = []
    if objects:
        for mon in objects:
            if mon.ai:
                if mon.ai.tamed:
                    saved_monsters.append(mon)

    # The list of objects with just the player
    objects = []
    objects.append(player)

    # Load up tamed monsters
    for mon in saved_monsters:
        objects.append(mon)


    # fill map with 'blocked' tiles
    world = [[Tile(True) for y in range(MAP_HEIGHT)] for x in range(MAP_WIDTH)]

    rooms = []
    num_rooms = 0

    # Rev up those map generators
    themap = dMap()

    '''
    Okay this takes some magic to get working but once you do you can create
    a ton of cool maps with it.

    * The first two values are the dimensions of the map.
    * The second one is the 'fail' rating.
         * Not sure what the heck that means but the higher it is, the more rooms you
           get.
    * The the fourth is the 'b1' value. What's a b1? No idea.
    Apparently it controlls the frequency of corridors.
    Lastly, the number of maximum rooms. Multiply the max_rooms by 4 because
    the rooms are pretty.
    '''

    # Template original map:
    #   themap.makeMap(MAP_WIDTH,MAP_HEIGHT-2,250,1,MAX_ROOMS*4)
    rooms = MAX_ROOMS + dungeon_level + int(math.floor((dungeon_level/4)*4)) + 2
    fail = 150 * int(math.floor((dungeon_level/3)*3)) + 100
    b1 = int(math.floor((dungeon_level / 6)*3)) + 1

    themap.makeMap(MAP_WIDTH, MAP_HEIGHT-2, fail, b1, rooms)

    # Turn ones and zeros into magic
    for y in range(MAP_HEIGHT-2):
            for x in range(MAP_WIDTH):
                    if themap.mapArr[y][x]==0:
                            world[x][y].blocked = False
                            world[x][y].block_sight = False
                    if themap.mapArr[y][x]==1:
                            world[x][y].blocked = True
                            world[x][y].block_sight = True
                    if themap.mapArr[y][x]==2:
                            world[x][y].blocked = True
                            world[x][y].block_sight = True

    # Make an FOV map
    fov_map = libtcod.map_new(MAP_WIDTH, MAP_HEIGHT)

    # Set FOV map
    for y in range(MAP_HEIGHT):
        for x in range(MAP_WIDTH):
            libtcod.map_set_properties(fov_map, x, y, not world[x][y].block_sight, not world[x][y].blocked)

    # Create stairs at some random ass location
    x = libtcod.random_get_int(0,0, MAP_WIDTH-1)
    y = libtcod.random_get_int(0,0, MAP_HEIGHT-1)

    # Reset the unblocked coords
    unblocked_world = []

    # Append all coordinate tuples to the unblocked list
    for y in range(MAP_HEIGHT):
        for x in range(MAP_WIDTH):
            if not world[x][y].blocked:
                unblocked_world.append((x, y))

    # Put player at a random position
    x, y = get_rand_unblocked_coord()
    player.x = x
    player.y = y

    # Place tamed monster on same tile
    for mon in objects:
        if mon.ai:
            if mon.ai.tamed:
                mon.backup_coord = get_rand_unblocked_coord()
                (mon.x, mon.y) = (x, y)

    # If player is moving downwards
    if stairs_up:
        # Randomly place downstairs
        x, y = get_rand_unblocked_coord()

        dstairs = Object(x, y, '>', 'down stairs', libtcod.white, always_visible=True)

        objects.append(dstairs)
        # This tends to cause issues in the later levels
        dstairs.send_to_back()  # So it's drawn below the monsters

        # Place upstairs on player
        ustairs = Object(player.x, player.y, '<', 'up stairs', libtcod.white, always_visible=True)

        objects.append(ustairs)
        # So it's drawn below the monsters
        ustairs.send_to_back()

    # If player is moving upwards
    else:
        # Place upstairs randomly
        x, y = get_rand_unblocked_coord()

        ustairs = Object(x, y, '<', 'up stairs', libtcod.white, always_visible=True)
        objects.append(ustairs)
        # This tends to cause issues in the later levels
        ustairs.send_to_back()  # So it's drawn below the monsters

        # Place downstairs on player
        dstairs = Object(player.x, player.y, '>', 'down stairs', libtcod.white, always_visible=True)

        objects.append(dstairs)
        # So it's drawn below the monsters
        dstairs.send_to_back()

    # Finally put stuff everywhere
    if dungeon_level == CSGO_FLOOR:
        de_dust()
    else:
        place_objects()

    # TODO: Biomes
    if dungeon_level >= 10:
        # initialize_theme()
        pass

def menu(header, options, width):
    ''' Create a menu that options can be selected from using the alphabet '''

    if len(options) > 26: raise ValueError('Cannot have a menu with more than \
                                            26 options.')

    # Calculate total height for the header (after auto-wrap) and one line per
    #   option
    header_height = libtcod.console_get_height_rect(con, 0, 0, width, SCREEN_HEIGHT, header)

    height = len(options) + header_height

    # Create an off-screen console that represents the menu's window
    window = libtcod.console_new(width, height)

    # Print the header, with auto-wrap
    tcod_set_fg(window, libtcod.white)
    libtcod.console_print_rect_ex(window, 0, 0, width, height,
        libtcod.BKGND_NONE, libtcod.LEFT, header)

    # Print all the options
    y = header_height
    letter_index = ord('a')
    for option_text in options:
        text = '(' + chr(letter_index) + ') ' + option_text
        tcod_print_ex(window, 0, y, libtcod.BKGND_NONE,
                                libtcod.LEFT, text)
        y += 1
        letter_index += 1

    # Blit the contents of 'window' to the root console
    x = SCREEN_WIDTH / 2 - width / 2
    y = SCREEN_HEIGHT / 2 - height / 2
    libtcod.console_blit(window, 0, 0, width, height, 0, x, y, 1.0, 0.7)

    # Present the root console to the player and wait for a key-press
    libtcod.console_flush()
    key = libtcod.console_wait_for_keypress(True)

    # Convert the ASCII code to an index; if it corresponds to an option,
    #   return it
    index = key.c - ord('a')
    if 0 <= index < len(options):
        return index
    else:
        return None

def message(new_msg, color=TEXT_COLORS['default']):
    ''' Send a message to the console at the bottom '''

    global old_msg, msg_counter

    # If the same message is going to be re-outputted, add a convenient counter
    if old_msg == new_msg:
        msg_counter += 1
        alt_msg = ''.join([new_msg, ' <x', str(msg_counter), '>'])
    # Otherwise reset the counter
    else:
        msg_counter = 1
        alt_msg = new_msg

    # Split the message if necessary, among multiple lines
    new_msg_lines = textwrap.wrap(alt_msg, MSG_WIDTH)

    for line in new_msg_lines:
        # Make sure the last line is overwritten for minimalism
        if old_msg == new_msg:
            game_msgs[len(game_msgs)-1] = (line, color)
        else:
            # If the buffer is full, remove the first line to make
            #   room for the new one
            if len(game_msgs) == MSG_HEIGHT:
                del game_msgs[0]

            # Add the new line as a tuple, with the text and the color
            game_msgs.append((line, color))

    # Store old message for comparison later
    old_msg = new_msg

def monster_death(monster):
    ''' Function called when monster dies '''

    # Transform it into a nasty corpse! it doesn't block, can't be
    # Attacked and doesn't move
    message(' '.join([monster.name.capitalize(), 'is dead!']),
        TEXT_COLORS['very_bad'])
    message('You gain ' + str(monster.fighter.xp) + ' experience points.',
        TEXT_COLORS['level_up'])
    monster.set_corpse()

def monster_death_slock(monster):
    ''' Function called when monster dies. Blinds player '''

    # Transform it into a nasty corpse! it doesn't block, can't be
    # Attacked and doesn't move
    message(' '.join([monster.name.capitalize(), 'is dead!']),
        TEXT_COLORS['very_bad'])
    message('You gain ' + str(monster.fighter.xp) + ' experience points.',
        TEXT_COLORS['level_up'])
    message(' '.join([monster.name.capitalize(),
        'casts a final spell in its dying moments!']))
    monster.set_corpse()
    # Blind
    cast_inflict_blindness()

def monster_death_talk(monster):
    ''' Function called when monster dies. Says dying words '''

    # Transform it into a nasty corpse! it doesn't block, can't be
    # Attacked and doesn't move

    # This function assumes that the assertions for the death functions worked
    # However the death talk isn't asserted

    # Try to get it
    try:
        assert monster_data[mon]['death_talk'] is not None
        mon_death_talk = monster_data[mon]['death_talk']
    # No big deal, just a problem for the JSON
    except AssertionError as e:
        logger.error('AssertionError: death_talk not found for ' + monster.name)
        logger.info('Defaulting death talk...')
        mon_death_talk = 'I was born to die just like a bug!'

    message(''.join([monster.name.capitalize(), ' says "', mon_death_talk,
        '"']), TEXT_COLORS['bad'])

    message(' '.join([monster.name.capitalize(), 'is dead!']),
        TEXT_COLORS['very_bad'])

    message('You gain ' + str(monster.fighter.xp) + ' experience points.',
        TEXT_COLORS['level_up'])

    monster.set_corpse()

def monster_occupy_check(dx, dy):
    ''' If a monster is in that location, return true '''

    for obj in objects:
        if (obj.x, obj.y) == (dx, dy) and obj.blocks:
            return True
    return False

def mouse_move_astar(tx, ty):
    ''' Click on a space to send player there '''

    monster = False

    if player_action == 'use':
        return None

    # Initially check for monsters
    for obj in objects:
        # If it's a fighter and in the fov, and it has an ai
        if libtcod.map_is_in_fov(fov_map, obj.x, obj.y) and \
        obj.fighter and obj.ai:
            # AND it's not tamed
            if not obj.ai.tamed:
                message('Cannot move: Monster in view!', TEXT_COLORS['debug'])
                monster = True

    try:
        # Can't move to blocked locations
        if is_blocked(tx, ty):
            message('Cannot move: Location is unexplored', TEXT_COLORS['debug'])
        # Can't move to unexplored locations
        elif not world[tx][ty].explored:
            message('Cannot move: Location is unexplored', TEXT_COLORS['debug'])
        # Can't move if blind
        elif blind:
            message('Cannot move: You are blind',
                libtcod.pink)
        # If there's no monster, start moving
        elif not monster:
            while not libtcod.console_is_window_closed() and not monster and \
            (player.x, player.y) != (tx, ty):
                render_all()
                # Present the root console
                libtcod.console_flush()

                for obj in objects:
                    # Continually scan for monsters
                    if libtcod.map_is_in_fov(fov_map, obj.x, obj.y) and \
                    obj.fighter and obj.ai:
                        if not obj.ai.tamed:
                            message('Cannot move: Monster in view!', TEXT_COLORS['debug'])
                            monster = True
                            continue

                # Move A*
                player.move_astar(tx, ty, True)
                fov_recompute()

                # AI takes turn
                for obj in objects:
                    if obj.ai:
                        obj.ai.take_turn()

                # Check the ground
                check_ground()

    # Player clicks outside of map
    except IndexError:
        message('Cannot move: Out of range', TEXT_COLORS['debug'])

def msgbox(text, width=50):
    ''' use menu() as a sort of \'message box\' '''
    menu(text, [], width)

def msgbox_stats(title):
    ''' Show a msgbox with player stats '''

    msgbox( title + '\n\n \
    Level: ' + str(player.level) + '\n \
    Floor: ' + str(dungeon_level) + '\n \
    Experience: ' + str(player.fighter.xp) + 'xp' + '\n \
    Next level at: ' + str(player.fighter.level_up_xp) + 'xp' + '\n \
    Maximum HP: ' + str(player.fighter.max_hp) + '\n \
    Attack: ' + str(player.fighter.power) + '\n \
    Defense: ' + str(player.fighter.defense) + '\n \
    Killstreak: ' + str(kill_count) + '\n \
    Time: ' + str(timer) + '\n\n \
    Press any key to continue...',
    CHARACTER_SCREEN_WIDTH)

def new_game():
    ''' Start a new game '''

    global player, edge, inventory, game_msgs, game_state, dungeon_level, \
            monster_data, items_data, dog, objects

    # Player
    # create object representing the player
    fighter_component = Fighter(hp=100, defense=1, power=4, xp=0, mana=100,
        accuracy=0, death_function=player_death, attack_msg=DEFAULT_ATTACK)

    player = Object(0, 0, PLAYER_CHARACTER, player_name, PLAYER_COLOR, blocks=True,
        fighter=fighter_component)

    objects.append(generate_monster('tameddog', 0, 0))

    # let player know they're invisible
    if INVISIBLE:
        player.color = libtcod.black

    # Initialize dungeon level
    dungeon_level = 1

    # Generate map (at this point it's not drawn to the screen)
    make_map()
    initialize_fov()

    game_state = 'playing'
    inventory = []

    # Create the list of game messages and their colors, starts empty
    game_msgs = []

    # A warm welcoming message!
    message('Welcome to EdgeQuest!', TEXT_COLORS['edge'])
    message('Press ? to see a list of commands!', TEXT_COLORS['debug'])

    render_all()
    # Present the root console
    libtcod.console_flush()

def next_level():
    ''' Go to next level '''

    global dungeon_level, max_dungeon_level, stairs_up

    # Go up
    dungeon_level += 1

    stairs_up = True

    message('After a rare moment of peace, you descend deeper into the \
        heart of the dungeon...', TEXT_COLORS['neutral'])

    make_map()  # Create a fresh new level!
    initialize_fov()

def place_objects():
    ''' Place objects on level '''
    # Maximum number of monsters per level
    max_monsters = from_dungeon_level([[4, 1], [7, 2], [13, 4],
        [20, 6], [21, 10]])

    # Chance of each monster
    monster_chances = {}
    # Monster name then chance
    for item in monster_data:
        monster_chances[str(monster_data[item]['id'])] = \
            from_dungeon_level(monster_data[item]['chance'])

    # Maximum number of items per level
    max_items = from_dungeon_level([[6, 1], [10, 3], [18, 6], [21, 7], [30, 9]])

    # Chance of each item (by default they have a chance of 0 at level 1,
    #   which then goes up)
    item_chances = {}
    # Item name then chance.
    for item in items_data:
        item_chances[str(items_data[item]['id'])] = \
            from_dungeon_level(items_data[item]['chance'])

    # Choose random number of monsters
    num_monsters = libtcod.random_get_int(0, 0, max_monsters+dungeon_level)

    for i in range(num_monsters):
        x, y = get_rand_unblocked_coord()

        choice = random_choice(monster_chances)

        monster = generate_monster(choice, x, y)

        # Add monster to object list
        objects.append(monster)

        # monster.send_to_back()

    # Choose random number of items
    num_items = libtcod.random_get_int(0, 0, max_items+dungeon_level)

    for i in range(num_items):
        x, y = get_rand_unblocked_coord()

        # Only place it if the tile is not blocked
        choice = random_choice(item_chances)

        item = generate_item(choice, x, y)

        objects.append(item)
        # Items appear below other objects
        item.send_to_back()
        # Items are visible even out-of-FOV, if in an explored area
        item.always_visible = True

def play_game():
    ''' Main game loop '''

    global key, mouse, player_action, timer, blind_counter

    player_action = None

    mouse = libtcod.Mouse()
    key = libtcod.Key()

    while not libtcod.console_is_window_closed():
        libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS | libtcod.EVENT_MOUSE, key, mouse)

        render_all()
        # Present the root console
        libtcod.console_flush()

        check_timer()

        # Erase all objects at their old locations, before they move
        for obj in objects:
            obj.clear()

        # Handle keys
        handle_keys()

        # Handle mouse
        if mouse.lbutton_pressed:
            mouse_move_astar(mouse.cx + camera.x, mouse.cy + camera.y)

        # Let monsters take their turn
        if game_state == 'playing' and player_action != 'didnt-take-turn':
            for obj in objects:
                if obj.ai:
                    obj.ai.take_turn()
            if blind:
                blind_counter += 1
            timer += 1

def player_death(player):
    ''' What happens when player dies '''

    # The game ended!
    global game_state
    if not GOD_MODE: # Debug
        message('You died!', TEXT_COLORS['very_bad'])

        # For added effect, transform the player into a corpse!
        player.set_player_corpse()
        player.color = libtcod.dark_red

        game_state = 'dead'

        render_all()

        # Present the root console
        libtcod.console_flush()

        game_over()

    else:
        # God mode debug hacks
        message('...But it refused!', TEXT_COLORS['fail'])
        player.fighter.hp = player.fighter.max_hp

def player_move(dx, dy):
    ''' Move player in a direction based on coords '''

    # The coordinates the player is moving to/attacking
    x = player.x + dx
    y = player.y + dy

    # Try to find an attackable object there
    target = None
    for obj in objects:
        if obj.fighter and obj.x == x and obj.y == y:
            target = obj
            break

    # Attack if target found, move otherwise
    if target is not None:
        # Displace tamed monsters
        if target.ai.tamed:
            target.displace(dx, dy)
        # Or attack monsters
        else:
            player.fighter.attack(target)
    else:
        player.move(dx, dy)
        fov_recompute()

def previous_level():
    ''' Go back up in the dungeon '''

    global dungeon_level, stairs_up
    # In case if you're that guy who likes going back for some reason

    # Go up
    dungeon_level -= 1

    # Set what stairs spawn
    stairs_up = False

    # Check win
    if dungeon_level == 0:
        # Win condition
        for item in inventory:
            if item.name == 'StatTrak Fedora | Fade (Fac New)':
                game_win()
        # Quitters are also failures
        else:
            choice = menu('Leave the Dungeon?', ['Yes', 'No'], 30)

            if choice == 0:
                game_over()
            else:
                render_all()
                # Present the root console
                libtcod.console_flush()
                choice = menu('You head back down into the depths...',
                                ['Continue'], 30)

    # Otherwise, continue ascent
    else:
        message('After a rare moment of peace, you ascend upwards towards \
            the surface...', TEXT_COLORS['neutral'])
        make_map()  # Create a fresh new level!
        initialize_fov()

def random_choice(chances_dict):
    ''' Choose one option from dictionary of chances, returning its key '''

    chances = chances_dict.values()
    strings = chances_dict.keys()

    return strings[random_choice_index(chances)]

def random_choice_index(chances):
    ''' Choose one option from list of chances, returning its index '''

    # The dice will land on some number between 1 and the sum of the chances
    dice = libtcod.random_get_int(0, 1, sum(chances))

    # Go through all chances, keeping the sum so far
    running_sum = 0
    choice = 0

    for w in chances:
        running_sum += w

        # See if the dice landed in the part that corresponds to this choice
        if dice <= running_sum:
            return choice
        choice += 1

def render_all():
    ''' Draw everything to the screen '''

    global blind, blind_counter

    # Move the camera
    camera.move(player.x, player.y)

    # Make sure the player isn't blind
    if not blind:
        # Do normal rendering
        # Recompute fov
        if camera.check_fov:
            camera.check_fov = False
            fov_recompute()

        # Draw all objects in the list, except the player. we want it to
        # Always appear over all other objects! so it's drawn later.
        for obj in objects:
            if obj.name != player.name:
                obj.draw()

        # Always draw the player
        player.draw()

        # Display a cursor under mouse coords
        tcod_set_char_bg(con, mouse.cx, mouse.cy, colors.color_ground_highlight)
        # blit the contents of 'con' to the root console
        libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)
        fov_recompute()

    # Blind players see nothing but themselves. (Deep, huh?)
    else:
        # Reduce time until vision returns
        if blind_counter == BLIND_LENGTH:
            blind = False
            blind_counter = 0
            message("Your vision returns!", TEXT_COLORS['magic'])

        # Always draw the player
        player.draw()

        tcod_clear(con)
        tcod_set_bg(con, libtcod.black)
        player.draw()
        libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)

    # Render the gui elements
    render_gui()

def render_bar(x, y, total_width, name, value, maximum, bar_color, back_color):
    ''' Render a bar (HP, experience). '''

    # first calculate the width of the bar
    bar_width = int(float(value) / maximum * total_width)

    # Render the background first
    tcod_set_bg(panel, back_color)
    libtcod.console_rect(panel, x, y, total_width, 1, False, libtcod.BKGND_SCREEN)

    # Now render the bar on top
    tcod_set_bg(panel, bar_color)
    if bar_width > 0:
        libtcod.console_rect(panel, x, y, bar_width, 1, False, libtcod.BKGND_SCREEN)

    # Finally, some centered text with the values
    tcod_set_fg(panel, libtcod.white)
    tcod_print_ex(panel, x + total_width / 2, y, libtcod.BKGND_NONE,
        libtcod.CENTER, name + ': ' + str(value) +
        '/' + str(maximum))

def render_bar_simple(x, y, total_width, name, value, color):
    ''' Extremely simple bar rendering

    Not intended to have values increase and decrease, but rather display
    one static value instead (attack, defense)

    '''

    # Render the background first
    tcod_set_bg(panel, color)
    libtcod.console_rect(panel, x, y, total_width, 1, False, libtcod.BKGND_SCREEN)

    # Now render the bar on top
    tcod_set_bg(panel, color)
    if total_width > 0:
        libtcod.console_rect(panel, x, y, total_width, 1, False, libtcod.BKGND_SCREEN)

    # Finally, some centered text with the values
    tcod_set_fg(panel, libtcod.white)
    tcod_print_ex(panel, x + total_width / 2, y, libtcod.BKGND_NONE,
        libtcod.CENTER, name + ': ' + str(value))

def render_equips(y_offset, slot):
    ''' Render equipment status based on slot '''

    # Get equipment in a slot
    equip = get_equipped_in_slot(slot)
    if not equip:
        equip = "None"
    else:
        # Shorten name if it's too long
        if len(equip.owner.name) > 12:
            equip = equip.short_name
        else:
            equip = equip.owner.name

    # Shorten name if it's one of the hands
    if slot == 'right hand':
        slot = 'RH'
    elif slot == 'left hand':
        slot = 'LH'

    render_bar_simple(1, y_offset, BAR_WIDTH, slot.capitalize(), equip, libtcod.black)

def render_gui():
    ''' Render all gui elements from the message console, to the enemies in the room '''

    # Prepare to render the GUI panel
    tcod_set_bg(panel, libtcod.black)
    tcod_clear(panel)

    # Also the message panel
    tcod_set_bg(msg_panel, libtcod.black)
    tcod_clear(msg_panel)

    # Show the player's stats
    tcod_print_ex(panel, 1 + BAR_WIDTH / 2, 0, libtcod.BKGND_NONE,
        libtcod.CENTER, player.name)

    # Show Perks
    render_perks(5)

    # Cool distinctions
    tcod_set_fg(panel, libtcod.gray)
    for y in range(SCREEN_HEIGHT):
        tcod_print_ex(panel, 0, y, libtcod.BKGND_NONE, libtcod.CENTER, '|')

    tcod_set_fg(msg_panel, libtcod.gray)

    for x in range(SCREEN_WIDTH):
        tcod_print_ex(msg_panel, x, 0, libtcod.BKGND_NONE, libtcod.CENTER,
            '-')

    render_bar(1, 1, BAR_WIDTH, 'HP', player.fighter.hp, player.fighter.max_hp,
        libtcod.light_red, libtcod.darker_red)

    # Self-explanatory bars
    # Edge
    render_bar(1, 2, BAR_WIDTH, 'Edge', player.fighter.mana,
        player.fighter.max_mana, libtcod.dark_fuchsia, libtcod.darker_fuchsia)

    # XP
    render_bar(1, 3, BAR_WIDTH, 'XP', player.fighter.xp, (LEVEL_UP_BASE +
        player.level * LEVEL_UP_FACTOR), libtcod.dark_yellow,
        libtcod.darker_yellow)

    # Dungeon level
    render_bar_simple(1, 4, BAR_WIDTH, 'Floor', str(dungeon_level),
        libtcod.light_blue)

    # Power
    render_bar_simple(1, 6, BAR_WIDTH/2, 'STR', str(player.fighter.power),
        libtcod.darker_chartreuse)

    # Defense
    render_bar_simple(1, 7, BAR_WIDTH/2, 'DEF', str(player.fighter.defense),
        libtcod.flame)

    # Accuracy
    render_bar_simple(BAR_WIDTH/2+1, 7, BAR_WIDTH/2, 'ACU', str(player.fighter.accuracy),
        libtcod.dark_blue)

    # Render a list of equipment slots and items in each slot
    for y, slot in enumerate(SLOT_LIST):
        render_equips(SCREEN_HEIGHT - len(SLOT_LIST) + y, slot)

    # Show all the monsters that the player can see and shows their health
    monsters_in_room = 0
    for obj in objects:
        # If it's in the player's fov, and the it's a fighter, and it's not the player
        # AND the player isn't blind...
        if libtcod.map_is_in_fov(fov_map, obj.x, obj.y) and obj.fighter and \
        obj.name != player.name and not blind:

            # Get the total monsters in the room, but limit it as we don't
            # want this to overlap the rendering of equips in slots
            monsters_in_room += 1
            if monsters_in_room > (SCREEN_HEIGHT - 18) / 2:
                continue
            else:
                # Basically for each closest monster, show it's full name
                # And a health bar

                # Set fg for the name
                tcod_set_fg(panel, obj.color)

                # Tamed AIs have a special flair
                flair = ""
                if obj.ai:
                    if obj.ai.tamed:
                        flair = '(tamed)'

                # Print the name of the monster
                tcod_print_ex(panel, 1, 7+(2*monsters_in_room),
                    libtcod.BKGND_NONE, libtcod.LEFT, ''.join([obj.char, ' ',
                    obj.name.capitalize() + ' ' + flair]))

                # And a health bar
                render_health_bar(1, 8+(2*monsters_in_room), BAR_WIDTH,
                    obj.fighter.hp, obj.fighter.base_max_hp, libtcod.red,
                    libtcod.dark_red)

    # Display names of objects under the mouse
    if not blind:
        tcod_set_fg(msg_panel, libtcod.light_gray)
        tcod_print_ex(msg_panel, 1, 0, libtcod.BKGND_NONE, libtcod.LEFT, \
            camera.get_names_under_mouse(mouse, objects, COORDS_UNDER_MOUSE))

    # Print the game messages, one line at a time
    y = 1
    for (line, color) in game_msgs:
        tcod_set_fg(msg_panel, color)
        tcod_print_ex(msg_panel, MSG_X, y, libtcod.BKGND_NONE, libtcod.LEFT, line)
        y += 1

    # Blit the contents of 'panel' and 'msg_panel' to the root console
    libtcod.console_blit(msg_panel, 0, 0, SCREEN_WIDTH, PANEL_HEIGHT, 0, 0, MSG_PANEL_Y)
    libtcod.console_blit(panel, 0, 0, PANEL_WIDTH, PANEL_HEIGHT, 0, SCREEN_WIDTH-PANEL_WIDTH,
        PANEL_Y)

def render_health_bar(x, y, total_width, value, maximum, bar_color, back_color):
    ''' This is a bar that doesn't show any values in it. Useful for enemy
    health bars '''

    # Render a bar (HP, experience, etc). first calculate the width of the bar
    bar_width = int(float(value) / maximum * total_width)

    # Render the background first
    tcod_set_bg(panel, back_color)
    libtcod.console_rect(panel, x, y, total_width, 1, False,
        libtcod.BKGND_SCREEN)

    # Now render the bar on top
    tcod_set_bg(panel, bar_color)
    if bar_width > 0:
        libtcod.console_rect(panel, x, y, bar_width, 1, False,
            libtcod.BKGND_SCREEN)

    # Finally, show the bar
    tcod_set_fg(panel, libtcod.white)
    tcod_print_ex(panel, x + total_width / 2, y, libtcod.BKGND_NONE, libtcod.CENTER, '')

def render_perks(y):
    ''' Render the perks at a certain y level on the panel '''

    if perk_mtndew >= PERK_BASE:
        tcod_set_fg(panel, libtcod.light_green)
        tcod_print_ex(panel, 1, y, libtcod.BKGND_NONE, libtcod.CENTER, '!')

    if perk_cokezero >= PERK_BASE:
        tcod_set_fg(panel, libtcod.violet)
        tcod_print_ex(panel, 2, y, libtcod.BKGND_NONE, libtcod.CENTER, '!')

    if perk_tazer >= PERK_BASE:
        tcod_set_fg(panel, libtcod.sky)
        tcod_print_ex(panel, 3, y, libtcod.BKGND_NONE, libtcod.CENTER, '=')

    if perk_incengren >= PERK_BASE:
        tcod_set_fg(panel, libtcod.light_red)
        tcod_print_ex(panel, 4, y, libtcod.BKGND_NONE, libtcod.CENTER, '*')

    if perk_fbang >= PERK_BASE:
        tcod_set_fg(panel, libtcod.azure)
        tcod_print_ex(panel, 5, y, libtcod.BKGND_NONE, libtcod.CENTER, '*')

def save_game():
    ''' Open a new empty shelve (possibly overwriting an old one)
    to write the game data '''

    choice = menu('Save and Quit?', ['Yes', 'No'], 24)

    if choice == 0:  # Yes
        file = shelve.open('savegame', 'n')
        file['world'] = world
        file['objects'] = objects
        file['player_index'] = objects.index(player)
        file['inventory'] = inventory
        file['game_msgs'] = game_msgs
        file['game_state'] = game_state
        file['dstairs_index'] = objects.index(dstairs)
        file['ustairs_index'] = objects.index(ustairs)
        file['dungeon_level'] = dungeon_level
        file['kill_count'] = kill_count
        file['blind'] = blind
        file['blind_counter'] = blind_counter
        file.close()
        render_all()
        # Present the root console
        libtcod.console_flush()
        choice = menu('Bye!', [], 6)
        exit()
    elif choice == 1:  # No
        pass

def sort_inventory():
    ''' Sorts inventory with equipment first followed by items '''

    global inventory

    equips = []
    items = []

    # Seperate equiped things and items
    for item in inventory:
        if item.equipment and item.equipment.is_equipped:
            equips.append(item)
        else:
            items.append(item)

    # Sort them...
    # I'll be honest I have no idea what the lambda thing does
    # but it looks like it sorts the object list by the names of the object
    equips = sorted(equips, key=lambda obj: obj.name)
    items = sorted(items, key=lambda obj: obj.name)

    # Then put equipment first, followed by items
    inventory = equips + items

def target_monster(max_range=None):
    ''' Returns a clicked monster inside FOV up to a range,
    or None if right-clicked '''

    while True:
        (x, y) = target_tile(max_range)

        if x is None:  # Player cancelled
            return None

        # Return the first clicked monster, otherwise continue looping
        for obj in objects:
            if obj.x == x and obj.y == y and obj.fighter and obj != player:
                return obj

def target_tile(max_range=None):
    ''' Return the position of a tile left-clicked in player's FOV
    (optionally in a range), or (None,None) if right-clicked. '''

    global key, mouse

    while True:
        # Render the screen. this erases the inventory and
        #   shows the names of objects under the mouse.
        libtcod.console_flush()

        libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS | libtcod.EVENT_MOUSE, key, mouse)

        render_all()

        # Present the root console
        libtcod.console_flush()
        (x, y) = (mouse.cx, mouse.cy)
        (x, y) = (camera.x + x, camera.y + y)  # From screen to map coordinates

        # Cancel if the player right-clicked or pressed Escape
        if mouse.rbutton_pressed or key.vk == libtcod.KEY_ESCAPE:
            return (None, None)

        # Accept the target if the player clicked in FOV,
        #   and in case a range is specified, if it's in that range
        if (mouse.lbutton_pressed and libtcod.map_is_in_fov(fov_map, x, y) and \
        (max_range is None or player.distance(x, y) <= max_range)):
            return x, y

def taunt():
    ''' Taunt enemies. Mostly just fluff '''

    taunts = [
        'Nothin\' personnel, kid',
        'M\'lady',
        'Notice me senpai',
        'Filthy gaijin go home',
        'Get rekt',
        'You fell for the meme, kiddo'
    ]

    # Say a random taunt
    message(''.join(['You say \'', random.choice(taunts), '\'']))

def toggle_fullscreen():
    ''' Toggle fullscreen mode in libtcod '''

    libtcod.console_set_fullscreen(not libtcod.console_is_fullscreen())

    logger.info('Toggled fullscreen mode')

def toggle_siphon():
    ''' Toggle the siphon spell

    Siphon just drains health. This is enabled at start.

    '''

    global activate_siphon
    if activate_siphon:
        activate_siphon = False
        message('You deativate your siphon ability', TEXT_COLORS['debug'])
    else:
        activate_siphon = True
        message('You activate your siphon ability', TEXT_COLORS['debug'])

def weapon_action_katana(weapon):
    ''' Katana action '''

    message('You examine the fine steel of the katana, surely folded over 1000 times', TEXT_COLORS['neutral'])

def weapon_action_knife(weapon):
    ''' Knife action '''

    message('You inspect your latest knife', TEXT_COLORS['neutral'])

def weapon_action_awp(weapon):
    ''' AWP action.

    The AWP is basically a magic wand that casts Lightning

    '''

    message('You no-scope with the AWP', TEXT_COLORS['magic'])
    cast_lightning()

def weapon_action_firearm(weapon):
    ''' Firearm action. Fires the gun. '''

    fire_weapon(weapon.equipment)

def weapon_action_else(weapon):
    ''' Emergency reserve action '''

    message('You stare deeply at your ' + weapon.name, TEXT_COLORS['fail'])

# ------------------------------------------------------------------------------

# Start ------------------------------------------------------------------------

# Logging is good!
logger.info('EdgeQuest Start')

# Backup in case if python -B doesn't get ran
sys.dont_write_bytecode = True

# Start a basic List
unblocked_world = [(0, 0)]

# Init a basic theme
initialize_theme(CURRENT_THEME)

# Check the arguments that are ran with edgequest.py
# If this gives problems, replace with `main_menu()`
check_args()

# ------------------------------------------------------------------------------
