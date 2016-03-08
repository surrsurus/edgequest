# Fortune.py

# Fortune generator for the fortune cookie item

'''

This file contains a list with random rumours and quotes and a function to
randomly return one of these entries with quotes around it.

* get_fortune - get a random fortune from the fortunes list

'''

# Imports ----------------------------------------------------------------------

from random import choice

# ------------------------------------------------------------------------------

# Fortune list -----------------------------------------------------------------

fortunes = [
    '...when fits of creativity run strong, more than one programmer or writer has been known to abandon the desktop for the more spacious floor. -Fred Brooks, Jr.',
    '80 percent of all statistics are made up on the spot, including this one.',
    'fortune: segmentation fault core dumped',
    '99% of all guys are within one standard deviation of your mom.',
    'A LISP programmer knows the value of everything, but the cost of nothing.',
    'Any given program, when running correctly, is obsolete.',
    'C++ : Where friends have access to your private members. -Gavin Russell Baker',
    'Documentation is the castor oil of programming. Managers know it must be good because the programmers hate it so much.',
    'Q. How many programmers does it take to change a light bulb? A. None. It\'s a hardware problem.',
    'If Java had true garbage collection, most programs would delete themselves upon execution. -Robert Sewell',
    'A man who turns green has eschewed protein.',
    'A mathematician is a machine for converting coffee into theorems.',
    'Abandon hope, all ye who press "ENTER" here.',
    'All that glitters has a high refractive index.',
    'All true wisdom is found on T-shirts.',
    'An effective way to deal with predators is to taste terrible.',
    'Did you know gullible is not in the dictionary?',
    'Sorry, no fortune this time. Better luck next cookie!',
    'Elbereth has quite a reputation around these parts.',
    'Consumption of home-made food is strictly forbidden in this dungeon.',
    'Didn\'t your mother tell you not to eat food off the floor?',
    'Help!  I\'m being held prisoner in a fortune cookie factory!',
    "It's not tax evasion if it's not illegal",
    'They say that fortune cookies are food for thought.',
    'Ever find a bomb? I hear it\'s a great one-use item!',
    'They say bombs must be planted at bomb sites.',
    'If you\'re lucky, you might find a pre-nerf R8',
    'They say you should trust fortune cookies!',
    'They say you should never trust fortune cookies!',
]

# ------------------------------------------------------------------------------

# Fortune function -------------------------------------------------------------

def get_fortune():
    ''' Get a random fortune from the list and return it with surrounding quotes '''
    return '    "' + choice(fortunes) + '"'

# ------------------------------------------------------------------------------
