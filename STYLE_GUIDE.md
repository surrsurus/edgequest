# Edgequest Style Guide

When hacking on edgequest to submit a PR, please make sure your contribution meets the following criteria, otherwise your PR will be denied (but most likely someone will request that you update your code to reflect this guide).

### Indentation
- Two space indentation

### Quotes
- Double quotes for all strings
- Single quotes for all characters

### Documentation
- Docstrings on public interfaces mandatory
- Line comments preferred, even for large paragraphs
- Must comment tricky parts of the code, the more comments the better
  - If you do not know whether something is tricky to others, comment it anyway

### Braces, Semicolons, Commas
- Opening braces on same line
- Semicolons after return statements
- Match statements need braces for multi line match arms, single lines not as much
  - If it increases readability to add braces for match arms, do it
- No trailing commas (On struct fields, json files, etc)

### Naming
- Follow [this](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Only short names allowed are things like `x`, `y`, `tx`, `ty`, `w`, `h`, `pos`, `me`, `con`, `bg`, `fg`, `dun`, `rng`
  - If it's short it must be descriptive like `cat`
  - Try not to abbreviate long words

#### Nomenclature
- `x` - x coordinate 
- `y` - y coordinate
- `tx` - target x coordinate (AIs use this to set locations where they want to move to)
- `ty` - target y coordinate AIs use this to set locations where they want to move to)
- `w` - width
- `h` - height
- `pos` - `Pos`ition, or xy coordinate pair
- `me` - `AI` referring to it's own assigned creature
- `con` - tcod console
- `bg` - background color (as `RGB` or tuple) 
- `fg` - foreground color (as `RGB` or tuple)
- `dun` - `Dungeon` object
- `rng` - Random number generator

### Imports
- Avoid importing * or everything at once
- Follow [this](https://doc.rust-lang.org/1.0.0/style/style/imports.html) for import order

