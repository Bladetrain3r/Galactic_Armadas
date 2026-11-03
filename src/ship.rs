//! Ship fields the scale scene actually stores.
//!
//! A later battle adds components, power, and weapon facings. This pass
//! keeps the columns the renderer and the drift sim share: identity, faction,
//! position, velocity, one facing, and action points.

use crate::hex::{self, Hex};

/// Placeholder heraldry until OWN-01 names the real components.
/// Colours are tuning defaults TD-M0-COLOR.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Faction {
    /// Imperial Royal Navy. A little tougher, in the fiction. Crimson here.
    Imperial,
    /// Allied People's Naval Forces. Slightly more gun, in the fiction. Blue here.
    Alliance,
}

impl Faction {
    pub fn rgb(self) -> (u8, u8, u8) {
        match self {
            Faction::Imperial => (196, 64, 72),
            Faction::Alliance => (64, 140, 210),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Fleet,
    /// The ship `Home` recenters on.
    Player,
    /// The contact `End` recenters on. Stand-in for the hard objective marker.
    Objective,
}

/// One instanced ship. The sim thread owns these.
#[derive(Clone, Copy, Debug)]
pub struct Ship {
    pub id: u32,
    pub q: f32,
    pub r: f32,
    pub vel_q: f32,
    pub vel_r: f32,
    pub facing: u8,
    pub faction: Faction,
    pub action_points: u8,
    pub role: Role,
}

impl Ship {
    pub fn hex(self) -> Hex {
        hex::round_axial(self.q, self.r)
    }
}

/// What the renderer is allowed to see. The sim publishes this and then
/// keeps working on its own ships. The render thread never writes it.
#[derive(Clone, Copy, Debug)]
pub struct ShipView {
    pub q: f32,
    pub r: f32,
    pub facing: u8,
    pub faction: Faction,
    pub role: Role,
}

impl From<&Ship> for ShipView {
    fn from(ship: &Ship) -> Self {
        Self {
            q: ship.q,
            r: ship.r,
            facing: ship.facing,
            faction: ship.faction,
            role: ship.role,
        }
    }
}
