# Edgequest [![Build Status](https://travis-ci.org/surrsurus/edgequest.svg?branch=master)](https://travis-ci.org/surrsurus/edgequest) ![Rust](https://img.shields.io/badge/language-rust-orange.svg)

Under heavy construction.


***Edgequest - The world's most complicated tile simulator***

Edgequest is a roguelike for all major platforms (pretty sure this can run on anything with rust and a screen). There's not a whole lot to the game (it's more of a tech demo anyway).

#### Features:
  - Scent: Monsters can smell you and track you by scent you emit. Other monsters and objects also emit scent of their own and interfere with detection. Scent 'lingers' which allows entities to be tracked very long distances with no other aid.
  - Sound: Everything makes sound. Since sound is purely an amplitude (at the moment) creatures can search via guessing sound epicenters and loud sounds might cause creatures to wake up.
  - Modular Creatures/Terrain: Basically an ECS but not really, it allows for creatures traits, stats, AI, tiles, to all be swapped out modularly whenever you want. It's a bit more rigid than an ECS, though it works similarly.
  - Generation Features: Dungeon generation is highly complex, featuring:
    - Biomes: Dungeons have radomly generated biomes that affect terrain, monster spawns, foliage, and overall feel. Multiple biomes are found in each level.
    - Structures: Dungeons can spawn structures which are 'pre-fabricated' (read: from text files) which are used to create interesting variety in the maps while having a little familiarity considering how random the whole game is.
    - Automata/Builders: Dungeons can utilize automata to generate random features that follow customized patterns in a standardized way, for example creating caves or narrow corridors. Builders are more clearly defined versions of automata in the sense that they are left less up to chance.
    - Foliage: Gotta make it look nice
    - Filters: Generation happens in filters, each of which add or subtract features from the landscape, meaning that at some point dungeon generation can be completely modular, or at least configured with some sort of file. All above features are actually filters.

And most importantly, no unsafe code.

## Requirements

- Rust Nightly (Make sure it's up to date!)
- SDL

## Installing SDL/Build Tools

Linux:

```
$ sudo apt-get install gcc g++ make libsdl1.2-dev
$ cargo run
```

Windows:

```
$ set PATH=%PATH%;C:\Program Files (x86)\Rust\bin;C:\MinGW\bin
$ cargo run
```

#### Note for Windows Users

You will probably need to change your default renderer in `config/cfg.yml`. At the bottom of the file there is a renderer option. Change that to `SDL`, and it should run fine.

OSX:

```
$ brew install sdl2
$ cargo run
```

#### Note for Mojave Users

This will not work if you have an xcode version above 9 installed. For some reason, sdl just flat out doesn't render but the game still technically works.

