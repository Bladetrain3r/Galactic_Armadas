# Checklist

Stage 2 of 3. Derived from [WISHLIST.md](WISHLIST.md). [SPEC.md](SPEC.md) is derived from this file. Build in milestone order: a minimal playable battle first, the full game last.

A box is done when a person can perform the test and see the result. Wishlist ids in italics are the source of the box. Wishes about tone (W-FAN-02, W-FAN-03) and the language write-up (W-TEC-05) have no single box; the coverage table says where they went.

**20,000 ships** is the acceptance stand-in for the decided phrase "tens of thousands" (W-ROL-01, W-TEC-04). It is a planning number so the test is concrete. Changing it is [OQ-22](SPEC.md#17-open-questions).

Where a test needs a rule the owner has not set, the spec offers an unapproved stand-in and names the open question. The test below still describes the outcome the owner already asked for.

## Milestone 0 — Window, grid, scale

A development scene is enough. It does not need to be a real scenario. Placeholder icons are enough.

- [ ] **C-M0-01** The application opens a window, draws through SDL, and exits cleanly. _W-TEC-01, W-TEC-02_
- [ ] **C-M0-02** The central view draws a hex grid large enough for a battle, between two sidebars. _W-FLD-01, W-UI-01_
- [ ] **C-M0-03** The camera starts moderately zoomed in. The player can pan across the whole grid and zoom in and out. _W-FLD-02_
- [ ] **C-M0-04** From elsewhere on the grid, the player can bring the view to their own ship, and to the hard objective, without scrolling the map by hand. _W-FLD-03_
- [ ] **C-M0-05** Keyboard and mouse can both pan and zoom. _W-UI-06, W-UI-07_
- [ ] **C-M0-06** A scene of at least 20,000 ship instances advances while the window keeps accepting input and drawing. The update does not hold one lock around the whole step. _W-TEC-04, W-ROL-01_
- [ ] **C-M0-07** At the starting zoom the view holds many ships, drawn as tactical icons on the holotable. _W-UI-04_

## Milestone 1 — One ship, one puzzle

The smallest playable battle: one player ship, one hard objective, one turn loop.

- [ ] **C-M1-01** A battle starts with the player in command of one ship and with exactly one hard objective. _W-ROL-01, W-SCN-03_
- [ ] **C-M1-02** The battle shows an order the player did not write, and the hard objective matches that order. _W-ROL-02_
- [ ] **C-M1-03** Each turn the player sets every weapon's orientation. A shot resolves along the orientation set for that weapon. _W-CMB-01_
- [ ] **C-M1-04** Each turn the player sets the state of each component on their ship, and resolution uses that state. _W-CMB-02, W-SHP-01_
- [ ] **C-M1-05** Each turn the player moves power from one subsystem to another. The gain and the loss are both visible before the turn resolves. _W-CMB-03_
- [ ] **C-M1-06** With two named components online, a documented synergy changes a result. Turning either component off removes the synergy. _W-SHP-02_
- [ ] **C-M1-07** The battle ends in success when its hard objective is completed, and in failure when the player ship is lost. What "lost" means is the damage stand-in in the spec until [OQ-04](SPEC.md#17-open-questions) is closed. _W-SCN-03_
- [ ] **C-M1-08** A full turn — weapons, components, power, and end turn — can be played with the keyboard alone, and the same turn can be played with the mouse. _W-UI-06, W-UI-07_
- [ ] **C-M1-09** During the battle the screen is two sidebars and the central grid, and weapons, components, and power are readable without leaving the battle. _W-UI-01, W-CMB-01, W-CMB-02, W-CMB-03_

## Milestone 2 — The battle around the ship

The same puzzle, inside the armada it belongs to.

- [ ] **C-M2-01** A played battle, past the Milestone 0 development scene, instances at least 20,000 ships. The player's next turn stays responsive while they advance. _W-ROL-01, W-TEC-04_
- [ ] **C-M2-02** A strike battle's hard objective is a marked command ship, communication node, or other designated target, and that target is visually distinct from other ships. _W-SCN-01, W-SCN-03_
- [ ] **C-M2-03** A hold battle's hard objective is to keep the player ship alive until a counterattack arrives, and the counterattack is an event the player can see. _W-SCN-02, W-SCN-03_
- [ ] **C-M2-04** Destroying a marked high-value target that is not the hard objective improves the recorded battle outcome. Leaving it alive does not. _W-SCN-04, W-SCN-06_
- [ ] **C-M2-05** An intel action, distinct from the hard objective, improves the recorded battle outcome. Skipping it does not. _W-SCN-05, W-SCN-06_
- [ ] **C-M2-06** One battle gives the player a small squadron and a command that changes what those ships do. Another battle has no squadron. Both show an order from outside the player. _W-ROL-02_
- [ ] **C-M2-07** The battle outcome is shown as its own readout, separate from the objective's success or failure. _W-SCN-06_
- [ ] **C-M2-08** The player cannot command ships outside their own ship and their squadron. Those ships still act. _W-ROL-03_

## Milestone 3 — Ship, captain, crew

Progression across battles. A session that passes through all three classes is enough; the ten-battle map arrives in Milestone 4.

- [ ] **C-M3-01** The player's ship moves from frigate to cruiser to battleship, and the three classes differ in component capacity or power. _W-PRG-01, W-SHP-01_
- [ ] **C-M3-02** A ship-management subscreen is where the detailed build is inspected and edited. In-battle component control from Milestone 1 still works. _W-UI-02, W-SHP-01_
- [ ] **C-M3-03** The captain persists from battle to battle, and at least one visible value changes because of a battle's result. _W-PRG-02_
- [ ] **C-M3-04** The crew persists from battle to battle, and at least one visible value changes because of a battle's result. _W-PRG-03_
- [ ] **C-M3-05** A progression subscreen shows ship class, captain, and crew, including the change from the latest battle. _W-UI-03, W-PRG-01, W-PRG-02, W-PRG-03_
- [ ] **C-M3-06** A cruiser or battleship build can express a synergy the starting frigate fit cannot, using the same synergy rule as C-M1-06. _W-SHP-02, W-PRG-01_

## Milestone 4 — A war of ten battles

The roguelike run.

- [ ] **C-M4-01** A run contains exactly ten battles, in order. _W-WAR-01_
- [ ] **C-M4-02** Before each battle the player chooses thrust or hold. Only the chosen battle is played. _W-WAR-02_
- [ ] **C-M4-03** The choice is presented on a war map. _W-WAR-01, W-WAR-02_
- [ ] **C-M4-04** After a battle, its outcome is visible on the war map before the next choice. _W-SCN-06, W-WAR-01_
- [ ] **C-M4-05** A thrust choice uses a strike objective (C-M2-02). A hold choice uses a hold objective (C-M2-03). _W-WAR-02, W-SCN-01, W-SCN-02_
- [ ] **C-M4-06** Finishing the tenth battle ends the run and shows the outcome of each battle. _W-WAR-01, W-SCN-06_

## Milestone 5 — Meta, holotable, ship it

The decided game around the run, and a build someone else can launch.

- [ ] **C-M5-01** A medal is earned by a stated achievement, names that achievement, and applies a small bonus the player can see. _W-MET-01_
- [ ] **C-M5-02** A run can include veteran crew by chance. A veteran crew differs from a non-veteran crew on the progression screen. _W-MET-02, W-PRG-03_
- [ ] **C-M5-03** With ironman on, loss of the player ship ends the run (the escape pod fails). The next run starts from scratch: a new frigate, a new captain, and a new crew. _W-MET-03_
- [ ] **C-M5-04** The player can change at least one thing about the holotable layout, and the change lasts for the session. Whether it lasts longer is [OQ-20](SPEC.md#17-open-questions). _W-UI-05_
- [ ] **C-M5-05** Every entity type used in a battle has an idle frame and an attack frame, in a sprite/SVG style, and no entity uses more than three frames. _W-ART-01, W-ART-02_
- [ ] **C-M5-06** A release is a standalone binary, plus only the files beside it that it needs, for more than one platform. It starts with no preinstalled language runtime. _W-TEC-01, W-TEC-03_
- [ ] **C-M5-07** That release can run the Milestone 0 scale scene and a Milestone 2 battle. _W-TEC-03, W-TEC-04_

## Not scheduled — proposed stretch

These boxes exist so the Could-items in the wishlist have a test. They are not part of the milestones. Accepting one is an owner decision.

- [ ] **C-P-01** The player sets a squadron doctrine and the squadron ships keep it while the player runs their own turn. _P-ROL-01_
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
| W-SHP-01 | C-M1-04, C-M3-01, C-M3-02 |
| W-SHP-02 | C-M1-06, C-M3-06 |
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
| W-TEC-05 | [SPEC.md §16](SPEC.md#16-language). Comparison and a recommendation; the choice stays open |

Proposed stretch wishes are the C-P boxes above, one each.
