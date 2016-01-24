import math
import shelve
import textwrap
import time
from random import *

import simplejson as json

from colors import *
from modules import libtcodpy as libtcod
from settings import *

######################################
# JSON
######################################

"""
JSON is used to store monster and item data

Here is an example JSON object

    "example": {
        "name": "Example Item", # Name of the item
        "id": "example",        # ID of the item
        "char": "!",            # Character used when drawn on map
        "color": "light_green", # Color of character on map
        "chance": [[45, 1]],    # Chance value beyond level, explained later
        "type": "item",         # Item type
        "effect": "heal"        # Item effect
        "slot": "right hand",   # Item slot
        "power": 10,            # Bonus to power
        "defense": 0,           # Bonus to defense
        "hp": 0,                # Bonus to HP
        "mana": 0               # Bonus to Edge
        ---------------------------------------
        Alternatively for monsters
		"hp":99,                # Health
		"defense":1,            # Defense
		"power":5,              # Power
		"xp":500,               # Experience
        "mana":0,               # Edge
		"death_func":"normal",  # Death function
        "ai": "talk",           # AI Type
        "speech": [
            "I'm a test item!"  # Speech text
        ],
        "rate": 65              # Rate of speech
    },

Not all of these options are used for each object, clearly

"""

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
game_state = "playing"

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

######################################
# Classes
######################################

class BasicMonster:
    """ AI for a basic monster. """
    def __init__(self):
        pass

    def take_turn(self):
        """Monster takes its turn. If you can see it, it can see you """
        monster = self.owner
        # If it's in the player's fov then it approaches them
        if libtcod.map_is_in_fov(fov_map, monster.x, monster.y):

            # Move towards player if far away
            if monster.distance_to(player) >= 2:
                monster.move_towards(player.x, player.y)

            # Close enough, attack! (if the player is still alive.)
            elif player.fighter.hp > 0:
                monster.fighter.attack(player)

        # Otherwise it just runs around randomly
        else:
            x = libtcod.random_get_int(0, -1, 1)
            y = libtcod.random_get_int(0, -1, 1)
            monster.move(x, y)

class ConfusedMonster:
    """ AI for a temporarily confused monster
    (reverts to previous AI after a while). """
    def __init__(self, old_ai, num_turns=CONFUSE_NUM_TURNS):
        self.old_ai = old_ai
        self.num_turns = num_turns

    def take_turn(self):
        """ Monster takes a turn, but moves randomly """
        if self.num_turns > 0:  # Still confused...
            # Move in a random direction, and decrease the number of turns confused
            self.owner.move(libtcod.random_get_int(0, -1, 1), libtcod.random_get_int(0, -1, 1))
            self.num_turns -= 1

        else:  # Restore the previous AI (this one will be deleted because it's not referenced anymore)
            self.owner.ai = self.old_ai
            message('The ' + self.owner.name + ' is no longer confused!', libtcod.red)

class dMap:
    # Black box map generation
    # Found it on RogueBasin, works a lot better than the tutorial one
    def __init__(self):
        self.roomList=[]
        self.cList=[]

    def makeMap(self,xsize,ysize,fail,b1,mrooms):
        """Generate random layout of rooms, corridors and other features"""
        # makeMap can be modified to accept arguments for values of failed, and percentile of features.
        # Create first room
        self.size_x = xsize
        self.size_y = ysize
        # initialize map to all walls
        self.mapArr=[]
        for y in range(ysize):
            tmp = []
            for x in range(xsize):
                    tmp.append(1)
            self.mapArr.append( tmp )

        w,l,t=self.makeRoom()
        while len(self.roomList)==0:
            y=randrange(ysize-1-l)+1
            x=randrange(xsize-1-w)+1
            p=self.placeRoom(l,w,x,y,xsize,ysize,6,0)
        failed=0
        while failed<fail: #The lower the value that failed< , the smaller the dungeon
            chooseRoom=randrange(len(self.roomList))
            ex,ey,ex2,ey2,et=self.makeExit(chooseRoom)
            feature=randrange(100)
            if feature<b1: #Begin feature choosing (more features to be added here)
                    w,l,t=self.makeCorridor()
            else:
                    w,l,t=self.makeRoom()
            roomDone=self.placeRoom(l,w,ex2,ey2,xsize,ysize,t,et)
            if roomDone==0: #If placement failed increase possibility map is full
                    failed+=1
            elif roomDone==2: #Possiblilty of linking rooms
                    if self.mapArr[ey2][ex2]==0:
                        if randrange(100)<7:
                            self.makePortal(ex,ey)
                        failed+=1
            else: #Otherwise, link up the 2 rooms
                    self.makePortal(ex,ey)
                    failed=0
                    if t<5:
                        tc=[len(self.roomList)-1,ex2,ey2,t]
                        self.cList.append(tc)
                        self.joinCorridor(len(self.roomList)-1,ex2,ey2,t,50)
            if len(self.roomList)==mrooms:
                    failed=fail
        self.finalJoins()

    def makeRoom(self):
        """Randomly produce room size"""
        rtype=5
        rwide=randrange(8)+3
        rlong=randrange(8)+3
        return rwide,rlong,rtype

    def makeCorridor(self):
        """Randomly produce corridor length and heading"""
        clength=randrange(18)+3
        heading=randrange(4)
        if heading==0: #North
            wd=1
            lg=-clength
        elif heading==1: #East
            wd=clength
            lg=1
        elif heading==2: #South
            wd=1
            lg=clength
        elif heading==3: #West
            wd=-clength
            lg=1
        return wd,lg,heading

    def placeRoom(self,ll,ww,xposs,yposs,xsize,ysize,rty,ext):
        """Place feature if enough space and return canPlace as true or false"""
        #Arrange for heading
        xpos=xposs
        ypos=yposs
        if ll<0:
            ypos+=ll+1
            ll=abs(ll)
        if ww<0:
            xpos+=ww+1
            ww=abs(ww)
        #Make offset if type is room
        if rty==5:
            if ext==0 or ext==2:
                    offset=randrange(ww)
                    xpos-=offset
            else:
                    offset=randrange(ll)
                    ypos-=offset
        #Then check if there is space
        canPlace=1
        if ww+xpos+1>xsize-1 or ll+ypos+1>ysize:
            canPlace=0
            return canPlace
        elif xpos<1 or ypos<1:
            canPlace=0
            return canPlace
        else:
            for j in range(ll):
                    for k in range(ww):
                        if self.mapArr[(ypos)+j][(xpos)+k]!=1:
                            canPlace=2
        #If there is space, add to list of rooms
        if canPlace==1:
            temp=[ll,ww,xpos,ypos]
            self.roomList.append(temp)
            for j in range(ll+2): #Then build walls
                    for k in range(ww+2):
                        self.mapArr[(ypos-1)+j][(xpos-1)+k]=2
            for j in range(ll): #Then build floor
                    for k in range(ww):
                        self.mapArr[ypos+j][xpos+k]=0
        return canPlace #Return whether placed is true/false

    def makeExit(self,rn):
        """Pick random wall and random point along that wall"""
        room=self.roomList[rn]
        while True:
            rw=randrange(4)
            if rw==0: #North wall
                    rx=randrange(room[1])+room[2]
                    ry=room[3]-1
                    rx2=rx
                    ry2=ry-1
            elif rw==1: #East wall
                    ry=randrange(room[0])+room[3]
                    rx=room[2]+room[1]
                    rx2=rx+1
                    ry2=ry
            elif rw==2: #South wall
                    rx=randrange(room[1])+room[2]
                    ry=room[3]+room[0]
                    rx2=rx
                    ry2=ry+1
            elif rw==3: #West wall
                    ry=randrange(room[0])+room[3]
                    rx=room[2]-1
                    rx2=rx-1
                    ry2=ry
            if self.mapArr[ry][rx]==2: #If space is a wall, exit
                    break
        return rx,ry,rx2,ry2,rw

    def makePortal(self,px,py):
        """Create doors in walls"""
        ptype=randrange(100)
        if ptype>90: #Secret door
            self.mapArr[py][px]=0
            return
        elif ptype>75: #Closed door
            self.mapArr[py][px]=0
            return
        elif ptype>40: #Open door
            self.mapArr[py][px]=0
            return
        else: #Hole in the wall
            self.mapArr[py][px]=0

    def joinCorridor(self,cno,xp,yp,ed,psb):
        """Check corridor endpoint and make an exit if it links to another room"""
        cArea=self.roomList[cno]
        if xp!=cArea[2] or yp!=cArea[3]: #Find the corridor endpoint
            endx=xp-(cArea[1]-1)
            endy=yp-(cArea[0]-1)
        else:
            endx=xp+(cArea[1]-1)
            endy=yp+(cArea[0]-1)
        checkExit=[]
        if ed==0: #North corridor
            if endx>1:
                    coords=[endx-2,endy,endx-1,endy]
                    checkExit.append(coords)
            if endy>1:
                    coords=[endx,endy-2,endx,endy-1]
                    checkExit.append(coords)
            if endx<self.size_x-2:
                    coords=[endx+2,endy,endx+1,endy]
                    checkExit.append(coords)
        elif ed==1: #East corridor
            if endy>1:
                    coords=[endx,endy-2,endx,endy-1]
                    checkExit.append(coords)
            if endx<self.size_x-2:
                    coords=[endx+2,endy,endx+1,endy]
                    checkExit.append(coords)
            if endy<self.size_y-2:
                    coords=[endx,endy+2,endx,endy+1]
                    checkExit.append(coords)
        elif ed==2: #South corridor
            if endx<self.size_x-2:
                    coords=[endx+2,endy,endx+1,endy]
                    checkExit.append(coords)
            if endy<self.size_y-2:
                    coords=[endx,endy+2,endx,endy+1]
                    checkExit.append(coords)
            if endx>1:
                    coords=[endx-2,endy,endx-1,endy]
                    checkExit.append(coords)
        elif ed==3: #West corridor
            if endx>1:
                    coords=[endx-2,endy,endx-1,endy]
                    checkExit.append(coords)
            if endy>1:
                    coords=[endx,endy-2,endx,endy-1]
                    checkExit.append(coords)
            if endy<self.size_y-2:
                    coords=[endx,endy+2,endx,endy+1]
                    checkExit.append(coords)
        for xxx,yyy,xxx1,yyy1 in checkExit: #Loop through possible exits
            if self.mapArr[yyy][xxx]==0: #If joins to a room
                    if randrange(100)<psb: #Possibility of linking rooms
                        self.makePortal(xxx1,yyy1)

    def finalJoins(self):
        """Final stage, loops through all the corridors to see if any can be joined to other rooms"""
        for x in self.cList:
            self.joinCorridor(x[0],x[1],x[2],x[3],10)

class TalkingMonster:
    """ An AI that says things """
    def __init__(self, speech, rate):
        self.speech = speech
        self.rate = rate

    def take_turn(self):
        """ Monster takes a normal turn, but says something """
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

            # Depending on the rate of speech set in the json, the monster may talk
            if libtcod.random_get_int(0, 0, 100) > self.rate:
                r = libtcod.random_get_int(0, 0, len(self.speech)-1)
                message("".join([monster.name, " says ", "\"", self.speech[r], "\""]), monster.color)

class Equipment:
    """ An object that can be equipped, yielding bonuses.
    automatically adds the Item component. """
    def __init__(self, slot, power_bonus=0, defense_bonus=0, max_hp_bonus=0,
                max_mana_bonus=0):
        self.power_bonus = power_bonus
        self.defense_bonus = defense_bonus
        self.max_hp_bonus = max_hp_bonus
        self.max_mana_bonus = max_mana_bonus

        self.slot = slot
        self.is_equipped = False

    def toggle_equip(self):
        """ Toggle equip/dequip status """
        if self.is_equipped:
            self.dequip()
        else:
            self.equip()

    def equip(self):
        """ If the slot is already being used do nothing,
        except for dual weilding """
        old_equipment = get_equipped_in_slot(self.slot)
        if old_equipment is not None:
            # If the item is to be equiped in the hands, find a free hand and equip it there
            # Essentially, this is dual weilding
            if self.slot == "left hand" or self.slot == "right hand":
                # Switch hands on equipment
                if self.slot == "left hand":
                    self.slot = "right hand"
                elif self.slot == "right hand":
                    self.slot = "left hand"

                message("You use your free hand to equip the " + self.owner.name)

            # If both hands are full, dequip something or else the player somehow
            # grows a new hand spontaneously
            if get_equipped_in_slot(self.slot) is not None:
                message(("But something is already there, so take off the " + old_equipment.owner.name + " for the " + self.owner.name), libtcod.light_red)

                # Switch the hands back on the equipment
                if self.slot == "left hand":
                    self.slot = "right hand"
                elif self.slot == "right hand":
                    self.slot = "left hand"

                old_equipment.dequip()

        # Equip object and show a message about it
        self.is_equipped = True
        message('Equipped ' + self.owner.name + ' on your ' + self.slot + '.', libtcod.light_green)

    def dequip(self):
        """ Dequip object and show a message about it """
        if not self.is_equipped: return
        self.is_equipped = False
        message('Dequipped ' + self.owner.name + ' from ' + self.slot + '.', libtcod.light_yellow)

class Fighter:
    """ Combat-related properties and methods (monster, player, NPC) """
    def __init__(self, hp, defense, power, xp, mana, death_function=None):
        self.base_max_hp = hp
        self.hp = hp
        self.base_defense = defense
        self.base_power = power
        self.xp = xp
        self.death_function = death_function
        self.mana = mana
        self.base_max_mana = mana

    @property
    def power(self):  # Return actual power, by summing up the bonuses from all equipped items
        bonus = sum(equipment.power_bonus for equipment in get_all_equipped(self.owner))
        return self.base_power + bonus

    @property
    def defense(self):  # Return actual defense, by summing up the bonuses from all equipped items
        bonus = sum(equipment.defense_bonus for equipment in get_all_equipped(self.owner))
        return self.base_defense + bonus

    @property
    def max_hp(self):  # Return actual max_hp, by summing up the bonuses from all equipped items
        bonus = sum(equipment.max_hp_bonus for equipment in get_all_equipped(self.owner))
        return self.base_max_hp + bonus

    @property
    def max_mana(self):  # Return actual mana, by summing up the bonuses from all equipped items
        bonus = sum(equipment.max_mana_bonus for equipment in get_all_equipped(self.owner))
        return self.base_max_mana + bonus

    def take_damage(self, damage):
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
            # Yield experience to the player, take some mana and give some health
            if self.owner != player:
                player.fighter.xp += self.xp
                # player.fighter.siphon() # Try to siphon life
                kill_count += 1 # Increment kill count

    def attack(self, target):
        # A simple formula for attack damage
        damage = self.power - target.fighter.defense

        if damage > 0:
            # Make the target take some damage
            message(" ".join([self.owner.name.capitalize(), "attacks",
                target.name, "for", str(damage), "hit points."]),
                    libtcod.red)
            target.fighter.take_damage(damage)
        else:
            message(" ".join([self.owner.name.capitalize(), "attacks",
                target.name, "but it has no effect!"]),
                    libtcod.light_red)

    def heal(self, amount):
        # Heal by the given amount, without going over the maximum
        self.hp += amount
        if self.hp > self.max_hp:
            self.hp = self.max_hp

    def cast(self, cost):
        # Not used. Not sure what this can be used for in the future
        if self.mana - cost < 0:
            message("You don't have enough mana to cast this!", libtcod.red)
        else:
            self.mana -= cost

    def siphon(self):
        # Steal life. Sort of like a regeneration system
        if self.mana - SIPHON_COST < 0:
            message("You try to siphon any life away, but you aren't edgy enough",
                libtcod.light_red)
            return "cancelled"

        self.mana -= SIPHON_COST
        self.heal(SIPHON_AMOUNT)

        message("You siphon life from the deceased", libtcod.fuchsia)

    def magic_missile(self):
        # Find closest monster
        monster = closest_monster(MISSILE_RANGE)
        if monster is None:  # No enemy found within maximum range
            message('No enemy is close enough to strike with your edge missile', libtcod.light_red)
            return "cancelled"

        # Fire a magic missile
        if self.mana - MISSILE_COST < 0:
            message("You try to fire an edge missile, but you aren't edgy enough",
                libtcod.light_red)
            return "cancelled"

        self.mana -= MISSILE_COST
        cast_magic_missile()

    def restore(self, ammount):
        # Give some mana back to the player
        self.mana += ammount
        if self.mana > self.max_mana:
            self.mana = self.max_mana

class Item:
    # An item that can be picked up and used.
    def __init__(self, use_function=None):
        self.use_function = use_function

    def pick_up(self):
        # Add to the player"s inventory and remove from the map
        if len(inventory) >= 26:
            message("Your inventory is full, cannot pick up " + self.owner.name +
                ".", libtcod.dark_fuchsia)

        else:
            inventory.append(self.owner)
            objects.remove(self.owner)
            message("You picked up a " + self.owner.name + "!", libtcod.light_green)

        # Special case: automatically equip, if the corresponding equipment
        # slot is unused
        equipment = self.owner.equipment
        if equipment is not None:
            if (equipment and get_equipped_in_slot(equipment.slot) is None) or \
                    (equipment.slot == "right hand" or equipment.slot == "left hand"):
                equipment.equip()

    def drop(self):
        # Special case: if the object has the Equipment component,
        # dequip it before dropping
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
        # Special case: if the object has the Equipment component, the "use"
        # action is to equip/dequip
        if self.owner.equipment:
            self.owner.equipment.toggle_equip()
            return
        # Just call the "use_function" if it is defined
        if self.use_function is None:
            message("The " + self.owner.name + " cannot be used.", libtcod.gray)
        else:
            if self.use_function() != "cancelled":
                # Destroy after use, unless it was cancelled for some reason
                inventory.remove(self.owner)

class Object:
    """
    This is a generic object: the player, a monster, an item, the stairs...
    It's always represented by a character on screen
    """

    def __init__(self, x, y, char, name, color, blocks=False, always_visible=False, fighter=None, ai=None, item=None, gold=None, equipment=None):
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
            # There must be an Item component for the Equipment component to work properly
            self.item = Item()
            self.item.owner = self

    def clear(self):
        # Erase the character that represents this object
        (x, y) = to_camera_coordinates(self.x, self.y)
        if x is not None:
            libtcod.console_put_char(con, x, y, ' ', libtcod.BKGND_NONE)

    def distance(self, x, y):
        # return the distance to some coordinates
        return math.sqrt((x - self.x) ** 2 + (y - self.y) ** 2)

    def distance_to(self, other):
        # Return the distance to another object
        dx = other.x - self.x
        dy = other.y - self.y
        return math.sqrt(dx ** 2 + dy ** 2)

    def draw(self):
        # Only show if it's visible to the player; or it's set to "always visible" and on an explored tile
        if ((libtcod.map_is_in_fov(fov_map, self.x, self.y)) or (self.always_visible and world[self.x][self.y].explored)):
            (x, y) = to_camera_coordinates(self.x, self.y)

            if x is not None:
                # Set the color and then draw the character that represents this object at its position
                libtcod.console_set_default_foreground(con, self.color)
                libtcod.console_put_char(con, x, y, self.char, libtcod.BKGND_NONE)

    def drop(self):
        # Add to the map and remove from the player's inventory. also, place it at the player's coordinates
        objects.append(self.owner)
        inventory.remove(self.owner)
        self.owner.x = player.x
        self.owner.y = player.y
        message('You dropped a ' + self.owner.name + '.', libtcod.yellow)

    def move(self, dx, dy):
        # Move by a given amount
        try:
            if self.name == "player" and WALL_HACK:
                self.x += dx
                self.y += dy
            elif not world[self.x + dx][self.y + dy].blocked and not monster_occupy_check(self.x+dx, self.y+dy):
                self.x += dx
                self.y += dy
        except IndexError:
            pass

    def move_towards(self, target_x, target_y):
        dx = 0
        dy = 0

        # First, try to move towards player by row
        if target_x == self.x:
            pass
        elif target_x < self.x:
            dx = -1
        elif target_x > self.x:
            dx = 1

        # But if a wall is there, don't move that way
        if world[self.x + dx][self.y].blocked:
            dx = 0

        # Second, try to move towards player by column
        if target_y == self.y:
            pass
        elif target_y < self.y:
            dy = -1
        elif target_y > self.y:
            dy = 1

        # But if a wall is there, don't move that way
        if world[self.x][self.y + dy].blocked:
            dy = 0

        # The result is an Ai that follows the player around turns, but it doesn't capitalize on diagonal movements
        # However, it's a good start compared to the crap vector tracing bullshit
        self.move(dx, dy)

        """
        # The crap vector tracing bullshit in question
        # this piece of shit can't even get the monsters to go around corners

        # Vector from this object to the target, and distance
        dx = target_x - self.x
        dy = target_y - self.y
        distance = math.sqrt(dx ** 2 + dy ** 2)

        # Normalize it to length 1 (preserving direction), then round it and
        # Convert to integer so the movement is restricted to the map grid
        dx = int(round(dx / distance))
        dy = int(round(dy / distance))
        if world[self.x + dx][self.y + dy].blocked:
            dx = int(math.floor(dx/distance))
            dy = int(math.floor(dy/distance))
            if world[self.x + dx][self.y + dy].blocked:
                dx = int(math.ceil(dx/distance))
                dy = int(math.ceil(dy/distance))
        """

    def send_to_back(self):
        # Make this object be drawn first, so all others appear above it if they're in the same tile.
        global objects
        objects.remove(self)
        objects.insert(0, self)

class Rect:
    # This will take top-left coordinates for a rectangle (in tiles, of course),
    # and its size, to define it in terms of two points: top-left (x1, y1) and bottom-right (x2, y2)
    def __init__(self, x, y, w, h):
        self.x1 = x
        self.y1 = y
        self.x2 = x + w
        self.y2 = y + h

    def center(self):
        center_x = (self.x1 + self.x2) / 2
        center_y = (self.y1 + self.y2) / 2
        return center_x, center_y

    def intersect(self, other):
        # returns true if this rectangle intersects with another one
        return (self.x1 <= other.x2 and self.x2 >= other.x1 and
                self.y1 <= other.y2 and self.y2 >= other.y1)

class Tile:
    # A tile of the map and its properties
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
# Functions
######################################

def cast_confuse():
    # Ask the player for a target to confuse
    message('Left-click an enemy to confuse it, or right-click to cancel.', libtcod.light_cyan)
    monster = target_monster(CONFUSE_RANGE)
    if monster is None: return 'cancelled'

    # Replace the monster's AI with a "confused" one; after some turns it will restore the old AI
    old_ai = monster.ai
    monster.ai = ConfusedMonster(old_ai)
    monster.ai.owner = monster  # Tell the new component who owns it
    message('The eyes of the ' + monster.name + ' look vacant, as he starts to stumble around!', libtcod.light_green)

def cast_fireball():
    # Ask the player for a target tile to throw a fireball at
    message('Left-click a target tile for the fireball, or right-click to cancel.',
        libtcod.light_cyan)
    (x, y) = target_tile()
    if x is None: return 'cancelled'
    message('The fireball explodes, burning everything within ' +
        str(FIREBALL_RADIUS) + ' tiles!', libtcod.orange)

    for obj in objects:  # Damage every fighter in range, including the player
        if obj.distance(x, y) <= FIREBALL_RADIUS and obj.fighter:
            message('The ' + obj.name + ' gets burned for ' + str(FIREBALL_DAMAGE)
                + ' hit points.', libtcod.orange)
            obj.fighter.take_damage(FIREBALL_DAMAGE)

    # Trash fucking animation like wow its shit
    (x, y) = to_camera_coordinates(x, y) # This is literally magical I still have no idea how it works
    libtcod.console_set_default_foreground(con, libtcod.red)
    for i in range(FIREBALL_RADIUS):
        libtcod.console_put_char(con, x, y, "4", libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x+i, y, "4", libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x-i, y, "4", libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x, y+i, "4", libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x, y-i, "4", libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x+i, y+i, "4", libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x-i, y-i, "4", libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x+i, y-i, "4", libtcod.BKGND_NONE)
        libtcod.console_put_char(con, x-i, y+i, "4", libtcod.BKGND_NONE)

    libtcod.console_flush()

def cast_heal():
    # Heal the player
    if player.fighter.hp == player.fighter.max_hp:
        message("You are already at full health.", libtcod.light_gray)
        return "cancelled"

    message("Your wounds start to feel better!", libtcod.light_violet)
    player.fighter.heal(HEAL_AMOUNT)

def cast_mana():
    # Give some mana back
    if player.fighter.mana == player.fighter.max_mana:
        message("You already have enough edge.")
        return "cancelled"

    message("You begin to feel edgy!", libtcod.light_flame)
    player.fighter.restore(MANA_AMOUNT)

def cast_magic_missile():
    # Find closest enemy (inside a maximum range) and damage it
    # assumes that you already have a monster in range
    monster = closest_monster(MISSILE_RANGE)

    # Zap it!
    message('A missile of pure edge strikes the ' + monster.name +
            ' with a loud airhorn! The damage is ' + str(LIGHTNING_DAMAGE) +
            ' hit points.', libtcod.light_blue)

    monster.fighter.take_damage(MISSILE_DAMAGE)

    # Animation test, courtesy of Trash Animation Studios(tm)
    dx = player.x
    dy = player.y
    # The one cool this is that the lightning bolt changes depending on where
    # the monster is
    if (dx < monster.x and dy < monster.y) or (dx > monster.x and dy > monster.y):
        char = "\\"
    elif (dx < monster.x and dy > monster.y) or (dx > monster.x and dy < monster.y):
        char = "/"
    elif (dx == monster.x and dy != monster.y):
        char = "|"
    elif (dx != monster.x and dy == monster.y):
        char = "-"
    else:
        char = "z"

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
    # Find closest enemy (inside a maximum range) and damage it
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
    if (dx < monster.x and dy < monster.y) or (dx > monster.x and dy > monster.y):
        char = "\\"
    elif (dx < monster.x and dy > monster.y) or (dx > monster.x and dy < monster.y):
        char = "/"
    elif (dx == monster.x and dy != monster.y):
        char = "|"
    elif (dx != monster.x and dy == monster.y):
        char = "-"
    else:
        char = "z"

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

def check_level_up():
    # See if the player's experience is enough to level-up
    level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR
    if player.fighter.xp >= level_up_xp:
        # It is! level up
        player.level += 1
        player.fighter.xp -= level_up_xp
        message('Your battle skills grow stronger! You reached level ' +
            str(player.level) + '!', libtcod.yellow)

        choice = None
        while choice == None:  # Keep asking until a choice is made
            choice = menu('Level up! Choose a stat to raise:\n',
                ['Constitution (+20 HP, from ' + str(player.fighter.max_hp) + ')',
                'Strength (+1 attack, from ' + str(player.fighter.power) + ')',
                'Agility (+1 defense, from ' + str(player.fighter.defense) + ')'], LEVEL_SCREEN_WIDTH)

        if choice == 0:
            player.fighter.max_hp += 20
            player.fighter.hp += 20
        elif choice == 1:
            player.fighter.power += 1
        elif choice == 2:
            player.fighter.defense += 1

        # render the screen
        libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS | libtcod.EVENT_MOUSE, key, mouse)
        render_all()

        libtcod.console_flush()

def check_timer():
    global timer
    # Timer based commands

    # Regenerate health
    if player.fighter.hp != player.fighter.max_hp:
        if timer % 10 == 0:
            player.fighter.heal(1)
            timer += 1

def closest_monster(max_range):
    # Find closest enemy, up to a maximum range, and in the player's FOV
    closest_enemy = None
    closest_dist = max_range + 1  # Start with (slightly more than) maximum range

    for obj in objects:
        if obj.fighter and not obj == player and libtcod.map_is_in_fov(fov_map, obj.x, obj.y):
            # Calculate distance between this obj and the player
            dist = player.distance_to(obj)
            if dist < closest_dist:  # It's closer, so remember it
                closest_enemy = obj
                closest_dist = dist
    return closest_enemy

def create_h_tunnel(x1, x2, y):
    global world
    # Horizontal tunnel
    for x in range(min(x1, x2), max(x1, x2) + 1):
        world[x][y].blocked = False
        world[x][y].block_sight = False

def create_v_tunnel(y1, y2, x):
    global world
    # Vertical tunnel
    for y in range(min(y1, y2), max(y1, y2) + 1):
        world[x][y].blocked = False
        world[x][y].block_sight = False

def create_room(room):
    global world
    # go through the tiles in the rectangle and make them passable
    for x in range(room.x1 + 1, room.x2):
        for y in range(room.y1 + 1, room.y2):
            world[x][y].blocked = False
            world[x][y].block_sight = False

def debug_spawn_monster():
    options = []
    ids = []
    for item in monster_data:
        options.append(monster_data[item]["name"])
        ids.append(monster_data[item]["id"])

    # If an item was chosen, return it
    index = menu("Choose a monster", options, INVENTORY_WIDTH)

    if index is not None:
        mon_id = ids[index]

        monster = generate_monster(ids[index], player.x+2, player.y)

        # Add monster to object list
        objects.append(monster)

def debug_spawn_console():
    key = libtcod.Key()
    monster_name = ""
    message("Enter a monster name", libtcod.red)
    while True:
        # render the screen
        if libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS | libtcod.EVENT_MOUSE, key, mouse):
            key_char = chr(key.c)
            if key.vk == libtcod.KEY_ENTER:
                break
            if key_char:
                if key.vk == libtcod.KEY_BACKSPACE:
                    if monster_name != "":
                        monster_name = monster_name[:-1]
                else:
                    monster_name = "".join([monster_name, key_char])
                message(monster_name, libtcod.red)

        render_all()

        libtcod.console_flush()

    try:
        for item in monster_data:
            if monster_data[item]["name"] == monster_name or monster_data[item]["id"] == monster_name:
                monster = generate_monster(monster_data[item]["id"], player.x+2, player.y)
                # Add monster to object list
                objects.append(monster)
    except:
        message("Failure", libtcod.blue)

def debug_spawn_item():
    options = []
    ids = []
    for item in items_data:
        options.append(items_data[item]["name"])
        ids.append(items_data[item]["id"])

    # If an item was chosen, return it
    index = menu("Choose an item", options, INVENTORY_WIDTH)

    if index is not None:
        item_id = ids[index]

        item = generate_item(item_id, player.x+2, player.y)

        # Add monster to object list
        objects.append(item)

def debug_kill_all():
    for obj in objects:
        if obj.ai:
            obj.fighter.take_damage(3000000000000)

def fov_recompute():
    global world

    move_camera(player.x, player.y)

    # Recompute FOV if needed (the player moved or something)
    libtcod.map_compute_fov(fov_map, player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO)
    libtcod.console_clear(con)

    # Go through all tiles, and set their background color according to the FOV
    for y in range(CAMERA_HEIGHT):
        for x in range(CAMERA_WIDTH):
            (map_x, map_y) = (camera_x + x, camera_y + y)
            visible = libtcod.map_is_in_fov(fov_map, map_x, map_y)

            wall = world[map_x][map_y].block_sight

            if not visible:
                # if it's not visible right now, the player can only see it if it's explored
                if world[map_x][map_y].explored:
                    # It's out of the player's FOV
                    if wall:
                        libtcod.console_set_char_background(con, x, y, color_dark_wall, libtcod.BKGND_SET)
                    else:
                        libtcod.console_set_char_background(con, x, y, color_dark_ground, libtcod.BKGND_SET)
            else:
                # It's visible
                if wall:
                    libtcod.console_set_char_background(con, x, y, color_light_wall, libtcod.BKGND_SET)
                else:
                    libtcod.console_set_char_background(con, x, y, color_light_ground, libtcod.BKGND_SET)
                # Since it's visible, explore it
                world[map_x][map_y].explored = True

def from_dungeon_level(table):
    # Returns a value that depends on level. the table specifies what
    # value occurs after each level, default is 0.

    # In case if you couldn't figure out what that means here's an example:
    #   input ->        [[25, 6]]
    #          chance ----/    \----- beyond this dungeon level
    # All of the chances are totalled and then it's that chance out of that total
    # It sort of made sense when I first heard about it but now it's basically
    # just magic coding black box bullshit
    for (value, level) in reversed(table):
        if dungeon_level >= level:
            return value
    return 0

def game_over():
    # Wow you lost the game

    # Show some stats and shit who cares really
    msgbox('You Died!\n\n \
    Level: ' + str(player.level) + '\n \
    Experience: ' + str(player.fighter.xp) + '\n \
    Maximum HP: ' + str(player.fighter.max_hp) + '\n \
    Attack: ' + str(player.fighter.power) + '\n \
    Defense: ' + str(player.fighter.defense) + '\n \
    Total Kills: ' + str(kill_count) + '\n\n \
    Press any key to continue...',
    CHARACTER_SCREEN_WIDTH)

    exit()

def generate_monster(monster_id, x, y):
    # Read color
    color = json_get_color(monster_data[monster_id]["color"])

    # Select a death function
    if monster_data[monster_id]["death_func"] == "normal":
        death = monster_death
    else:
        print("Error: death function does not exist")
        exit()

    # Select an AI
    if monster_data[monster_id]["ai"] == "normal":
        ai = BasicMonster()
    elif monster_data[monster_id]["ai"] == "talk":
        ai = TalkingMonster(monster_data[monster_id]["speech"], monster_data[monster_id]["rate"])
    else:
        print("Error: ai does not exist")
        exit()

    """
    Example:
    # Create an orc
    fighter_component = Fighter(hp=20, defense=0, power=4, xp=35, death_function=monster_death)
    ai_component = BasicMonster()
    monster = Object(x, y, 'o', 'orc', libtcod.desaturated_green,
        blocks=True, fighter=fighter_component, ai=ai_component)
    """

    fighter_component = Fighter(hp=int(monster_data[monster_id]["hp"]), defense=int(monster_data[monster_id]["defense"]),
        power=int(monster_data[monster_id]["power"]), xp=int(monster_data[monster_id]["xp"]), mana=int(monster_data[monster_id]["mana"]), death_function=death)

    monster = Object(x, y, monster_data[monster_id]["char"], monster_data[monster_id]["name"], color,
        blocks=True, fighter = fighter_component, ai=ai)

    return monster

def generate_item(item_id, x, y):
    color = json_get_color(items_data[item_id]["color"])

    """
    Example:
    # Create a sword
    equipment_component = Equipment(slot='right hand', power_bonus=1)
    item = Object(x, y, '/', 'katana', libtcod.sky, equipment=equipment_component)

    * Items MUST use Item class and item_component
    * Equipmnt MUST use Equipment class and equip_component

    Please look at the json for more info on properties of both
    """

    if items_data[item_id]["type"] == "item":

        if items_data[item_id]["effect"] == "heal":
            item_component = Item(use_function=cast_heal)
        elif items_data[item_id]["effect"] == "fireball":
            item_component = Item(use_function=cast_fireball)
        elif items_data[item_id]["effect"] == "confuse":
            item_component = Item(use_function=cast_confuse)
        elif items_data[item_id]["effect"] == "lightning":
            item_component = Item(use_function=cast_lightning)
        elif items_data[item_id]["effect"] == "mana":
            item_component = Item(use_function=cast_mana)

        item = Object(x, y, items_data[item_id]["char"], items_data[item_id]["name"],
            color, item=item_component)

    elif items_data[item_id]["type"] == "equipment":
        equip_component = Equipment(slot=items_data[item_id]["slot"],
            power_bonus=items_data[item_id]["power"], defense_bonus=items_data[item_id]["defense"],
            max_hp_bonus=items_data[item_id]["hp"], max_mana_bonus=items_data[item_id]["mana"])

        item = Object(x, y, items_data[item_id]["char"], items_data[item_id]["name"],
            color, equipment=equip_component)

    elif items_data[item_id]["type"] == "gold":
        item = Object(x, y, items_data[item_id]["char"], items_data[item_id]["name"], color)

    return item

def get_equipped_in_slot(slot):
    # Returns the equipment in a slot, or None if it's empty
    for obj in inventory:
        if obj.equipment and obj.equipment.slot == slot and obj.equipment.is_equipped:
            return obj.equipment
    return None

def get_all_equipped(obj):
    # Returns a list of equipped items
    if obj == player:
        equipped_list = []
        for item in inventory:
            if item.equipment and item.equipment.is_equipped:
                equipped_list.append(item.equipment)
        return equipped_list
    else:
        return []  #other objects have no equipment

def get_gold(ammount):
    # Not sure how to implement gold
    player.gold += ammount

def get_names_under_mouse():
    global mouse

    # Return a string with the names of all objects under the mouse
    (x, y) = (mouse.cx, mouse.cy)
    (x, y) = (camera_x + x, camera_y + y)  # From screen to map coordinates

    # Create a list with the names of all objects at the mouse's coordinates and in FOV
    names = [obj.name for obj in objects
             if obj.x == x and obj.y == y and libtcod.map_is_in_fov(fov_map, obj.x, obj.y)]

    names = ", ".join(names)  # Join the names, separated by commas

    # Read Coords. Debug
    if COORDS_UNDER_MOUSE:
        names += "( " + str(x) + ", " + str(y) + " )"

    return names.capitalize()

def handle_keys():
    # Handle keypresses sent to the console. Executes other things, makes game playable
    global check_fov, game_state, objects, player_action, key, timer

    # Alt-Enter for Fullscreen
    if key.vk == libtcod.KEY_ENTER and key.lalt:
        libtcod.console_set_fullscreen(not libtcod.console_is_fullscreen())

    if game_state == "playing":
        # End game with escape
        if key.vk == libtcod.KEY_ESCAPE:
            player_action = "didnt-take-turn"
            save_game()

        key_char = chr(key.c)

        # Movement keys
        if key_char in ("1", "2", "3", "4", "6", "7", "8", "9"):
            if key_char == "8":
                player_move(0, -1)
            elif key_char == "2":
                player_move(0, 1)
            elif key_char == "4":
                player_move(-1, 0)
            elif key_char == "6":
                player_move(1, 0)
            elif key_char == "7":
                player_move(-1, -1)
            elif key_char == "9":
                player_move(1, -1)
            elif key_char == "1":
                player_move(-1, 1)
            elif key_char == "3":
                player_move(1, 1)

            check_fov = True

            for obj in objects:  # Look for an item in the player's tile
                if obj.x == player.x and obj.y == player.y and obj.item:
                    message(" ".join(["You see a", obj.name, "here."]), libtcod.white)

            player_action = "move"

        elif key_char in ["5", "."]:
            check_fov = True
            message("You wait", libtcod.gray)
            player_action = "wait"

        elif key_char == "g":
            # Pick up an item
            for obj in objects:  # Look for an item in the player's tile
                if obj.x == player.x and obj.y == player.y and obj.item:
                    obj.item.pick_up()
                    player_action = 'pickup'
                    break
            else:
                message("There is nothing there to pick up", libtcod.gray)
                player_action = "didnt-take-turn"

        elif key_char == "i":
            # Show the inventory
            chosen_item = inventory_menu("Press the key next to an item to use it, or any other to cancel.\n")
            if chosen_item is not None:
                chosen_item.use()
                player_action = 'use'

        elif key_char == "d":
            # Show the inventory; if an item is selected, drop it
            chosen_item = inventory_menu('Press the key next to an item to drop it, or any other to cancel.\n')
            if chosen_item is not None:
                chosen_item.drop()
                player_action = 'drop'

        # Reset the map (DEBUG)
        elif key_char == "r":
            # Empty objects and re-add the player so the game is playable
            objects = []
            objects.insert(0, player)

            # Clear screen
            for x in range(SCREEN_WIDTH):
                for y in range(SCREEN_HEIGHT):
                    libtcod.console_put_char(con, x, y, " ", libtcod.BKGND_BURN)

            # Make a new map
            make_map()
            fov_recompute()
            player_action = "didnt-take-turn"

        elif key_char == ">":
            #go down stairs, if the player is on them
            if (dstairs.x == player.x and dstairs.y == player.y) or STAIR_HACK:
                next_level()

        elif key_char == "<":
            #go up stairs, if the player is on them
            if (ustairs.x == player.x and ustairs.y == player.y) or STAIR_HACK:
                previous_level()

        elif key_char == 'c':
            #show character information
            level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR
            msgbox('Character Information\n\nLevel: ' + str(player.level) + '\nExperience: ' + str(player.fighter.xp) +
                '\nExperience to level up: ' + str(level_up_xp) + '\n\nMaximum HP: ' + str(player.fighter.max_hp) +
                '\nAttack: ' + str(player.fighter.power) + '\nDefense: ' + str(player.fighter.defense), CHARACTER_SCREEN_WIDTH)

        elif key_char == 'b':
            debug_spawn_monster()
            player_action = 'didnt-take-turn'

        elif key_char == 'v':
            debug_kill_all()
            player_action = 'didnt-take-turn'

        elif key_char == 'h':
            debug_spawn_item()
            player_action = 'didnt-take-turn'

        elif key_char == 'l':
            debug_spawn_console()
            player_action = 'didnt-take-turn'

        elif key_char == "m":
            status = player.fighter.magic_missile()
            if status != "cancelled":
                player_action = "casting"
            else:
                player_action = "didnt-take-turn"

        else:
            player_action = "didnt-take-turn"

def initialize_fov():
    global check_fov, fov_map
    check_fov = True

    # create the FOV map, according to the generated map
    fov_map = libtcod.map_new(MAP_WIDTH, MAP_HEIGHT)
    for y in range(MAP_HEIGHT):
        for x in range(MAP_WIDTH):
            libtcod.map_set_properties(fov_map, x, y, not world[x][y].block_sight, not world[x][y].blocked)

def inventory_menu(header):
    # Show a menu with each item of the inventory as an option
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
    # First test the map tile
    if world[x][y].blocked:
        return True

    # Now check for any blocking objects
    for obj in objects:
        if obj.blocks and obj.x == x and obj.y == y:
            return True

    return False

def json_get_color(color_str):
    # Get some swag ass colors from a string to some color. mind blow
    colors = {
        "desaturated_green": libtcod.desaturated_green,
        "darker_green": libtcod.darker_green,
        "lime": libtcod.lime,
        "violet": libtcod.violet,
        "light_yellow": libtcod.light_yellow,
        "sky": libtcod.sky,
        "azure": libtcod.azure,
        "light_turquoise": libtcod.light_turquoise,
        "light_sea": libtcod.light_sea,
        "light_red": libtcod.light_red,
        "dark_gray": libtcod.dark_gray,
        "light_green": libtcod.light_green,
        "gray": libtcod.gray,
        "darker_orange": libtcod.darker_orange,
        "silver": libtcod.silver,
        "white": libtcod.white,
        "dark_crimson": libtcod.dark_crimson,
        "crimson": libtcod.crimson,
        "chartreuse": libtcod.chartreuse
    }

    return colors[color_str]

def letter_to_number(letter):
    number = ord(letter) - 96
    return number

def load_game():
    # Open the previously saved shelve and load the game data
    # I have no idea how this shelve shit works but it's magic
    global world, objects, player, inventory, game_msgs, game_state, dungeon_level, dstairs, ustairs

    file = shelve.open('savegame', 'r')
    world = file['world']
    objects = file['objects']
    player = objects[file['player_index']]  # Get index of player in objects list and access it
    inventory = file['inventory']
    game_msgs = file['game_msgs']
    game_state = file['game_state']
    dstairs = objects[file['dstairs_index']]
    ustairs = objects[file['ustairs_index']]
    dungeon_level = file['dungeon_level']
    kill_count = file['kill_count']
    file.close()

    initialize_fov()

def main_menu():
    img = libtcod.image_load('images/menu_background.png')

    while not libtcod.console_is_window_closed():
        # Show the background image, at twice the regular console resolution
        libtcod.image_blit_2x(img, 0, 0, 0)

        # Show the game's title, and some credits!
        libtcod.console_set_default_foreground(0, libtcod.light_yellow)
        libtcod.console_print_ex(0, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 4, libtcod.BKGND_NONE, libtcod.CENTER,
                                 'Edgequest')
        libtcod.console_print_ex(0, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 + 3, libtcod.BKGND_NONE, libtcod.CENTER,
                                 'What hath God wrought?')

        libtcod.console_set_default_foreground(0, libtcod.black)
        libtcod.console_print_ex(0, SCREEN_WIDTH / 2, SCREEN_HEIGHT - 2, libtcod.BKGND_NONE, libtcod.CENTER,
                                 'By Gray')

        # Show options and wait for the player's choice
        choice = menu('Options', ['Play a new game', 'Continue last game', 'Quit'], 24)

        if choice == 0:  # New game
            new_game()
            play_game()
        if choice == 1:  # Load last game
            try:
                load_game()
            except:
                msgbox('\n No saved game to load.\n', 24)
                continue
            play_game()
        elif choice == 2:  # Quit
            exit()

def make_map():
    # Make a map
    global world, fov_map, objects, dstairs, ustairs

    # The list of objects with just the player
    objects = [player]

    # fill map with "blocked" tiles
    world = [[Tile(True) for y in range(MAP_HEIGHT)] for x in range(MAP_WIDTH)]

    rooms = []
    num_rooms = 0

    # Rev up those map generators
    themap = dMap()

    # Okay this takes some magic to get working but once you do you can create
    # a ton of cool maps with it.
    #
    # the first two values are the dimensions of the map (x and y for all
    # you stupid people out there). The second one is the "fail" rating
    # Not sure what the fuck that means but the higher it is, the more rooms you get
    # Then the fourth is the "b1" value. Like what the fuck is a b1?
    # Apparently it controlls the frequency of corridors
    # Corridors are shit so I keep it at 1
    # Lastly, the number of maximum rooms. Multiply the max_rooms by 4 because
    # the rooms are pretty small
    themap.makeMap(MAP_WIDTH,MAP_HEIGHT,250,1,MAX_ROOMS*4)

    # Turn ones and zeros into magic
    for y in range(MAP_HEIGHT):
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

    # Put shit everywhere
    place_objects()

    # Make an FOV map
    fov_map = libtcod.map_new(MAP_WIDTH, MAP_HEIGHT)

    for y in range(MAP_HEIGHT):
        for x in range(MAP_WIDTH):
            libtcod.map_set_properties(fov_map, x, y, not world[x][y].block_sight, not world[x][y].blocked)

    # Create stairs at some random ass location
    x = libtcod.random_get_int(0,0, MAP_WIDTH-1)
    y = libtcod.random_get_int(0,0, MAP_HEIGHT-1)

    while (world[x][y].blocked):
        x = libtcod.random_get_int(0,0, MAP_WIDTH-1)
        y = libtcod.random_get_int(0,0, MAP_HEIGHT-1)

    dstairs = Object(x, y, '>', 'down stairs', libtcod.white, always_visible=True)
    objects.append(dstairs)
    dstairs.send_to_back()  #so it's drawn below the monsters


    # Same for player
    x = libtcod.random_get_int(0,0, MAP_WIDTH-1)
    y = libtcod.random_get_int(0,0, MAP_HEIGHT-1)

    while (world[x][y].blocked):
        x = libtcod.random_get_int(0,0, MAP_WIDTH-1)
        y = libtcod.random_get_int(0,0, MAP_HEIGHT-1)

    player.x = x
    player.y = y

    # Make stairs going up because why not
    ustairs = Object(player.x, player.y, '<', 'up stairs', libtcod.white, always_visible=True)
    objects.append(ustairs)
    ustairs.send_to_back()  #so it's drawn below the monsters

def menu(header, options, width):
    if len(options) > 26: raise ValueError('Cannot have a menu with more than 26 options.')

    # Calculate total height for the header (after auto-wrap) and one line per option
    header_height = libtcod.console_get_height_rect(con, 0, 0, width, SCREEN_HEIGHT, header)
    height = len(options) + header_height

    # Create an off-screen console that represents the menu's window
    window = libtcod.console_new(width, height)

    # Print the header, with auto-wrap
    libtcod.console_set_default_foreground(window, libtcod.white)
    libtcod.console_print_rect_ex(window, 0, 0, width, height, libtcod.BKGND_NONE, libtcod.LEFT, header)

    # Print all the options
    y = header_height
    letter_index = ord('a')
    for option_text in options:
        text = '(' + chr(letter_index) + ') ' + option_text
        libtcod.console_print_ex(window, 0, y, libtcod.BKGND_NONE, libtcod.LEFT, text)
        y += 1
        letter_index += 1

    # Blit the contents of "window" to the root console
    x = SCREEN_WIDTH / 2 - width / 2
    y = SCREEN_HEIGHT / 2 - height / 2
    libtcod.console_blit(window, 0, 0, width, height, 0, x, y, 1.0, 0.7)

    # Present the root console to the player and wait for a key-press
    libtcod.console_flush()
    key = libtcod.console_wait_for_keypress(True)

    # Convert the ASCII code to an index; if it corresponds to an option, return it
    index = key.c - ord('a')
    if 0 <= index < len(options):
        return index
    else:
        return None

def msgbox(text, width=50):
    menu(text, [], width)  # use menu() as a sort of "message box"

def message(new_msg, color=libtcod.white):
    global old_msg, msg_counter

    # If the same message is going to be re-outputted, add a convenient counter
    if old_msg == new_msg:
        msg_counter += 1
        alt_msg = "".join([new_msg, " <x", str(msg_counter), ">"])
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
            # If the buffer is full, remove the first line to make room for the new one
            if len(game_msgs) == MSG_HEIGHT:
                del game_msgs[0]

            # Add the new line as a tuple, with the text and the color
            game_msgs.append((line, color))

    # Store old message for comparison later
    old_msg = new_msg

def monster_death(monster):
    # transform it into a nasty corpse! it doesn't block, can't be
    # Attacked and doesn't move
    message(" ".join([monster.name.capitalize(), "is dead!"]), libtcod.darker_red)
    message('You gain ' + str(monster.fighter.xp) + ' experience points.', libtcod.orange)
    monster.char = '%'
    monster.color = libtcod.dark_red
    monster.blocks = False
    monster.fighter = None
    monster.ai = None
    monster.send_to_back()
    monster.name = " ".join(["remains of", monster.name])

def monster_occupy_check(dx, dy):
    # If a monster is in that location, return true
    for obj in objects:
        if (obj.x, obj.y) == (dx, dy) and obj.blocks:
            return True
    return False

def move_camera(target_x, target_y):
    global camera_x, camera_y, check_fov

    # New camera coordinates (top-left corner of the screen relative to the map)
    x = target_x - CAMERA_WIDTH / 2  # Coordinates so that the target is at the center of the screen
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
    global player, edge, inventory, game_msgs, game_state, dungeon_level, monster_data, items_data

    # Player
    # create object representing the player
    fighter_component = Fighter(hp=100, defense=1, power=4, xp=0, mana=100, death_function=player_death,)
    player = Object(0, 0, '@', 'player', libtcod.white, blocks=True, fighter=fighter_component)

    player.level = 1

    # Initialize dungeon level
    dungeon_level = 1

    # Generate map (at this point it's not drawn to the screen)
    make_map()
    initialize_fov()

    game_state = 'playing'
    inventory = []

    # Initial equipment: a dagger
    equipment_component = Equipment(slot='right hand', power_bonus=1)
    obj = Object(0, 0, '-', 'dagger', libtcod.sky, equipment=equipment_component)
    inventory.append(obj)
    equipment_component.equip()
    obj.always_visible = True

    # Create the list of game messages and their colors, starts empty
    game_msgs = []

    # A warm welcoming message!
    message("Welcome!", libtcod.lighter_yellow)

def next_level():
    global dungeon_level, max_dungeon_level

    dungeon_level += 1

    message("After a rare moment of peace, you descend deeper into the heart of the dungeon...", libtcod.red)
    make_map()  # Create a fresh new level!
    initialize_fov()

def place_objects():
    # Maximum number of monsters per level
    max_monsters = from_dungeon_level([[25, 1], [30, 4], [40, 6]])

    # Chance of each monster
    monster_chances = {}
    for item in monster_data: # Monster name then chance. Chance works like this [chance, from this level], [chance, from this level]
        monster_chances[str(monster_data[item]["id"])] = from_dungeon_level(monster_data[item]["chance"])

    # Maximum number of items per level
    max_items = from_dungeon_level([[15, 1], [20, 4]])

    # Chance of each item (by default they have a chance of 0 at level 1, which then goes up)
    item_chances = {}
    for item in items_data: # Item name then chance. Chance works like this [chance, from this level], [chance, from this level]
        item_chances[str(items_data[item]["id"])] = from_dungeon_level(items_data[item]["chance"])

    # Choose random number of monsters
    num_monsters = libtcod.random_get_int(0, 0, max_monsters)

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
    num_items = libtcod.random_get_int(0, 0, max_items)

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
            item.send_to_back()  # Items appear below other objects
            item.always_visible = True  # Items are visible even out-of-FOV, if in an explored area

def play_game():
    global key, mouse, player_action, timer

    player_action = None

    mouse = libtcod.Mouse()
    key = libtcod.Key()

    while not libtcod.console_is_window_closed():
        # render the screen
        libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS | libtcod.EVENT_MOUSE, key, mouse)
        render_all()

        libtcod.console_flush()

        check_level_up()

        check_timer()

        # Erase all objects at their old locations, before they move
        for obj in objects:
            obj.clear()

        # Handle keys
        handle_keys()

        # Let monsters take their turn
        if game_state == 'playing' and player_action != 'didnt-take-turn':
            for obj in objects:
                if obj.ai:
                    obj.ai.take_turn()
            timer += 1

def player_death(player):
    # The game ended!
    global game_state
    if not GOD_MODE:
        message("You died!", libtcod.dark_red)

        # For added effect, transform the player into a corpse!
        player.char = "%"
        player.color = libtcod.dark_red

        game_state = "dead"

        render_all()
        libtcod.console_flush()

        game_over()

def player_move(dx, dy):
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
    global dungeon_level
    # In case if you're that guy who likes going back for some reason

    dungeon_level -= 1

    if dungeon_level == 0:
        choice = menu('Leave the Dungeon?', ['Yes', 'No'], 30)

        if choice == 0:
            game_over()
        else:
            render_all()
            libtcod.console_flush()
            choice = menu('You head back down into the depths...', ['Continue'], 30)

    else:
        message("After a rare moment of peace, you descend ascend upwards towards the surface...", libtcod.red)
        make_map()  # Create a fresh new level!
        initialize_fov()

def random_choice(chances_dict):
    # Choose one option from dictionary of chances, returning its key
    chances = chances_dict.values()
    strings = chances_dict.keys()

    return strings[random_choice_index(chances)]

def random_choice_index(chances):
    # Choose one option from list of chances, returning its index

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
    # Draw everything to the screen
    global check_fov, camera_x, camera_y

    move_camera(player.x, player.y)

    if check_fov:
        check_fov = False
        fov_recompute()

    for obj in objects:
        obj.draw()

    # Draw all objects in the list, except the player. we want it to
    # Always appear over all other objects! so it's drawn later.
    for obj in objects:
        if obj != player:
            obj.draw()
    player.draw()

    # blit the contents of "con" to the root console
    libtcod.console_blit(con, 0, 0, MAP_WIDTH, MAP_HEIGHT, 0, 0, 0)

    # Prepare to render the GUI panel
    libtcod.console_set_default_background(panel, libtcod.black)
    libtcod.console_clear(panel)

    # Also the message panel
    libtcod.console_set_default_background(msg_panel, libtcod.black)
    libtcod.console_clear(msg_panel)

    # Show the player's stats
    # Health bar
    render_bar(1, 1, BAR_WIDTH, "HP", player.fighter.hp, player.fighter.max_hp,
               libtcod.light_red, libtcod.darker_red)

    # Mana bar
    render_bar(1, 2, BAR_WIDTH, "Edge", player.fighter.mana, player.fighter.max_mana,
        libtcod.dark_fuchsia, libtcod.darker_fuchsia)

    # Show the level too, because I'm in a giving mood
    render_bar_simple(1, 3, BAR_WIDTH, "Level", str(dungeon_level), libtcod.light_blue)

    # Show stats
    render_bar_simple(1, 5, BAR_WIDTH, "Attack", str(player.fighter.power), libtcod.dark_chartreuse)
    render_bar_simple(1, 6, BAR_WIDTH, "Defense", str(player.fighter.defense), libtcod.flame)

    # This is actually kind of cool. It shows all the monsters that the player
    # can see and shows their health
    monsters_in_room = 0
    for obj in objects:
        if libtcod.map_is_in_fov(fov_map, obj.x, obj.y) and obj.fighter and obj.name != "player":
            monsters_in_room += 1
            libtcod.console_set_default_foreground(panel, obj.color)
            libtcod.console_print_ex(panel, 1, 6+(2*monsters_in_room), libtcod.BKGND_NONE,
                libtcod.LEFT, "".join([obj.char, " ", obj.name]))
            render_health_bar(1, 7+(2*monsters_in_room), BAR_WIDTH, obj.fighter.hp,
                obj.fighter.base_max_hp, libtcod.red, libtcod.dark_red)

    # Display names of objects under the mouse
    libtcod.console_set_default_foreground(msg_panel, libtcod.light_gray)
    libtcod.console_print_ex(msg_panel, 1, 0, libtcod.BKGND_NONE, libtcod.LEFT,
        get_names_under_mouse())

    # Print the game messages, one line at a time
    y = 1
    for (line, color) in game_msgs:
        libtcod.console_set_default_foreground(msg_panel, color)
        libtcod.console_print_ex(msg_panel, MSG_X, y, libtcod.BKGND_NONE, libtcod.LEFT, line)
        y += 1

    # Blit the contents of "panel" and "msg_panel" to the root console
    libtcod.console_blit(panel, 0, 0, PANEL_WIDTH, PANEL_HEIGHT, 0, SCREEN_WIDTH-PANEL_WIDTH, PANEL_Y)
    libtcod.console_blit(msg_panel, 0, 0, SCREEN_WIDTH, PANEL_HEIGHT, 0, 0, MSG_PANEL_Y)

def render_bar(x, y, total_width, name, value, maximum, bar_color, back_color):
    # Render a bar (HP, experience, etc). first calculate the width of the bar
    bar_width = int(float(value) / maximum * total_width)

    # Render the background first
    libtcod.console_set_default_background(panel, back_color)
    libtcod.console_rect(panel, x, y, total_width, 1, False, libtcod.BKGND_SCREEN)

    # Now render the bar on top
    libtcod.console_set_default_background(panel, bar_color)
    if bar_width > 0:
        libtcod.console_rect(panel, x, y, bar_width, 1, False, libtcod.BKGND_SCREEN)

    # Finally, some centered text with the values
    libtcod.console_set_default_foreground(panel, libtcod.white)
    libtcod.console_print_ex(panel, x + total_width / 2, y, libtcod.BKGND_NONE, libtcod.CENTER,
                             name + ": " + str(value) + "/" + str(maximum))

def render_bar_simple(x, y, total_width, name, value, color):
    # I wanted a bar that just has one thing in it without all that fancy shit

    # Render the background first
    libtcod.console_set_default_background(panel, color)
    libtcod.console_rect(panel, x, y, total_width, 1, False, libtcod.BKGND_SCREEN)

    # Now render the bar on top
    libtcod.console_set_default_background(panel, color)
    if total_width > 0:
        libtcod.console_rect(panel, x, y, total_width, 1, False, libtcod.BKGND_SCREEN)

    # Finally, some centered text with the values
    libtcod.console_set_default_foreground(panel, libtcod.white)
    libtcod.console_print_ex(panel, x + total_width / 2, y, libtcod.BKGND_NONE, libtcod.CENTER,
                             name + ": " + str(value))

def render_health_bar(x, y, total_width, value, maximum, bar_color, back_color):
    # This is a bar that doesn't show any values in it. #Minimalism

    # Render a bar (HP, experience, etc). first calculate the width of the bar
    bar_width = int(float(value) / maximum * total_width)

    # Render the background first
    libtcod.console_set_default_background(panel, back_color)
    libtcod.console_rect(panel, x, y, total_width, 1, False, libtcod.BKGND_SCREEN)

    # Now render the bar on top
    libtcod.console_set_default_background(panel, bar_color)
    if bar_width > 0:
        libtcod.console_rect(panel, x, y, bar_width, 1, False, libtcod.BKGND_SCREEN)

    # Finally, some centered text with the values
    libtcod.console_set_default_foreground(panel, libtcod.white)
    libtcod.console_print_ex(panel, x + total_width / 2, y, libtcod.BKGND_NONE, libtcod.CENTER, "")

def save_game():
    choice = menu('Save and Quit?', ['Yes', 'No'], 24)

    if choice == 0:  # Save and Quit
        # Open a new empty shelve (possibly overwriting an old one) to write the game data
        file = shelve.open('savegame', 'n')
        file['world'] = world
        file['objects'] = objects
        file['player_index'] = objects.index(player)  # Index of player in objects list
        file['inventory'] = inventory
        file['game_msgs'] = game_msgs
        file['game_state'] = game_state
        file['dstairs_index'] = objects.index(dstairs)
        file['ustairs_index'] = objects.index(ustairs)
        file['dungeon_level'] = dungeon_level
        file['kill_count'] = kill_count
        file.close()
        render_all()
        libtcod.console_flush()
        choice = menu('Bye!', [], 6)
        exit()
    elif choice == 1:  # Get me out of here!
        pass

def sort_inventory():
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
        # Sweet jesus up in heaven i have no fucking clue what black magic this lambda
        # shit is but lordy oh lordy it's beautiful
    equips = sorted(equips, key=lambda obj: obj.name)
    items = sorted(items, key=lambda obj: obj.name)

    # Then put equipment first, followed by items
    inventory = equips + items

def target_monster(max_range=None):
    # Returns a clicked monster inside FOV up to a range, or None if right-clicked
    while True:
        (x, y) = target_tile(max_range)

        if x is None:  # Player cancelled
            return None

        # Return the first clicked monster, otherwise continue looping
        for obj in objects:
            if obj.x == x and obj.y == y and obj.fighter and obj != player:
                return obj

def target_tile(max_range=None):
    # Return the position of a tile left-clicked in player's FOV
    # (optionally in a range), or (None,None) if right-clicked.
    global key, mouse
    while True:
        # Render the screen. this erases the inventory and shows the names of objects under the mouse.
        libtcod.console_flush()
        libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS | libtcod.EVENT_MOUSE, key, mouse)
        render_all()
        (x, y) = (mouse.cx, mouse.cy)
        (x, y) = (camera_x + x, camera_y + y)  # From screen to map coordinates


        if mouse.rbutton_pressed or key.vk == libtcod.KEY_ESCAPE:
            return (None, None)  # Cancel if the player right-clicked or pressed Escape

        # Accept the target if the player clicked in FOV, and in case a range is specified, if it's in that range
        if (mouse.lbutton_pressed and libtcod.map_is_in_fov(fov_map, x, y) and
                (max_range is None or player.distance(x, y) <= max_range)):
            return x, y

def to_camera_coordinates(x, y):
    # Magic.
    # convert coordinates on the map to coordinates on the screen
    (x, y) = (x - camera_x, y - camera_y)

    if x < 0 or y < 0 or x >= CAMERA_WIDTH or y >= CAMERA_HEIGHT:
        return None, None  # If it's outside the view, return nothing

    return x, y

######################################
# Objects
######################################

# Player object
player = None

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
# !!! Implement user font selection
libtcod.console_set_custom_font("images/terminal8x12_gs_tc.png",
    libtcod.FONT_TYPE_GREYSCALE | libtcod.FONT_LAYOUT_TCOD)

# Initialize root console
libtcod.console_init_root(SCREEN_WIDTH, SCREEN_HEIGHT, "Edgequest Pre-Alpha", False)

# And another
con = libtcod.console_new(MAP_WIDTH, MAP_HEIGHT)

# And one for a player-centered focus
dcon = libtcod.console_new(SCREEN_WIDTH, SCREEN_HEIGHT)

# FPS Limit (Not Essential)
libtcod.sys_set_fps(LIMIT_FPS)

# Mouse and Keyboard detection
mouse = libtcod.Mouse()
key = libtcod.Key()

# Camera coordinates
(camera_x, camera_y) = (0, 0)

######################################
# Main Loop
######################################

main_menu()
