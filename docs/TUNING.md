# Tuning defaults

These are the numbers and unstated rules the build uses wherever the owner has not chosen. They are not decided design. Change a row here, then change the section it names. The wishlist wins when it actually states a rule. A tuning default never overrides a decided sentence.

Ids match the old open-question numbers (`TD-34` was `OQ-34`) so earlier notes still find them.

## Defaults

| ID | Default | Why this one | Lives in |
| --- | --- | --- | --- |
| TD-34 | Taking one power point off a live subsystem costs 2 AP. Assigning one spare point costs 1 AP. | The one-point discount is decided. A base of 2 leaves spare power cheaper and still lets a frigate shoot. | [SPEC §7](SPEC.md#7-power) |
| TD-35 | The interior is a paused second view during the player's turn. Crew may walk any path of rooms before the turn resolves. A crew member who ends the turn in a damaged room repairs one crippled component or one damaged line. Ship AP is not spent. | FTL walks crew while the fight is paused. Charging ship AP for every step would swallow the first battle. | [SPEC Interior](SPEC.md#interior) |
| TD-36 | A room is one fitted component or one distribution node. Power lines are the doors. Crew aboard: frigate 4, cruiser 6, battleship 8. | The rooms are the parts you already repair. Four crew is a small FTL start, and the first battle is a frigate. | [SPEC Interior](SPEC.md#interior) |
| TD-37 | Integrity starts at 2. At 1 the component is crippled. At 0 it is critical, with a red zone 1 deep, or 2 deep if the component is over-engineered. Dry dock is a separate hit, not a deeper red zone. | Two hits show all three tiers. No fabrication math in the first battle. | [SPEC §6](SPEC.md#6-ships-and-components) |
| TD-38 | A dry-docked component is replaced on the ship screen and does not end the run. Only a massive hole in the hull, or a shredded chassis, ends the run. | The component rule says "replace it." The run-ending rule was about the chassis. | [SPEC §6](SPEC.md#6-ships-and-components) |
| TD-39 | Evacuation restarts the current battle. The ship, the fit, and the crew are kept. | The smallest setback that is still a loss. The first battle does not need a campaign clock. | [SPEC §6](SPEC.md#6-ships-and-components) |
| TD-40 | Abandoning while any combat component still works, with no damage cause, ends the run. Every combat component at the mild crippled tier or worse loses the battle and uses TD-39. That damage does not end the run. | "Without an in-game cause" is the voluntary leave. The crippled-ship case already has a cause. | [SPEC §6](SPEC.md#6-ships-and-components) |
| TD-44 | Power lines can be diverted and repaired. Plumbing is a separate pipe set, fixed on the chassis, with no action in the first build. | The power puzzle stays playable. Pipes do not freeze the grid. | [SPEC §7](SPEC.md#7-power) |
| TD-27 | The player ship acts, then the squadron, then every other ship that has AP. Each sees the state the previous actor left. | The player solves the position, then the fleet answers. | [SPEC §4](SPEC.md#4-turn-structure) |
| TD-41 | A vessel has an interior when it is on a hex adjacent to the player. | One hex is the smallest bubble. The far fleet stays a batch. | [SPEC Interior](SPEC.md#interior) |
| TD-42 | Crew in the interior repair, and do nothing else. | The bone is repair. Fires and vents can be added without changing the map. | [SPEC Interior](SPEC.md#interior) |
| TD-17 | Beyond 8 hexes a ship has one facing, one volley, and the AP skip. It has no power grid and no interior. Within 8 hexes, hull and weapon facings are real. | Twenty thousand ships cannot each run the player's puzzle. Eight hexes is one sensor bubble. | [SPEC §5](SPEC.md#5-battlefield-and-scale) |
| TD-06 | A ship of the line is a battleship that is not the flagship. A wing is one token with a craft count of 4. | The class list stays frigate, cruiser, battleship. Strike craft do not each become a full ship. | [SPEC §10](SPEC.md#10-orders-and-the-squadron) |
| TD-08 | Intel is one action: sensors powered, 1 AP, a marked contact within 4 hexes, once per contact. Destroying the contact grants no intel. | One extra command. Not a third game. | [SPEC §9](SPEC.md#9-objectives-high-value-targets-intel) |
| TD-09 | High-value targets and intel already earned stay on the outcome if the objective fails. The next battle still happens. | The war keeps the outcome, not only the win flag. | [SPEC §9](SPEC.md#9-objectives-high-value-targets-intel) |
| TD-10 | The outcome is three lines: objective done or failed, high-value targets destroyed out of those present, intel gathered out of those present. Later battles do not read those lines, except a missed order (TD-07). | The map can show the lines. There is no hidden score to tune yet. | [SPEC §9](SPEC.md#9-objectives-high-value-targets-intel) |
| TD-30 | Squad Command runs from 0 to 3. It rises by 1 when the squadron carries out its order. Only 3 unlocks a battleship squadron. Frigate and cruiser squadrons have no skill gate. | Three steps fit ten battles. The only decided gate is the battleship. | [SPEC §11](SPEC.md#11-progression-and-meta) |
| TD-12 | The ship refits to a cruiser after battle 3 and to a battleship after battle 7. Same captain and crew. | Two upgrades inside the ten-battle run. | [SPEC §6](SPEC.md#6-ships-and-components) |
| TD-13 | The captain's citations rise by 1 when the hard objective is completed. Crew experience rises by 1 when the ship survives the battle. No other captain or crew stats. | One visible number each, beside the decided Squad Command skill. | [SPEC §11](SPEC.md#11-progression-and-meta) |
| TD-24 | Between battles the ship screen fits and unfits refittable components and cannot change plumbing. In battle the sidebar toggles fitted components on and off. Repair is the interior. | Three screens, three jobs. | [SPEC §6](SPEC.md#6-ships-and-components) |
| TD-32 | Imperial `belt_plate` gives every component +1 starting integrity. Alliance `rapid_mount` makes weapon hits deal +1 integrity damage. Capturing one adds it on the ship screen. The ids are placeholders. | One toughness piece and one gun piece, which is the decided difference. | [SPEC Factions](SPEC.md#factions) |
| TD-07 | If an order is missed and no other ship completed it, the next battle starts at −2 AP. In battles 1–5 the order is tracked beside the objective. In battles 6–10 the order is part of the hard objective. | One number, then a single step-up halfway through the war. | [SPEC §3](SPEC.md#3-war-and-battles) |
| TD-11 | All ten battles are played unless the run has already ended (shredded chassis, causeless abandonment, or ironman). | The skeleton is a full war. An early strategic loss can be tuned later. | [SPEC §3](SPEC.md#3-war-and-battles) |
| TD-31 | Boarding starts when the player gives focus-to-capture on a ship whose combat components are all crippled or worse. The clock is 8 turns. The map is five rooms in a line, bridge at one end and engineering at the other. Everyone has a sidearm and a cutter. The player also picks one tool. Both rooms must be taken before the clock or the ship auto-destructs. | The shortest crawl that still has two rooms and a clock inside a 5 to 10 minute play. | [SPEC Boarding](SPEC.md#boarding) |
| TD-45 | A normal evacuation uses TD-39. In ironman, evacuation ends the run. | Ironman is the mode where the escape pod fails. | [SPEC §11](SPEC.md#11-progression-and-meta) |
| TD-14 | At run start, a 1-in-10 roll makes the crew veteran and sets experience to 2. Otherwise experience starts at 0. | Rare, and visible on one screen. | [SPEC §11](SPEC.md#11-progression-and-meta) |
| TD-15 | Medals last for the current run. Ironman deletes them with the run. There is no cross-run locker. | The first loop is one war. | [SPEC §11](SPEC.md#11-progression-and-meta) |
| TD-16 | One save is written on the war map after each battle. There is no reload inside a battle. Ironman deletes that save when the run ends. | A run can be resumed. Ironman still has no second chance. | [SPEC §15](SPEC.md#15-architecture) |
| TD-20 | Holotable toggles: faction tint, objective markers, squadron markers, weapon-direction hints. They last for the session and are not written to disk. | The smallest customisation that is still a holotable. | [SPEC §12](SPEC.md#12-ui-and-controls) |
| TD-21 | Entities use idle and attack only. The third frame slot stays empty. | The required pair is the whole bone. | [SPEC §13](SPEC.md#13-art) |
| TD-22 | The simulation does not run on the GPU. Budget about 512 MB of VRAM for the window and the sprite atlas. | Fits the container and the 8 GB box without a discrete GPU. | [SPEC §15](SPEC.md#15-architecture) |
| TD-SCALE | The scale test uses 20,000 ships. | The low end of "tens of thousands," enough to prove the batch. | [SPEC §5](SPEC.md#5-battlefield-and-scale), [CHECKLIST](CHECKLIST.md) Milestone 0 |
| TD-FIT | Slots and generators: frigate 4 and 2, cruiser 7 and 3, battleship 10 and 4. | A cruiser can hold the sample synergy pair. A frigate cannot. | [SPEC §6](SPEC.md#6-ships-and-components) |
| TD-RNG | Weapon range and the intel range are both 4 hexes. | One distance for "can I affect that contact." | [SPEC §8](SPEC.md#8-weapons) |

The sample synergy pair `sensor_lock` plus `focused_lance` is the content loaded for the first synergy test. It is sample content in [SPEC §6](SPEC.md#6-ships-and-components), not a faction or a final catalog.

## Milestone 0 scale scene

Added for the first window. These are defaults, not owner decisions. The rows above are unchanged.

### Deviation

Rendering uses the `sdl2` crate 0.38 (`bundled`, `static-link`), not SDL3. [SPEC §16](SPEC.md#16-language-and-coding-guideline) names SDL3 as the target and tells the build to start on the mature SDL2 crate if the SDL3 bindings would block Milestone 0. The `sdl3` crate's own readme, at 0.22, still says the migration is in progress and that features are missing. The sim/render split does not depend on which SDL this is.

The scene drifts ships. It does not resolve a battle turn. That is the Milestone 0 development scene.

### Defaults

| ID | Default | Why this one | Lives in |
| --- | --- | --- | --- |
| TD-M0-VIEW | Pointy-top hexes. A hex is 22 px from center to corner at zoom 1, which is the opening zoom. Zoom clamps to 0.06–2.5. Arrow keys pan at 900 screen pixels per second. The wheel and `-` / `=` multiply zoom by 1.12 around the cursor. | Zoom 1 shows many ships and not the whole armada. 0.06 fits the radius-82 map in a modest window. | `src/camera.rs` |
| TD-M0-LOD | A facing chevron at 14 px and above, a diamond from 5 px, a point below that. Grid outlines are skipped below 3.5 px, and only every fifth hex is drawn from 3.5 to 8 px. | Drawing 20,000 outlines is the cost that does not buy a readable picture. | `src/render.rs` |
| TD-M0-SIM | 20,000 ships on a map of radius 82. The sim thread steps at 30 Hz. Drift is about 0.15 hex per second. Aside from the player, one ship in eight starts with 0 action points and is never given a decision. The player is an Imperial frigate at (0, 0) with 10 AP. The objective marker is an Alliance ship at (60, −24). | TD-SCALE is the count. The rest makes the skip rule and the two camera jumps visible. | `src/sim.rs` |
| TD-M0-COLOR | Imperial crimson (196, 64, 72). Alliance blue (64, 140, 210). The player outline is gold. The objective outline is amber. | Placeholder heraldry until OWN-01. The two colours have to separate at point size. | `src/ship.rs`, `src/render.rs` |

## Owner-only

Taste and lore. Nothing in the build waits on these. Placeholder ids in TD-32 are what the game loads until they are replaced.

| ID | Question |
| --- | --- |
| OWN-01 | What are the real names and blurbs of the Imperial and Alliance special components? |
