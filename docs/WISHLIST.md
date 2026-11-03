# Wishlist

Stage 1 of 3. [CHECKLIST.md](CHECKLIST.md) turns this list into milestones. [SPEC.md](SPEC.md) says how to build those milestones.

This file is the owner's design for **Battles of Galactic Armadas**, plus a few stretch ideas. Priority tags are Must / Should / Could. A Must can wait until a later milestone. The tag means the full game includes it.

**Decided** entries are the owner's. **Proposed** entries were added in this planning pass and are not approved. Design gaps are not filled in here; they are [open questions in the spec](SPEC.md#17-open-questions).

## Pillars

- The game is a turn-based combat-puzzle roguelike. Builds are deep, and a fight is a puzzle.
- The player is the captain of one ship inside a battle of tens of thousands of ships. The player takes orders, and sometimes commands a small squadron. The rest of the armada fights on its own orders.
- Each turn the player works the ship itself: weapon orientations, individual components, and power routed between subsystems.
- Each battle has one hard objective. High-value targets and intel improve the battle's overall outcome, and that outcome is what matters.
- A run is a war map of ten battles. Each step is a choice between two objectives, thrust or hold.

Tone and craft references, with this game's own mechanics: Rift Wizard and Desktop Dungeons (build depth, combat as a puzzle), ToME (combat scenarios), Legend of Galactic Heroes and the Honorverse (fleet warfare).

## Fantasy and tone

| ID | Tag | Wish |
| --- | --- | --- |
| W-FAN-01 | Must | The game is a turn-based combat-puzzle roguelike. |
| W-FAN-02 | Should | Writing and presentation aim at large-scale fleet warfare, in the spirit of Legend of Galactic Heroes and the Honorverse. |
| W-FAN-03 | Should | Build depth and puzzle-like fights take Rift Wizard and Desktop Dungeons as the reference. Scenario shape takes ToME as the reference. |

## Role and scale

| ID | Tag | Wish |
| --- | --- | --- |
| W-ROL-01 | Must | The player captains one ship in a battle tens of thousands of ships strong. |
| W-ROL-02 | Must | The player takes orders, and in some battles commands a small squadron. |
| W-ROL-03 | Must | The player is a captain inside the armada. Ships outside their ship and their squadron fight on orders of their own. |

### Proposed

| ID | Tag | Wish |
| --- | --- | --- |
| P-ROL-01 | Could | Squadron doctrines (screen, line, focus) that allied ships keep while the player fights their own ship. |

## Combat puzzle

| ID | Tag | Wish |
| --- | --- | --- |
| W-CMB-01 | Must | Each turn the player sets weapon orientations, and weapons resolve along those orientations. |
| W-CMB-02 | Must | Each turn the player manages individual components. |
| W-CMB-03 | Must | Each turn the player routes power between subsystems, with a real tradeoff. |

## Ships and builds

| ID | Tag | Wish |
| --- | --- | --- |
| W-SHP-01 | Must | Ship builds are highly detailed: many components, chosen and arranged as a build. |
| W-SHP-02 | Must | Components are synergistic. Combinations change the result beyond the sum of the parts. |

### Proposed

| ID | Tag | Wish |
| --- | --- | --- |
| P-SHP-01 | Could | Components, classes, and scenarios can be added as content packs loaded from data files, with no code change. |

## Progression

| ID | Tag | Wish |
| --- | --- | --- |
| W-PRG-01 | Must | The player's ship class progresses frigate, then cruiser, then battleship. |
| W-PRG-02 | Must | The captain progresses across the run. |
| W-PRG-03 | Must | The crew progresses across the run. |

## Scenarios and outcomes

| ID | Tag | Wish |
| --- | --- | --- |
| W-SCN-01 | Must | Most scenarios are strikes against a key target: a command ship, a communication node, or another designated target. |
| W-SCN-02 | Must | Some scenarios are defensive: hold out until a counterattack. |
| W-SCN-03 | Must | Each battle has one hard objective. |
| W-SCN-04 | Must | Destroying high-value targets improves the overall battle outcome. |
| W-SCN-05 | Must | Intel gathered improves the overall battle outcome. |
| W-SCN-06 | Must | The overall battle outcome is the result that matters. |

### Proposed

| ID | Tag | Wish |
| --- | --- | --- |
| P-SCN-01 | Could | A battle may include a scripted complication (reinforcements, a failing ally) that still leaves the battle with one hard objective. |

## The war

| ID | Tag | Wish |
| --- | --- | --- |
| W-WAR-01 | Must | A run is a war map of ten battles. |
| W-WAR-02 | Must | At each step the player chooses between two objectives, thrust or hold, and plays the one they chose. |

Thrust is the offensive choice and lines up with strike scenarios (W-SCN-01). Hold is the defensive choice and lines up with holding out for a counterattack (W-SCN-02).

### Proposed

| ID | Tag | Wish |
| --- | --- | --- |
| P-WAR-01 | Could | The war map shows a front that advances or buckles from the outcomes of thrust and hold. |

## Meta

| ID | Tag | Wish |
| --- | --- | --- |
| W-MET-01 | Should | Medals are achievements that also grant a small bonus. |
| W-MET-02 | Should | There is a chance of veteran crew. |
| W-MET-03 | Should | Ironman mode: the escape pod fails. The run is over, and the next start is from scratch. |

### Proposed

| ID | Tag | Wish |
| --- | --- | --- |
| P-MET-01 | Could | An after-action citation records the hard objective, high-value targets, and intel for each battle of the run. |

## Battlefield and camera

| ID | Tag | Wish |
| --- | --- | --- |
| W-FLD-01 | Must | The battlefield is a hex grid. |
| W-FLD-02 | Must | The camera starts moderately zoomed in, and the player can move it and zoom it freely. |
| W-FLD-03 | Must | A large battle stays easy to navigate. |

### Proposed

| ID | Tag | Wish |
| --- | --- | --- |
| P-FLD-01 | Could | A far zoom shows the armada as a readable picture. Commands stay at the normal zoom. |

## Interface

| ID | Tag | Wish |
| --- | --- | --- |
| W-UI-01 | Must | The main screen is two sidebars around a central battlefield. |
| W-UI-02 | Must | A subscreen is dedicated to ship management. |
| W-UI-03 | Must | A subscreen is dedicated to progression. |
| W-UI-04 | Must | The battlefield shows a dense field of ships as tactical icons on a holotable. |
| W-UI-05 | Should | The holotable is customizable. |
| W-UI-06 | Must | The game is playable by keyboard. |
| W-UI-07 | Must | The game is mouse friendly. A turn that can be played on the keyboard can also be played with the mouse. |

### Proposed

| ID | Tag | Wish |
| --- | --- | --- |
| P-UI-01 | Could | Keys can be rebound. |

## Art

| ID | Tag | Wish |
| --- | --- | --- |
| W-ART-01 | Must | Art direction is sprite / SVG. |
| W-ART-02 | Must | Each entity uses two or three frames. Idle and attack are the pair every entity has. |

### Proposed

| ID | Tag | Wish |
| --- | --- | --- |
| P-ART-01 | Could | Short music stingers or radio-chatter lines punctuate an order arriving and a counterattack appearing. Presentation only. |

## Technology

| ID | Tag | Wish |
| --- | --- | --- |
| W-TEC-01 | Must | The game is a standalone application. |
| W-TEC-02 | Must | Rendering is portable 2D through SDL. |
| W-TEC-03 | Must | The game ships as standalone binaries for multiple platforms. |
| W-TEC-04 | Must | Concurrency is native and stays clear of a web of thread locks. Battles are heavily instanced, with huge numbers of ships. |
| W-TEC-05 | Must | The implementation language is not chosen. The spec compares candidates and recommends one. The owner decides. |

## Where this list stops

Anything the tables above do not say is unspecified. In particular, movement ranges, facing, the subsystem list, damage, turn order, squadron size, what an order demands, how intel is gathered, how the battle outcome is scored, when a ship changes class, what captain and crew attributes are, save games, factions, target platforms, and the language are open. They are collected in [SPEC.md §17](SPEC.md#17-open-questions). Proposals in the spec are ways to build the checklist in the meantime. They are not extra wishes.
