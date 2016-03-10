# Struct_proc.py

# Structure processer

# Imports ----------------------------------------------------------------------

import random
from index import *

# ------------------------------------------------------------------------------

# Functions --------------------------------------------------------------------

def get_struct():
    ''' Gets a random structure and returns a 2d list '''

    # Open random structure file
    file = random.choice(STRUCTURE_INDEX)
    file = open(file, 'r')

    struct = []
    # Parse by line
    for line in file:
        row = []
        # And by character
        for char in list(line):
            # Skipping over newlines
            if char != '\n':
                row.append(char)
        # Then stitching it into a 2d list
        struct.append(row)

    # Return a 2d list, randomly rotated
    return (rotate(random.randint(0,3), struct), file.name)

def rotate(times, list):
    ''' Rotate a 2d array '''
    for i in range(times):
        list = zip(*list[::-1])
    return list


# ------------------------------------------------------------------------------
