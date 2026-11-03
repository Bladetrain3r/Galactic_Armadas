# Checklist

Stage 2 of 3. Derived from [WISHLIST.md](WISHLIST.md). [SPEC.md](SPEC.md) is derived from this file. Build in milestone order: a minimal playable battle first, the full game last.

A box is done when a person can perform the test and see the result. Wishlist ids in italics are the source of the box. Tone wishes (W-FAN-02, W-FAN-03) have no single box; the coverage table says where they went. The coding guideline (W-TEC-05) is C-M0-08 and [SPEC.md §16](SPEC.md#16-language-and-coding-guideline).

Build order: Milestone 0, then 1 (the first playable battle), 2, 3, 4, 6 (boarding), then 5 (the release). Boarding is part of the full game and is not required to call Milestone 1 playable.

**20,000 ships** is tuning default [TD-SCALE](TUNING.md) for "tens of thousands" (W-ROL-01, W-TEC-04). A turn of that scene finishes in under a second (W-TEC-07). VRAM is tuning default [TD-22](TUNING.md). Where a box needs a number the owner has not set, it uses [TUNING.md](TUNING.md).

Where a test needs a rule the owner has not set, the box uses the tuning default in [TUNING.md](TUNING.md). The test below is that default.

## Milestone 0 — Window, grid, scale

A development scene is enough. It does not need to be a real scenario. Placeholder icons are enough.

- [ ] **C-M0-01** The application opens a window, draws through SDL, and exits cleanly. _W-TEC-01, W-TEC-02_
- [ ] **C-M0-02** The central view draws a hex grid large enough for a battle, between two sidebars. _W-FLD-01, W-UI-01_
- [ ] **C-M0-03** The camera starts moderately zoomed in. The player can pan across the whole grid and zoom in and out. _W-FLD-02_
- [ ] **C-M0-04** From elsewhere on the grid, the player can bring the view to their own ship, and to the hard objective, without scrolling the map by hand. _W-FLD-03_
- [ ] **C-M0-05** Keyboard and mouse can both pan and zoom. _W-UI-06, W-UI-07_
- [ ] **C-M0-06** A scene of at least 20,000 ship instances advances in under a second on 4 cores and 8 GB, with no other workload, while the window keeps accepting input and drawing. The update does not hold one lock around the whole step. Ships without the action points to act are not given a decision. _W-TEC-04, W-TEC-07, W-ROL-01, W-AP-01_
- [ ] **C-M0-07** At the starting zoom the view holds many ships, drawn as tactical icons on the holotable. _W-UI-04_
- [ ] **C-M0-08** The first gameplay module is split by system, uses straightforward Rust, and a reader who writes little Rust can follow it. It does not rely on a stack of traits, macros, or other ceremony to say something simple. _W-TEC-05_
- [ ] **C-M0-09** That scale scene also runs inside a container limited to 4 cores and 8 GB. _W-TEC-06, W-TEC-07_

## Milestone 1 — One ship, one puzzle

The smallest playable battle: one player ship, one hard objective, one turn loop.

- [ ] **C-M1-01** A battle starts with the player in command of one ship and with exactly one hard objective. _W-ROL-01, W-SCN-03_
- [ ] **C-M1-02** The battle shows an order the player did not write, and the hard objective matches that order. _W-ROL-02_
- [ ] **C-M1-03** Each turn the player sets every weapon's facing, separate from the hull's facing. A shot resolves along the weapon facing. _W-CMB-01, W-FAC-01_
- [ ] **C-M1-04** Each turn the player sets the state of each component on their ship, and resolution uses that state. _W-CMB-02, W-SHP-01_
- [ ] **C-M1-05** Each turn the player routes power among the decided subsystems. The gain and the loss are both visible before the turn resolves. _W-CMB-03, W-PWR-01_
- [ ] **C-M1-06** With two named components online, a documented synergy changes a result. Turning either component off removes the synergy. _W-SHP-02_
- [ ] **C-M1-07** The battle ends in success when its hard objective is completed. Evacuation restarts that battle and keeps the ship, the fit, and the crew. Ordering an abandon while a combat component still works ends the run. _W-SCN-03, W-DMG-08, W-DMG-09_ TD-39, TD-40
- [ ] **C-M1-08** A full turn — movement, facings, weapons, components, power, a crew repair in the interior, and end turn — can be played with the keyboard alone, and the same turn can be played with the mouse. _W-UI-06, W-UI-07, W-INT-01_
- [ ] **C-M1-09** During the battle the screen is two sidebars and the central grid, and weapons, components, facings, and power are readable without leaving the battle. _W-UI-01, W-CMB-01, W-CMB-02, W-CMB-03_
- [ ] **C-M1-10** The player ship has momentum and coasts until it brakes or changes vector. Accelerating spends the whole turn and moves the ship at most 3 hexes. Braking spends the whole turn as well, because it is the same action with the force reversed. _W-MOV-01, W-MOV-02, W-MOV-05_
- [ ] **C-M1-11** Turning the hull one hex facing costs 1 action point. After that spend, the hull facing and a weapon facing can disagree. _W-MOV-03, W-FAC-01_
- [ ] **C-M1-12** A frigate starts with 10 action points, a cruiser with 8, and a battleship with 6. Those are the tuning starts. Commands, including movement, spend from that pool. A ship with nothing left to spend receives no new decision that turn. _W-AP-01, W-AP-02, W-AP-03, W-MOV-04_
- [ ] **C-M1-13** Assigning one point of spare power costs exactly one action point less than taking one point off a subsystem that is already drawing power. _W-PWR-04_
- [ ] **C-M1-14** The player ship has more than one generator, distribution nodes, and power lines. Diverting a line changes what it feeds. Damaging a main line out of a generator lowers how much power can reach a subsystem. Crew who reach that line through the interior restore the delivery. A control that never sends crew does not. _W-PWR-02, W-PWR-03, W-INT-01_
- [ ] **C-M1-15** A hit can leave a component crippled: power or another resource routes worse, or the component shows a minor overheat or surge. The run continues. _W-DMG-01, W-DMG-05_
- [ ] **C-M1-16** When every combat-capable component is at least crippled (the mild tier), the ship is abandoned, the battle is lost, and the run restarts that battle the same way evacuation does. The run does not end. _W-DMG-03_ TD-40
- [ ] **C-M1-17** A component whose integrity reaches 0 enters the critical red zone and can be brought back without fabricating a new part. Critical damage to cooling or to the reactor can detonate the ship. A component that is only crippled does not detonate it. _W-DMG-04, W-DMG-06_
- [ ] **C-M1-18** A massive hole in the ship, or a shredded chassis, ends the run. _W-DMG-02_
- [ ] **C-M1-19** The player can move crew through the player ship's interior. Repair happens because those crew do the work at the damaged place. _W-INT-01, W-INT-02_
- [ ] **C-M1-20** A component at dry dock is shown as a hole where it was, or as fused parts, and the ship cannot use it again until that component is replaced. _W-DMG-07_

## Milestone 2 — The battle around the ship

The same puzzle, inside the armada it belongs to.

- [ ] **C-M2-01** A played battle, past the Milestone 0 development scene, instances at least 20,000 ships. The player's next turn stays responsive, and processing that turn stays under a second on the Milestone 0 machine. _W-ROL-01, W-TEC-04, W-TEC-07_
- [ ] **C-M2-02** A strike battle's hard objective is a marked command ship, communication node, or other designated target, and that target is visually distinct from other ships. _W-SCN-01, W-SCN-03_
- [ ] **C-M2-03** A hold battle's hard objective is to keep the player ship alive until a counterattack arrives, and the counterattack is an event the player can see. _W-SCN-02, W-SCN-03_
- [ ] **C-M2-04** Destroying a marked high-value target that is not the hard objective improves the recorded battle outcome. Leaving it alive does not. _W-SCN-04, W-SCN-06_
- [ ] **C-M2-05** An intel action, distinct from the hard objective, improves the recorded battle outcome. Skipping it does not. _W-SCN-05, W-SCN-06_
- [ ] **C-M2-06** One battle gives the player a frigate squadron: the player's frigate as C2, 2 to 4 escorts, and, when the battle includes them, 8 strike craft in 2 wings. Focus, hold, protect, and follow each change what those ships do. Another battle has no squadron. Both show an order from outside the player. _W-ROL-02, W-SQD-01, W-SQD-02_
- [ ] **C-M2-07** The battle outcome is shown as its own readout, separate from the objective's success or failure. _W-SCN-06_
- [ ] **C-M2-08** The player has no command over ships outside their own ship and their squadron. Those ships still act. _W-ROL-03_
- [ ] **C-M2-09** A ship far from the player shows a coarse facing. A ship close to the player shows its real hull facing and weapon facings. _W-FAC-02_
- [ ] **C-M2-10** In that large battle, ships without enough action points to act are skipped, and ships with points still act. _W-AP-01_
- [ ] **C-M2-11** The battle states a strategic objective for each side, and the player's order is the part of their side's objective they were given. _W-ORD-02_
- [ ] **C-M2-12** Ships of the Imperial Royal Navy and the Allied People's Naval Forces are both present and named. An Imperial ship of a class weathers more than the Alliance ship of that class. The Alliance ship hits harder. _W-FCT-01, W-FCT-02_
- [ ] **C-M2-13** A ship immediately next to the player has an interior of the same kind as the player's. A ship far from the player does not. _W-INT-02_

## Milestone 3 — Ship, captain, crew

Progression across battles. A session that passes through all three classes is enough; the ten-battle map arrives in Milestone 4.

- [ ] **C-M3-01** The player's ship moves from frigate to cruiser to battleship, and the three classes differ in component capacity or power. _W-PRG-01, W-SHP-01_
- [ ] **C-M3-02** A ship-management subscreen is where the detailed build is inspected and edited. In-battle component control from Milestone 1 still works. _W-UI-02, W-SHP-01_
- [ ] **C-M3-10** On that screen a component with a clear purpose can be refitted, and the chassis plumbing cannot. _W-SHP-03_
- [ ] **C-M3-03** The captain persists from battle to battle, and at least one visible value changes because of a battle's result. _W-PRG-02_
- [ ] **C-M3-04** The crew persists from battle to battle, and at least one visible value changes because of a battle's result. _W-PRG-03_
- [ ] **C-M3-05** A progression subscreen shows ship class, captain, and crew, including the change from the latest battle. _W-UI-03, W-PRG-01, W-PRG-02, W-PRG-03_
- [ ] **C-M3-06** A cruiser or battleship build can express a synergy the starting frigate fit cannot, using the same synergy rule as C-M1-06. _W-SHP-02, W-PRG-01_
- [ ] **C-M3-07** Squad Command is visible and can increase. Below its maximum, a battleship squadron cannot be ordered. At the maximum, the player can command one: 1 flagship, 2 ships of the line, 8 supporting cruisers or frigates, and 16 to 32 strike craft. _W-SQD-01, W-SQD-03_
- [ ] **C-M3-08** A cruiser squadron is 1 to 2 cruisers, 2 to 4 frigates, and 16 strike craft in 4 wings, and it accepts the same four commands as C-M2-06. _W-SQD-01, W-SQD-02_
- [ ] **C-M3-09** Fighting the other faction, the player can capture one of its special components and fit it. _W-FCT-02_

## Milestone 4 — A war of ten battles

The roguelike run.

- [ ] **C-M4-01** A run contains exactly ten battles, in order. _W-WAR-01_
- [ ] **C-M4-02** Before each battle the player chooses thrust or hold. Only the chosen battle is played. _W-WAR-02_
- [ ] **C-M4-03** The choice is presented on a war map. _W-WAR-01, W-WAR-02_
- [ ] **C-M4-04** After a battle, its outcome is visible on the war map before the next choice. _W-SCN-06, W-WAR-01_
- [ ] **C-M4-05** A thrust choice uses a strike objective (C-M2-02). A hold choice uses a hold objective (C-M2-03). _W-WAR-02, W-SCN-01, W-SCN-02_
- [ ] **C-M4-06** Finishing the tenth battle ends the run and shows the outcome of each battle. _W-WAR-01, W-SCN-06_
- [ ] **C-M4-07** Ignoring an order, when no other ship carries it out, changes the campaign by more than ignoring a side target does. If another ship carries the order out, that larger change does not land. _W-ORD-01_
- [ ] **C-M4-08** The player's order is tied more tightly to their side's strategic objective in a late battle than in the first battle. _W-ORD-02_

## Milestone 6 — Boarding

Build this once Milestone 1 can damage a ship. It is not part of the first playable battle. The release in Milestone 5 includes it.

- [ ] **C-M6-01** From a battle, the player enters a boarding action that plays as its own roguelike and takes about 5 to 10 minutes. _W-BRD-01_
- [ ] **C-M6-02** Taking both the bridge and engineering before time runs out captures the ship. Failing either one auto-destructs it. _W-BRD-02_
- [ ] **C-M6-03** The player's boarding kit is a shorter equipment list than the ship build. The player's crew and the defenders are both issued standard gear from that list. _W-BRD-03_

## Milestone 5 — Meta, holotable, ship it

The decided game around the run, and a build someone else can launch.

- [ ] **C-M5-01** A medal is earned by a stated achievement, names that achievement, and applies a small bonus the player can see. _W-MET-01_
- [ ] **C-M5-02** A run can include veteran crew by chance. A veteran crew differs from a non-veteran crew on the progression screen. _W-MET-02, W-PRG-03_
- [ ] **C-M5-03** With ironman on, loss of the player ship ends the run (the escape pod fails). The next run starts from scratch: a new frigate, a new captain, and a new crew. _W-MET-03_
- [ ] **C-M5-04** The player can toggle faction tint, objective markers, squadron markers, and weapon-direction hints on the holotable. The change lasts for the session and is gone on the next launch. _W-UI-05_ TD-20
- [ ] **C-M5-05** Every entity type used in a battle has an idle frame and an attack frame, in a sprite/SVG style, and no entity uses more than three frames. _W-ART-01, W-ART-02_
- [ ] **C-M5-06** A release is a standalone binary, plus only the files beside it that it needs, for Windows and for Linux. It starts with no preinstalled language runtime. _W-TEC-01, W-TEC-03_
- [ ] **C-M5-07** That release can run the Milestone 0 scale scene and a Milestone 2 battle, and a turn of each finishes in under a second. _W-TEC-03, W-TEC-04, W-TEC-07_
- [ ] **C-M5-08** The release states 16 GB of RAM as the official minimum. _W-TEC-07_

## Not scheduled — proposed stretch

These boxes exist so the Could-items in the wishlist have a test. They are not part of the milestones. Accepting one is an owner decision.

- [ ] **C-P-02** A battle contains a scripted complication and still has exactly one hard objective. _P-SCN-01_
- [ ] **C-P-03** A content pack of a component and a scenario loads from data files and appears in game with no code change. _P-SHP-01_
- [ ] **C-P-04** The war map's front marker moves after a thrust outcome and after a hold outcome, in different directions. _P-WAR-01_
- [ ] **C-P-05** The run summary cites, per battle, the hard objective, high-value targets destroyed, and intel gathered. _P-MET-01_
- [ ] **C-P-06** Zoomed fully out, the armada is readable, and weapon, component, and power commands are unavailable at that zoom. _P-FLD-01_
- [ ] **C-P-07** The player rebinds a command and the new key performs it. _P-UI-01_
- [ ] **C-P-08** An order arriving, and a counterattack arriving, each play a short stinger or chatter line. _P-ART-01_

## Coverage

Every decided wish is listed. "Shape" means the milestones produce it and no single box owns it.

| Wish | Boxes |
| --- | --- |
| W-FAN-01 | Shape of M1 plus M4 (a combat puzzle, played as a run) |
| W-FAN-02 | C-M5-05, and the holotable in C-M0-07. Tone has no pass/fail box |
| W-FAN-03 | C-M1-03 through C-M1-06, C-M3-06. Reference only |
| W-ROL-01 | C-M0-06, C-M1-01, C-M2-01 |
| W-ROL-02 | C-M1-02, C-M2-06 |
| W-ROL-03 | C-M2-08 |
| W-CMB-01 | C-M1-03, C-M1-09 |
| W-CMB-02 | C-M1-04, C-M1-09 |
| W-CMB-03 | C-M1-05, C-M1-09 |
| W-MOV-01 | C-M1-10 |
| W-MOV-02 | C-M1-10 |
| W-MOV-03 | C-M1-11 |
| W-MOV-04 | C-M1-12 |
| W-MOV-05 | C-M1-10 |
| W-FAC-01 | C-M1-03, C-M1-11 |
| W-FAC-02 | C-M2-09 |
| W-AP-01 | C-M0-06, C-M1-12, C-M2-10 |
| W-AP-02 | C-M1-12 |
| W-AP-03 | C-M1-12 |
| W-PWR-01 | C-M1-05 |
| W-PWR-02 | C-M1-14 |
| W-PWR-03 | C-M1-14 |
| W-PWR-04 | C-M1-13 |
| W-DMG-01 | C-M1-15 |
| W-DMG-02 | C-M1-18 |
| W-DMG-03 | C-M1-16 |
| W-DMG-04 | C-M1-17 |
| W-DMG-05 | C-M1-15 |
| W-DMG-06 | C-M1-17 |
| W-DMG-07 | C-M1-20 |
| W-DMG-08 | C-M1-07 |
| W-DMG-09 | C-M1-07 |
| W-INT-01 | C-M1-08, C-M1-14, C-M1-19 |
| W-INT-02 | C-M1-19, C-M2-13 |
| W-BRD-01 | C-M6-01 |
| W-BRD-02 | C-M6-02 |
| W-BRD-03 | C-M6-03 |
| W-SQD-01 | C-M2-06, C-M3-07, C-M3-08 |
| W-SQD-02 | C-M2-06, C-M3-08 |
| W-SQD-03 | C-M3-07 |
| W-ORD-01 | C-M4-07 |
| W-ORD-02 | C-M2-11, C-M4-08 |
| W-FCT-01 | C-M2-12 |
| W-FCT-02 | C-M2-12, C-M3-09 |
| W-SHP-01 | C-M1-04, C-M3-01, C-M3-02 |
| W-SHP-02 | C-M1-06, C-M3-06 |
| W-SHP-03 | C-M3-10 |
| W-PRG-01 | C-M3-01, C-M3-05, C-M3-06 |
| W-PRG-02 | C-M3-03, C-M3-05 |
| W-PRG-03 | C-M3-04, C-M3-05, C-M5-02 |
| W-SCN-01 | C-M2-02, C-M4-05 |
| W-SCN-02 | C-M2-03, C-M4-05 |
| W-SCN-03 | C-M1-01, C-M1-07, C-M2-02, C-M2-03 |
| W-SCN-04 | C-M2-04 |
| W-SCN-05 | C-M2-05 |
| W-SCN-06 | C-M2-04, C-M2-05, C-M2-07, C-M4-04, C-M4-06 |
| W-WAR-01 | C-M4-01, C-M4-03, C-M4-04, C-M4-06 |
| W-WAR-02 | C-M4-02, C-M4-03, C-M4-05 |
| W-MET-01 | C-M5-01 |
| W-MET-02 | C-M5-02 |
| W-MET-03 | C-M5-03 |
| W-FLD-01 | C-M0-02 |
| W-FLD-02 | C-M0-03 |
| W-FLD-03 | C-M0-04 |
| W-UI-01 | C-M0-02, C-M1-09 |
| W-UI-02 | C-M3-02 |
| W-UI-03 | C-M3-05 |
| W-UI-04 | C-M0-07 |
| W-UI-05 | C-M5-04 |
| W-UI-06 | C-M0-05, C-M1-08 |
| W-UI-07 | C-M0-05, C-M1-08 |
| W-ART-01 | C-M5-05 |
| W-ART-02 | C-M5-05 |
| W-TEC-01 | C-M0-01, C-M5-06 |
| W-TEC-02 | C-M0-01 |
| W-TEC-03 | C-M5-06, C-M5-07 |
| W-TEC-04 | C-M0-06, C-M2-01, C-M5-07 |
| W-TEC-05 | C-M0-08, [SPEC.md §16](SPEC.md#16-language-and-coding-guideline) |
| W-TEC-06 | C-M0-09 |
| W-TEC-07 | C-M0-06, C-M0-09, C-M2-01, C-M5-07, C-M5-08 |

Proposed stretch wishes are the C-P boxes above, one each. Squadron commands are the decided set in W-SQD-02.
