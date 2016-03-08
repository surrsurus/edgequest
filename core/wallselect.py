# Wallselect.py

# Wall decorations

'''

Get random characters to display on top of walls in EdgeQuest

* wallselect - takes the world, an x y coordinate pair, and returns a character
based on the surrounding walls that are connected

'''

# wallselect function ----------------------------------------------------------

def wallselect(world, x, y):

    # 4 Way Intersection
    if world[x][y+1].block_sight and world[x][y-1].block_sight and \
    world[x-1][y].block_sight and world[x+1][y].block_sight:
        return '+'
    # Four 3 Way Intersections
    elif world[x][y+1].block_sight and world[x][y-1].block_sight and \
    world[x-1][y].block_sight:
        return ' '
    elif world[x][y+1].block_sight and world[x][y-1].block_sight and \
    world[x+1][y].block_sight:
        return ' '
    elif world[x][y+1].block_sight and \
    world[x-1][y].block_sight and world[x+1][y].block_sight:
        return ' '
    elif world[x][y-1].block_sight and \
    world[x-1][y].block_sight and world[x+1][y].block_sight:
        return ' '

    # Two 2 Way Intersections
    elif world[x][y+1].block_sight and world[x][y-1].block_sight:
        return '|'
    elif world[x+1][y].block_sight and world[x-1][y].block_sight:
        return '-'

    # Four Corners
    elif world[x][y+1].block_sight and world[x-1][y].block_sight:
        return 'o'
    elif world[x][y-1].block_sight and world[x+1][y].block_sight:
        return 'o'
    elif world[x+1][y].block_sight and world[x][y+1].block_sight:
        return 'o'
    elif world[x-1][y].block_sight and world[x][y+1].block_sight:
        return 'o'

    # Pillars/Columns
    else:
        return '#'

# ------------------------------------------------------------------------------
