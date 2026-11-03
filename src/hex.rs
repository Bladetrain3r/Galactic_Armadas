//! Axial hex coordinates for the battlefield.
//!
//! `(q, r)` is the axial pair from proposal P-HEX-01. The cube coordinate
//! `s` is always `-q - r`, so it is computed when a distance needs it and
//! not stored. Direction 0 is +q (east on the pointy-top map). The other
//! five directions run counter-clockwise from there.

/// One hex on the battlefield.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl Hex {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// Cube coordinate `s`. Axial storage keeps only `q` and `r`.
    pub fn s(self) -> i32 {
        -self.q - self.r
    }
}

/// Neighbor steps, indexed by facing 0–5.
pub const NEIGHBORS: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

pub const SQRT_3: f32 = 1.732_050_8;

/// The hex `direction` steps away from `hex`. Directions wrap, so 6 is 0.
pub fn neighbor(hex: Hex, direction: u8) -> Hex {
    let (dq, dr) = NEIGHBORS[(direction as usize) % 6];
    Hex::new(hex.q + dq, hex.r + dr)
}

/// Cube distance. This is the battlefield range used for weapons and intel later.
pub fn distance(a: Hex, b: Hex) -> i32 {
    let dq = (a.q - b.q).abs();
    let dr = (a.r - b.r).abs();
    let ds = (a.s() - b.s()).abs();
    (dq + dr + ds) / 2
}

/// Pointy-top hex center, in pixels. `size` is the center-to-corner length.
pub fn to_pixel(hex: Hex, size: f32) -> (f32, f32) {
    axial_to_pixel(hex.q as f32, hex.r as f32, size)
}

/// Same mapping as [`to_pixel`], for a ship that sits between hex centers.
pub fn axial_to_pixel(q: f32, r: f32, size: f32) -> (f32, f32) {
    let x = size * (SQRT_3 * q + SQRT_3 * 0.5 * r);
    let y = size * (1.5 * r);
    (x, y)
}

/// Inverse of [`axial_to_pixel`]. The result is fractional; round it to land on a hex.
pub fn pixel_to_axial(x: f32, y: f32, size: f32) -> (f32, f32) {
    let q = (SQRT_3 / 3.0 * x - y / 3.0) / size;
    let r = (2.0 / 3.0 * y) / size;
    (q, r)
}

/// Nearest hex to a fractional axial position.
pub fn round_axial(q: f32, r: f32) -> Hex {
    let s = -q - r;
    let mut rq = q.round();
    let mut rr = r.round();
    let rs = s.round();
    let dq = (rq - q).abs();
    let dr = (rr - r).abs();
    let ds = (rs - s).abs();
    // Rounding each axis can break q + r + s = 0. Repair the component
    // that moved the furthest, which is the usual cube-round rule.
    if dq > dr && dq > ds {
        rq = -rr - rs;
    } else if dr > ds {
        rr = -rq - rs;
    }
    Hex::new(rq as i32, rr as i32)
}

pub fn pixel_to_hex(x: f32, y: f32, size: f32) -> Hex {
    let (q, r) = pixel_to_axial(x, y, size);
    round_axial(q, r)
}

/// Every hex whose cube distance from the origin is at most `radius`.
pub fn hexes_in_radius(radius: i32) -> impl Iterator<Item = Hex> {
    let q_min = -radius;
    let q_max = radius;
    (q_min..=q_max).flat_map(move |q| {
        let r_min = (-radius).max(-q - radius);
        let r_max = radius.min(-q + radius);
        (r_min..=r_max).map(move |r| Hex::new(q, r))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_coordinate_sums_to_zero() {
        for q in -5..=5 {
            for r in -5..=5 {
                let hex = Hex::new(q, r);
                assert_eq!(hex.q + hex.r + hex.s(), 0);
            }
        }
    }

    #[test]
    fn neighbors_are_one_step_and_distinct() {
        let origin = Hex::new(0, 0);
        let mut seen = Vec::new();
        for direction in 0..6 {
            let next = neighbor(origin, direction);
            assert_eq!(distance(origin, next), 1, "direction {direction}");
            assert!(
                !seen.contains(&next),
                "direction {direction} repeated {next:?}"
            );
            seen.push(next);
        }
        assert_eq!(neighbor(origin, 6), neighbor(origin, 0));
        assert_eq!(neighbor(origin, 0), Hex::new(1, 0));
    }

    #[test]
    fn distance_is_symmetric_and_matches_known_pairs() {
        let origin = Hex::new(0, 0);
        assert_eq!(distance(origin, origin), 0);
        let far = Hex::new(3, -2);
        // Cube deltas: dq 3, dr 2, ds 1. Distance is half the sum.
        assert_eq!(distance(origin, far), 3);
        assert_eq!(distance(far, origin), 3);
        let other = Hex::new(-4, 5);
        assert_eq!(distance(origin, other), distance(other, origin));
        assert_eq!(distance(far, other), distance(other, far));
    }

    #[test]
    fn pixel_roundtrip_lands_on_the_same_hex() {
        let size = 22.0;
        for hex in hexes_in_radius(12) {
            let (x, y) = to_pixel(hex, size);
            let back = pixel_to_hex(x, y, size);
            assert_eq!(back, hex, "{hex:?} -> ({x}, {y}) -> {back:?}");
        }
    }

    #[test]
    fn round_axial_picks_the_hex_a_point_sits_in() {
        // A little off the origin, still closer to (0, 0) than to a neighbor.
        assert_eq!(round_axial(0.1, -0.1), Hex::new(0, 0));
        // Just past the boundary toward +q.
        assert_eq!(round_axial(0.9, 0.05), Hex::new(1, 0));
    }

    #[test]
    fn radius_count_matches_the_hex_formula() {
        // 3 * r * (r + 1) + 1 hexes in a disk of radius r.
        for radius in [0, 1, 4, 82] {
            let count = hexes_in_radius(radius).count();
            let expected = 3 * radius * (radius + 1) + 1;
            assert_eq!(count, expected as usize);
            for hex in hexes_in_radius(radius) {
                assert!(distance(hex, Hex::new(0, 0)) <= radius);
            }
        }
    }
}
