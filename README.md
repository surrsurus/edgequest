# edgequest

EdgeQuest is a 'roguelike' in Python based on the [Roguebasin Tutorial](http://www.roguebasin.com/index.php?title=Complete_Roguelike_Tutorial,_using_python%2Blibtcod) but with several key differences, namely being the feature to store items and monsters inside of JSON files, making modular additions possible without needing to alter the python source in any way.

A roguelike is a game featuring perma-death, random map generation, turn-based combat, and RPG elements.

Some say making a roguelike in python is pointless, due to the slow speeds that may come as a part of constant A* pathfinding, FOV, djikstra maps, and dynamic lighting, but to the naysayers I say "Eh, you're probably right"

# Note

I am no longer continuing to work on EdgeQuest at the moment. The current master and release work completely fine and are bug free (mostly). Edgequest will return shortly, but until then, enjoy the game!

# Why EdgeQuest?

There are certainly other much better roguelikes out there that are faster, pack more features, have less bugs, and written in faster languages. EdgeQuest is just something fun for me to do that other people like, because it's humorous and has some level of actual playability and enjoyment created from ASCII text. Also I don't know about any other roguelike that lets you dual weild an AWP and an M9 bayonet whilst chugging mountain dew and tazing John Cena.


## Requirements

1. Python 2.7 (32 bit)
2. Enough edge to cut yourself on
3. 10 minutes of free time

EdgeQuest is packaged with [simplejson](https://github.com/simplejson/simplejson) and [libtcod](https://bitbucket.org/libtcod/libtcod) already so no additional libraries should be needed (outside of SDL).


## Download

You can download the latest master [here](https://github.com/surrsurus/edgequest/archive/master.zip) or just click the `Download Zip` button at the top of the page. The master release should be stable enough to play, but becomes outdated fairly quickly, so please use the `download-master` script in the scripts folder to download the latest master.

If you want more stability, download the latest release [here](https://github.com/surrsurus/edgequest/releases)

If you want bleeding edge development, download the testing branch [here](https://github.com/surrsurus/edgequest/archive/testing.zip). Note that this version is extremely prone to bugs.


## Run Edgequest

#### Linux (Debian-based)

It should be as simple as running the `linux-run` script included with the repository

All imports are from the standard library (with the exception of simplejson and libtcod, included with edgequest), so you should have no problem with running this on a linux machine

If you get an SDL error along the lines of `libsdl1.2.0 not found` try:

`sudo apt-get --reinstall libsdl1.2debian`

Max tried to run the code in Manjaro Linux and it didn't work so there is an issue with pacman-based distros.

#### Windows (7 & 10)

On windows you will need to download and install [Python2.7](https://www.python.org/downloads/release/python-2711/).

Make sure it's the 32 bit version!

Once you have it downloaded, simply extract the zip file and double click on the `edgequest` file. (It should have a little blue and yellow snake icon on it!)

If you have any problems please notify me by creating an issue on the issue tracker [here](https://github.com/surrsurus/edgequest/issues).

I was not intending to support Windows, so there may be minor issues.

#### Mac OS

Kevin playtested it on his Macbook Air and he used a live USB of linux mint.
Considering he's the only one I know with a mac, and he found a way to circumvent this problem, there is no planned mac version.


## Issue Tracking

If you come across any bug or have an idea for an enhancement, please create a github issue. It helps me track the bugs and also makes sure I don't have to playtest all the time (Though, I'm not complaining if I have to).

Create an issue [here](https://github.com/surrsurus/edgequest/issues).


## Wiki

Please check the [wiki](https://github.com/surrsurus/edgequest/wiki) for further information


## Modding

EdgeQuest is very hackable. There is a `settings.py` file that can be edited to change values of things in the game, and JSON files that can be edited to add monsters and items. This may not be the most robust modding system, but it gives the player with enough motivation to create brand new things with the existing code.

Please take a look at the [JSON Documentation](https://github.com/surrsurus/edgequest/wiki/JSON-Documentation) if you have further interest in modding.

## Community

[EdgeQuest subreddit](https://www.reddit.com/r/edgequest/)

## License

[![CC0] (https://licensebuttons.net/l/GPL/2.0/88x62.png)](https://www.gnu.org/licenses/gpl-3.0.en.html)

This code is released under the GNU GENERAL PUBLIC LICENSE. All works in this repository are meant to be utilized under this license with the exception of the [simplejson](https://github.com/simplejson/simplejson) and [libtcod](https://bitbucket.org/libtcod/libtcod) which are not my property. Please see the respective licenses.
