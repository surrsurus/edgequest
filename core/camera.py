# Camera.py

# Move the camera view of the world

# Imports ----------------------------------------------------------------------

# You only need to import the camera settings
from settings.settings import MAP_WIDTH, MAP_HEIGHT, CAMERA_WIDTH, CAMERA_HEIGHT

# ------------------------------------------------------------------------------

# Global variables -------------------------------------------------------------

# Camera coordinates
(x, y) = (0, 0)

# Check fov
check_fov = True

# ------------------------------------------------------------------------------

# Functions --------------------------------------------------------------------

def get_names_under_mouse(mouse, objects, debug=False):
    ''' Self explanatory name
    Step 1. Move mouse over map, item, tile, player, etc
    Step 2. See name
    '''

    # Return a string with the names of all objects under the mouses
    # From screen to map coordinates
    (tx, ty) = (x + mouse.cx, y + mouse.cy)

    # Create a list with the names of all objects at the mouse's
    # coordinates and in FOV
    names = [obj.name for obj in objects if obj.x == tx and obj.y == ty]

    # Join the names, separated by commas
    names = ', '.join(names)

    # Read Coords. Debug
    if debug:
        # Coordinate string
        names += '( ' + str(tx) + ', ' + str(ty) + ' )'
        for obj in objects:
            if obj.x == tx and obj.y == ty:
                if obj.ai:
                    # Also show where it's headed to if a monster
                    tx, ty = obj.ai.backup_coord
                    names += ' going to: ( ' + str(tx) + ', ' + str(ty) + ' )'

    if names:
        return '['+names.capitalize()+']'
    else:
        return ''

def move(target_x, target_y):
    ''' Move camera to coordinates '''

    global x, y, check_fov

    # New camera coordinates (top-left corner of the screen relative to the map)
    # Coordinates so that the target is at the center of the screen
    tx = target_x - CAMERA_WIDTH / 2
    ty = target_y - CAMERA_HEIGHT / 2

    # Make sure the camera doesn't see outside the map
    if tx < 0:
        tx = 0
    if ty < 0:
        ty = 0
    if tx > MAP_WIDTH - CAMERA_WIDTH - 1:
        tx = MAP_WIDTH - CAMERA_WIDTH - 1
    if ty > MAP_HEIGHT - CAMERA_HEIGHT - 1:
        ty = MAP_HEIGHT - CAMERA_HEIGHT - 1

    if tx != x or ty != y:
        check_fov = True

    (x, y) = (tx, ty)

def to_coords(target_x, target_y):
    ''' convert coordinates on the map to coordinates on the screen '''

    (tx, ty) = (target_x - x, target_y - y)

    # If it's outside the view, return nothing
    if tx < 0 or ty < 0 or tx >= CAMERA_WIDTH or ty >= CAMERA_HEIGHT:
        return None, None

    return tx, ty
