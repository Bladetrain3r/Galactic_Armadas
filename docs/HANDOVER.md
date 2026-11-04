# Handover — 4 Nov 2026

Nightly note for the owner. [PR #2](https://github.com/Bladetrain3r/Galactic_Armadas/pull/2) stays open. Do not merge it until the next testing round.

## Current state

Milestone 0 is in the tree: a standalone binary, an SDL window, a hex holotable between two sidebars, and 20,000 drifting ships. The sim thread owns the ships and publishes a snapshot. The window thread draws and owns the camera. It does not write a ship.

What works:

- Pan with the arrow keys or by dragging the map. Zoom with `-` / `=`, or the wheel.
- Home / End, and the sidebar buttons, jump to the player ship and the marked contact.
- Icons are faction-coloured chevrons up close (Imperial crimson, Alliance blue), diamonds further out, and points when the armada fills the window. The player has a gold outline. The objective has an amber one.
- Ships with no action points coast. They are not given a decision. The scale scene starts 2,499 of them that way.

Measured:

| Where | What |
| --- | --- |
| Owner's machine | About 165 FPS at zoom 0.18, 20,000 ships, about 14,500 drawn, sim step 0.19 ms |
| Cloud VM, Xvfb, SDL software renderer, 4 cores | About 270–510 FPS depending on zoom. Whole map drawn as points stayed near 370 FPS. One sim step about 0.3 ms. Thirty steps of the ship table alone took 8.1 ms |

## Fixes on PR #2

- **Windows CMake 4.** The runner's CMake 4.4 rejected SDL2's old `cmake_minimum_required`. The build sets `CMAKE_POLICY_VERSION_MINIMUM=3.5`. Ubuntu's CMake 3.31 ignores that variable.
- **PipeWire.** Bundled SDL 2.0 does not compile against PipeWire 1.4 headers (`pw_node_enum_params` now takes `struct pw_node *`). `cmake/sdl-no-pipewire.cmake` turns that backend off. PulseAudio and ALSA stay available. If a previous configure already ran, `cargo clean -p sdl2-sys` once.
- **Arrow keys.** Left looks left and Up looks up. Mouse drag was already right and was left alone.

CI on `292f99a` was green: Ubuntu 24.04, Windows, and a Fedora 44 job whose PipeWire headers have the breaking signature.

## Build and run

SDL2 is compiled into the binary. No separate SDL install. Full package lists are in the [README](../README.md).

Linux: a C toolchain, CMake, and the X11 `-dev` packages, then `cargo run --release`.

Windows: MSVC C++ build tools and CMake on `PATH`, then `cargo run --release`.

```
cargo test --release
galactic-armadas --frames 300
galactic-armadas --frames 300 --zoom 0.18
```

## Known issues

- **C-M0-09** is still open. The 20,000-ship scene has not been run inside a container capped at 4 cores and 8 GB.
- Faction specials are still the placeholders `belt_plate` and `rapid_mount` ([OWN-01](TUNING.md#owner-only)).
- Rendering is the `sdl2` crate, not SDL3. The spec allows that while the SDL3 bindings are still migrating. Noted in [TUNING.md](TUNING.md).

## Next step

Milestone 1: one player ship, one hard objective, one turn of the combat puzzle. Start that after the owner tests PR #2. Leave the PR open until then.

## Open design ponderings

Not decided. Written up as Could-items in the [wishlist](WISHLIST.md) and as unscheduled boxes C-P-09 and C-P-10. They belong with the Milestone 2 battle, and they are not part of that milestone until the owner accepts them.

- **Battlefield conditions** (P-FLD-02). Terrain and battle-wide modifiers need a design. Starter ideas only: nebulae shorten sensor and targeting range; debris blocks or slows movement and gives cover; gravity wells bend momentum; ion storms disrupt power lines and shields; minefields; a battle-wide effect such as a solar flare, or comms jamming that affects squad commands.
- **Commanders and shoals** (P-ROL-01). AI commanders issue goals (what needs to happen, not how). Ships far from the player move as shoals and choose their own approach. Ships close to the player are decided one by one. That is a two-tier sim whose detail follows proximity, which matches the sim thread, the rule that a ship with no action points only coasts, and the requirement that a turn stay under a second.
