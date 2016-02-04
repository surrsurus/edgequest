
import math
import shelve
import sys
import textwrap
import time
from random import *

from colors import *
from modules import libtcodpy as libtcod
from modules.dmap import dMap
from modules import simplejson as json
from modules.wallselect import wallselect
from settings import *

# Backup in case if python -B doesn't get ran
sys.dont_write_bytecode = True

######################################
# JSON
######################################

# Monster JSON
monster_json = 'json/monster.json'
# Load monsters
with open(monster_json) as json_data:
    monster_data = json.load(json_data)

# Items JSON
items_json = 'json/items.json'
# Load items
with open(items_json) as json_data:
    items_data = json.load(json_data)

######################################
# Game Variables
######################################

# Game State
game_state = 'playing'

# Blindness
blind = False
blind_counter = 0

# Siphon
activate_siphon = True

# Player action
player_action = None

# Message store
old_msg = None
msg_counter = 1

# Dungeon level
dungeon_level = 1

# Timer
timer = 0

# Killstreak
kill_count = 0

# Stairs direction
stairs_up = True

######################################
# Classes
######################################

class BasicMonster:
    ''' AI for a basic monster. '''
    def __init__(self):
        pass

    def take_turn(self):
        '''Monster takes its turn. If you can see it, it can see you '''
        monster = self.owner
        # If it's in the player's fov then it approaches them
        if libtcod.map_is_in_fov(fov_map, monster.x, monster.y):

            # Move towards player if far away
            if monster.distance_to(player) >= 2:
                monster.move_astar(player.x, player.y, False)

            # Close enough, attack! (if the player is still alive.)
            elif player.fighter.hp > 0:
                monster.fighter.attack(player)

        # Otherwise it just runs around randomly
        else:
            x = libtcod.random_get_int(0, -1, 1)
            y = libtcod.random_get_int(0, -1, 1)
            monster.move(x, y)

class ConfusedMonster:
    ''' AI for a temporarily confused monster
    (reverts to previous AI after a while). '''
    def __init__(self, old_ai, num_turns=CONFUSE_NUM_TURNS):
        self.old_ai = old_ai
        self.num_turns = num_turns

    def take_turn(self):
        ''' Monster takes a turn, but moves randomly '''
        if self.num_turns > 0:  # Still confused...
            # Move in a random direction, and decrease the number of
            #   turns confused
            self.owner.move(libtcod.random_get_int(0, -1, 1),
                            libtcod.random_get_int(0, -1, 1))
            self.num_turns -= 1

        # Restore the previous AI
        #   (this one will be deleted because it's not referenced anymore)
        else:
            self.owner.ai = self.old_ai
            message('The ' + self.owner.name + ' is no longer confused!',
                    libtcod.red)

class TalkingMonster:
    ''' An AI that says things '''
    def __init__(self, speech, rate):
        self.speech = speech
        self.rate = rate

    def take_turn(self):
        ''' Monster takes a normal turn, but says something '''
        # A basic monster takes its turn. If you can see it, it can see you
        monster = self.owner

        # If monster is in FOV...
        if libtcod.map_is_in_fov(fov_map, monster.x, monster.y):

            # Move towards player if far away
            if monster.distance_to(player) >= 2:
                monster.move_towards(player.x, player.y)

            # Close enough, attack! (if the player is still alive.)
            elif player.fighter.hp > 0:
                monster.fighter.attack(player)

            # Depending on the rate of speech set in the json,
            #   the monster may talk
            # Rate must be a value from 0 - 99
            # The higher rate is, the less frequent the monster will talk
            if libtcod.random_get_int(0, 0, 100) > self.rate:
                # Say a random line
                r = libtcod.random_get_int(0, 0, len(self.speech)-1)
                message(''.join([monster.name.capitalize(), ' says ', '\'',
                        self.speech[r], '\'']), monster.color)

class Equipment:
    ''' An object that can be equipped, yielding bonuses.
    automatically adds the Item component. '''
    def __init__(self, slot, power_bonus=0, defense_bonus=0, max_hp_bonus=0,
                max_mana_bonus=0, attack_msg=None, weapon_func=None,
                ranged_bonus=0, short_name=None):
        self.power_bonus = power_bonus
        self.defense_bonus = defense_bonus
        self.max_hp_bonus = max_hp_bonus
        self.max_mana_bonus = max_mana_bonus
        self.ranged_bonus = ranged_bonus

        self.attack_msg = attack_msg
        self.weapon_func = weapon_func

        self.slot = slot
        self.is_equipped = False

        self.short_name = short_name

    def toggle_equip(self):
        ''' Toggle equip/dequip status '''
        if self.is_equipped:
            self.dequip()
        else:
            self.equip()

    def equip(self):
        ''' If the slot is already being used do nothing,
        except for dual weilding '''
        old_equipment = get_equipped_in_slot(self.slot)
        if old_equipment is not None:
            # If the item is to be equiped in the hands, find a
            #   free hand and equip it there
            # Essentially, this is dual weilding
            if self.slot == 'left hand' or self.slot == 'right hand':
                # Switch hands on equipment
                if self.slot == 'left hand':
                    self.slot = 'right hand'
                elif self.slot == 'right hand':
                    self.slot = 'left hand'

                message('You use your free hand to equip the ' +
                        self.owner.name)

                if self.attack_msg:
                    player.fighter.attack_msg = self.attack_msg
                else:
                    player.fighter.attack_msg = None

            # If both hands are full, dequip something or else the player
            #   somehow grows a new hand spontaneously
            if get_equipped_in_slot(self.slot) is not None:
                message(('But something is already there, so take off the ' + \
                        old_equipment.owner.name + ' for the ' +
                        self.owner.name), libtcod.light_red)

                # Switch the hands back on the equipment
                if self.slot == 'left hand':
                    self.slot = 'right hand'
                elif self.slot == 'right hand':
                    self.slot = 'left hand'

                old_equipment.dequip()

        # Equip object and show a message about it
        self.is_equipped = True
        message('Equipped ' + self.owner.name + ' on your ' + self.slot + '.',
                libtcod.light_green)

    def dequip(self):
        ''' Dequip object and show a message about it '''
        if not self.is_equipped: return
        self.is_equipped = False
        message('Dequipped ' + self.owner.name + ' from ' + self.slot + '.',
                libtcod.light_yellow)
        if self.slot == 'left hand':
            item = get_equipped_in_slot('right hand')
            if item != None:
                player.fighter.attack_msg = item.attack_msg
            else:
                player.fighter.attack_msg = 'punches'
        elif self.slot == 'right hand':
            item = get_equipped_in_slot('left hand')
            if item != None:
                player.fighter.attack_msg = item.attack_msg
            else:
                player.fighter.attack_msg = 'punches'

    def weapon_function(self):
        ''' Weapons have a special function that can be activated '''
        function = self.weapon_func
        if function is not None:
            function(self.owner)

class Fighter:
    ''' Combat-related properties and methods (monster, player, NPC) '''
    def __init__(self, hp, defense, power, xp, mana, death_function=None, attack_msg=None):
        self.base_max_hp = hp
        self.hp = hp
        self.base_defense = defense
        self.base_power = power
        self.xp = xp
        self.death_function = death_function
        self.mana = mana
        self.base_max_mana = mana
        self.attack_msg = attack_msg

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

    def take_damage(self, damage):
        ''' Harm self by certain amount of damage '''
        global kill_count
        # Apply damage if possible
        if damage > 0:
            self.hp -= damage

        # Check for death. if there's a death function, call it
        if self.hp <= 0:
            self.hp = 0
            function = self.death_function
            if function is not None:
                function(self.owner)
            # Yield experience to the player, take some mana
            #   and give some health
            if self.owner != player:
                player.fighter.xp += self.xp
                check_level_up()
                if activate_siphon:
                    player.fighter.siphon() # Try to siphon life
                kill_count += 1 # Increment kill count

    def attack(self, target):
        ''' A simple formula for attack damage '''
        damage = self.power - target.fighter.defense

        if damage > 0:
            # Make the target take some damage
            if self.attack_msg:
                message(' '.join([self.owner.name.capitalize(), self.attack_msg,
                        target.name.capitalize(), 'for', str(damage), 'hit points.']),
                        libtcod.red)
            else:
                message(' '.join([self.owner.name.capitalize(), 'attacks',
                        target.name.capitalize(), 'for', str(damage), 'hit points.']),
                        libtcod.red)
            target.fighter.take_damage(damage)
        else:
            message(' '.join([self.owner.name.capitalize(), 'attacks',
                target.name.capitalize(), 'but it has no effect!']),
                    libtcod.light_red)

    def heal(self, amount):
        ''' Heal by the given amount, without going over the maximum '''
        self.hp += amount
        if self.hp > self.max_hp:
            self.hp = self.max_hp

    def cast(self, cost):
        ''' Not used. Not sure what this can be used for in the future '''
        if self.mana - cost < 0:
            message('You don\'t have enough mana to cast this!', libtcod.red)
        else:
            self.mana -= cost

    def siphon(self):
        ''' Steal life. Sort of like a regeneration system '''
        if self.mana - SIPHON_COST < 0:
            message('You try to siphon any life away, but you aren\'t edgy enough',
                    libtcod.light_red)
            return 'cancelled'

        self.mana -= SIPHON_COST
        self.heal(SIPHON_AMOUNT)

        message('You siphon life from the deceased', libtcod.fuchsia)

    def magic_missile(self):
        ''' Fire a magic missile '''
        # Find closest monster
        monster = closest_monster(MISSILE_RANGE)
        if monster is None:  # No enemy found within maximum range
            message('No enemy is close enough to strike with your edge missile',
                    libtcod.light_red)
            return 'cancelled'

        # Fire a magic missile
        if self.mana - MISSILE_COST < 0:
            message('You try to fire an edge missile, but you aren\'t edgy enough',
                    libtcod.light_red)
            return 'cancelled'

        self.mana -= MISSILE_COST
        cast_magic_missile()

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
        if len(inventory) >= 26:
            message('Your inventory is full, cannot pick up ' +
                self.owner.name + '.', libtcod.dark_fuchsia)

        else:
            inventory.append(self.owner)
            objects.remove(self.owner)
            message('You picked up a ' + self.owner.name + '!',
                    libtcod.light_green)

            # Special case: automatically equip, if the corresponding equipment
            # slot is unused
            equipment = self.owner.equipment
            if equipment is not None:
                if (equipment and get_equipped_in_slot(equipment.slot) is None) or \
                (equipment.slot == 'right hand' or equipment.slot == 'left hand'):
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
        message('You dropped a ' + self.owner.name + '.', libtcod.yellow)

    def use(self):
        ''' Use an item '''
        # pecial case: if the object has the Equipment component, the 'use'
        #   action is to equip/dequip
        if self.owner.equipment:
            self.owner.equipment.toggle_equip()
            return

        # Just call the 'use_function' if it is defined
        if self.use_function is None:
            message('The ' + self.owner.name + ' cannot be used.', libtcod.gray)
        else:
            if self.use_function() != 'cancelled':
                # Destroy after use, unless it was cancelled for some reason
                inventory.remove(self.owner)

class Object:
    '''
    This is a generic object: the player, a monster, an item, the stairs...
    It's always represented by a character on screen
    '''
    def __init__(self, x, y, char, name, color, blocks=False,
                always_visible=False, fighter=None, ai=None, item=None,
                gold=None, equipment=None):
        self.always_visible = always_visible
        self.char = char
        self.name = name
        self.color = color
        self.blocks = blocks
        self.x = x
        self.y = y

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

    def clear(self):
        ''' Erase the character that represents this object '''
        (x, y) = to_camera_coordinates(self.x, self.y)
        if x is not None:
            libtcod.console_put_char(con, x, y, ' ', libtcod.BKGND_NONE)

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
        if ((libtcod.map_is_in_fov(fov_map, self.x, self.y)) or \
        (self.always_visible and world[self.x][self.y].explored)):
            (x, y) = to_camera_coordinates(self.x, self.y)

            if x is not None:
                # Set the color and then draw the character that
                #   represents this object at its position
                libtcod.console_set_default_foreground(con, self.color)
                libtcod.console_print_ex(con, x, y,
                                        libtcod.BKGND_NONE, libtcod.CENTER,
                                        self.char)

    def drop(self):
        ''' Add to the map and remove from the player's inventory.
        also, place it at the player's coordinates '''
        objects.append(self.owner)
        inventory.remove(self.owner)
        self.owner.x = player.x
        self.owner.y = player.y
        message('You dropped a ' + self.owner.name + '.', libtcod.yellow)

    def move(self, dx, dy):
        ''' Move by a given amount '''
        try:
            if self.name == player_name and WALL_HACK:
                self.x += dx
                self.y += dy
            elif not world[self.x + dx][self.y + dy].blocked and not \
            monster_occupy_check(self.x+dx, self.y+dy):
                self.x += dx
                self.y += dy
        except IndexError:
            pass

    def move_astar(self, tx, ty, player_move):
        ''' A* Algorithm for pathfinding towards target '''
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
        if not libtcod.path_is_empty(my_path) and \
        libtcod.path_size(my_path) < 25 \
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
        if not world[self.x + dx][self.y + dy].blocked and not \
        monster_occupy_check(self.x+dx, self.y+dy):
            self.move(dx, dy)
        # Otherwise if the space adjacent to the monster on the y axis is open
        #   go there
        elif not world[self.x][self.y + dy].blocked and \
        not monster_occupy_check(self.x, self.y + dy):
            self.move(0, dy)
        # Otherwise if the space adjacent to the monster on the x axis is open
        #   go there
        elif not world[self.x + dx][self.y].blocked and \
        not monster_occupy_check(self.x + dx, self.y):
            self.move(dx, 0)
        # Otherwise do nothing
        else:
            pass

    def send_to_back(self):
        # Make this object be drawn first, so all others appear
        #   above it if they're in the same tile.
        global objects
        objects.remove(self)
        objects.insert(0, self)

class Rect:
    ''' This will take top-left coordinates for a rectangle
    (in tiles, of course), and its size, to define it in terms of two points:
    top-left (x1, y1) and bottom-right (x2, y2) '''
    def __init__(self, x, y, w, h):
        self.x1 = x
        self.y1 = y
        self.x2 = x + w
        self.y2 = y + h

    def center(self):
        ''' Get center of rectangle '''
        center_x = (self.x1 + self.x2) / 2
        center_y = (self.y1 + self.y2) / 2
        return center_x, center_y

    def intersect(self, other):
        ''' Returns true if this rectangle intersects with another one '''
        return (self.x1 <= other.x2 and self.x2 >= other.x1 and
                self.y1 <= other.y2 and self.y2 >= other.y1)

class Tile:
    ''' A tile of the map and its properties '''
    def __init__(self, blocked, block_sight=None):
        self.blocked = blocked

        if FOG_OF_WAR_ENABLED:
            self.explored = False
        else:
            self.explored = True

        # By default, if a tile is blocked, it also blocks sight
        if block_sight is None:
            block_sight = blocked
        self.block_sight = block_sight

######################################
# Objects
######################################

# Player object
player = None
player_name = None

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

# Check FOV if true
check_fov = True

# Stairs objects
dstairs = None
ustairs = None

######################################
# GUI Objects
######################################

panel = libtcod.console_new(PANEL_WIDTH, SCREEN_HEIGHT)
msg_panel = libtcod.console_new(SCREEN_WIDTH, PANEL_HEIGHT)

######################################
# Tcod and Init
######################################

# Font
libtcod.console_set_custom_font('images/terminal8x12_gs_tc.png',
    libtcod.FONT_TYPE_GREYSCALE | libtcod.FONT_LAYOUT_TCOD)

# Initialize root console
libtcod.console_init_root(SCREEN_WIDTH, SCREEN_HEIGHT,
                            'Edgequest Pre-Alpha', False)

# And another
con = libtcod.console_new(MAP_WIDTH, MAP_HEIGHT)

# And one for a player-centered focus
dcon = libtcod.console_new(SCREEN_WIDTH, SCREEN_HEIGHT)

# FPS Limit
libtcod.sys_set_fps(LIMIT_FPS)

# Mouse and Keyboard detection
mouse = libtcod.Mouse()
key = libtcod.Key()

# Camera coordinates
(camera_x, camera_y) = (0, 0)

######################################
# Functions
######################################

def cast_confuse():
    ''' Ask the player for a target to confuse '''
    message('Left-click an enemy to confuse it, or right-click to cancel.',
            libtcod.light_cyan)
    monster = target_monster(CONFUSE_RANGE)
    if monster is None: return 'cancelled'

    # Replace the monster's AI with a 'confused' one; after some turns it will
    #   restore the old AI
    old_ai = monster.ai
    monster.ai = ConfusedMonster(old_ai)
    monster.ai.owner = monster  # Tell the new component who owns it
    message('The eyes of the ' + monster.name +
            ' look vacant, as he starts to stumble around!',
            libtcod.light_green)

    render_all()
    # Present the root console
    libtcod.console_flush()

def cast_fireball():
    ''' Ask the player for a target tile to throw a fireball at '''
    message('Left-click a target tile for the fireball, or right-click to cancel.',
            libtcod.light_cyan)
    (x, y) = target_tile()
    if x is None: return 'cancelled'
    message('The fireball explodes, burning everything within ' +
        str(FIREBALL_RADIUS) + ' tiles!', libtcod.orange)

    for obj in objects:  # Damage every fighter in range, including the player
        if obj.distance(x, y) <= FIREBALL_RADIUS and obj.fighter:
            message('The ' + obj.name + ' gets burned for ' +
                    str(FIREBALL_DAMAGE) + ' hit points.', libtcod.orange)
            obj.fighter.take_damage(FIREBALL_DAMAGE)

    # This is literally magical I still have no idea how it works
    (x, y) = to_camera_coordinates(x, y)
    # Really bad animation
    libtcod.console_set_default_foreground(con, libtcod.red)
    for i in range(FIREBALL_RADIUS):
        libtcod.console_put_char(con, x, y, '4', libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x+i, y, '4', libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x-i, y, '4', libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x, y+i, '4', libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x, y-i, '4', libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x+i, y+i, '4', libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x-i, y-i, '4', libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x+i, y-i, '4', libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x-i, y+i, '4', libtcod.BKGND_NONE)

    libtcod.console_flush()

def cast_heal():
    ''' Heal the player '''
    if player.fighter.hp == player.fighter.max_hp:
        message('You are already at full health.', libtcod.light_gray)
        return 'cancelled'

    message('Your wounds start to feel better!', libtcod.light_violet)
    player.fighter.heal(HEAL_AMOUNT)

def cast_inflict_blindness():
    ''' Inflict blindness. Basically just limit what gets rendered '''
    global blind, blind_counter
    blind = True
    blind_counter = 0
    message("You are blinded!", libtcod.dark_sea)

def cast_mana():
    ''' Give some mana back '''
    if player.fighter.mana == player.fighter.max_mana:
        message('You already have enough edge.')
        return 'cancelled'

    message('You begin to feel edgy!', libtcod.light_flame)
    player.fighter.restore(MANA_AMOUNT)

def cast_magic_missile():
    ''' Find closest enemy (inside a maximum range) and damage it
    assumes that you already have a monster in range '''
    monster = closest_monster(MISSILE_RANGE)

    # Zap it!
    message('A missile of pure edge strikes the ' + monster.name +
            ' with a loud airhorn! The damage is ' + str(MISSILE_DAMAGE) +
            ' hit points.', libtcod.light_blue)

    monster.fighter.take_damage(MISSILE_DAMAGE)

    # Animation test, courtesy of Trash Animation Studios(tm)
    dx = player.x
    dy = player.y
    # The one cool this is that the lightning bolt changes depending on where
    # the monster is
    if (dx < monster.x and dy < monster.y) or \
    (dx > monster.x and dy > monster.y):
        char = '\\'
    elif (dx < monster.x and dy > monster.y) or \
    (dx > monster.x and dy < monster.y):
        char = '/'
    elif (dx == monster.x and dy != monster.y):
        char = '|'
    elif (dx != monster.x and dy == monster.y):
        char = '-'
    else:
        char = 'z'

    # While, the distance to the monster is greater than 2
    # Aka go towards it until it's one space away
    while  math.sqrt((monster.x-dx) ** 2 + (monster.y-dy) ** 2) >= 2:
        # First, try to move towards monster by row
        if monster.x == dx:
            pass
        elif monster.x < dx:
            dx += -1
        elif monster.x > dx:
            dx += 1

        # Second, try to move towards player by column
        if monster.y == dy:
            pass
        elif monster.y < dy:
            dy += -1
        elif monster.y > dy:
            dy += 1

        (x, y) = to_camera_coordinates(dx, dy)
        libtcod.console_set_default_foreground(con, libtcod.light_purple)
        libtcod.console_put_char(con, x, y, char, libtcod.BKGND_NONE)

    libtcod.console_flush()

def cast_lightning():
    ''' Find closest enemy (inside a maximum range) and damage it '''
    monster = closest_monster(LIGHTNING_RANGE)
    if monster is None:  # No enemy found within maximum range
        message('No enemy is close enough to strike.', libtcod.red)
        return 'cancelled'

    # Zap it!
    message('A lighting bolt strikes the ' + monster.name +
            ' with a loud thunder! The damage is ' + str(LIGHTNING_DAMAGE) +
            ' hit points.', libtcod.light_blue)
    monster.fighter.take_damage(LIGHTNING_DAMAGE)

    # Animation test, courtesy of Trash Animation Studios(tm)
    dx = player.x
    dy = player.y
    # The one cool this is that the lightning bolt changes depending on where
    # the monster is
    if (dx < monster.x and dy < monster.y) or \
    (dx > monster.x and dy > monster.y):
        char = '\\'
    elif (dx < monster.x and dy > monster.y) or \
    (dx > monster.x and dy < monster.y):
        char = '/'
    elif (dx == monster.x and dy != monster.y):
        char = '|'
    elif (dx != monster.x and dy == monster.y):
        char = '-'
    else:
        char = 'z'

    while (dx, dy) != (monster.x, monster.y):
        libtcod.console_flush()
        # First, try to move towards monster by row
        if monster.x == dx:
            pass
        elif monster.x < dx:
            dx += -1
        elif monster.x > dx:
            dx += 1

        # Second, try to move towards player by column
        if monster.y == dy:
            pass
        elif monster.y < dy:
            dy += -1
        elif monster.y > dy:
            dy += 1

        (x, y) = to_camera_coordinates(dx, dy)
        libtcod.console_set_default_foreground(con, libtcod.light_azure)
        libtcod.console_put_char(con, x, y, char, libtcod.BKGND_NONE)

    libtcod.console_flush()

def check_ground():
    for obj in objects:  # Look for an item in the player's tile
        if obj.x == player.x and obj.y == player.y and obj != player:
            message(' '.join(['You see a', obj.name, 'here.']),
                    libtcod.white)

def check_level_up():
    ''' See if the player's experience is enough to level-up '''
    level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR
    if player.fighter.xp >= level_up_xp:

        render_all()
        # Present the root console
        libtcod.console_flush()

        # It is! level up
        player.level += 1
        player.fighter.xp -= level_up_xp
        message('Your battle skills grow stronger! You reached level ' +
            str(player.level) + '!', libtcod.yellow)

        choice = None
        while choice == None:  # Keep asking until a choice is made
            choice = menu('Level up! Choose a stat to raise:\n',
                ['Constitution (+20 HP, from ' + str(player.fighter.max_hp) +
                ')',
                'Strength (+1 attack, from ' + str(player.fighter.power) +
                ')',
                'Agility (+1 defense, from ' + str(player.fighter.defense) +
                ')'], LEVEL_SCREEN_WIDTH)

        if choice == 0:
            player.fighter.max_hp += 20
            player.fighter.hp += 20
        elif choice == 1:
            player.fighter.power += 1
        elif choice == 2:
            player.fighter.defense += 1

        # Pause
        libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS, key, mouse)

def check_timer():
    global timer
    # Timer based commands

    # Regenerate health
    if player.fighter.hp != player.fighter.max_hp:
        if timer % 8 == 0:
            player.fighter.heal(1)
            timer += 1

def choose_name():
    ''' Choose a name for the hero '''
    global player_name

    key = libtcod.Key()
    name = ''

    # Set the screen to black
    libtcod.console_set_default_background(con, libtcod.black)

    # Set text color to yellow
    libtcod.console_set_default_foreground(con, libtcod.light_yellow)

    # Dispbox style key getting
    while not libtcod.console_is_window_closed():
        # Limit FPS
        time.sleep(1/LIMIT_FPS*2)

        # Check for keypresses
        if libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS, key, mouse):

            key_char = chr(key.c)
            if key.vk in FULLSCREEN_KEYS:
                libtcod.console_set_fullscreen(not \
                                                libtcod.console_is_fullscreen())
            # Enter submits name
            elif key.vk == libtcod.KEY_ENTER:
                break
            # Backspace deletes line
            elif key.vk == libtcod.KEY_BACKSPACE:
                if len(name) == 1:
                    name = ''
                else:
                    name = name[:-1]
            # Shift causes a problem in libtcod so make sure nothing happens if
            #   pressed
            elif key.vk == libtcod.KEY_SHIFT:
                pass
            # Add char to string
            elif key_char:
                name = ''.join([name, key_char])

        # Clear screen
        libtcod.console_clear(con)

        # Prompt for name
        libtcod.console_print_ex(con, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 4,
                                libtcod.BKGND_NONE, libtcod.CENTER,
                                'Choose a name for the hero')

        # Blit to screen
        libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)

        # Show name
        dispbox('\n' + name + '\n', len(name))

        # Present the root console
        libtcod.console_flush()

    # In case if the name isn't anything
    if name == '':
        name = 'player'

    player_name = name.capitalize()

def closest_monster(max_range):
    # Find closest enemy, up to a maximum range, and in the player's FOV
    closest_enemy = None
    # Start with (slightly more than) maximum range
    closest_dist = max_range + 1

    for obj in objects:
        if obj.fighter and not obj == player and \
        libtcod.map_is_in_fov(fov_map, obj.x, obj.y):
            # Calculate distance between this obj and the player
            dist = player.distance_to(obj)
            if dist < closest_dist:  # It's closer, so remember it
                closest_enemy = obj
                closest_dist = dist
    return closest_enemy

def debug_spawn_console(json_list):
    ''' Spawn a mini-console to spawn-in monsters or items '''
    # Needs to have JSON data
    if json_list not in ['monster', 'item']:
        raise Exception('NoDataForObject')

    # Message displaying what will be spawned
    if json_list == 'monster':
        message('Enter a monster name', libtcod.red)
    elif json_list == 'item':
        message('Enter an item name', libtcod.red)

    # Show new message
    render_all()
    libtcod.console_flush()

    key = libtcod.Key()
    name = ''
    check = True

    # Loop to show input from player
    while not libtcod.console_is_window_closed():

        # This loop has a tendency to eat all the cpu
        time.sleep(1/LIMIT_FPS*2)

        # Render before drawing a new dispbox
        render_all()

        # Check for keypresses
        if libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS, key, mouse):
            # Enter submits name
            key_char = chr(key.c)
            if key.vk == libtcod.KEY_ENTER:
                break
            elif key.vk == libtcod.KEY_F4:
                libtcod.console_set_fullscreen(not \
                                                libtcod.console_is_fullscreen())
            # Backspace deletes character
            elif key.vk == libtcod.KEY_BACKSPACE:
                if len(name) == 1:
                    name = ''
                else:
                    name = name[:-1]
            # Esc quits
            elif key.vk == libtcod.KEY_ESCAPE:
                check = False
                break
            elif key.vk == libtcod.KEY_SHIFT:
                pass
            elif key_char != '':
                name = ''.join([name, key_char])

            dispbox('\n' + name + '\n', len(name))

    # Names have the ability to not exist, considering player is giving input
    found = False
    if json_list == 'monster' and check:
        for mon in monster_data:
            if monster_data[mon]['name'] == name or \
            monster_data[mon]['id'] == name:
                obj = generate_monster(monster_data[mon]['id'], player.x+2,
                                        player.y)
                # Add monster to object list
                objects.append(obj)
                message('Spawned a ' + name)
                found = True
    elif json_list == 'item' and check:
        for item in items_data:
            if items_data[item]['name'] == name or \
            items_data[item]['id'] == name:
                obj = generate_item(items_data[item]['id'], player.x,
                                    player.y)

                # Add item to object list
                objects.append(obj)
                message('Spawned a ' + name)
                found = True

    if not found and check:
        message('Failed to find a ' + name)

def debug_kill_all():
    ''' Kill everything with an ai '''
    for obj in objects:
        if obj.ai:
            obj.fighter.take_damage(sys.maxint)

def dispbox(header, width=50):
    ''' Like a msgbox but only lasts for one frame '''
    # Calculate total height for the header
    height = libtcod.console_get_height_rect(con, 0, 0, width, SCREEN_HEIGHT,
                                            header)

    # Create an off-screen console that represents the menu's window
    window = libtcod.console_new(width, height)

    # Print the header, with auto-wrap
    libtcod.console_set_default_foreground(window, libtcod.white)
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
    if len(inventory) == 0:
        options = ['Inventory is empty.']
    else:
        options = []
        sort_inventory()
        for item in inventory:
            # Only get equipment
            if item.equipment:
                text = item.name
                if item.equipment.is_equipped:
                    text = text + ' (on ' + item.equipment.slot + ')'
                options.append(text)

    if len(options) == 0:
        options = ['No equipment']

    index = menu(header, options, INVENTORY_WIDTH)

    # If an item was chosen, return it
    if index is None or len(inventory) == 0:
        return None
    return inventory[index].item

def fire_weapon(equipment):
    ''' Find closest enemy and shoot it '''
    monster = closest_monster(FIREARM_RANGE)
    if monster is None:  # No enemy found within maximum range
        message('No enemy is close enough to shoot.', libtcod.red)
        return 'cancelled'

    damage = equipment.ranged_bonus - monster.fighter.defense

    if damage > 0:

        # Zap it!
        message(player_name + ' shoots the ' + monster.name +
                ' with the ' + equipment.owner.name + '! The damage is ' +
                str(damage) +
                ' hit points.', libtcod.light_red)
        monster.fighter.take_damage(damage)

    else:

        message(player_name + ' shoots the ' + monster.name +
            ' with the ' + equipment.owner.name +
            'but the shot reflects off the armor!', libtcod.light_red)

    # Animation test, courtesy of Trash Animation Studios(tm)
    dx = player.x
    dy = player.y
    # The one cool this is that the lightning bolt changes depending on where
    # the monster is
    if (dx < monster.x and dy < monster.y) or \
    (dx > monster.x and dy > monster.y):
        char = '\\'
    elif (dx < monster.x and dy > monster.y) or \
    (dx > monster.x and dy < monster.y):
        char = '/'
    elif (dx == monster.x and dy != monster.y):
        char = '|'
    elif (dx != monster.x and dy == monster.y):
        char = '-'
    else:
        char = 'z'

    while (dx, dy) != (monster.x, monster.y):
        libtcod.console_flush()
        # First, try to move towards monster by row
        if monster.x == dx:
            pass
        elif monster.x < dx:
            dx += -1
        elif monster.x > dx:
            dx += 1

        # Second, try to move towards player by column
        if monster.y == dy:
            pass
        elif monster.y < dy:
            dy += -1
        elif monster.y > dy:
            dy += 1

        (x, y) = to_camera_coordinates(dx, dy)
        libtcod.console_set_default_foreground(con, libtcod.yellow)
        libtcod.console_put_char(con, x, y, char, libtcod.BKGND_NONE)

    libtcod.console_flush()

def fov_recompute():
    ''' Recompute fov '''
    global world

    move_camera(player.x, player.y)

    # Recompute FOV if needed (the player moved or something)
    libtcod.map_compute_fov(fov_map, player.x, player.y, TORCH_RADIUS,
                            FOV_LIGHT_WALLS, FOV_ALGO)
    libtcod.console_clear(con)

    # Go through all tiles, and set their background color according to the FOV
    for y in range(CAMERA_HEIGHT):
        for x in range(CAMERA_WIDTH):
            (map_x, map_y) = (camera_x + x, camera_y + y)
            visible = libtcod.map_is_in_fov(fov_map, map_x, map_y)

            wall = world[map_x][map_y].block_sight

            if not visible:
                # if it's not visible right now, the player can only see it
                #   if it's explored
                if world[map_x][map_y].explored:
                    # It's out of the player's FOV
                    if wall:
                        c = wallselect(world, map_x, map_y)
                        libtcod.console_set_char_background(con, x, y,
                                                    color_dark_wall,
                                                    libtcod.BKGND_SET)
                    else:
                        libtcod.console_set_char_background(con, x, y,
                                                color_dark_ground,
                                                libtcod.BKGND_SET)
            else:
                # It's visible
                if wall:
                    c = wallselect(world, map_x, map_y)
                    libtcod.console_put_char_ex(con, x, y, c,
                                                libtcod.white,
                                                libtcod.light_gray)
                else:
                    libtcod.console_set_char_background(con, x, y, libtcod.gray,
                                            libtcod.BKGND_SET)
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

    # Show some stats and stuff
    msgbox('You Died!\n\n \
    Level: ' + str(player.level) + '\n \
    Floor: ' + str(dungeon_level) + '\n \
    Experience: ' + str(player.fighter.xp) + '\n \
    Maximum HP: ' + str(player.fighter.max_hp) + '\n \
    Attack: ' + str(player.fighter.power) + '\n \
    Defense: ' + str(player.fighter.defense) + '\n \
    Total Kills: ' + str(kill_count) + '\n\n \
    Press any key to continue...',
    CHARACTER_SCREEN_WIDTH)

    exit()

def game_win():
    ''' Win the game! '''

    # Show some stats and stuff
    msgbox('You Win!\n\n \
    Level: ' + str(player.level) + '\n \
    Floor: ' + str(dungeon_level) + '\n \
    Experience: ' + str(player.fighter.xp) + '\n \
    Maximum HP: ' + str(player.fighter.max_hp) + '\n \
    Attack: ' + str(player.fighter.power) + '\n \
    Defense: ' + str(player.fighter.defense) + '\n \
    Total Kills: ' + str(kill_count) + '\n\n \
    Press any key to continue...',
    CHARACTER_SCREEN_WIDTH)

    exit()

def generate_monster(monster_id, x, y):
    ''' Generate monster from json '''
    # Read color
    color = json_get_color(monster_data[monster_id]['color'])

    # Select a death function
    if monster_data[monster_id]['death_func'] == 'normal':
        death = monster_death
    elif monster_data[monster_id]['death_func'] == 'slock':
        death = monster_death_slock
    elif monster_data[monster_id]['death_func'] == 'talk':
        death = monster_death_talk
    else:
        death = monster_death

    # Select an AI
    if monster_data[monster_id]['ai'] == 'normal':
        ai = BasicMonster()
    elif monster_data[monster_id]['ai'] == 'talk':
        ai = TalkingMonster(monster_data[monster_id]['speech'],
                            monster_data[monster_id]['rate'])
    else:
        print('Error: ai does not exist')
        exit()

    '''
    Example:
    # Create an orc
    fighter_component = Fighter(hp=20, defense=0, power=4, xp=35,
                                death_function=monster_death)
    ai_component = BasicMonster()
    monster = Object(x, y, 'o', 'orc', libtcod.desaturated_green,
        blocks=True, fighter=fighter_component, ai=ai_component)
    '''

    fighter_component = Fighter(hp=int(monster_data[monster_id]['hp']),
                            defense=int(monster_data[monster_id]['defense']),
                            power=int(monster_data[monster_id]['power']),
                            xp=int(monster_data[monster_id]['xp']),
                            mana=int(monster_data[monster_id]['mana']),
                            death_function=death,
                            attack_msg=monster_data[monster_id]['attack_msg'])

    monster = Object(x, y, monster_data[monster_id]['char'],
                    monster_data[monster_id]['name'], color, blocks=True,
                    fighter = fighter_component, ai=ai)

    return monster

def generate_item(item_id, x, y):
    ''' Generate items from json '''
    color = json_get_color(items_data[item_id]['color'])

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

    if items_data[item_id]['type'] == 'item':

        if items_data[item_id]['effect'] == 'heal':
            item_component = Item(use_function=cast_heal)
        elif items_data[item_id]['effect'] == 'fireball':
            item_component = Item(use_function=cast_fireball)
        elif items_data[item_id]['effect'] == 'confuse':
            item_component = Item(use_function=cast_confuse)
        elif items_data[item_id]['effect'] == 'lightning':
            item_component = Item(use_function=cast_lightning)
        elif items_data[item_id]['effect'] == 'mana':
            item_component = Item(use_function=cast_mana)

        item = Object(x, y, items_data[item_id]['char'],
                        items_data[item_id]['name'], color, item=item_component)

    elif items_data[item_id]['type'] in ('equipment', 'firearm'):

        if items_data[item_id]['subtype'] == 'weapon':
            if items_data[item_id]['weapon_func'] == 'knife':
                func = weapon_action_knife
            elif items_data[item_id]['weapon_func'] == 'katana':
                func = weapon_action_katana
            elif items_data[item_id]['weapon_func'] == 'awp':
                func = weapon_action_awp
            else:
                func = weapon_action_else
            equip_component = Equipment(slot=items_data[item_id]['slot'],
                            power_bonus=items_data[item_id]['power'],
                            defense_bonus=items_data[item_id]['defense'],
                            max_hp_bonus=items_data[item_id]['hp'],
                            max_mana_bonus=items_data[item_id]['mana'],
                            attack_msg=items_data[item_id]['attack_msg'],
                            weapon_func=func,
                            short_name=items_data[item_id]['short_name'])
        elif items_data[item_id]['subtype'] == 'firearm':
            if items_data[item_id]['weapon_func'] == 'firearm':
                func = weapon_action_firearm
            else:
                func = weapon_action_else
            equip_component = Equipment(slot=items_data[item_id]['slot'],
                            power_bonus=items_data[item_id]['power'],
                            defense_bonus=items_data[item_id]['defense'],
                            max_hp_bonus=items_data[item_id]['hp'],
                            max_mana_bonus=items_data[item_id]['mana'],
                            attack_msg=items_data[item_id]['attack_msg'],
                            weapon_func=func,
                            ranged_bonus=items_data[item_id]['ranged'],
                            short_name=items_data[item_id]['short_name'])
        elif items_data[item_id]['subtype'] == 'armor':
            equip_component = Equipment(slot=items_data[item_id]['slot'],
                            power_bonus=items_data[item_id]['power'],
                            defense_bonus=items_data[item_id]['defense'],
                            max_hp_bonus=items_data[item_id]['hp'],
                            max_mana_bonus=items_data[item_id]['mana'],
                            short_name=items_data[item_id]['short_name'])

        item = Object(x, y, items_data[item_id]['char'],
                        items_data[item_id]['name'], color,
                        equipment=equip_component)

    elif items_data[item_id]['type'] == 'gold':
        item = Object(x, y, items_data[item_id]['char'],
                        items_data[item_id]['name'], color)

    return item

def get_equipped_in_slot(slot):
    ''' Returns the equipment in a slot, or None if it's empty '''
    for obj in inventory:
        if obj.equipment and obj.equipment.slot == \
        slot and obj.equipment.is_equipped:
            return obj.equipment
    return None

def get_all_equipped(obj):
    ''' Returns a list of equipped items '''
    if obj == player:
        equipped_list = []
        for item in inventory:
            if item.equipment and item.equipment.is_equipped:
                equipped_list.append(item.equipment)
        return equipped_list
    else:
        return []  #other objects have no equipment

def get_names_under_mouse():
    ''' Self explanatory name '''
    global mouse

    # Return a string with the names of all objects under the mouses
    # From screen to map coordinates
    (x, y) = (camera_x + mouse.cx, camera_y + mouse.cy)

    # Create a list with the names of all objects at the mouse's
    #   coordinates and in FOV
    names = [obj.name for obj in objects
             if obj.x == x and obj.y == y]

    names = ', '.join(names)  # Join the names, separated by commas

    # Read Coords. Debug
    if COORDS_UNDER_MOUSE:
        names += '( ' + str(x) + ', ' + str(y) + ' )'

    if names:
        return '['+names.capitalize()+']'
    else:
        return ''

def git_screen():
    # Clear screen
    libtcod.console_clear(con)

    # Set the screen to black
    libtcod.console_set_default_background(con, libtcod.black)

    libtcod.console_set_default_foreground(con, libtcod.light_yellow)
    libtcod.console_print_ex(con, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 4,
                            libtcod.BKGND_NONE, libtcod.CENTER,
                            'Thank you for playing EdgeQuest')

    # Blit to screen
    libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)

    msgbox(
    'Make sure to check the latest master version on github periodically!\n\n' +
    'Press any key to continue...',
    40)

def handle_keys():
    ''' Handle keypresses sent to the console. Executes other things,
    makes game playable '''
    global check_fov, game_state, objects, player_action, key, timer

    # F4 for Fullscreen
    if key.vk in FULLSCREEN_KEYS:
        libtcod.console_set_fullscreen(not \
                                        libtcod.console_is_fullscreen())

    if game_state == 'playing':
        # End game with escape
        if key.vk == libtcod.KEY_ESCAPE:
            player_action = 'didnt-take-turn'
            save_game()

        key_char = chr(key.c)

        # Movement keys
        if key_char in movement_keys or key.vk in (libtcod.KEY_UP,
                                                    libtcod.KEY_DOWN,
                                                    libtcod.KEY_LEFT,
                                                    libtcod.KEY_RIGHT):
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
            check_fov = True

            check_ground()

            player_action = 'move'

        # Wait
        elif key_char in ('5', '.'):
            check_fov = True
            message('You wait', libtcod.gray)
            player_action = 'wait'

        elif key_char == 'g':
            # Pick up an item
            for obj in objects:  # Look for an item in the player's tile
                if obj.x == player.x and obj.y == player.y and obj.item:
                    obj.item.pick_up()
                    player_action = 'pickup'
                    break
            else:
                message('There is nothing there to pick up', libtcod.gray)
                player_action = 'didnt-take-turn'

        elif key_char == 'i':
            # Show the inventory
            chosen_item = inventory_menu(
            'Press the key next to an item to use it, or any other to cancel.\
            \n')
            if chosen_item is not None:
                chosen_item.use()
                player_action = 'use'

        elif key_char == 'e':
            # Show equipment
            chosen_item = equipment_menu(
            'Press the key next to an item to equip/dequip it, or any other to cancel.\
            \n')
            if chosen_item is not None:
                chosen_item.use()
                player_action = 'use'

        elif key_char == 'd':
            # Show the inventory; if an item is selected, drop it
            chosen_item = inventory_menu(
            'Press the key next to an item to drop it, or any other to cancel.\
            \n')
            if chosen_item is not None:
                chosen_item.drop()
                player_action = 'drop'

        # Reset the map (DEBUG)
        elif key_char == 'r':
            # Empty objects and re-add the player so the game is playable
            objects = []
            objects.insert(0, player)

            # Clear screen
            for x in range(SCREEN_WIDTH):
                for y in range(SCREEN_HEIGHT):
                    libtcod.console_put_char(con, x, y, ' ', libtcod.BKGND_BURN)

            # Make a new map
            make_map()
            fov_recompute()
            player_action = 'didnt-take-turn'

        elif key_char == '>':
            # Go down stairs, if the player is on them
            if (dstairs.x == player.x and dstairs.y == player.y) or STAIR_HACK:
                next_level()

        elif key_char == '<':
            # Go up stairs, if the player is on them
            if (ustairs.x == player.x and ustairs.y == player.y) or STAIR_HACK:
                previous_level()

        elif key_char == 'c':
            # Show character information
            level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR
            msgbox('Character Information\n\nLevel: ' + str(player.level) +
                    '\nFloor: ' + str(dungeon_level) +
                    '\n\nExperience: ' + str(player.fighter.xp) +
                    '\nExperience to level up: ' + str(level_up_xp) +
                    '\n\nMaximum HP: ' + str(player.fighter.max_hp) +
                    '\nAttack: ' + str(player.fighter.power) +
                    '\nDefense: ' + str(player.fighter.defense) +
                    '\nKillstreak: ' + str(kill_count),
                    CHARACTER_SCREEN_WIDTH)

        elif key_char == 'q':
            # Toggle the siphon ability
            toggle_siphon()
            player_action = 'didnt-take-turn'

        elif key_char == 't':
            # Taunt
            taunt()
            player_action = 'taunt'

        elif key_char == 'f':
            # Activate weapon
            right = get_equipped_in_slot('right hand')
            left = get_equipped_in_slot('left hand')
            if left:
                left.weapon_function()
                player_action = 'activating'

            if right:
                right.weapon_function()
                player_action = 'activating'

        elif key_char == 'm':
            status = player.fighter.magic_missile()
            if status != 'cancelled':
                player_action = 'casting'
            else:
                player_action = 'didnt-take-turn'

        elif key_char == '?':
            how_to_play()
            player_action = 'didnt-take-turn'

        # Debug commands

        elif key_char == 'z':
            debug_spawn_console('monster')
            player_action = 'didnt-take-turn'

        elif key_char == 'x':
            debug_spawn_console('item')
            player_action = 'didnt-take-turn'

        elif key_char == 'v':
            debug_kill_all()
            player_action = 'didnt-take-turn'

        elif key_char == 'o':
            objects.append(generate_monster('fleck', player.x, player.y + 2))

        else:
            player_action = 'didnt-take-turn'

def how_to_play():
    msgbox(
    'How To Play\n\n \
    Numpad/Arrowkeys/Vim keys: Move \n \
    Click: Move to spot \n \
    . - Wait \n \
    i - Open Inventory \n \
    e - Open Equipment\n \
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
    z - Spawn monster console\n \
    x - Spawn item console\n \
    v - Kill all on level\n\n \
    Press any key to continue...',
    CHARACTER_SCREEN_WIDTH)

def initialize_fov():
    ''' Initialize the fov '''
    global check_fov, fov_map
    check_fov = True

    # Create the FOV map, according to the generated map
    fov_map = libtcod.map_new(MAP_WIDTH, MAP_HEIGHT)
    for y in range(MAP_HEIGHT):
        for x in range(MAP_WIDTH):
            libtcod.map_set_properties(fov_map, x, y,
                                        not world[x][y].block_sight,
                                        not world[x][y].blocked)

def intro_cutscene():
    ''' Show a cutscene '''
    # Text to be displayed in the intro
    intro = [
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
    buff = ['' for x in range(SCREEN_HEIGHT)]

    intro_wall = buff + intro

    # Set Colors
    libtcod.console_set_default_background(con, libtcod.black)
    libtcod.console_set_default_foreground(con, libtcod.light_yellow)

    key = libtcod.Key()
    # We take the y and subtract it from the y val so that the text moves up
    #   the screen
    for y in range(len(intro_wall)+1):
        # Able to break in the middle of the cutscene
        if libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS, key, mouse):
            if key.vk in FULLSCREEN_KEYS:
                libtcod.console_set_fullscreen(not \
                                                libtcod.console_is_fullscreen())
            elif key.vk == libtcod.KEY_ENTER:
                break

        if libtcod.console_is_window_closed():
            exit()

        libtcod.console_clear(con)
        # Draw the wall at the y coord
        for i, line in enumerate(intro_wall):
            libtcod.console_print_ex(con, SCREEN_WIDTH / 2, i-y,
                                    libtcod.BKGND_NONE, libtcod.CENTER,
                                    line)
        libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)
        libtcod.console_flush()
        time.sleep(.75)

def inventory_menu(header):
    ''' Show a menu with each item of the inventory as an option '''
    if len(inventory) == 0:
        options = ['Inventory is empty.']
    else:
        options = []
        sort_inventory()
        for item in inventory:
            text = item.name
            #show additional information, in case it's equipped
            if item.equipment and item.equipment.is_equipped:
                text = text + ' (on ' + item.equipment.slot + ')'
            options.append(text)

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
    ''' Get translate json color string into libtcod colors '''
    colors = {
        'desaturated_green': libtcod.desaturated_green,
        'darker_green': libtcod.darker_green,
        'lime': libtcod.lime,
        'violet': libtcod.violet,
        'light_yellow': libtcod.light_yellow,
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
        'red': libtcod.red
    }

    return colors[color_str]

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
    img = libtcod.image_load('images/menu_background.png')

    git_screen()

    while not libtcod.console_is_window_closed():
        # Show the background image, at twice the regular console resolution
        libtcod.image_blit_2x(img, 0, 0, 0)

        # Show the game's title, and some credits!
        libtcod.console_set_default_foreground(0, libtcod.light_yellow)
        libtcod.console_print_ex(0, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 4,
                                libtcod.BKGND_NONE, libtcod.CENTER,
                                'Edgequest')
        libtcod.console_print_ex(0, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 + 4,
                                libtcod.BKGND_NONE, libtcod.CENTER,
                                'What hath God wrought?')

        libtcod.console_set_default_foreground(0, libtcod.black)
        libtcod.console_print_ex(0, SCREEN_WIDTH / 2, SCREEN_HEIGHT - 2,
                                libtcod.BKGND_NONE, libtcod.CENTER, 'By Gray')

        # Show options and wait for the player's choice
        choice = menu('Options', ['Play a new game', 'Continue last game',
                        'How to play', 'Quit'], 24)

        if choice == 0:  # New game
            intro_cutscene()
            choose_name()
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
        elif choice == 3:  # Quit
            exit()

def make_map():
    ''' Make a map '''
    global world, fov_map, objects, dstairs, ustairs

    # The list of objects with just the player
    objects = [player]

    # fill map with 'blocked' tiles
    world = [[Tile(True) for y in range(MAP_HEIGHT)] for x in range(MAP_WIDTH)]

    rooms = []
    num_rooms = 0

    # Rev up those map generators
    themap = dMap()

    '''
    Okay this takes some magic to get working but once you do you can create
    a ton of cool maps with it.

    the first two values are the dimensions of the map. The second one is the
    'fail' rating.
    Not sure what the heck that means but the higher it is, the more rooms you
    get.
    Then the fourth is the 'b1' value. What's a b1? No idea.
    Apparently it controlls the frequency of corridors.
    I don't like corridors so I keep it at 1.
    Lastly, the number of maximum rooms. Multiply the max_rooms by 4 because
    the rooms are pretty.
    '''
    themap.makeMap(MAP_WIDTH,MAP_HEIGHT-2,250,1,MAX_ROOMS*4)

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

    for y in range(MAP_HEIGHT):
        for x in range(MAP_WIDTH):
            libtcod.map_set_properties(fov_map, x, y,
                                        not world[x][y].block_sight,
                                        not world[x][y].blocked)

    # Create stairs at some random ass location
    x = libtcod.random_get_int(0,0, MAP_WIDTH-1)
    y = libtcod.random_get_int(0,0, MAP_HEIGHT-1)

    while (world[x][y].blocked):
        x = libtcod.random_get_int(0,0, MAP_WIDTH-1)
        y = libtcod.random_get_int(0,0, MAP_HEIGHT-1)

    dstairs = Object(x, y, '>', 'down stairs', libtcod.white,
                    always_visible=True)
    objects.append(dstairs)
    # This tends to cause issues in the later levels
    dstairs.send_to_back()  # So it's drawn below the monsters


    # Same for player
    x = libtcod.random_get_int(0,0, MAP_WIDTH-1)
    y = libtcod.random_get_int(0,0, MAP_HEIGHT-1)

    while (world[x][y].blocked):
        x = libtcod.random_get_int(0,0, MAP_WIDTH-1)
        y = libtcod.random_get_int(0,0, MAP_HEIGHT-1)

    player.x = x
    player.y = y

    # Make stairs going up/down on player position
    if stairs_up:
        ustairs = Object(player.x, player.y, '<', 'up stairs', libtcod.white,
                        always_visible=True)

        objects.append(ustairs)
        # So it's drawn below the monsters
        ustairs.send_to_back()

    else:
        dstairs = Object(player.x, player.y, '>', 'down stairs', libtcod.white,
                        always_visible=True)

        objects.append(dstairs)
        # So it's drawn below the monsters
        dstairs.send_to_back()

    # Finally put stuff everywhere
    place_objects()

def menu(header, options, width):
    ''' Create a menu that options can be selected from using the alphabet '''
    if len(options) > 26: raise ValueError('Cannot have a menu with more than \
                                            26 options.')

    # Calculate total height for the header (after auto-wrap) and one line per
    #   option
    header_height = libtcod.console_get_height_rect(con, 0, 0, width,
                                                    SCREEN_HEIGHT, header)
    height = len(options) + header_height

    # Create an off-screen console that represents the menu's window
    window = libtcod.console_new(width, height)

    # Print the header, with auto-wrap
    libtcod.console_set_default_foreground(window, libtcod.white)
    libtcod.console_print_rect_ex(window, 0, 0, width, height,
                                    libtcod.BKGND_NONE, libtcod.LEFT, header)

    # Print all the options
    y = header_height
    letter_index = ord('a')
    for option_text in options:
        text = '(' + chr(letter_index) + ') ' + option_text
        libtcod.console_print_ex(window, 0, y, libtcod.BKGND_NONE,
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

def message(new_msg, color=libtcod.white):
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
    # transform it into a nasty corpse! it doesn't block, can't be
    # Attacked and doesn't move
    message(' '.join([monster.name.capitalize(), 'is dead!']),
            libtcod.darker_red)
    message('You gain ' + str(monster.fighter.xp) + ' experience points.',
            libtcod.orange)
    monster.char = '%'
    monster.color = libtcod.dark_red
    monster.blocks = False
    monster.fighter = None
    monster.ai = None
    monster.send_to_back()
    monster.name = ' '.join(['remains of', monster.name])

def monster_death_slock(monster):
    ''' Function called when monster dies. Blinds player '''
    # transform it into a nasty corpse! it doesn't block, can't be
    # Attacked and doesn't move
    message(' '.join([monster.name.capitalize(), 'is dead!']),
            libtcod.darker_red)
    message('You gain ' + str(monster.fighter.xp) + ' experience points.',
            libtcod.orange)
    message(' '.join([monster.name.capitalize(),
            'casts a final spell in its dying moments!']))
    monster.char = '%'
    monster.color = libtcod.dark_red
    monster.blocks = False
    monster.fighter = None
    monster.ai = None
    monster.send_to_back()
    monster.name = ' '.join(['remains of', monster.name])
    # Blind
    cast_inflict_blindness()

def monster_death_talk(monster):
    ''' Function called when monster dies. Says dying words '''
    # transform it into a nasty corpse! it doesn't block, can't be
    # Attacked and doesn't move
    for mon in monster_data:
        if monster.name == monster_data[mon]['name']:
            death_speech = monster_data[mon]['death_talk']
    message(''.join([monster.name.capitalize(), ' says "', death_speech,
            '"']), libtcod.darker_red)
    message(' '.join([monster.name.capitalize(), 'is dead!']),
            libtcod.darker_red)
    message('You gain ' + str(monster.fighter.xp) + ' experience points.',
            libtcod.orange)
    monster.char = '%'
    monster.color = libtcod.dark_red
    monster.blocks = False
    monster.fighter = None
    monster.ai = None
    monster.send_to_back()
    monster.name = ' '.join(['remains of', monster.name])

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
        if libtcod.map_is_in_fov(fov_map, obj.x, obj.y) and \
        obj.fighter and \
        obj.name != player.name:
            message('Monster in view!', libtcod.pink)
            monster = True

    try:
        if world[tx][ty].blocked or not world[tx][ty].explored:
            message('Cannot travel there', libtcod.pink)
        elif blind:
            message('That\'s not a good idea considering your blindness',
                libtcod.pink)
        else:
            message('Moving...', libtcod.pink)
            while not libtcod.console_is_window_closed() and not monster and \
            (player.x, player.y) != (tx, ty):
                render_all()
                # Present the root console
                libtcod.console_flush()

                for obj in objects:
                    if libtcod.map_is_in_fov(fov_map, obj.x, obj.y) and \
                    obj.fighter and \
                    obj.name != player.name:
                        message('Monster in view!', libtcod.pink)
                        monster = True
                        continue

                player.move_astar(tx, ty, True)
                fov_recompute()

                # AI takes turn
                for obj in objects:
                    if obj.ai:
                        obj.ai.take_turn()

                check_ground()

    except IndexError:
        message('Out of range', libtcod.pink)

def msgbox(text, width=50):
    ''' use menu() as a sort of \'message box\' '''
    menu(text, [], width)

def move_camera(target_x, target_y):
    ''' Move camera to coordinates '''
    global camera_x, camera_y, check_fov

    # New camera coordinates (top-left corner of the screen relative to the map)
    # Coordinates so that the target is at the center of the screen
    x = target_x - CAMERA_WIDTH / 2
    y = target_y - CAMERA_HEIGHT / 2

    # Make sure the camera doesn't see outside the map
    if x < 0:
        x = 0
    if y < 0:
        y = 0
    if x > MAP_WIDTH - CAMERA_WIDTH - 1:
        x = MAP_WIDTH - CAMERA_WIDTH - 1
    if y > MAP_HEIGHT - CAMERA_HEIGHT - 1:
        y = MAP_HEIGHT - CAMERA_HEIGHT - 1

    if x != camera_x or y != camera_y:
        check_fov = True

    (camera_x, camera_y) = (x, y)

def new_game():
    ''' Start a new game '''
    global player, edge, inventory, game_msgs, game_state, dungeon_level, \
            monster_data, items_data

    # Player
    # create object representing the player
    fighter_component = Fighter(hp=100, defense=1, power=4, xp=0, mana=100,
                                death_function=player_death,)
    player = Object(0, 0, PLAYER_CHARACTER, player_name, PLAYER_COLOR, blocks=True,
                    fighter=fighter_component)

    player.level = 1

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
    message('Welcome!', libtcod.lighter_yellow)

    render_all()
    # Present the root console
    libtcod.console_flush()

def next_level():
    ''' Go to next level '''
    global dungeon_level, max_dungeon_level, stairs_up

    dungeon_level += 1

    stairs_up = True

    message('After a rare moment of peace, you descend deeper into the heart of the dungeon...',
            libtcod.red)
    make_map()  # Create a fresh new level!
    initialize_fov()

def place_objects():
    ''' Place objects on level '''
    # Maximum number of monsters per level
    max_monsters = from_dungeon_level([[15, 1], [30, 4], [35, 6]])

    # Chance of each monster
    monster_chances = {}
    # Monster name then chance
    for item in monster_data:
        monster_chances[str(monster_data[item]['id'])] = \
            from_dungeon_level(monster_data[item]['chance'])

    # Maximum number of items per level
    max_items = from_dungeon_level([[10, 1], [20, 3], [25, 4]])

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
        # Choose random spot for this monster
        x = libtcod.random_get_int(0, 0, MAP_WIDTH-1)
        y = libtcod.random_get_int(0, 0, MAP_HEIGHT-1)
        while(world[x][y].blocked):
            x = libtcod.random_get_int(0, 0, MAP_WIDTH-1)
            y = libtcod.random_get_int(0, 0, MAP_HEIGHT-1)

        # Only place it if the tile is not blocked
        if not is_blocked(x, y):
            choice = random_choice(monster_chances)

            monster = generate_monster(choice, x, y)

            # Add monster to object list
            objects.append(monster)

    # Choose random number of items
    num_items = libtcod.random_get_int(0, 0, max_items+dungeon_level)

    for i in range(num_items):
        # Choose random spot for this monster
        x = libtcod.random_get_int(0, 0, MAP_WIDTH-1)
        y = libtcod.random_get_int(0, 0, MAP_HEIGHT-1)
        while(world[x][y].blocked):
            x = libtcod.random_get_int(0, 0, MAP_WIDTH-1)
            y = libtcod.random_get_int(0, 0, MAP_HEIGHT-1)

        # Only place it if the tile is not blocked
        if not is_blocked(x, y):
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
        libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS |
                                    libtcod.EVENT_MOUSE, key, mouse)

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
            mouse_move_astar(mouse.cx + camera_x, mouse.cy + camera_y)

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
        message('You died!', libtcod.dark_red)

        # For added effect, transform the player into a corpse!
        player.char = '%'
        player.color = libtcod.dark_red

        game_state = 'dead'

        render_all()
        # Present the root console
        libtcod.console_flush()

        game_over()
    else:
        message('...But it refused!', libtcod.crimson)
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
        player.fighter.attack(target)
    else:
        player.move(dx, dy)
        fov_recompute()

def previous_level():
    ''' Go back up in the dungeon '''
    global dungeon_level, stairs_up
    # In case if you're that guy who likes going back for some reason

    dungeon_level -= 1

    stairs_up = False

    if dungeon_level == 0:
        for item in inventory:
            if item.name == 'StatTrak Fedora | Fade (Fac New)':
                game_win()
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

    else:
        message('After a rare moment of peace, you ascend upwards towards the surface...',
                libtcod.red)
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
    global check_fov, camera_x, camera_y, blind, blind_counter

    move_camera(player.x, player.y)

    if not blind:
        if check_fov:
            check_fov = False
            fov_recompute()

        # Draw all objects in the list, except the player. we want it to
        # Always appear over all other objects! so it's drawn later.
        for obj in objects:
            if obj.name != player.name:
                obj.draw()

    else:
        if blind_counter == BLIND_LENGTH:
            blind = False
            blind_counter = 0
            message("Your vision returns!", libtcod.light_sea)
    player.draw()

    if not blind:
        # Display a cursor under mouse coords
        libtcod.console_set_char_background(con, mouse.cx, mouse.cy,
                                            color_light_ground)
        # blit the contents of 'con' to the root console
        libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)
        fov_recompute()
    else:
        libtcod.console_clear(con)
        libtcod.console_set_default_background(con, libtcod.black)
        player.draw()
        libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)

    # Prepare to render the GUI panel
    libtcod.console_set_default_background(panel, libtcod.black)
    libtcod.console_clear(panel)

    # Also the message panel
    libtcod.console_set_default_background(msg_panel, libtcod.black)
    libtcod.console_clear(msg_panel)

    # Show the player's stats
    libtcod.console_print_ex(panel, 1 + BAR_WIDTH / 2, 1, libtcod.BKGND_NONE,
                            libtcod.CENTER, player.name)

    # Cool distinctions
    libtcod.console_set_default_foreground(panel, libtcod.gray)
    for y in range(SCREEN_HEIGHT):
        libtcod.console_print_ex(panel, 0, y, libtcod.BKGND_NONE,
                                    libtcod.CENTER, '|')
    libtcod.console_set_default_foreground(msg_panel, libtcod.gray)
    for x in range(SCREEN_WIDTH):
        libtcod.console_print_ex(msg_panel, x, 0, libtcod.BKGND_NONE,
                                    libtcod.CENTER, '-')

    render_bar(1, 2, BAR_WIDTH, 'HP', player.fighter.hp, player.fighter.max_hp,
               libtcod.light_red, libtcod.darker_red)

    # Self-explanatory bars
    render_bar(1, 3, BAR_WIDTH, 'Edge', player.fighter.mana,
                player.fighter.max_mana, libtcod.dark_fuchsia,
                libtcod.darker_fuchsia)

    render_bar(1, 4, BAR_WIDTH, 'XP', player.fighter.xp, (LEVEL_UP_BASE +
                player.level * LEVEL_UP_FACTOR),
                libtcod.dark_yellow, libtcod.darker_yellow)

    render_bar_simple(1, 5, BAR_WIDTH, 'Floor', str(dungeon_level),
                                                    libtcod.light_blue)


    render_bar_simple(1, 7, BAR_WIDTH, 'Attack', str(player.fighter.power),
                        libtcod.dark_chartreuse)
    render_bar_simple(1, 8, BAR_WIDTH, 'Defense', str(player.fighter.defense),
                        libtcod.flame)

    # Render equipment
    slot_list = [
        'right hand',
        'left hand',
        'head',
        'face',
        'neck',
        'torso',
        'hands',
        'legs',
        'accessory'
    ]
    for y, slot in enumerate(slot_list):
        render_equips(SCREEN_HEIGHT - len(slot_list) + y, slot)

    # Show all the monsters that the player can see and shows their health
    monsters_in_room = 0
    for obj in objects:
        if libtcod.map_is_in_fov(fov_map, obj.x, obj.y) and obj.fighter and \
        obj.name != player.name and not blind:
            monsters_in_room += 1
            if monsters_in_room > (SCREEN_HEIGHT - 20) / 2:
                continue
            else:
                libtcod.console_set_default_foreground(panel, obj.color)
                libtcod.console_print_ex(panel, 1, 9+(2*monsters_in_room),
                                        libtcod.BKGND_NONE, libtcod.LEFT,
                                        ''.join([obj.char, ' ',
                                        obj.name.capitalize()]))
                render_health_bar(1, 10+(2*monsters_in_room), BAR_WIDTH,
                                    obj.fighter.hp, obj.fighter.base_max_hp,
                                    libtcod.red, libtcod.dark_red)

    # Display names of objects under the mouse
    if not blind:
        libtcod.console_set_default_foreground(msg_panel, libtcod.light_gray)
        libtcod.console_print_ex(msg_panel, 1, 0, libtcod.BKGND_NONE, libtcod.LEFT,
                                get_names_under_mouse())

    # Print the game messages, one line at a time
    y = 1
    for (line, color) in game_msgs:
        libtcod.console_set_default_foreground(msg_panel, color)
        libtcod.console_print_ex(msg_panel, MSG_X, y, libtcod.BKGND_NONE,
                                libtcod.LEFT, line)
        y += 1

    # Blit the contents of 'panel' and 'msg_panel' to the root console
    libtcod.console_blit(msg_panel, 0, 0, SCREEN_WIDTH, PANEL_HEIGHT, 0, 0,
                        MSG_PANEL_Y)
    libtcod.console_blit(panel, 0, 0, PANEL_WIDTH, PANEL_HEIGHT, 0,
                            SCREEN_WIDTH-PANEL_WIDTH, PANEL_Y)

def render_bar(x, y, total_width, name, value, maximum, bar_color, back_color):
    ''' Render a bar (HP, experience). '''
    # first calculate the width of the bar
    bar_width = int(float(value) / maximum * total_width)

    # Render the background first
    libtcod.console_set_default_background(panel, back_color)
    libtcod.console_rect(panel, x, y, total_width, 1, False,
                            libtcod.BKGND_SCREEN)

    # Now render the bar on top
    libtcod.console_set_default_background(panel, bar_color)
    if bar_width > 0:
        libtcod.console_rect(panel, x, y, bar_width, 1, False,
                            libtcod.BKGND_SCREEN)

    # Finally, some centered text with the values
    libtcod.console_set_default_foreground(panel, libtcod.white)
    libtcod.console_print_ex(panel, x + total_width / 2, y, libtcod.BKGND_NONE,
                            libtcod.CENTER, name + ': ' + str(value) +
                            '/' + str(maximum))

def render_bar_simple(x, y, total_width, name, value, color):
    ''' Extremely simple bar rendering
    Not intended to have values increase and decrease, but rather display
    one static value instead (attack, defense)'''

    # Render the background first
    libtcod.console_set_default_background(panel, color)
    libtcod.console_rect(panel, x, y, total_width, 1, False,
                        libtcod.BKGND_SCREEN)

    # Now render the bar on top
    libtcod.console_set_default_background(panel, color)
    if total_width > 0:
        libtcod.console_rect(panel, x, y, total_width, 1, False,
                            libtcod.BKGND_SCREEN)

    # Finally, some centered text with the values
    libtcod.console_set_default_foreground(panel, libtcod.white)
    libtcod.console_print_ex(panel, x + total_width / 2, y, libtcod.BKGND_NONE,
                            libtcod.CENTER, name + ': ' + str(value))

def render_equips(y_offset, slot):
    equip = get_equipped_in_slot(slot)
    if not equip:
        equip = "None"
    else:
        if len(equip.owner.name) > 12:
            equip = equip.short_name
        else:
            equip = equip.owner.name

    if slot == 'right hand':
        slot = 'RH'
    elif slot == 'left hand':
        slot = 'LH'

    render_bar_simple(1, y_offset, BAR_WIDTH, slot.capitalize(), equip,
                        libtcod.black)

def render_health_bar(x, y, total_width, value, maximum, bar_color, back_color):
    ''' This is a bar that doesn't show any values in it. Useful for enemy
    health bars '''

    # Render a bar (HP, experience, etc). first calculate the width of the bar
    bar_width = int(float(value) / maximum * total_width)

    # Render the background first
    libtcod.console_set_default_background(panel, back_color)
    libtcod.console_rect(panel, x, y, total_width, 1, False,
                        libtcod.BKGND_SCREEN)

    # Now render the bar on top
    libtcod.console_set_default_background(panel, bar_color)
    if bar_width > 0:
        libtcod.console_rect(panel, x, y, bar_width, 1, False,
                            libtcod.BKGND_SCREEN)

    # Finally, show the bar
    libtcod.console_set_default_foreground(panel, libtcod.white)
    libtcod.console_print_ex(panel, x + total_width / 2, y, libtcod.BKGND_NONE,
                            libtcod.CENTER, '')

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
    #   but it looks like it sorts the object list by the names of the object
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
        libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS |
                                    libtcod.EVENT_MOUSE, key, mouse)
        render_all()
        # Present the root console
        libtcod.console_flush()
        (x, y) = (mouse.cx, mouse.cy)
        (x, y) = (camera_x + x, camera_y + y)  # From screen to map coordinates

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

    message(''.join(['You say \'', taunts[randint(0,len(taunts)-1)], '\'']))

def toggle_siphon():
    ''' Toggle the siphon spell '''
    global activate_siphon
    if activate_siphon:
        activate_siphon = False
        message('You deativate your siphon ability', libtcod.magenta)
    else:
        activate_siphon = True
        message('You activate your siphon ability', libtcod.magenta)

def to_camera_coordinates(x, y):
    ''' convert coordinates on the map to coordinates on the screen '''
    (x, y) = (x - camera_x, y - camera_y)

    # If it's outside the view, return nothing
    if x < 0 or y < 0 or x >= CAMERA_WIDTH or y >= CAMERA_HEIGHT:
        return None, None

    return x, y

def weapon_action_katana(weapon):
    ''' Katana action '''
    message('You examine the fine steel of the katana, surely folded over 1000 times')

def weapon_action_knife(weapon):
    ''' Knife action '''
    message('You flaunt your latest knife')

def weapon_action_awp(weapon):
    ''' AWP action '''
    message('You no-scope with the AWP')
    cast_lightning()

def weapon_action_firearm(weapon):
    ''' Firearm action '''
    fire_weapon(weapon.equipment)

def weapon_action_else(weapon):
    ''' Emergency reserve action '''
    message('You stare deeply at your ' + weapon.name)

######################################
# Main Loop
######################################

main_menu()
