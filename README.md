# Battles of Galactic Armadas

You are the captain of one ship in a battle of tens of thousands. You take orders, fight a turn-based puzzle of weapon arcs, components, and power, and sometimes lead a squadron. Each battle has one hard objective. High-value targets and the intel you gather change the battle's outcome, and that outcome is what matters. The ship grows from frigate to cruiser to battleship, and the captain and crew grow with it. A run is a war map of ten battles, each a choice between thrust and hold.

The design documents below are the game. The code so far is the Milestone 0 scale scene: a window, a hex holotable, and 20,000 drifting ships.

## Scale scene

`galactic-armadas` opens a window on the SDL thread and runs the ship table on a second thread. The two sides talk by a snapshot channel. The sim never calls SDL, and the window never writes a ship.

The camera starts moderately zoomed in on the player ship (Imperial, gold outline, at the origin). `End` or the objective button jumps to the marked contact (Alliance, amber outline). Zoomed out, ships become diamonds and then points, and the whole radius-82 map fits in the window.

```
cargo run --release
```

| Action | Keyboard | Mouse |
| --- | --- | --- |
| Pan | Arrow keys | Drag the map |
| Zoom | `-` / `=` | Wheel |
| Own ship | Home | Home button, left sidebar |
| Objective | End | Objective button, left sidebar |
| Quit | Esc | Close the window |

```
cargo test --release
galactic-armadas --frames 300
galactic-armadas --frames 300 --zoom 0.06
```

`--frames` exits after N presented frames and prints average FPS and sim step time. `--zoom` overrides the opening zoom (`1` is the default, `0.06` fits the map). `--look objective` starts on the marked contact. `--screenshot file.bmp` writes the last frame.

Measured on a 4-core VM under Xvfb, using SDL's software renderer because no OpenGL driver was available: about 270–530 FPS with all 20,000 ships in the scene, and about 0.3 ms for one sim step including the snapshot copy. Thirty steps of the ship table alone took 8.1 ms. The sim thread does not use the GPU (TD-22).

## Build

SDL2 is compiled from source and linked into the binary (`bundled` and `static-link` on the `sdl2` crate). You do not install SDL. You do need a C toolchain, CMake, and the window-system headers SDL's video backend compiles against.

Install Rust from [rustup](https://rustup.rs). Then, from this directory:

### Linux

```
sudo apt install build-essential cmake pkg-config \
  libx11-dev libxext-dev libxrandr-dev libxi-dev libxcursor-dev \
  libxfixes-dev libxrender-dev libxss-dev libxxf86vm-dev libxinerama-dev
cargo run --release
```

The X11 packages are headers and the usual client libraries. A desktop already has the shared libraries; the `-dev` packages are for the build. The finished binary still loads those system X11 libraries at run time. It does not need `libsdl2` installed.

Recent PipeWire headers fail to compile the bundled SDL 2.0 audio backend (`pw_node_enum_params` now takes `struct pw_node *`). The build turns that backend off. PulseAudio and ALSA are unchanged. If a previous attempt already configured SDL, run `cargo clean -p sdl2-sys` once so CMake picks up the change.

### Windows

Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the Desktop development with C++ workload, and install [CMake](https://cmake.org/download/) (the rustup defaults find MSVC; CMake has to be on `PATH`). Then, in that environment:

```
cargo run --release
```

CMake compiles SDL into the binary. The binary uses the normal Win32 libraries. CMake 4 no longer accepts SDL2's old `cmake_minimum_required`; the repo sets `CMAKE_POLICY_VERSION_MINIMUM=3.5` for the build so current CMake still configures it.

Rendering uses the `sdl2` crate rather than SDL3. The spec allows that fallback while the SDL3 bindings are still migrating. The note is in [docs/TUNING.md](docs/TUNING.md).

## Design pipeline

Read the pipeline in order. Each document cites the one before it. Tuning defaults are not owner decisions. Proposals are engineering choices.

1. [Wishlist](docs/WISHLIST.md) — everything the game should be, grouped by area and tagged must / should / could, including stretch ideas.
2. [Checklist](docs/CHECKLIST.md) — testable milestones from a minimal playable battle to the full game.
3. [Spec](docs/SPEC.md) — systems, data, turn structure, UI, content, and architecture. The language is Rust. The coding guideline (legible, modular, for a reader who does not write much Rust) is in the spec.

[Tuning defaults](docs/TUNING.md) — every unset number and rule, in one table, with the one lore question that is still open.

[Handover](docs/HANDOVER.md) — where the build stands tonight. Not a design stage.

## License

[MIT](LICENSE).
