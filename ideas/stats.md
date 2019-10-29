# Stats

## Intrinsic Attributes
These attributes are fundamentally a part of each creature.

### The Senses
This ability group defines how aware the creature is of it's surroundings.

#### Perception [PER]
The ability to see and use the eyes.

  - Defines how large sight interaction radius is, and how far the player can see
  - - 2 Points of PER = 1 unit of radius
  - Affects accuracy of projectile weapons

#### Olfaction [OLF]
The ability to smell and use the nose

  - Defines how large the scent interaction radius is, and how far the player can 'see' scents
  - - 2 Points of OLF = 1 unit of radius
  - Can be used in place of perception for accuracy calculations

### The Body
This ability group defines a creature's kinesthesia and physical tolerances

#### Agility
The ability to be deft in one's movements.

  - Defines a creature's speed/action points/position in the timing queue (a time system is not implemented thus making this hard to determine)
  - Affects dodge chance

#### Fortitude
The ability to endure what awaits.

  - Affects total hit points
  - Affects recovery from status effects

### The Mind

#### Reason
The ability to use logic and think formally.

  - Affects ability to disarm traps/unlock doors/unlock containers
  - Affects ability to read/comprehend spell scrolls and use magic items

#### Insight
The ability to use intuition and understand the world.

  - Affects total sanity points

## Finite Attributes
These attributes are finite and have severe consequences for running out of them

### Health Points
An abstraction that represents how much trauma a creature can take before dying.

### Sanity Points
An abstraction that represents a creature's mental state.

  - Reading scrolls, using magic items, costs sanity
  - Coming in contact with unknowable things reduces sanity

## Equipment Attributes
All armors have both an armor value (AV), and an evasion value (EV). Armor offers a direct reduction of incoming damage. Evasion offers a chance to completely dodge an attack. Lighter armors have more EV than AV, and vice versa.

### Armor [AV]
Offers a direct damage reduction.

### Evasion [EV]
Offers a direct dodge chance.