//! Library half of the scale scene.
//!
//! The window binary in `main.rs` calls [`app::run`]. Everything else is a
//! module the sim or the renderer can open on its own: hex math, the camera,
//! the ship table, the sim thread, and drawing.

pub mod app;
pub mod camera;
pub mod font;
pub mod hex;
pub mod render;
pub mod ship;
pub mod sim;
