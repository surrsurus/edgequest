# Edgequest - Season Two [![Build Status](https://travis-ci.org/surrsurus/edgequest.svg?branch=master)](https://travis-ci.org/surrsurus/edgequest)

Currently rewriting in Rust. Under heavy construction

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

You might also need to change the default renderer in `config/cfg.yml` to `SDL`. That variable is at the bottom of the file.

OSX:

```
$ brew install pkg-config sdl
$ cargo run
```