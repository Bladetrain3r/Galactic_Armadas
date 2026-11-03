# Spec

Stage 3 of 3. Derived from [CHECKLIST.md](CHECKLIST.md), which is derived from [WISHLIST.md](WISHLIST.md). Section headers cite the checklist boxes they serve.

Two kinds of text sit in this file, and they stay apart:

- **Decided.** A restatement of the owner's design, tightened only enough to implement the checklist. If this file and the wishlist ever disagree, the wishlist wins.
- **Proposal (unapproved).** A stand-in invented so a box can be built before the owner answers the related open question. Sample numbers and sample names are proposals. They exist to be replaced.

Open questions are collected in [§17](#17-open-questions). A milestone can be built while they are open by using the stand-in in that section. Shipping a stand-in as the real rule is an owner decision, not a default.

The implementation language is **Rust**. [§16](#16-language-and-coding-guideline) is the coding guideline: legible, modular Rust for a reader who does not write much Rust.

## 1. Pillars

_Checklist: the shape of Milestones 1 through 4._

Decided:

- Turn-based combat-puzzle roguelike. A fight is a setup of the ship, then a result the player can explain.
- The player is one captain. Their ship is fully managed. A squadron is sometimes theirs to command, with the composition and the four orders in [§10](#10-orders-and-the-squadron). The armada around them is instanced and acts on its own orders.
- Time is turns of action points. Frigates have the most. Ships have momentum, a top speed of 3 hexes a turn, and separate hull and weapon facings.
- One hard objective per battle. High-value targets destroyed, and intel gathered, improve the overall battle outcome. The outcome is what the war records. An ignored order that nobody else completes has a moderately outsized effect on the campaign.
- A run is ten battles on a war map. Each step offers thrust (a strike) or hold (a defense) and plays the one the player picks.
- The ship class goes frigate, then cruiser, then battleship. The captain and the crew progress too.

## 2. Glossary

Decided terms. Where the mechanic is still open, the entry says so.

| Term | Meaning |
| --- | --- |
| Player ship | The one ship whose weapons, components, and power the player sets each turn. |
| Squadron | Allied ships the player may command in a battle. Some battles have none. Composition is by class (W-SQD-01). Commands are focus, hold, protect, and follow. |
| Squad Command | An improvable skill. A battleship squadron requires it at maximum. |
| Order | A campaign objective shown for the battle. The hard objective agrees with it. Ignoring it has a moderately outsized campaign effect unless another ship completes it. |
| Action points | The budget a ship spends on commands in a turn. Starting values for tuning are frigate 10, cruiser 8, battleship 6. A ship that cannot afford an action is skipped. |
| Hard objective | The single mission goal of a battle. Completing it ends the battle in success (C-M1-07). |
| Strategic objective | What one side is trying to do in the battle. Both sides have one. The player's order is their piece of their side's objective, and the link tightens as the campaign goes on. |
| High-value target | A marked ship or node that is not the hard objective. Destroying it improves the battle outcome. |
| Intel | Information the player gathers by an action that is not the hard objective. The action: [OQ-08](#17-open-questions). |
| Battle outcome | The record of the battle that matters. Shown on its own, apart from success or failure of the objective (C-M2-07), and kept on the war map. Shape of the record: [OQ-10](#17-open-questions). |
| Thrust | The offensive choice on a war-map step. Uses a strike objective. |
| Hold | The defensive choice on a war-map step. Uses a hold-until-counterattack objective. |
| Holotable | The central battlefield view: dense tactical icons, in the role of a fleet command table. |
| Ironman | A mode in which the escape pod fails. The next start is from scratch (C-M5-03). How that meets evacuation, which does not end the run, is [OQ-45](#17-open-questions). |
| Crippled | A component tier. Mostly a resource-routing fault, or a minor overheat or surge. |
| Critical | A component tier. Significant damage, and no new part has to be fabricated. Integrity has hit 0 and the component is in a red zone whose depth follows its durability and over-engineering. On cooling or the reactor, this tier may detonate the ship. |
| Dry dock (component) | A hole where the component was, or major parts fused. The component has to be replaced. |
| Dry dock (ship) | A massive hole in the ship, or a shredded chassis. Ends the run. Whether a dry-docked component is this event is [OQ-38](#17-open-questions). |
| Evacuation | A loss that pushes in-run progress back and does not end the run. How far is [OQ-39](#17-open-questions). |
| Abandonment | Crippling every combat-capable component abandons the ship. Abandonment without an in-game cause ends the run. Which events are the causeless ones is [OQ-40](#17-open-questions). |
| Interior | The small FTL-like roguelike inside the player ship and most immediately proximate vessels. Crew move through it. Repair is one of their actions. |
| Imperial Royal Navy | One of the two factions. Its ships are a little tougher. |
| Allied People's Naval Forces | The other faction. Its ships have slightly more gun. |

## 3. War and battles

_Checklist: C-M4-01, C-M4-02, C-M4-03, C-M4-04, C-M4-05, C-M4-06, C-M4-07, C-M4-08, C-M2-07, C-M2-11._

Decided flow:

1. The run is a sequence of ten steps on a war map.
2. Each step offers two battles, one thrust and one hold. The player picks one. The other is not played.
3. Each battle has a strategic objective for each side. The player's order is linked to their side's objective, and a later battle in the ten links them more tightly than an early one (C-M4-08).
4. The battle is played to success or failure of its one hard objective, while high-value targets and intel update the outcome.
5. If the player ignores the order and no other ship completes it, the campaign effect is moderately outsized compared with missing a side target (C-M4-07). Another ship completing it absorbs that effect.
6. The outcome is written onto that step of the map, where the player can read it before the next choice.
7. After the tenth battle, the run ends on a summary of the ten outcomes.

> **Proposal P-WAR-MAP (unapproved).** The map is a single column of ten steps. Each step shows two named choices. The unchosen battle leaves no outcome. The map stores, per played step, objective success or failure plus the outcome record from [P-OUT-01](#9-objectives-high-value-targets-intel). A moving front is the separate stretch wish P-WAR-01, checklist C-P-04.

The campaign already changes when an order is ignored (above). Whether the war can end before ten battles is [OQ-11](#17-open-questions). What else an outcome stores, and whether high-value targets and intel change later battles on top of the order effect, is [OQ-10](#17-open-questions). The stand-in still plays all ten steps, shows the outcome lines on the map, and does not feed those lines into later battle setup except for the decided order effect. How large "moderately outsized" is, and the schedule of the tightening link, are the rest of [OQ-07](#17-open-questions).

## 4. Turn structure

_Checklist: C-M1-03, C-M1-04, C-M1-05, C-M1-07, C-M1-08, C-M1-09, C-M1-10, C-M1-11, C-M1-12, C-M0-06, C-M2-10._

Decided:

- Time is turns. Each ship has an action-point pool. Starting values, which the owner gave as a guess to tune: frigate 10, cruiser 8, battleship 6 (W-AP-03). Frigates still have the most.
- A ship that does not have the points to do anything is skipped. The simulation does not process a decision for it.
- The player spends points on commands for their own ship: acceleration and braking, hull facing, weapon facings, component state, and power. The crew carries the commands out. Movement draws on the same pool as the other commands.
- Turning the hull costs 1 action point per hex facing (C-M1-11).
- Acceleration is a full-turn action. That ship takes no other command while it accelerates. It moves at most 3 hexes that turn. Braking applies the opposite force and costs the same, so it is a full-turn action too (C-M1-10). No separate point price was stated for that shared cost.
- Ships have momentum. They keep their vector until they brake or change it.
- Keyboard alone can play the turn, and the mouse can play the same turn.

> **Proposal P-TURN-01 (unapproved).** Stand-in for who acts before whom ([OQ-27](#17-open-questions)). The action-point numbers above are decided and are not part of this proposal.
>
> 1. **Prepare.** The order, the hard objective, and both sides' strategic objectives are on screen. The player spends action points, including any crew orders in the interior. They commit.
> 2. **Resolve.** Ships that still have a legal command act in this order: the player ship, then squadron ships, then every other ship with enough points. Each acts on the latest state. Acceleration or braking resolves as that ship's entire action. Weapons fire along their own facings when those weapons are online and powered. Damage, objectives, high-value targets, and intel update.
> 3. **Report.** The outcome readout matches those flags.

A sample of a component synergy, used by C-M1-06, is in [§6](#6-ships-and-components). It is sample content.

## 5. Battlefield and scale

_Checklist: C-M0-02, C-M0-03, C-M0-04, C-M0-06, C-M0-07, C-M2-01, C-M2-08, C-M2-09, C-M1-10._

Decided:

- Hex grid. Camera opens moderately zoomed in, pans freely, zooms freely.
- Navigation can bring the view to the player ship and to the hard objective without traversing the grid by hand (C-M0-04).
- At that opening zoom the player sees many ships, as icons.
- A battle instances at least 20,000 ships. That count is the planning stand-in for "tens of thousands". Processing a turn of that scene takes under a second on 4 cores and 8 GB with nothing else running (W-TEC-07). The official minimum memory is 16 GB.
- The window keeps accepting input and drawing while those ships advance. No single lock is held around the whole update. Ships without the action points to act are not given a decision.
- Hull facing and weapon facings are separate. A ship the player is not close to may use a cheap facing approximation or a batched update. The fiction is sensor falloff: the computers spend their time on what is close (C-M2-09).
- The player commands only their ship and, when present, their squadron.
- Movement: momentum, coasting until brake or vector change, acceleration and braking as the same full-turn action with opposite force, at most 3 hexes in a turn, hull turns at 1 action point per facing ([§4](#4-turn-structure)).

> **Proposal P-HEX-01 (unapproved).** Axial hex coordinates `(q, r)`. Neighbors are the six axial directions, numbered 0–5, which weapon orientation reuses. Distance is cube distance.

> **Proposal P-SIM-01 (unapproved).** Stand-in for the part of [OQ-17](#17-open-questions) the owner has not closed. Distance approximations for facing, and skipping ships with no action points, are already decided. This proposal only covers the rest.
>
> Every ship is an instance with an id, a faction, a hex, a velocity, a hull facing, and an order. Fidelity then splits:
>
> - **Player ship.** Full components, the power grid, and a facing per weapon.
> - **Squadron.** The four commands, plus a reduced combat stat. The player does not set their components. Strike craft are one instance per wing, with a craft count ([P-WING-01](#10-orders-and-the-squadron)).
> - **Armada, far from the player.** Order, faction, hex, velocity, one approximate facing, and one volley stat. They do not run the power grid.
>
> Updates of the far armada run in parallel over spatial chunks. A chunk writes a private buffer. A following pass applies ships on chunk borders. Workers never lock a ship, and they never call SDL. One task per ship is the failure this proposal is here to avoid.
>
> VRAM is [OQ-22](#17-open-questions). C-M0-06 and C-M2-01 pass when a turn finishes in under a second on 4 cores and 8 GB and the camera can pan during the update.

> **Proposal P-SIM-02 (unapproved).** At the end of a resolve, the simulation publishes an immutable snapshot and keeps working on the other buffer. The render thread reads only the snapshot.

## 6. Ships and components

_Checklist: C-M1-04, C-M1-06, C-M1-07, C-M1-15, C-M1-16, C-M1-17, C-M1-18, C-M1-20, C-M3-01, C-M3-02, C-M3-06, C-M3-10._

Decided:

- A build is a set of individual components on a hull. The player sets each component's state on their turn.
- Two components can form a synergy: both online, the result changes; either off, the change disappears.
- Frigate, cruiser, and battleship differ in how much build they can hold.
- A ship-management subscreen exists for the build (W-UI-02). What it changes beyond the in-battle controls is [OQ-24](#17-open-questions).
- Components are fairly fine-grained. Most components with a clear purpose are refittable. Plumbing, pipes and the like, is fixed on the chassis (W-SHP-03, C-M3-10).
- A component's damage tiers are:
  - **Crippled.** Mostly a resource-routing fault, or a minor overheat or surge (C-M1-15).
  - **Critical.** Significant damage, and no new fabrication. After integrity hits 0 the component is in a red zone. The depth of that zone varies with durability and over-engineering (C-M1-17). Critical damage to cooling or to the reactor may detonate the ship.
  - **Dry dock.** A hole where the component was, or major parts fused. It has to be replaced (C-M1-20).
- A massive hole in the ship, or a shredded chassis, ends the run (C-M1-18). That is the earlier ship-scale rule. It uses the same words as component dry dock. [OQ-38](#17-open-questions) asks whether they are one event.
- Crippling every combat-capable component abandons the ship (C-M1-16). "Crippled" is also the mild tier above. [OQ-40](#17-open-questions) asks whether that abandonment still means the mild tier.
- Evacuation is a loss. It pushes in-run progress back and does not end the run (C-M1-07, W-DMG-08). How far is [OQ-39](#17-open-questions).
- Abandonment without an in-game cause ends the run (W-DMG-09). Which abandonments that sentence covers is [OQ-40](#17-open-questions).

Starting integrity, and the numbers for durability and over-engineering, are [OQ-37](#17-open-questions). Whether the divertable power lines are the fixed plumbing is [OQ-44](#17-open-questions).

> **Proposal P-DMG-01 (unapproved).** Sample numbers for [OQ-37](#17-open-questions) only. A component starts at integrity 3. The red zone is 1 more point of damage, or 2 if the component is tagged over-engineered. A dry-dock result is a separate flag on the hit, not a deeper red zone. None of these figures is approved. The old sound-or-crippled boolean is retired.

> **Proposal P-FIT-01 (unapproved).** Sample slot counts, so C-M3-01 has something to load. Power comes from generators ([§7](#7-power)), not from a single class budget. Not tuned.
>
> | Class | Component slots | Generators |
> | --- | --- | --- |
> | Frigate | 4 | 2 |
> | Cruiser | 7 | 3 |
> | Battleship | 10 | 4 |
>
> The frigate's starting fit holds one of the two synergy parts in the sample below. The cruiser's slot count holds both. That is the C-M3-06 test. When the class actually changes is [OQ-12](#17-open-questions). The stand-in offers the cruiser refit after the third battle is resolved and the battleship refit after the seventh, as a hull change on the ship-management screen. Any other trigger replaces this.
>
> Stand-in for [OQ-24](#17-open-questions) and C-M3-02: between battles the ship-management subscreen fits and unfits components that have a clear purpose, and does not refit plumbing. During a battle the sidebar toggles components that are already fitted. Repair is the interior, not a button on that sidebar.

> **Proposal P-SYNERGY-01 (unapproved).** Sample content for C-M1-06. Named parts beyond this pair are content. The refit rule is W-SHP-03.
>
> - `sensor_lock` (sensors): while online and powered, the current target is marked for the turn.
> - `focused_lance` (weapons): fires along its orientation. If the target is marked, the hull damage is higher.
>
> Both online and the sensors subsystem powered: the lance hits harder. Either component offline, or sensors unpowered: it does not. The ship-management screen is where a later class gains the second part. The battle screen is where each part is toggled.

## 7. Power

_Checklist: C-M1-05, C-M1-09, C-M1-13, C-M1-14._

Decided:

- The subsystems are drives, weapons, shields, sensors, ECM, life support, tactical computers, heat pumps, capacitors and batteries, ammo loaders, powered structural support, and doors and bulkheads.
- A ship has multiple generators, distribution nodes through the hull, and power lines between them.
- A line can be diverted, damaged, and repaired. A damaged line can cap how much power reaches what it feeds. A main lane leaving a generator is the case C-M1-14 tests.
- Points may sit unassigned. Assigning one point of spare power costs one action point less than re-allocating one point off a subsystem that is already drawing it (C-M1-13). The slower case is the engineering crew rebalancing the grid.
- Both the gain and the loss are visible before the player commits.
- Repair of a line is a crew action in the interior ([Interior](#interior)), not a sidebar spend.

The absolute action-point cost of moving one point off a live subsystem is [OQ-34](#17-open-questions). The spare-power discount of one is already decided, so the spare cost is that number minus one.

> **Proposal P-POW-01 (unapproved).** Sample grid for one frigate, so the boxes have a layout. Two generators. Each feeds a distribution node through a main lane. Nodes feed the subsystems through branch lines. A weapon fires only if it is sound, online, and its branch can carry its cost. Diverting a branch swaps which subsystem it feeds. Stand-in for OQ-34, not a decision: taking one point off a live subsystem costs 2 action points, so assigning one spare point costs 1.

## Interior

_Checklist: C-M1-08, C-M1-14, C-M1-19, C-M2-13._

Decided:

- Repair is a crew action (W-INT-01).
- The player ship is a small FTL-like roguelike of its own. Crew move through the interior to repair, and to do the other work of that layer (W-INT-02).
- Most vessels immediately proximate to the player are the same kind of interior (C-M2-13). The far armada is not.

How that layer sits inside a battle turn, and what a crew move spends, is [OQ-35](#17-open-questions). What counts as a room, and how many crew are aboard, is [OQ-36](#17-open-questions). How near "immediately proximate" is is [OQ-41](#17-open-questions). Which actions the "etc." covers besides repair is [OQ-42](#17-open-questions). OQ-42 does not block the first battle: repair alone satisfies C-M1-14 and C-M1-19.

> **Proposal P-INT-01 (unapproved).** Stand-in for OQ-35 and OQ-36. The interior is a second view opened from the battle. Rooms are the fitted components plus the distribution nodes, joined by the power lines. A frigate has 4 crew. During the battle turn the player orders a crew member into an adjacent room for 1 of the ship's action points. A crippled component or a damaged line is repaired when a crew member spends a further action point in its room. A dry-docked component is not repaired this way; it needs replacement. This timing and this headcount are not approved.

## 8. Weapons

_Checklist: C-M1-03, C-M1-11, C-M2-09._

Decided: the hull has a facing, and each weapon has its own facing. The player sets weapon facings on their turn. A shot resolves along that weapon's facing. Turning the hull takes time and does not drag the weapon facings with it. Ships far from the player may show a cheap approximation instead of every facing.

> **Proposal P-WPN-01 (unapproved).** Each facing is a hex direction 0–5 from [P-HEX-01](#5-battlefield-and-scale), stored separately for the hull and for each weapon. A weapon affects targets along its ray out to a per-weapon range. Range and arcs are content. "Close" for C-M2-09 means within 8 hexes of the player ship until a real sensor range exists. That radius is not approved.

Legend of Galactic Heroes and the Honorverse set the tone of fleet warfare (W-FAN-02). Weapon rules are the facings above, plus whatever components the catalog in [OQ-04](#17-open-questions) adds. Missiles, wedges, broadsides, and spinal mounts can be content on top of a facing and a range. None of those mounts is decided.

## 9. Objectives, high-value targets, intel

_Checklist: C-M1-01, C-M1-02, C-M1-07, C-M2-02, C-M2-03, C-M2-04, C-M2-05, C-M2-07, C-M4-04, C-M4-06._

Decided:

- One hard objective. Success is completing it. The battle ends in failure when the player ship is abandoned, evacuated, detonated, or disabled ([§6](#6-ships-and-components)).
- Strike objectives name a command ship, a communication node, or another designated target, and the target is marked.
- Hold objectives ask the player ship to survive until a counterattack the player can see.
- A marked high-value target is not the hard objective. Destroying it improves the outcome.
- An intel action is not the hard objective. Performing it improves the outcome.
- The outcome readout is distinct from the success/failure flag, and it is what the war map keeps.

> **Proposal P-OUT-01 (unapproved).** Stand-in for [OQ-08](#17-open-questions), [OQ-09](#17-open-questions), and [OQ-10](#17-open-questions).
>
> The outcome record of a battle is three visible lines: objective (done or failed), high-value targets destroyed out of those present, intel gathered out of the intel actions present. The map and the end-of-run summary show those lines as themselves.
>
> Sample strike content: objective type `destroy` aimed at a flagged entity; two other entities tagged `hvt`. Sample hold content: objective type `survive_until_counterattack`; on a scripted turn, allied ships arrive at a stated map edge and the objective completes if the player ship has not been lost.
>
> Sample intel action: a marked contact can be scanned instead of fired on. Scanning sets that contact's intel flag. Destroying it does not.
>
> In the stand-in, a lost player ship fails the objective and still keeps any high-value and intel lines already earned, and the war continues to the next step. That is a placeholder for OQ-09.

## 10. Orders and the squadron

_Checklist: C-M1-02, C-M2-06, C-M2-08, C-M2-11, C-M3-07, C-M3-08, C-M4-07, C-M4-08._

Decided:

- An order the player did not write is on screen, and the hard objective matches it.
- The order is a key objective for the campaign. The running example is to strike a cruiser formation so it cannot reinforce a flank elsewhere.
- If the player ignores it and no other ship completes it, the campaign effect is moderately outsized. Another ship completing it absorbs that effect (C-M4-07).
- Each battle has a strategic objective for both sides. The player's order is linked to their side's objective, more tightly in a later battle than in an early one (C-M2-11, C-M4-08).
- Some battles attach no squadron.
- When they do, composition follows the player's class:
  - **Frigate:** the frigate as C2, 2 to 4 escorts, and optionally 8 strike craft in 2 wings.
  - **Cruiser:** 1 to 2 cruisers, 2 to 4 frigates, and 16 strike craft in 4 wings.
  - **Battleship:** 1 flagship, 2 ships of the line, 8 supporting cruisers or frigates, and 16 to 32 strike craft. Available only when Squad Command is at its maximum.
- The commands are focus (one or more targets, to fire on or to capture), hold (the area around a position), protect (one or more targets), and follow (keep formation on the player's ship).
- Ships outside the player ship and the squadron still act on their own orders.
- Squadron control is the four commands above.

How large the outsized effect is, and the steps by which the strategic link tightens, are [OQ-07](#17-open-questions). What a ship of the line is, and whether a strike craft is its own ship, are [OQ-06](#17-open-questions). Whether "focus to capture" is what starts a boarding action is part of [OQ-31](#17-open-questions).

> **Proposal P-WING-01 (unapproved).** A wing is one instance with a craft count and one approximate facing. The frigate's two wings hold 4 craft each. The cruiser's four wings hold 4 each. A battleship wing count follows the 16 to 32 total. Individual craft are not simulated until OQ-06 says to.
>
> **Proposal P-SKL-01 (unapproved).** Stand-in for [OQ-30](#17-open-questions). Squad Command is an integer 0 to 3. Maximum is 3. The sample raises it by one when a battle ends with the squadron's order carried out. Frigate and cruiser squadrons have no skill gate in this stand-in, because none was stated.

## 11. Progression and meta

_Checklist: C-M3-01, C-M3-03, C-M3-04, C-M3-05, C-M3-06, C-M3-07, C-M5-01, C-M5-02, C-M5-03._

Decided:

- Class order is frigate, then cruiser, then battleship, with a real difference in the build ([P-FIT-01](#6-ships-and-components) until real numbers exist).
- Squad Command is an improvable skill on the captain. At its maximum the player can command a battleship squadron. Below that, they cannot (C-M3-07). The level cap and what raises the skill are [OQ-30](#17-open-questions).
- Captain and crew are persistent across the battles of a run, and a battle can change a visible value on each. Attributes besides Squad Command are [OQ-13](#17-open-questions).
- The progression subscreen shows class, captain, crew, Squad Command, and the latest change.
- A medal names an achievement and grants a small visible bonus.
- Veteran crew can appear by chance, and the progression screen shows the difference.
- Ironman: the escape pod fails. The next run starts from scratch (C-M5-03).

> **Proposal P-PRG-01 (unapproved).** Stand-in for the attributes [OQ-13](#17-open-questions) has not named. Squad Command is decided and is not part of this guess.
>
> - Captain: `name`, `citations`, `squad_command`. `citations` increments by one when a battle ends with the hard objective complete.
> - Crew: `experience`, `veteran`. `experience` increments by one when a battle ends with the player ship alive.
>
> The progression screen shows the old and new numbers.

> **Proposal P-VET-01 (unapproved).** Stand-in for [OQ-14](#17-open-questions). At run start, the crew becomes veteran with probability 1/10 and starts with `experience` 2 instead of 0. The progression screen shows the flag and the number. The probability is a sample constant.

> **Proposal P-MEDAL-01 (unapproved).** Stand-in for [OQ-15](#17-open-questions). One sample medal, `First blood`: destroy any high-value target. Bonus: one extra unassigned power point on the next battle only. The medal's name and the extra point are both on screen. Medals in the stand-in live inside the run. A failed ironman run discards them with the captain, the crew, and the ship. Cross-run medals stay in OQ-15.

Class-change timing stays the [P-FIT-01](#6-ships-and-components) stand-in until [OQ-12](#17-open-questions).

## Factions

_Checklist: C-M2-12, C-M3-09._

Decided:

- The factions are the Imperial Royal Navy and the Allied People's Naval Forces.
- The difference is mostly lore and cosmetics.
- Imperial ships are a little tougher. Alliance ships have slightly more gun.
- Each side has one or two special components. A player on the other side has to capture them to fit them (C-M3-09).

The names of those components are [OQ-32](#17-open-questions).

> **Proposal P-FCT-01 (unapproved).** Sample pieces so C-M3-09 can load. Imperial `belt_plate` adds toughness. Alliance `rapid_mount` adds gun. Capturing one adds it to the ship-management screen. The names are disposable.

## Boarding

_Checklist: C-M6-01, C-M6-02, C-M6-03._

Decided. This is not required for the first playable battle.

- A boarding action is a roguelike played inside the battle. A play runs about 5 to 10 minutes (C-M6-01).
- The boarders must take the bridge and engineering in time. Missing either one auto-destructs the ship (C-M6-02).
- The player uses a separate, simpler equipment system. The player's crew and the defenders use standard gear (C-M6-03).

How a boarding starts, what "in time" is in turns or seconds, the map, and the gear list are [OQ-31](#17-open-questions). Focus-to-capture may be the start; that link is not decided.

> **Proposal P-BRD-01 (unapproved).** Stand-in for OQ-31. Ordering focus-to-capture on a ship with every combat component already crippled starts the boarding. The clock is 8 battle-turns, inside the 5 to 10 minute play. The map is a line of rooms with the bridge at one end and engineering at the other. Standard gear is a sidearm and a cutter. The player's kit is those plus one chosen tool. None of this layout is approved.

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
> | Rotate weapon facing | `,` / `.` | Click a direction pip on the weapon row |
> | Rotate hull | `<` / `>` | Click a direction pip on the hull |
> | Accelerate / brake | `W` / `S` | Buttons on the right sidebar |
> | Toggle component | Shortcut shown on the row | Click the row |
> | Repair selected line or component | `R` | Repair button |
> | Move one power point | `[` / `]` on the selected subsystem | `+` / `−` on the subsystem |
> | Squadron command | `1`–`4` for focus, hold, protect, follow | Command buttons on the left sidebar |
> | End turn | Enter | End-turn button |
> | Ship screen / progression / back | `F1` / `F2` / Esc | Matching buttons |

## 13. Art

_Checklist: C-M0-07, C-M5-05._

Decided: sprite/SVG style. Two or three frames per entity. Idle and attack are the required pair. A third frame may be added within that cap. Placeholder icons may satisfy Milestone 0; C-M5-05 is the art bar.

> **Proposal P-ART-02 (unapproved).** Source art is SVG. A build step rasterizes it into a sprite atlas the SDL renderer samples. The third frame slot may stay empty. What a third frame depicts, if it is used, is [OQ-21](#17-open-questions). Stingers and chatter are the separate stretch P-ART-01 (C-P-08).

## 14. Content files

_Checklist: C-M2-02, C-M2-03, C-M4-02, C-M4-05, and any box that loads a battle._

The owner did not choose a file format. This section is entirely a proposal.

> **Proposal P-DAT-01 (unapproved).** Author content as TOML, because it stays readable in review. A Rust-native format may replace it later. Loading a pack with no code change is the stretch wish P-SHP-01 (C-P-03). The stand-in only requires the game to load its own data files.
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

_Checklist: C-M0-01, C-M0-06, C-M0-08, C-M0-09, C-M2-01, C-M5-06, C-M5-07, C-M5-08._

Decided constraints:

- A standalone process, SDL for portable 2D, native concurrency, heavily instanced battles, no lock held across a whole update.
- Binaries for Windows and Linux. No preinstalled language runtime.
- A turn of the 20,000-ship scene finishes in under a second. The low target, with no other workload, is 4 cores and 8 GB. The official minimum is 16 GB (C-M5-08).
- The same scene can run in a container capped at 4 cores and 8 GB (C-M0-09).
- Code follows the [coding guideline](#16-language-and-coding-guideline).

How much of the work moves to the GPU, and therefore how much VRAM is required, is [OQ-22](#17-open-questions).

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

## 16. Language and coding guideline

_Checklist: C-M0-08, C-M5-06, W-TEC-05._

Decided. The game is written in Rust.

This is the project coding guideline, in the owner's words: keep it legible and modular for someone who does not write much Rust, and avoid over-verbosity.

What that means in this codebase:

- Split modules by system: movement, action points, weapons, the power grid, damage, the interior, squadron orders, boarding, the war map, and rendering. A reader can open one of those and follow it.
- Prefer plain functions, concrete structs, and straight control flow. Reach for a trait, a generic, or a macro when it removes a real repetition, not as the default shape of a feature.
- Name things for the game (a power line, a wing, a facing), not for the pattern they implement.
- Keep the simulation as tables of ship data, as in [§15](#15-architecture), so the hot path stays a loop a new Rust reader can trace. The type system should make "a worker does not touch SDL" obvious. It should not make a component rule hard to read.
- Comments say why a rule exists. They do not restate the code, and they do not grow into a second spec.

SDL3 is the rendering target. If the SDL3 bindings block Milestone 0, start on the mature SDL2 crate and move once they do not. That fallback is a schedule note, not a second language.

Windows and Linux are the shipped platforms (W-TEC-03). The earlier comparison against C#, Go, Zig, and Odin is closed by this decision.

## 17. Open questions

Answered questions are listed under [Closed](#closed-questions). What remains is a gap. A stand-in next to a row is a way to keep building. It is not an answer.

Ordered here by what they block. The first playable battle is Milestone 1. Later rows block a later milestone. VRAM and the release-meta rows do not block that first battle.

| ID | Question | Blocks | Stand-in until answered |
| --- | --- | --- | --- |
| OQ-34 | Taking one power point off a live subsystem costs how many action points? Spare assignment is that cost minus one per point. | M1 | 2 to take a point off a live subsystem, so spare assignment costs 1 (P-POW-01). |
| OQ-35 | During a battle, is the interior a layer the player directs inside the turn, and what does a crew member spend to move or repair? | M1 | A second view. One ship action point moves one crew into an adjacent room, and another repairs there (P-INT-01). |
| OQ-36 | What is a room on that map, and how many crew walk it? | M1 | Rooms are fitted components and distribution nodes. A frigate has 4 crew (P-INT-01). |
| OQ-37 | What integrity does a component start with, and how do durability and over-engineering set the depth of the red zone after 0? | M1 | Integrity 3, red zone 1, or 2 if over-engineered (P-DMG-01). |
| OQ-38 | Does a dry-docked component (a hole where it was, or fused parts that must be replaced) end the run, or is the run-ending case only the ship-scale shredded chassis? | M1 | Only the shredded chassis ends the run. A dry-docked component waits for replacement. |
| OQ-39 | How far does evacuation push in-run progress back? | M1 | Back to the start of the current battle, ship and crew kept. |
| OQ-40 | Which abandonments are "without an in-game cause"? Does crippling every combat-capable component, which already abandons the ship, count as that run-ending case, or as a caused abandonment with a different result? | M1 | Causeless means the player leaves a ship that still has a working combat component. The all-components-crippled case stays an abandonment caused by damage and does not, by itself, end the run. |
| OQ-44 | Are the divertable, repairable power lines the plumbing that is fixed per chassis, or are they separate from those pipes? | M1 | Power lines stay divertable and repairable. Plumbing is a different, fixed set of pipes. |
| OQ-27 | Among ships that have enough action points, who acts first? | M2 | Player, then squadron, then the rest, each on the latest state (P-TURN-01). |
| OQ-41 | How near is "immediately proximate" for a ship that has an interior? | M2 | Adjacent hexes only. |
| OQ-42 | Besides repair, what do crew do in the interior? | After M1 repair works | Nothing else until named. |
| OQ-17 | Do ships far from the player simulate the power grid and component damage, or only the decided coarse facing and the action-point skip? | M2 | Far ships are a volley stat plus one facing (P-SIM-01). |
| OQ-06 | Is a ship of the line a battleship or another hull, and is a strike craft its own ship or only a count on a wing? | M2, M3 | A ship of the line is a battleship that is not the flagship. A wing is one instance with a craft count (P-WING-01). |
| OQ-08 | What does the player do to gather intel? | M2 | Scan a marked contact. Destroying it does not count (P-OUT-01). |
| OQ-09 | If the hard objective fails, do high-value targets and intel already earned still stand? | M2 | They stand, and the war continues (P-OUT-01). |
| OQ-10 | Besides the decided effect of an ignored order, what else is stored as the battle outcome, and do high-value targets and intel change later battles? | M2 readout, M4 consequences | Three visible lines. Later battles do not read them (P-OUT-01). |
| OQ-30 | What is Squad Command's maximum, what raises it, and is there a gate below a battleship squadron? | M3 | Levels 0–3, up one when the squadron's order is carried out. No gate below battleship (P-SKL-01). |
| OQ-12 | What triggers frigate to cruiser to battleship, and is the change a refit or a new hull? | M3 | Refit offered after battle 3 and battle 7 (P-FIT-01). |
| OQ-13 | Which captain and crew attributes exist besides Squad Command, and what changes them? | M3 | Citations and crew experience (P-PRG-01). |
| OQ-24 | What does the ship-management subscreen change that the battle sidebars do not? | M3 | The subscreen fits and unfits purpose-built components and leaves plumbing fixed. The battle screen toggles fitted components. Repair is the interior. |
| OQ-32 | What are the one or two special components each faction has, the ones the other side must capture? | M3 | Imperial `belt_plate`, Alliance `rapid_mount` (P-FCT-01). |
| OQ-07 | How large is the "moderately outsized" campaign effect of an ignored order, and by what steps does the link to the strategic objective tighten across the ten battles? | M4 | The ignored order shifts the next battle's outcome lines by one step, and the link is a sentence in battle 1 and a second fail condition in battle 10. |
| OQ-11 | Can the war end before ten battles? | M4 | All ten steps are played. |
| OQ-31 | How does a boarding start, what is "in time", and what are the map and the standard gear? | M6, not M1 | Focus-to-capture on a fully crippled ship. Eight battle-turns. A corridor, a sidearm, and a cutter (P-BRD-01). |
| OQ-45 | Ironman says the escape pod fails and the run starts over. Evacuation is a loss that does not end the run. Does ironman turn an evacuation into a run-ending loss? | M5, not M1 | Ironman ends the run on evacuation. A normal run takes the progress setback. |
| OQ-14 | When is the veteran chance rolled, and what does a veteran change? | M5 | Once at run start, 1 in 10, starting experience 2 (P-VET-01). |
| OQ-15 | Do medals persist across runs, and does ironman clear that store too? | M5 | Medals live inside the run. Ironman discards them with the run (P-MEDAL-01). |
| OQ-16 | Is there save and reload when ironman is off? | M5 | The run is in memory only. |
| OQ-20 | What may the player customize on the holotable, and does it persist past the session? | M5 | Four layer toggles, session only (P-UI-02). |
| OQ-21 | Is the optional third frame used, and what does it show? | M5 | The slot stays empty (P-ART-02). |
| OQ-22 | How much of a turn should run on the GPU, and what VRAM does that imply? Cores, RAM, and the one-second turn are already decided. | Release tuning, not M1 | Rendering stays on the CPU. VRAM is whatever the window needs. |

### Closed questions

Moved into the decided sections. The ids are kept so older notes still point somewhere.

| ID | Now decided as |
| --- | --- |
| OQ-01 | Momentum, coasting, acceleration as a full-turn action, max 3 hexes. A hull turn is 1 action point per facing. Braking costs the same as acceleration. |
| OQ-02 | Separate hull and weapon facings. Distant ships use a cheap approximation. |
| OQ-03 | The subsystem list, the generator and line grid, and unassigned power. Spare power costs one action point less per point than taking power off a live subsystem. The base cost remains OQ-34. |
| OQ-04 | Component tiers: crippled, critical (red zone after integrity 0), dry dock (replace the part). Refittable when they have a clear purpose. Plumbing fixed per chassis. Numbers remain OQ-37. |
| OQ-05 | Action points. Frigates have the most. Starting pools for tuning: 10, 8, and 6. |
| OQ-25 | Closed with OQ-05. Frigate 10, cruiser 8, battleship 6, as tuning starts. |
| OQ-26 | Hull turn is 1 action point per hex facing. Braking is the opposite force to acceleration and costs the same full turn. |
| OQ-28 | Closed with OQ-03. The discount is one action point per power point. |
| OQ-29 | Repair is a crew action inside the interior roguelike. |
| OQ-33 | Evacuation sets in-run progress back and does not end the run. Abandonment without an in-game cause ends the run. The reach of those two sentences remains OQ-39 and OQ-40. |
| OQ-18 | Imperial Royal Navy and Allied People's Naval Forces. Component names remain OQ-32. |
| OQ-19 | Windows and Linux. |
| OQ-23 | Rust, under the coding guideline in §16. |

## 18. Section index

| Section | Checklist |
| --- | --- |
| 1 Pillars | Shape of M1–M4 |
| 2 Glossary | Terms used by the boxes |
| 3 War and battles | C-M4-01–C-M4-08, C-M2-07, C-M2-11 |
| 4 Turn structure | C-M1-03, C-M1-04, C-M1-05, C-M1-07, C-M1-08, C-M1-10–C-M1-12, C-M0-06, C-M2-10 |
| 5 Battlefield and scale | C-M0-02–C-M0-04, C-M0-06, C-M0-07, C-M1-10, C-M2-01, C-M2-08, C-M2-09 |
| 6 Ships and components | C-M1-04, C-M1-06, C-M1-07, C-M1-15–C-M1-18, C-M1-20, C-M3-01, C-M3-02, C-M3-06, C-M3-10 |
| 7 Power | C-M1-05, C-M1-09, C-M1-13, C-M1-14 |
| Interior | C-M1-08, C-M1-14, C-M1-19, C-M2-13 |
| 8 Weapons | C-M1-03, C-M1-11, C-M2-09 |
| 9 Objectives and intel | C-M1-01, C-M1-02, C-M1-07, C-M2-02–C-M2-05, C-M2-07, C-M4-04, C-M4-06 |
| 10 Orders and squadron | C-M1-02, C-M2-06, C-M2-08, C-M2-11, C-M3-07, C-M3-08, C-M4-07, C-M4-08 |
| 11 Progression and meta | C-M3-01, C-M3-03–C-M3-07, C-M5-01–C-M5-03 |
| Factions | C-M2-12, C-M3-09 |
| Boarding | C-M6-01–C-M6-03 |
| 12 UI and controls | C-M0-02–C-M0-05, C-M1-08, C-M1-09, C-M3-02, C-M3-05, C-M5-04 |
| 13 Art | C-M0-07, C-M5-05 |
| 14 Content files | C-M2-02, C-M2-03, C-M4-02, C-M4-05 |
| 15 Architecture | C-M0-01, C-M0-06, C-M0-08, C-M0-09, C-M2-01, C-M5-06–C-M5-08 |
| 16 Language and coding guideline | C-M0-08, C-M5-06, W-TEC-05 |
| 17 Open questions | Gaps those boxes still have |

## 19. Proposal index

Unapproved, and still this planning pass's own ideas. Stretch wishes in the wishlist are tested by C-P-02, C-P-03, C-P-04, C-P-05, C-P-06, C-P-07, and C-P-08. The earlier squadron-doctrine stretch is retired: the four commands are decided.

| ID | Where | What it stands in for |
| --- | --- | --- |
| P-WAR-MAP | §3 | Map of ten steps, two choices, outcome lines stored |
| P-TURN-01 | §4 | Who acts first |
| P-HEX-01 | §5 | Axial hexes, directions 0–5 |
| P-SIM-01 | §5 | What far ships simulate besides facing and the action-point skip |
| P-SIM-02 | §5 | Snapshot buffer to the renderer |
| P-DMG-01 | §6 | Sample integrity and red-zone depth |
| P-FIT-01 | §6 | Sample slot and generator counts, sample refit timing |
| P-SYNERGY-01 | §6 | Sample component pair |
| P-POW-01 | §7 | Sample two-generator layout, and a sample base cost of 2 action points |
| P-INT-01 | Interior | Sample rooms, crew count, and move cost |
| P-WPN-01 | §8 | Facings as hex directions, sample "close" radius of 8 |
| P-OUT-01 | §9 | Three-line outcome, sample strike, hold, and intel |
| P-WING-01 | §10 | A wing is one instance with a craft count |
| P-SKL-01 | §10 | Squad Command levels 0–3 |
| P-PRG-01 | §11 | Citations and experience, beside the decided Squad Command skill |
| P-VET-01 | §11 | Veteran roll at run start |
| P-MEDAL-01 | §11 | Sample medal |
| P-FCT-01 | Factions | Sample capturable components |
| P-BRD-01 | Boarding | How a boarding starts, the clock, the map, the gear |
| P-UI-02 | §12 | Sidebar contents, subscreens, camera jumps, layer toggles |
| P-UI-03 | §12 | Default bindings |
| P-ART-02 | §13 | SVG source, sprite atlas at build |
| P-DAT-01 | §14 | TOML content files |
| P-ARC-01 | §15 | Content, simulation, presentation, command buffer |

Retired, because the owner answered them: the old hull-point destruction rule, the one-action-point sidebar repair (P-REP-01), the four-subsystem power budget that had to be fully assigned, the squadron of three with only focus and hold, the macOS platform guess, and the unapproved language recommendation.

