# edgequest
Roguelike in Python based on the [Roguebasin Tutorial](http://www.roguebasin.com/index.php?title=Complete_Roguelike_Tutorial,_using_python%2Blibtcod) but with several key differences, namely being the feature to store items and monsters inside of JSON files, making modular additions possible without needing to alter the python source in any way.

Some say making a roguelike in python is pointless, due to the slow speeds that may come as a part of constant A* pathfinding, FOV, djikstra maps, and dynamic lighting, but to the naysayers I say "Eh, you're probably right"

Utilizes the [simplejson python library](https://github.com/simplejson/simplejson)

## Requirements
1. Python 2.7
2. Enough edge to cut yourself on
3. 10 minutes of free time

## Run Edgequest

#### Linux (Debian-based)

It should be as simple as running the `run` script included with the repository

All imports are from the standard library (with the exception of simplejson, included with edgequest), so you should have no problem with running this on a linux machine

If you get an SDL error along the lines of `libsdl1.2.0 not found` try:

`sudo apt-get --reinstall libsdl1.2debian`

Max tried to run the code in Manjaro Linux and it didn't work so there is an issue with pacman-based distros.

#### Windows

Currently does not run on windows, but you will need the 32 bit version of python2.7 to start.

#### Mac OS

Kevin playtested it on his Macbook Air and he used a live USB of linux mint.
Considering he's the only one I know with a mac, and he found a way to circumvent this problem, there is no planned mac version.

## Wiki

Please check the [wiki](https://github.com/TriangularEgg/edgequest/wiki) for further information
