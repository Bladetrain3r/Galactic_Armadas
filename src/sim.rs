//! Ship simulation, off the SDL thread.
//!
//! The render thread sends [`SimCommand`] and receives [`Snapshot`] values.
//! A step mutates only the sim's own ship table. It does not lock that table,
//! and it does not call SDL. The published snapshot is immutable; the renderer
//! reads it and never writes a ship.

use std::sync::mpsc::{
    self, Receiver, RecvTimeoutError, Sender, SyncSender, TryRecvError, TrySendError,
};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::hex::{self, Hex};
use crate::ship::{Faction, Role, Ship, ShipView};

/// Low end of "tens of thousands" (TD-SCALE).
pub const SHIP_COUNT: usize = 20_000;

/// Disk the scale scene fills. Big enough to hold [`SHIP_COUNT`] ships.
pub const MAP_RADIUS: i32 = 82;

/// Where `End` puts the camera. Far enough that the player has to jump or pan.
pub const OBJECTIVE_HEX: Hex = Hex { q: 60, r: -24 };

/// Sim steps per second. The render thread is not tied to this rate.
const STEP_HZ: f32 = 30.0;

/// Ships with action points reconsider their drift about twice a second,
/// staggered so one step does not touch every ship.
const DECISION_PERIOD: u64 = 15;

/// Drift speed before the per-ship variation. Hexes per second.
const DRIFT_HEX_PER_SEC: f32 = 0.15;

/// Frigate starting pool (W-AP-03). The scale scene does not spend it yet.
const FRIGATE_AP: u8 = 10;

pub struct Snapshot {
    pub ships: Vec<ShipView>,
    pub player_index: usize,
    pub objective_index: usize,
    /// Wall time of the step that produced this picture, in milliseconds.
    pub step_ms: f32,
    pub tick: u64,
    /// How many ships were given a decision on that step.
    pub decisions: u32,
    /// Ships that coast and are never given a decision.
    pub idle_ships: u32,
}

pub enum SimCommand {
    Stop,
}

pub struct SimHandle {
    commands: Sender<SimCommand>,
    snapshots: Receiver<Arc<Snapshot>>,
    thread: JoinHandle<()>,
}

impl SimHandle {
    /// Latest picture, if the sim has published one since the last take.
    /// Never blocks: a slow step must not freeze the window.
    pub fn latest(&self) -> Option<Arc<Snapshot>> {
        let mut found = None;
        while let Ok(snapshot) = self.snapshots.try_recv() {
            found = Some(snapshot);
        }
        found
    }

    pub fn stop(self) -> Result<(), String> {
        let _ = self.commands.send(SimCommand::Stop);
        self.thread
            .join()
            .map_err(|_| "sim thread panicked before exit".to_string())
    }
}

pub fn start() -> Result<(SimHandle, Arc<Snapshot>), String> {
    let (cmd_tx, cmd_rx) = mpsc::channel();
    // One slot. If the renderer has not taken the previous picture, the sim
    // drops the new one and keeps stepping. The step itself never waits.
    let (snap_tx, snap_rx) = mpsc::sync_channel(1);
    let thread = thread::Builder::new()
        .name("sim".into())
        .spawn(move || sim_loop(cmd_rx, snap_tx))
        .map_err(|err| err.to_string())?;
    let first = snap_rx
        .recv_timeout(Duration::from_secs(5))
        .map_err(|err| match err {
            RecvTimeoutError::Timeout => "sim thread did not publish a snapshot".to_string(),
            RecvTimeoutError::Disconnected => "sim thread stopped during startup".to_string(),
        })?;
    Ok((
        SimHandle {
            commands: cmd_tx,
            snapshots: snap_rx,
            thread,
        },
        first,
    ))
}

fn sim_loop(commands: Receiver<SimCommand>, snapshots: SyncSender<Arc<Snapshot>>) {
    let mut world = World::new();
    publish(&snapshots, world.snapshot(0.0));
    let step_dt = 1.0 / STEP_HZ;
    let pace = Duration::from_secs_f32(step_dt);
    loop {
        match commands.try_recv() {
            Ok(SimCommand::Stop) | Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {}
        }
        let started = Instant::now();
        world.step(step_dt);
        // The copy into the snapshot is part of the step the renderer waits on.
        let mut picture = world.snapshot(0.0);
        picture.step_ms = started.elapsed().as_secs_f32() * 1000.0;
        if !publish(&snapshots, picture) {
            break;
        }
        let spent = started.elapsed();
        if let Some(rest) = pace.checked_sub(spent) {
            thread::sleep(rest);
        }
    }
}

/// Returns false when the renderer has gone away.
fn publish(snapshots: &SyncSender<Arc<Snapshot>>, snapshot: Snapshot) -> bool {
    match snapshots.try_send(Arc::new(snapshot)) {
        Ok(()) => true,
        Err(TrySendError::Full(_)) => true,
        Err(TrySendError::Disconnected(_)) => false,
    }
}

pub struct World {
    ships: Vec<Ship>,
    tick: u64,
    decisions_last_step: u32,
    idle_ships: u32,
}

impl World {
    pub fn new() -> Self {
        let mut ships = Vec::with_capacity(SHIP_COUNT);
        ships.push(make_ship(0, Hex::new(0, 0), Role::Player));
        ships.push(make_ship(1, OBJECTIVE_HEX, Role::Objective));
        for hex in hex::hexes_in_radius(MAP_RADIUS) {
            if ships.len() == SHIP_COUNT {
                break;
            }
            if hex == Hex::new(0, 0) || hex == OBJECTIVE_HEX {
                continue;
            }
            let id = ships.len() as u32;
            ships.push(make_ship(id, hex, Role::Fleet));
        }
        let idle_ships = ships.iter().filter(|ship| ship.action_points == 0).count() as u32;
        Self {
            ships,
            tick: 0,
            decisions_last_step: 0,
            idle_ships,
        }
    }

    /// Advance every ship by `dt` seconds.
    ///
    /// Momentum applies to everyone. A ship with no action points is not
    /// given a decision: `decide` is skipped, and the ship only coasts.
    pub fn step(&mut self, dt: f32) -> u32 {
        let tick = self.tick;
        let mut decisions = 0;
        for ship in &mut self.ships {
            coast(ship, dt);
            if ship.action_points == 0 {
                continue;
            }
            if tick % DECISION_PERIOD == u64::from(ship.id) % DECISION_PERIOD {
                decide(ship);
                decisions += 1;
            }
        }
        self.tick = tick + 1;
        self.decisions_last_step = decisions;
        decisions
    }

    fn snapshot(&self, step_ms: f32) -> Snapshot {
        Snapshot {
            ships: self.ships.iter().map(ShipView::from).collect(),
            player_index: 0,
            objective_index: 1,
            step_ms,
            tick: self.tick,
            decisions: self.decisions_last_step,
            idle_ships: self.idle_ships,
        }
    }
}

fn make_ship(id: u32, hex: Hex, role: Role) -> Ship {
    let facing = (id % 6) as u8;
    let (vel_q, vel_r) = velocity(facing, id);
    let faction = match role {
        Role::Player => Faction::Imperial,
        Role::Objective => Faction::Alliance,
        Role::Fleet => {
            if hex.q >= 0 {
                Faction::Alliance
            } else {
                Faction::Imperial
            }
        }
    };
    // One ship in eight has nothing to spend, so the scale step has to skip
    // them. The player ship keeps the frigate pool either way.
    let action_points = if role == Role::Player {
        FRIGATE_AP
    } else if id % 8 == 0 {
        0
    } else {
        FRIGATE_AP
    };
    Ship {
        id,
        q: hex.q as f32,
        r: hex.r as f32,
        vel_q,
        vel_r,
        facing,
        faction,
        action_points,
        role,
    }
}

fn velocity(facing: u8, id: u32) -> (f32, f32) {
    let step = hex::neighbor(Hex::new(0, 0), facing);
    let speed = DRIFT_HEX_PER_SEC * (0.75 + (id % 5) as f32 * 0.1);
    (step.q as f32 * speed, step.r as f32 * speed)
}

fn coast(ship: &mut Ship, dt: f32) {
    ship.q += ship.vel_q * dt;
    ship.r += ship.vel_r * dt;
    if hex::distance(ship.hex(), Hex::new(0, 0)) <= MAP_RADIUS {
        return;
    }
    // The development map has an edge. Bounce back onto it.
    ship.q -= ship.vel_q * dt;
    ship.r -= ship.vel_r * dt;
    ship.vel_q = -ship.vel_q;
    ship.vel_r = -ship.vel_r;
    ship.facing = (ship.facing + 3) % 6;
}

fn decide(ship: &mut Ship) {
    // Trivial intent: a small course change, then keep coasting on the new vector.
    let turn = match ship.id % 3 {
        0 => 1,
        1 => 5,
        _ => 0,
    };
    ship.facing = (ship.facing + turn) % 6;
    let (vel_q, vel_r) = velocity(ship.facing, ship.id);
    ship.vel_q = vel_q;
    ship.vel_r = vel_r;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_scale_scene_has_player_objective_and_twenty_thousand_ships() {
        let world = World::new();
        assert_eq!(world.ships.len(), SHIP_COUNT);
        assert_eq!(world.ships[0].role, Role::Player);
        assert_eq!(world.ships[0].faction, Faction::Imperial);
        assert_eq!(world.ships[0].action_points, FRIGATE_AP);
        assert_eq!(world.ships[1].role, Role::Objective);
        assert_eq!(world.ships[1].hex(), OBJECTIVE_HEX);
        assert!(world.idle_ships > 0);
        assert!(hex::distance(OBJECTIVE_HEX, Hex::new(0, 0)) > 8);
    }

    #[test]
    fn ships_without_action_points_are_not_given_a_decision() {
        let mut world = World::new();
        let tick = world.tick;
        let due = world
            .ships
            .iter()
            .filter(|ship| {
                ship.action_points > 0
                    && tick % DECISION_PERIOD == u64::from(ship.id) % DECISION_PERIOD
            })
            .count() as u32;
        let idle_due = world
            .ships
            .iter()
            .filter(|ship| {
                ship.action_points == 0
                    && tick % DECISION_PERIOD == u64::from(ship.id) % DECISION_PERIOD
            })
            .count();
        assert!(
            idle_due > 0,
            "the scene should include ships with nothing to spend"
        );
        let decisions = world.step(1.0 / STEP_HZ);
        assert_eq!(decisions, due);
    }

    #[test]
    fn thirty_steps_of_the_scale_scene_finish_within_a_second() {
        let mut world = World::new();
        let started = Instant::now();
        for _ in 0..30 {
            world.step(1.0 / STEP_HZ);
        }
        let elapsed = started.elapsed();
        eprintln!(
            "scale scene: 30 steps of {} ships took {:.2} ms",
            SHIP_COUNT,
            elapsed.as_secs_f32() * 1000.0
        );
        assert!(
            elapsed < Duration::from_secs(1),
            "30 steps took {elapsed:?}, which is over the one-second turn budget"
        );
    }
}
