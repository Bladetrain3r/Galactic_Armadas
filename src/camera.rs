//! The holotable camera. It is presentation state: the sim never reads it.
//!
//! World coordinates are pixels at zoom 1, from [`crate::hex::axial_to_pixel`]
//! with [`BASE_HEX_SIZE`]. Zoom is a multiplier on that. The opening zoom is
//! the "moderately zoomed in" view from the spec; the low end fits the map.

use crate::hex::{self, Hex};

/// Center-to-corner length of a hex, in world pixels, at zoom 1.
pub const BASE_HEX_SIZE: f32 = 22.0;

/// Zoom when the window opens. Many ships are on screen; the whole armada is not.
pub const OPENING_ZOOM: f32 = 1.0;

/// Low enough that the radius-82 map fits in a modest window.
pub const MIN_ZOOM: f32 = 0.06;

pub const MAX_ZOOM: f32 = 2.5;

/// Arrow-key pan, in screen pixels per second. Screen space keeps the feel
/// similar at every zoom.
pub const PAN_PX_PER_SEC: f32 = 900.0;

/// Mouse-wheel and `-` / `=` step.
pub const ZOOM_STEP: f32 = 1.12;

pub struct Camera {
    pub center_x: f32,
    pub center_y: f32,
    pub zoom: f32,
}

impl Camera {
    /// Look at `hex` at the opening zoom.
    pub fn opening_at(hex: Hex) -> Self {
        let (center_x, center_y) = hex::to_pixel(hex, BASE_HEX_SIZE);
        Self {
            center_x,
            center_y,
            zoom: OPENING_ZOOM,
        }
    }

    /// On-screen center-to-corner length of a hex.
    pub fn hex_px(&self) -> f32 {
        BASE_HEX_SIZE * self.zoom
    }

    pub fn world_to_screen(&self, wx: f32, wy: f32, view_w: f32, view_h: f32) -> (f32, f32) {
        let sx = (wx - self.center_x) * self.zoom + view_w * 0.5;
        let sy = (wy - self.center_y) * self.zoom + view_h * 0.5;
        (sx, sy)
    }

    pub fn screen_to_world(&self, sx: f32, sy: f32, view_w: f32, view_h: f32) -> (f32, f32) {
        let wx = (sx - view_w * 0.5) / self.zoom + self.center_x;
        let wy = (sy - view_h * 0.5) / self.zoom + self.center_y;
        (wx, wy)
    }

    /// Drag the map with the pointer. `dx` and `dy` are screen pixels.
    pub fn pan_screen(&mut self, dx: f32, dy: f32) {
        self.center_x -= dx / self.zoom;
        self.center_y -= dy / self.zoom;
    }

    /// Zoom while keeping the world point under `(sx, sy)` fixed on screen.
    pub fn zoom_at(&mut self, factor: f32, sx: f32, sy: f32, view_w: f32, view_h: f32) {
        let before = self.screen_to_world(sx, sy, view_w, view_h);
        self.zoom = (self.zoom * factor).clamp(MIN_ZOOM, MAX_ZOOM);
        let after = self.screen_to_world(sx, sy, view_w, view_h);
        self.center_x += before.0 - after.0;
        self.center_y += before.1 - after.1;
    }

    pub fn look_at_hex(&mut self, hex: Hex) {
        let (x, y) = hex::to_pixel(hex, BASE_HEX_SIZE);
        self.center_x = x;
        self.center_y = y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_looked_at_hex_sits_at_the_view_center() {
        let hex = Hex::new(4, -2);
        let camera = Camera::opening_at(hex);
        let (wx, wy) = hex::to_pixel(hex, BASE_HEX_SIZE);
        let (sx, sy) = camera.world_to_screen(wx, wy, 800.0, 600.0);
        assert!((sx - 400.0).abs() < 0.01);
        assert!((sy - 300.0).abs() < 0.01);
    }

    #[test]
    fn zoom_keeps_the_cursor_world_point_still() {
        let mut camera = Camera::opening_at(Hex::new(0, 0));
        let (view_w, view_h) = (900.0, 700.0);
        let cursor = (220.0, 480.0);
        let before = camera.screen_to_world(cursor.0, cursor.1, view_w, view_h);
        camera.zoom_at(ZOOM_STEP, cursor.0, cursor.1, view_w, view_h);
        let (sx, sy) = camera.world_to_screen(before.0, before.1, view_w, view_h);
        assert!((sx - cursor.0).abs() < 0.05, "sx {sx}");
        assert!((sy - cursor.1).abs() < 0.05, "sy {sy}");
    }

    #[test]
    fn pan_moves_the_center_opposite_the_drag() {
        let mut camera = Camera::opening_at(Hex::new(0, 0));
        let start_x = camera.center_x;
        camera.pan_screen(100.0, 0.0);
        assert!(camera.center_x < start_x);
    }
}
