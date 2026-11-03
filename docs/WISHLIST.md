# Wishlist

Stage 1 of 3. [CHECKLIST.md](CHECKLIST.md) turns this list into milestones. [SPEC.md](SPEC.md) says how to build those milestones.

This file is the owner's design for **Battles of Galactic Armadas**, plus a few stretch ideas. Priority tags are Must / Should / Could. A Must can wait until a later milestone. The tag means the full game includes it.

**Decided** entries are the owner's. **Proposed** entries were added in this planning pass and are not approved. Design gaps are not filled in here; they are [open questions in the spec](SPEC.md#17-open-questions).

## Pillars

- The game is a turn-based combat-puzzle roguelike. Builds are deep, and a fight is a puzzle.
- The player is the captain of one ship inside a battle of tens of thousands of ships. The player takes orders, and sometimes commands a squadron. The rest of the armada fights on its own orders.
- Each turn the player spends action points on the ship itself: hull and weapon facings, momentum, individual components, and a power grid. Frigates have the most action points.
- Each battle has one hard objective. High-value targets and intel improve the battle's overall outcome, and that outcome is what matters.
- A run is a war map of ten battles. Each step is a choice between two objectives, thrust or hold.

Tone and craft references, with this game's own mechanics: Rift Wizard and Desktop Dungeons (build depth, combat as a puzzle), ToME (combat scenarios), Legend of Galactic Heroes and the Honorverse (fleet warfare). Nexus: The Jupiter Incident is the reference for how a ship is crippled, abandoned, or lost.

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
| W-ROL-02 | Must | The player takes orders, and in some battles commands a squadron. |
| W-ROL-03 | Must | The player is a captain inside the armada. Ships outside their ship and their squadron fight on orders of their own. |

## Combat puzzle

| ID | Tag | Wish |
| --- | --- | --- |
| W-CMB-01 | Must | Each turn the player sets weapon facings, and weapons resolve along those facings. |
| W-CMB-02 | Must | Each turn the player manages individual components. |
| W-CMB-03 | Must | Each turn the player routes power between subsystems, with a real tradeoff. |

## Movement and facing

| ID | Tag | Wish |
| --- | --- | --- |
| W-MOV-01 | Must | Ships have momentum. They coast on their vector until they brake or change vector. |
| W-MOV-02 | Must | Acceleration is a full-turn action. A ship moves at most 3 hexes in a turn. |
| W-MOV-03 | Must | Turning the hull takes time. |
| W-MOV-04 | Must | Movement commands spend the same action-point budget as other commands. The crew carries the orders out. |
| W-FAC-01 | Must | The hull and the weapons have separate facings. |
| W-FAC-02 | Must | Ships the player is not close to use a cheap facing approximation or a batch. In the fiction, sensor detail falls off with distance because the ship's computers prioritize what is close. |

## Action points

| ID | Tag | Wish |
| --- | --- | --- |
| W-AP-01 | Must | Ships act on action points. A unit without enough points to do something is skipped; no decision is processed for it. |
| W-AP-02 | Must | Frigates have the most action points. They are fast, and a small crew is responsive. |

## Power grid

| ID | Tag | Wish |
| --- | --- | --- |
| W-PWR-01 | Must | Subsystems are drives, weapons, shields, sensors, ECM, life support, tactical computers, heat pumps, capacitors and batteries, ammo loaders, powered structural support, and doors and bulkheads. |
| W-PWR-02 | Must | A ship has multiple generators, distribution nodes spread through the hull, and power lines. Lines can be diverted, damaged, and repaired. |
| W-PWR-03 | Must | A damaged line can limit the power that reaches a subsystem. A main lane that leaves a generator is the severe case. |
| W-PWR-04 | Must | Power points may sit unassigned. Putting unassigned power onto a subsystem (a sudden shield boost) is quicker than taking power off a subsystem that is already drawing it, because the engineering crew has to rebalance the grid. |

## Damage and loss

| ID | Tag | Wish |
| --- | --- | --- |
| W-DMG-01 | Must | Repairable damage comes from regular weapons fire and from focused damage. It can disable the ship or force an evacuation. It does not, by itself, end the run. |
| W-DMG-02 | Must | Dry-dock damage is a massive hole or significant shredding of the chassis. It needs a dry dock, or it writes the ship off. This kind of damage ends the run. |
| W-DMG-03 | Must | Crippling every combat-capable component abandons the ship. |
| W-DMG-04 | Must | Critical damage to cooling or to the reactor may detonate the ship. |

## Boarding

| ID | Tag | Wish |
| --- | --- | --- |
| W-BRD-01 | Must | A boarding action is a smaller roguelike inside the battle, and a play of it runs about 5 to 10 minutes. |
| W-BRD-02 | Must | The boarders have to take both the bridge and engineering in time. Otherwise the ship auto-destructs. |
| W-BRD-03 | Must | The player uses a separate, simpler equipment system while boarding. The player's crew and the defenders both use standard gear. |

## Squadron command

| ID | Tag | Wish |
| --- | --- | --- |
| W-SQD-01 | Must | Squadron composition follows the player's class. A frigate squadron is a frigate as C2, 2 to 4 escorts, and may include 8 strike craft in 2 wings. A cruiser squadron is 1 to 2 cruisers, 2 to 4 frigates, and 16 strike craft in 4 wings. A battleship squadron is 1 flagship, 2 ships of the line, 8 supporting cruisers or frigates, and 16 to 32 strike craft. |
| W-SQD-02 | Must | Squadron commands are focus (one or more targets, to fire on or to capture), hold (the area around a position), protect (one or more targets), and follow (keep formation with the player's ship). |
| W-SQD-03 | Must | Squad Command is an improvable skill. A battleship squadron can be commanded only when that skill is at its maximum. |

## Orders and the campaign

| ID | Tag | Wish |
| --- | --- | --- |
| W-ORD-01 | Must | An order is a key objective for the campaign. If the player ignores it and no other ship carries it out, the effect on the campaign is moderately outsized. One shape of that order is to hit a cruiser formation so it cannot reinforce a flank elsewhere. |
| W-ORD-02 | Must | Each battle has a strategic objective for both sides. The player's order is linked to their side's objective, and that link gets tighter as the campaign progresses. |

## Factions

| ID | Tag | Wish |
| --- | --- | --- |
| W-FCT-01 | Must | The two factions are the Imperial Royal Navy and the Allied People's Naval Forces. |
| W-FCT-02 | Must | The difference is mostly lore and cosmetics. Imperial ships are a little tougher. Alliance ships have slightly more gun. Each side has one or two special components that a player on the other side has to capture. |

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
| W-TEC-03 | Must | The game ships as standalone binaries for Windows and Linux. |
| W-TEC-04 | Must | Concurrency is native and stays clear of a web of thread locks. Battles are heavily instanced, with huge numbers of ships. |
| W-TEC-05 | Must | The game is written in Rust, legibly and in modules, for a reader who does not write much Rust, and without over-verbosity. That is the project coding guideline. |
| W-TEC-06 | Should | The game can run in a container so a test is easy to repeat. |
| W-TEC-07 | Must | A turn processes in under a second. The low target, with no other workload on the machine, is 4 cores and 8 GB of RAM. The official minimum is 16 GB of RAM. |

## Where this list stops

Movement, facings, the power grid, damage and loss, boarding, action points, squadron composition and commands, campaign orders, the two factions, Windows and Linux, the memory and turn-time targets, and Rust are decided above. Anything still missing is an open question in [SPEC.md §17](SPEC.md#17-open-questions): action-point amounts, the time cost of a hull turn and of braking, how "quicker" power allocation is measured, how repair is performed, the component catalog, damage thresholds, strike craft and ships of the line, intel, the battle-outcome record, when the ship changes class, captain and crew attributes besides Squad Command, medals, veterans, saves, the holotable, the third animation frame, VRAM, and the boarding map. Proposals in the spec are ways to build the checklist in the meantime. They are not extra wishes.
