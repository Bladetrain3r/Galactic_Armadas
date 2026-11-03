# Battles of Galactic Armadas

You are the captain of one ship in a battle of tens of thousands. You take orders, fight a turn-based puzzle of weapon arcs, components, and power, and sometimes lead a squadron. Each battle has one hard objective. High-value targets and the intel you gather change the battle's outcome, and that outcome is what matters. The ship grows from frigate to cruiser to battleship, and the captain and crew grow with it. A run is a war map of ten battles, each a choice between thrust and hold.

This repository is the design. There is no game code yet.

## Design pipeline

Read in order. Each document cites the one before it. Entries marked as proposals or open questions were not part of the owner's design.

1. [Wishlist](docs/WISHLIST.md) — everything the game should be, grouped by area and tagged must / should / could, including stretch ideas.
2. [Checklist](docs/CHECKLIST.md) — testable milestones from a minimal playable battle to the full game.
3. [Spec](docs/SPEC.md) — systems, data, turn structure, UI, content, and architecture. The language is Rust. The coding guideline (legible, modular, for a reader who does not write much Rust) is in the spec.

## License

[MIT](LICENSE).
