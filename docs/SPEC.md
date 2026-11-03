# Spec

Stage 3 of 3. Derived from [CHECKLIST.md](CHECKLIST.md), which is derived from [WISHLIST.md](WISHLIST.md). Section headers cite the checklist boxes they serve.

Two kinds of text sit in this file, and they stay apart:

- **Decided.** A restatement of the owner's design, tightened only enough to implement the checklist. If this file and the wishlist ever disagree, the wishlist wins.
- **Proposal (unapproved).** A stand-in invented so a box can be built before the owner answers the related open question. Sample numbers and sample names are proposals. They exist to be replaced.

Open questions are collected in [§17](#17-open-questions). A milestone can be built while they are open by using the stand-in in that section. Shipping a stand-in as the real rule is an owner decision, not a default.

The implementation language is **not chosen**. [§16](#16-language) compares options and recommends one.

## 1. Pillars

_Checklist: the shape of Milestones 1 through 4._

Decided:

- Turn-based combat-puzzle roguelike. A fight is a setup of the ship, then a result the player can explain.
- The player is one captain. Their ship is fully managed. A small squadron is sometimes theirs to command. The armada around them is instanced and acts on its own orders.
- One hard objective per battle. High-value targets destroyed, and intel gathered, improve the overall battle outcome. The outcome is what the war records.
- A run is ten battles on a war map. Each step offers thrust (a strike) or hold (a defense) and plays the one the player picks.
- The ship class goes frigate, then cruiser, then battleship. The captain and the crew progress too.

## 2. Glossary

Decided terms. Where the mechanic is still open, the entry says so.

| Term | Meaning |
| --- | --- |
| Player ship | The one ship whose weapons, components, and power the player sets each turn. |
| Squadron | A small group of allied ships the player may command in a given battle. Some battles have none. Size and depth of command: [OQ-06](#17-open-questions). |
| Order | An instruction from outside the player, shown for the battle, which the hard objective agrees with. What it costs to ignore: [OQ-07](#17-open-questions). |
| Hard objective | The single mission goal of a battle. Completing it ends the battle in success (C-M1-07). |
| High-value target | A marked ship or node that is not the hard objective. Destroying it improves the battle outcome. |
| Intel | Information the player gathers by an action that is not the hard objective. The action: [OQ-08](#17-open-questions). |
| Battle outcome | The record of the battle that matters. Shown on its own, apart from success or failure of the objective (C-M2-07), and kept on the war map. Shape of the record: [OQ-10](#17-open-questions). |
| Thrust | The offensive choice on a war-map step. Uses a strike objective. |
| Hold | The defensive choice on a war-map step. Uses a hold-until-counterattack objective. |
| Holotable | The central battlefield view: dense tactical icons, in the role of a fleet command table. |
| Ironman | A mode in which the escape pod fails. Ship loss ends the run, and the next start is from scratch (C-M5-03). |

## 3. War and battles

_Checklist: C-M4-01, C-M4-02, C-M4-03, C-M4-04, C-M4-05, C-M4-06, C-M2-07._

Decided flow:

1. The run is a sequence of ten steps on a war map.
2. Each step offers two battles, one thrust and one hold. The player picks one. The other is not played.
3. The battle is played to success or failure of its one hard objective, while high-value targets and intel update the outcome.
4. The outcome is written onto that step of the map, where the player can read it before the next choice.
5. After the tenth battle, the run ends on a summary of the ten outcomes.

> **Proposal P-WAR-MAP (unapproved).** The map is a single column of ten steps. Each step shows two named choices. The unchosen battle leaves no outcome. The map stores, per played step, objective success or failure plus the outcome record from [P-OUT-01](#9-objectives-high-value-targets-intel). A moving front is the separate stretch wish P-WAR-01, checklist C-P-04.

Whether a bad outcome can end the war early, and whether an outcome changes a later battle, are [OQ-10](#17-open-questions) and [OQ-11](#17-open-questions). The stand-in plays all ten steps regardless of results, and later battles do not read earlier numbers. The map still shows them, which is what C-M4-04 tests.

## 4. Turn structure

_Checklist: C-M1-03, C-M1-04, C-M1-05, C-M1-07, C-M1-08, C-M1-09._

Decided, and nothing more: time is turns. On the player's turn they set weapon orientations, component states, and power routing for their own ship. Then the turn resolves. Keyboard alone can do this, and the mouse can do it too.

> **Proposal P-TURN-01 (unapproved).** Stand-in for [OQ-05](#17-open-questions).
>
> 1. **Prepare.** The order and the hard objective are already on screen. The player sets orientation per weapon, on or off per component, and a power allocation. They commit.
> 2. **Resolve.** Every ship that acts this turn acts from the state at the start of the resolve, so there is no initiative stat. Player weapons fire along their orientations if they are online and powered. Squadron and armada ships carry out their own orders. Damage is applied. Objective, high-value target, and intel flags update.
> 3. **Report.** The outcome readout changes to match those flags.
>
> Movement is not part of this stand-in. If [OQ-01](#17-open-questions) adds movement, it becomes a Prepare choice and a resolve substep before weapons fire.
>
> The battle ends after the report in which the hard objective is complete, or in which the player ship's hull reaches zero ([P-DMG-01](#6-ships-and-components)).

A sample of Prepare, used by C-M1-06, is in [§6](#6-ships-and-components). It is sample content.

## 5. Battlefield and scale

_Checklist: C-M0-02, C-M0-03, C-M0-04, C-M0-06, C-M0-07, C-M2-01, C-M2-08._

Decided:

- Hex grid. Camera opens moderately zoomed in, pans freely, zooms freely.
- Navigation can bring the view to the player ship and to the hard objective without traversing the grid by hand (C-M0-04).
- At that opening zoom the player sees many ships, as icons.
- A battle instances at least 20,000 ships. That count is the planning stand-in for "tens of thousands".
- The window keeps accepting input and drawing while those ships advance. No single lock is held around the whole update.
- The player commands only their ship and, when present, their squadron.

> **Proposal P-HEX-01 (unapproved).** Axial hex coordinates `(q, r)`. Neighbors are the six axial directions, numbered 0–5, which weapon orientation reuses. Distance is cube distance.

> **Proposal P-SIM-01 (unapproved).** Stand-in for [OQ-17](#17-open-questions) and the scale boxes.
>
> Every ship is an instance with an id, a faction, a hex, a hull, and an order it is following. Fidelity then splits:
>
> - **Player ship.** Full components, power, and per-weapon orientation.
> - **Squadron.** Order plus a reduced combat stat. Full micromanagement is [OQ-06](#17-open-questions), and the stand-in does not require it to finish a battle.
> - **Armada.** Order, hull, faction, hex, and one volley stat. They do not run the player's component puzzle.
>
> Armada updates run in parallel over spatial chunks of the grid. A chunk writes a private buffer. A following pass applies ships on chunk borders. Workers never lock a ship, and they never call SDL. One task per ship is the failure mode this proposal is here to avoid.
>
> The machine and the allowed turn time for the 20,000-ship scene are [OQ-22](#17-open-questions). Until that is set, C-M0-06 and C-M2-01 pass if the camera can be panned during the update and the window does not freeze through it.

> **Proposal P-SIM-02 (unapproved).** At the end of a resolve, the simulation publishes an immutable snapshot and keeps working on the other buffer. The render thread reads only the snapshot.

## 6. Ships and components

_Checklist: C-M1-04, C-M1-06, C-M3-01, C-M3-02, C-M3-06._

Decided:

- A build is a set of individual components on a hull. The player sets each component's state on their turn.
- Two components can form a synergy: both online, the result changes; either off, the change disappears.
- Frigate, cruiser, and battleship differ in how much build they can hold (component capacity or power).
- A ship-management subscreen exists for the build (W-UI-02). What it changes beyond the in-battle component controls is [OQ-24](#17-open-questions).

> **Proposal P-DMG-01 (unapproved).** Stand-in for [OQ-04](#17-open-questions), and the meaning of "lost" in C-M1-07. Each ship has integer hull. At zero the ship is lost and the battle ends in failure. Weapons reduce hull. Components have two states, online and offline. Per-component damage waits on the owner.

> **Proposal P-FIT-01 (unapproved).** Sample class budgets, so C-M3-01 has something to load. Not tuned.
>
> | Class | Component slots | Power budget |
> | --- | --- | --- |
> | Frigate | 4 | 6 |
> | Cruiser | 7 | 10 |
> | Battleship | 10 | 14 |
>
> The frigate's starting fit holds one of the two synergy parts in the sample below. The cruiser's slot count holds both. That is the C-M3-06 test. When the class actually changes is [OQ-12](#17-open-questions). The stand-in offers the cruiser refit after the third battle is resolved and the battleship refit after the seventh, as a hull change on the ship-management screen. Any other trigger replaces this.
>
> Stand-in for [OQ-24](#17-open-questions) and C-M3-02: between battles the ship-management subscreen fits and unfits components. During a battle the sidebar only toggles components that are already fitted.

> **Proposal P-SYNERGY-01 (unapproved).** Sample content for C-M1-06. The real component list is [OQ-04](#17-open-questions), together with damage.
>
> - `sensor_lock` (sensors): while online and powered, the current target is marked for the turn.
> - `focused_lance` (weapons): fires along its orientation. If the target is marked, the hull damage is higher.
>
> Both online and the sensors subsystem powered: the lance hits harder. Either component offline, or sensors unpowered: it does not. The ship-management screen is where a later class gains the second part. The battle screen is where each part is toggled.

## 7. Power

_Checklist: C-M1-05, C-M1-09._

Decided: each turn the player routes a limited supply of power between subsystems. Giving power to one subsystem takes it from another, and both effects are visible before the turn commits.

> **Proposal P-POW-01 (unapproved).** Stand-in for [OQ-03](#17-open-questions). Four sample subsystems: drives, weapons, shields, sensors. The player assigns every point of the class budget; the sum equals the budget. A weapon resolves only if its component is online and the weapons allocation covers that weapon's cost. The C-M1-05 demonstration moves one point from sensors to weapons: the sensor readout falls, the weapon readout rises, both before commit. Drives have no movement effect until [OQ-01](#17-open-questions) says the ship changes hexes. Shields have no effect beyond a visible readout until [OQ-04](#17-open-questions) defines damage beyond hull.

## 8. Weapons

_Checklist: C-M1-03._

Decided: each weapon has an orientation the player sets each turn. The shot resolves along that orientation.

> **Proposal P-WPN-01 (unapproved).** Stand-in for [OQ-02](#17-open-questions). Orientation is an absolute hex direction, 0–5, from [P-HEX-01](#5-battlefield-and-scale). The weapon affects targets along that ray out to a per-weapon range. There is no separate hull facing in the stand-in. If the owner adds facing, orientation becomes relative to it and this proposal is retired.

Legend of Galactic Heroes and the Honorverse set the tone of fleet warfare (W-FAN-02). They do not supply the weapon rules. Missiles, wedges, broadsides, and spinal mounts are all available later as content on top of orientation-plus-range. None of them is decided.

## 9. Objectives, high-value targets, intel

_Checklist: C-M1-01, C-M1-02, C-M1-07, C-M2-02, C-M2-03, C-M2-04, C-M2-05, C-M2-07, C-M4-04, C-M4-06._

Decided:

- One hard objective. Success is completing it. The battle also ends in failure if the player ship is lost; the stand-in for "lost" is [P-DMG-01](#6-ships-and-components).
- Strike objectives name a command ship, a communication node, or another designated target, and the target is marked.
- Hold objectives ask the player ship to survive until a counterattack the player can see.
- A marked high-value target is not the hard objective. Destroying it improves the outcome.
- An intel action is not the hard objective. Performing it improves the outcome.
- The outcome readout is distinct from the success/failure flag, and it is what the war map keeps.

> **Proposal P-OUT-01 (unapproved).** Stand-in for [OQ-08](#17-open-questions), [OQ-09](#17-open-questions), and [OQ-10](#17-open-questions).
>
> The outcome record of a battle is three visible lines: objective (done or failed), high-value targets destroyed out of those present, intel gathered out of the intel actions present. The map and the end-of-run summary show those lines as themselves.
>
> Sample strike content: objective type `destroy` aimed at a flagged entity; two other entities tagged `hvt`. Sample hold content: objective type `survive_until_counterattack`; on a scripted turn, allied ships arrive at a stated map edge and the objective completes if the player ship still has hull.
>
> Sample intel action: a marked contact can be scanned instead of fired on. Scanning sets that contact's intel flag. Destroying it does not.
>
> In the stand-in, a lost player ship fails the objective and still keeps any high-value and intel lines already earned, and the war continues to the next step. That is a placeholder for OQ-09.

## 10. Orders and the squadron

_Checklist: C-M1-02, C-M2-06, C-M2-08._

Decided:

- An order the player did not write is on screen, and the hard objective matches it.
- Some battles attach a small squadron. The player can issue a command that changes what those ships do.
- Some battles attach no squadron.
- Ships outside the player ship and the squadron still act, and the player has no command over them.

> **Proposal P-SQD-01 (unapproved).** Stand-in for [OQ-06](#17-open-questions) and [OQ-07](#17-open-questions).
>
> An order is data on the battle: a sentence and the objective id it belongs to. The stand-in does not punish ignoring it, because the objective is the same instruction stated as a win condition. A harsher order system replaces this when OQ-07 is answered.
>
> A squadron battle attaches three allied ships. The player gives them one command, `focus` (a target) or `hold` (a hex). They pursue that command with the reduced stat block from [P-SIM-01](#5-battlefield-and-scale). The player does not set their components or power. Three is a stand-in for "small". Doctrines richer than focus and hold are the stretch wish P-ROL-01 (C-P-01).

## 11. Progression and meta

_Checklist: C-M3-01, C-M3-03, C-M3-04, C-M3-05, C-M3-06, C-M5-01, C-M5-02, C-M5-03._

Decided:

- Class order is frigate, then cruiser, then battleship, with a real difference in the build ([P-FIT-01](#6-ships-and-components) until real numbers exist).
- Captain and crew are persistent across the battles of a run, and a battle can change a visible value on each.
- The progression subscreen shows class, captain, crew, and the latest change.
- A medal names an achievement and grants a small visible bonus.
- Veteran crew can appear by chance, and the progression screen shows the difference.
- Ironman: the escape pod fails. The next run starts from scratch (C-M5-03).

> **Proposal P-PRG-01 (unapproved).** Stand-in for [OQ-13](#17-open-questions).
>
> - Captain: `name`, `citations`. `citations` increments by one when a battle ends with the hard objective complete.
> - Crew: `experience`, `veteran`. `experience` increments by one when a battle ends with the player ship alive.
>
> The progression screen shows the old and new numbers. Further attributes wait on OQ-13.

> **Proposal P-VET-01 (unapproved).** Stand-in for [OQ-14](#17-open-questions). At run start, the crew becomes veteran with probability 1/10 and starts with `experience` 2 instead of 0. The progression screen shows the flag and the number. The probability is a sample constant.

> **Proposal P-MEDAL-01 (unapproved).** Stand-in for [OQ-15](#17-open-questions). One sample medal, `First blood`: destroy any high-value target. Bonus: +1 power budget on the next battle only. The medal's name and the budget change are both on screen. Medals in the stand-in live inside the run. A failed ironman run discards them with the captain, the crew, and the ship. Cross-run medals stay in OQ-15.

Class-change timing stays the [P-FIT-01](#6-ships-and-components) stand-in until [OQ-12](#17-open-questions).

## 12. UI and controls

_Checklist: C-M0-02, C-M0-03, C-M0-04, C-M0-05, C-M1-08, C-M1-09, C-M3-02, C-M3-05, C-M5-04._

Decided layout: a central battlefield with a sidebar on each side. Ship management and progression are subscreens. The battlefield is the holotable, and the player can change its layout (W-UI-05). C-M5-04 accepts a change that lasts for the session; anything longer is [OQ-20](#17-open-questions). Keyboard can play a turn. Mouse can play the same turn.

> **Proposal P-UI-02 (unapproved).** Which sidebar holds what is not decided. Stand-in:
>
> ```text
> ┌────────────┬──────────────────────────────┬────────────┐
> │ Order      │                              │ Class      │
> │ Objective  │         Holotable            │ Hull       │
> │ Outcome    │         hex grid             │ Power      │
> │ Squadron   │                              │ Weapons    │
> │            │                              │ Components │
> └────────────┴──────────────────────────────┴────────────┘
> ```
>
> The ship-management subscreen and the progression subscreen replace the center and leave the sidebars in place. Return goes back to the holotable.
>
> C-M0-04 stand-in: a "own ship" control and an "objective" control move the camera. Both have a key and a clickable control.
>
> C-M5-04 stand-in: layer toggles for faction tint, objective markers, squadron markers, and weapon-direction hints. Stored for the session. What else customization includes, and whether it outlives the session, is [OQ-20](#17-open-questions).

> **Proposal P-UI-03 (unapproved).** Default bindings. Any set that satisfies C-M1-08 replaces this table. Rebinding is the stretch wish P-UI-01 (C-P-07), not part of the stand-in.
>
> | Action | Key | Mouse |
> | --- | --- | --- |
> | Pan | Arrow keys | Drag the grid |
> | Zoom | `-` / `=` | Wheel |
> | Own ship / objective | `Home` / `End` | Buttons on the left sidebar |
> | Cycle weapons | Tab | Click the weapon row |
> | Rotate orientation | `,` / `.` | Click a direction pip on the row |
> | Toggle component | Shortcut shown on the row | Click the row |
> | Move one power point | `[` / `]` on the selected subsystem | `+` / `−` on the subsystem |
> | End turn | Enter | End-turn button |
> | Ship screen / progression / back | `F1` / `F2` / Esc | Matching buttons |

## 13. Art

_Checklist: C-M0-07, C-M5-05._

Decided: sprite/SVG style. Two or three frames per entity. Idle and attack are the required pair. A third frame may be added within that cap. Placeholder icons may satisfy Milestone 0; C-M5-05 is the art bar.

> **Proposal P-ART-02 (unapproved).** Source art is SVG. A build step rasterizes it into a sprite atlas the SDL renderer samples. The third frame slot may stay empty. What a third frame depicts, if it is used, is [OQ-21](#17-open-questions). Stingers and chatter are the separate stretch P-ART-01 (C-P-08).

## 14. Content files

_Checklist: C-M2-02, C-M2-03, C-M4-02, C-M4-05, and any box that loads a battle._

The owner did not choose a file format. This section is entirely a proposal.

> **Proposal P-DAT-01 (unapproved).** Author content as TOML, because it is language-neutral and readable in reviews. A native format may replace it after [§16](#16-language) is decided. Loading a pack with no code change is the stretch wish P-SHP-01 (C-P-03). The stand-in only requires the game to load its own data files.
>
> Sample only:
>
> ```toml
> [battle]
> id = "sample_strike"
> kind = "thrust" # thrust | hold
> order = "Destroy the enemy flagship."
>
> [battle.objective]
> type = "destroy" # or survive_until_counterattack
> target = "flagship"
>
> [[battle.hvt]]
> id = "relay_4"
>
> [[battle.intel]]
> id = "relay_4_scan"
> ```
>
> A war file lists ten steps, each with a `thrust` battle id and a `hold` battle id.

## 15. Architecture

_Checklist: C-M0-01, C-M0-06, C-M2-01, C-M5-06, C-M5-07._

Decided constraints: a standalone process, SDL for portable 2D, standalone binaries for more than one platform, native concurrency, heavily instanced battles, no lock held across a whole update.

> **Proposal P-ARC-01 (unapproved).** Three areas, talking through data rather than shared objects.
>
> ```text
> content files ──►  simulation  ──snapshot──►  presentation
>                    (workers)   ◄──commands──  (SDL, main thread)
> ```
>
> - **Content** loads definitions (classes, components, battles, the war) into plain tables.
> - **Simulation** owns ship instances and runs [P-TURN-01](#4-turn-structure) and [P-SIM-01](#5-battlefield-and-scale). It never calls SDL.
> - **Presentation** owns the window, input, camera, and drawing. It never mutates simulation state. Input becomes a command buffer consumed at the next Prepare commit. Drawing reads the latest snapshot ([P-SIM-02](#5-battlefield-and-scale)).
>
> The SDL main-thread rule (some platforms require window and GL or renderer calls on the thread that created them) is respected by keeping presentation on that thread. Workers exist for the ship update only.
>
> Determinism is part of this proposal: a seed stored on the battle drives armada intent, so the same commands produce the same result. That makes the checklist re-runnable. The owner did not ask for it. Keep the snapshot boundary even if the seed is dropped.

Save and reload are [OQ-16](#17-open-questions). The stand-in keeps the run in memory. Ironman still ends the run on ship loss because the in-memory run is discarded.

## 16. Language

_Checklist: W-TEC-05, and the binary and scale boxes C-M0-06, C-M5-06, C-M5-07._

**Decision status: open.** The recommendation at the end is [P-LANG-01](#recommendation), unapproved. The owner chooses, including a language this section sells short.

### What the language has to do

From the decided tech wishes:

1. Compile to standalone binaries for more than one platform, with no preinstalled language runtime.
2. Call SDL for 2D, from a main thread, while other threads exist.
3. Update a huge number of ship instances (acceptance bar: 20,000, with headroom past that) without a lock around the world and without one task per ship.
4. Carry a rules-heavy, content-driven game: components, synergies, scenarios, a war map.

[OQ-19](#17-open-questions) is the platform list. The comparison assumes desktop Windows, Linux, and macOS, which is [P-PLAT-01](#recommendation) and equally unapproved.

### Comparison

| | Standalone binary | SDL fit | Concurrency that stays out of lock webs | Huge ship counts | Writing the puzzle rules |
| --- | --- | --- | --- | --- | --- |
| **C# with NativeAOT**, SDL3 bindings | NativeAOT publishes a per-platform binary and does not need a desktop runtime installed. Trimming and AOT warnings are the tax. SDL bindings exist; they need an early spike because callbacks and trimming meet here. | Real bindings. The risky part is NativeAOT, not 2D itself. | `async/await` is a poor fit for a 20,000-ship tick. A `Task` per ship is the madness to avoid. `Parallel.For` over spans is the version of P-ARC-01 that C# can do well. A garbage collector still shares the process. | Strong if the sim is structs and arrays from the first week. Weak if ships become objects. | The best of the set. Fast iteration on rules, UI text, and content tools. |
| **Go**, SDL via cgo | Single static binary is routine. | cgo is required. SDL wants the main thread for the window; the Go runtime wants to move goroutines between OS threads. That fight is the known pain of Go plus SDL, and it sits on the critical path of C-M0-01 and C-M0-06. | Goroutines and channels are the clearest syntax here for "native async", and they match a command buffer well. | The collector is good and still has to scan a large live set every turn unless the sim is slices of plain values with almost no pointers. | Fine for rules. The game ecosystem is thin. Cross-compiling with cgo means a C toolchain per target. |
| **Rust**, SDL3 (SDL2 crate as fallback) | Standalone binaries, static where the platform allows. | The `sdl2` crate is mature. SDL3 bindings are younger. Fallback: start on SDL2 if SDL3 bindings block Milestone 0, and move once they are not the risk. | Ownership makes P-ARC-01 a type error when someone passes the SDL window to a worker. Chunk updates (Rayon or a small scoped thread pool) match P-SIM-01. Channels or a pair of buffers carry the snapshot and the commands. | No garbage collector. Packed arrays are the ordinary way to write this. Best headroom of the set past 20,000 full instances. | Slower iteration. The borrow checker taxes UI and any web of game-state references. The architecture in §15 avoids most of that tax by keeping the sim as tables. |
| **Zig** | Small standalone binaries, direct C interop, so SDL is a natural call. | Strong. | Threads exist. The no-lock rule is a convention the team holds, not a check the compiler makes. Async is not a reason to pick Zig. | As good as Rust if the arrays are written that way. Manual memory, explicit allocators. | Pleasant C-like code for rules. Smaller library ecosystem. Language change is still something to watch. |
| **Odin** | Standalone binaries. Designed for this kind of game. SDL bindings ship with the language vendor collection. | Strong, and used by shipped games. | Same as Zig: capable, conventional rather than enforced. | Data-oriented by habit, which is what the ship tables want. | The most game-shaped language on the list. Smaller ecosystem than Rust or C#. |

C++ with SDL3 is the shortest path to a window and the easiest path back to a lock per entity. It can implement P-ARC-01. It does not help the team keep it. Not recommended as the default.

### Recommendation

<a id="recommendation"></a>

> **Proposal P-LANG-01 (unapproved). Recommend Rust. The owner decides.**
>
> The constraint that will actually hurt is instancing a battle of tens of thousands of ships, in parallel, next to an SDL main thread, without a nest of locks. Rust matches that constraint directly: packed ship tables, a scoped parallel update, a snapshot to the renderer, and a type system that refuses to hand SDL to a worker. Standalone multi-platform binaries are routine.
>
> The honest cost is iteration speed on the puzzle rules. If, once Milestone 1 is under way, that cost dominates the simulation problem, the better switch is **C# with NativeAOT**, keeping the same architecture: struct arrays, `Parallel.For` over chunks, no `Task` per ship, SDL only on the main thread. Take that path at the start instead if the owner already knows they want the faster rules language and accepts a disciplined sim. Prove either choice with the Milestone 0 scale scene before writing the war.
>
> Go is the best reading of the words "native async", and the worst reading of "SDL on the main thread plus a huge live simulation". Pick it only after a prototype shows cgo and the collector are comfortable at 20,000 ships.
>
> Zig or Odin are credible if the owner wants game-shaped or C-shaped code and will hold the snapshot rule by review. They do not beat Rust at making that rule structural. Prefer Odin over Zig if the reason is the game, and Zig over Odin if the reason is C interop and a tiny runtime.

> **Proposal P-PLAT-01 (unapproved).** Aim the binaries at Windows, Linux, and macOS. Mobile is out of this proposal. [OQ-19](#17-open-questions) confirms or replaces the list. C-M5-06 passes with any two of them.

## 17. Open questions

Each of these is a gap in the owner's design. The stand-in next to it is a way to keep building, not an answer. Closing one means editing the decided column of the relevant section and retiring or adopting the proposal.

| ID | Question | Stand-in until answered | Boxes it touches |
| --- | --- | --- | --- |
| OQ-01 | Does a ship change hexes on a turn, and what is the movement budget for the player ship and for others? | Ships do not move. Weapons orient in place. | C-M1-03, later battles |
| OQ-02 | Is there a hull facing in addition to per-weapon orientation? | Orientation is an absolute hex direction. No facing. | C-M1-03 |
| OQ-03 | Which subsystems receive power, and may any power sit unallocated? | Drives, weapons, shields, sensors. Every point is assigned. | C-M1-05 |
| OQ-04 | How is a ship destroyed, and can components be damaged rather than only toggled? What is the real component list? | Integer hull, destroyed at zero. Components are online or offline. Sample pair `sensor_lock` + `focused_lance`. | C-M1-04, C-M1-06, C-M1-07, C-M3-06 |
| OQ-05 | Do all ships resolve at once, do sides alternate, or is there a speed or action-point system? | One prepare step, then a simultaneous resolve from the state at the start of it. | C-M1-03, C-M1-07 |
| OQ-06 | How many ships are in a "small" squadron, and does the player set their components or only give orders? | Three ships. Commands are `focus` and `hold` only. | C-M2-06, C-M2-08 |
| OQ-07 | What does an order require beyond the objective text, and what happens if the player ignores it? | The order is a sentence plus the objective id. Ignoring it has no extra penalty. | C-M1-02 |
| OQ-08 | What does the player do to gather intel? | Scan a marked contact. Destroying it does not count. | C-M2-05 |
| OQ-09 | If the hard objective fails, do high-value targets and intel already earned still stand, and can that outcome still matter to the war? | They stand, and the war continues. | C-M1-07, C-M2-04, C-M2-07, C-M4-04 |
| OQ-10 | What is stored as the battle outcome, and do earlier outcomes change later battles? | Three visible lines: objective, high-value targets, intel. Later battles ignore the numbers. The map shows them. | C-M2-07, C-M4-04, C-M4-06 |
| OQ-11 | Can the war end before ten battles? | All ten steps are played. | C-M4-01, C-M4-06 |
| OQ-12 | What triggers frigate to cruiser to battleship, and is the change a refit or a new hull? | Refit offered after battle 3 and battle 7, on the ship-management screen. | C-M3-01 |
| OQ-13 | Which captain and crew attributes exist, and what changes them? | Captain `citations`, crew `experience`, as in P-PRG-01. | C-M3-03, C-M3-04, C-M3-05 |
| OQ-14 | When is the veteran chance rolled, and what does a veteran change? | Once, at run start, probability 1/10, starting experience 2. | C-M5-02 |
| OQ-15 | Do medals persist across runs, and does ironman clear that store too? | Medals live inside the run. Ironman discards them with the run. | C-M5-01, C-M5-03 |
| OQ-16 | Is there save and reload when ironman is off, mid-battle or only between battles? | The run is in memory only. | C-M5-03, architecture |
| OQ-17 | Do armada ships run the full component, power, and orientation simulation? | Only the player ship does. Squadron is reduced. Armada is a volley stat. | C-M0-06, C-M2-01 |
| OQ-18 | One opposing force or several, and do they have names? | Two factions, `own` and `enemy`, unnamed beyond that. | C-M2-02, content |
| OQ-19 | Which platforms count as "multiple", and is mobile in scope? | Windows, Linux, macOS. Mobile excluded. Any two satisfy C-M5-06. | C-M5-06 |
| OQ-20 | What may the player customize on the holotable, and does it persist past the session? | Four layer toggles, session only. | C-M5-04 |
| OQ-21 | Is the optional third frame used, and what does it show? | Slot exists and stays empty. | C-M5-05 |
| OQ-22 | What machine, and what turn time, define the 20,000-ship test? The count itself is a stand-in and may be raised. | Pass if the camera pans during the update and the window does not freeze. | C-M0-06, C-M2-01 |
| OQ-23 | Which language? | Recommendation: Rust. See [§16](#16-language). | W-TEC-05, C-M5-06 |
| OQ-24 | What does the ship-management subscreen change that the battle sidebars do not? | The subscreen fits and unfits components. The battle screen toggles fitted ones. | C-M3-02 |

## 18. Section index

| Section | Checklist |
| --- | --- |
| 1 Pillars | Shape of M1–M4 |
| 2 Glossary | Terms used by the boxes |
| 3 War and battles | C-M4-01–C-M4-06, C-M2-07 |
| 4 Turn structure | C-M1-03, C-M1-04, C-M1-05, C-M1-07, C-M1-08, C-M1-09 |
| 5 Battlefield and scale | C-M0-02, C-M0-03, C-M0-04, C-M0-06, C-M0-07, C-M2-01, C-M2-08 |
| 6 Ships and components | C-M1-04, C-M1-06, C-M3-01, C-M3-02, C-M3-06 |
| 7 Power | C-M1-05, C-M1-09 |
| 8 Weapons | C-M1-03 |
| 9 Objectives and intel | C-M1-01, C-M1-02, C-M1-07, C-M2-02–C-M2-05, C-M2-07, C-M4-04, C-M4-06 |
| 10 Orders and squadron | C-M1-02, C-M2-06, C-M2-08 |
| 11 Progression and meta | C-M3-01, C-M3-03–C-M3-06, C-M5-01–C-M5-03 |
| 12 UI and controls | C-M0-02–C-M0-05, C-M1-08, C-M1-09, C-M3-02, C-M3-05, C-M5-04 |
| 13 Art | C-M0-07, C-M5-05 |
| 14 Content files | C-M2-02, C-M2-03, C-M4-02, C-M4-05 |
| 15 Architecture | C-M0-01, C-M0-06, C-M2-01, C-M5-06, C-M5-07 |
| 16 Language | W-TEC-05, C-M0-06, C-M5-06, C-M5-07 |
| 17 Open questions | Gaps those boxes still have |

## 19. Proposal index

Unapproved. Stretch wishes live in the wishlist and are tested by C-P-01, C-P-02, C-P-03, C-P-04, C-P-05, C-P-06, C-P-07, and C-P-08. The ids below are spec stand-ins only.

| ID | Where | What it stands in for |
| --- | --- | --- |
| P-WAR-MAP | §3 | Map of ten steps, two choices, no extra strategic layer |
| P-TURN-01 | §4 | Prepare, simultaneous resolve, report |
| P-HEX-01 | §5 | Axial hexes, directions 0–5 |
| P-SIM-01 | §5 | Fidelity tiers and chunked parallel update |
| P-SIM-02 | §5 | Snapshot buffer to the renderer |
| P-DMG-01 | §6 | Hull points, components online or offline |
| P-FIT-01 | §6 | Sample slot and power budgets, sample refit timing |
| P-SYNERGY-01 | §6 | Sample component pair |
| P-POW-01 | §7 | Sample subsystems and a full allocation |
| P-WPN-01 | §8 | Absolute hex-direction orientation and range |
| P-OUT-01 | §9 | Three-line outcome, sample strike, hold, and intel |
| P-SQD-01 | §10 | Order text, squadron of three, focus or hold |
| P-PRG-01 | §11 | Citations and experience |
| P-VET-01 | §11 | Veteran roll at run start |
| P-MEDAL-01 | §11 | Sample medal |
| P-UI-02 | §12 | Sidebar contents, subscreens, camera jumps, layer toggles |
| P-UI-03 | §12 | Default bindings |
| P-ART-02 | §13 | SVG source, sprite atlas at build |
| P-DAT-01 | §14 | TOML content files |
| P-ARC-01 | §15 | Content, simulation, presentation, command buffer |
| P-LANG-01 | §16 | Recommend Rust |
| P-PLAT-01 | §16 | Windows, Linux, macOS |
