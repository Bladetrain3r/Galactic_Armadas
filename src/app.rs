//! Window, input, and the SDL thread.
//!
//! Presentation owns the window and the camera. Ship state lives on the sim
//! thread. Input becomes camera motion. Drawing reads the latest snapshot
//! and never writes a ship.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::time::Instant;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mouse::MouseButton;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowBuilder};
use sdl2::VideoSubsystem;

use crate::camera::{arrow_pan_direction, Camera, MAX_ZOOM, MIN_ZOOM, PAN_PX_PER_SEC, ZOOM_STEP};
use crate::hex::{self, Hex};
use crate::render::{self, FrameInfo, Hud, Layout};
use crate::sim::{self, Snapshot};

struct Config {
    frames: Option<u32>,
    zoom: Option<f32>,
    screenshot: Option<PathBuf>,
    /// When true, the camera opens on the objective marker instead of the player.
    look_at_objective: bool,
}

pub fn run() -> Result<(), String> {
    let config = Config::from_args()?;
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let mut canvas = open_canvas(&video)?;
    eprintln!("renderer: {}", canvas.info().name);

    let (sim_handle, first) = sim::start()?;
    let mut snapshot = first;
    let mut camera = Camera::opening_at(ship_hex(&snapshot, snapshot.player_index));
    if let Some(zoom) = config.zoom {
        camera.zoom = zoom.clamp(MIN_ZOOM, MAX_ZOOM);
    }
    if config.look_at_objective {
        camera.look_at_hex(ship_hex(&snapshot, snapshot.objective_index));
    }

    let mut events = sdl.event_pump()?;
    let mut vertices = Vec::<sdl2::render::Vertex>::with_capacity(8_192);
    let mut dragging = false;
    let mut mouse = (0i32, 0i32);
    let mut hud = empty_hud();

    let mut last = Instant::now();
    let started = Instant::now();
    let mut fps_window = Instant::now();
    let mut fps_frames = 0u32;
    let mut fps = 0.0f32;
    let mut frame_ms = 0.0f32;
    let mut presented = 0u32;
    let mut sim_ms_sum = 0.0f64;
    let mut max_sim_ms = 0.0f32;
    let mut sim_samples = 0u32;
    // The first snapshot is the picture from before any step, so its 0 ms
    // timing is not a measurement.
    let mut last_sim_tick = snapshot.tick;
    let mut quit = false;
    let mut failure: Option<String> = None;

    while !quit {
        let now = Instant::now();
        let mut dt = now.duration_since(last).as_secs_f32();
        last = now;
        // A stalled frame should not fling the camera across the map.
        if dt > 0.05 {
            dt = 0.05;
        }

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => quit = true,
                Event::Window {
                    win_event: WindowEvent::Close,
                    ..
                } => quit = true,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => quit = true,
                    Keycode::Home => camera.look_at_hex(ship_hex(&snapshot, snapshot.player_index)),
                    Keycode::End => {
                        camera.look_at_hex(ship_hex(&snapshot, snapshot.objective_index))
                    }
                    Keycode::Minus | Keycode::KpMinus => {
                        zoom_view(&mut camera, &canvas, 1.0 / ZOOM_STEP, None);
                    }
                    Keycode::Equals | Keycode::KpPlus => {
                        zoom_view(&mut camera, &canvas, ZOOM_STEP, None);
                    }
                    _ => {}
                },
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    let (px, py) = to_drawable(&canvas, x, y);
                    mouse = (px, py);
                    if hud.own_ship.contains_point(Point::new(px, py)) {
                        camera.look_at_hex(ship_hex(&snapshot, snapshot.player_index));
                    } else if hud.objective.contains_point(Point::new(px, py)) {
                        camera.look_at_hex(ship_hex(&snapshot, snapshot.objective_index));
                    } else if in_view(&canvas, px, py) {
                        dragging = true;
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => dragging = false,
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    let (px, py) = to_drawable(&canvas, x, y);
                    mouse = (px, py);
                    if dragging {
                        let (dx, dy) = to_drawable_delta(&canvas, xrel, yrel);
                        camera.pan_screen(dx, dy);
                    }
                }
                Event::MouseWheel { y, .. } => {
                    if y != 0 {
                        let factor = if y > 0 { ZOOM_STEP } else { 1.0 / ZOOM_STEP };
                        let cursor = if in_view(&canvas, mouse.0, mouse.1) {
                            Some(mouse)
                        } else {
                            None
                        };
                        zoom_view(&mut camera, &canvas, factor, cursor);
                    }
                }
                _ => {}
            }
        }
        if quit {
            break;
        }

        pan_from_keys(&events.keyboard_state(), &mut camera, dt);
        if let Some(newer) = sim_handle.latest() {
            snapshot = newer;
        }

        let frame_started = Instant::now();
        match render::draw(
            &mut canvas,
            FrameInfo {
                camera: &camera,
                snapshot: &snapshot,
                fps,
                frame_ms,
            },
            &mut vertices,
        ) {
            Ok(next) => hud = next,
            Err(err) => {
                failure = Some(err);
                break;
            }
        }

        presented += 1;
        let save_now = config.screenshot.as_ref().is_some()
            && config.frames.is_some_and(|limit| presented >= limit);
        if save_now {
            if let Some(path) = &config.screenshot {
                match save_screenshot(&canvas, path) {
                    Ok(()) => eprintln!("wrote {}", path.display()),
                    Err(err) => {
                        failure = Some(err);
                        break;
                    }
                }
            }
        }
        canvas.present();

        frame_ms = frame_started.elapsed().as_secs_f32() * 1000.0;
        fps_frames += 1;
        // Count each sim step once. The renderer may draw the same snapshot
        // for several frames while the next step is still running.
        if snapshot.tick != last_sim_tick {
            last_sim_tick = snapshot.tick;
            sim_ms_sum += f64::from(snapshot.step_ms);
            sim_samples += 1;
            if snapshot.step_ms > max_sim_ms {
                max_sim_ms = snapshot.step_ms;
            }
        }
        let window_secs = fps_window.elapsed().as_secs_f32();
        // A handful of frames is enough for the overlay. The one-second
        // print below still uses a full window.
        if fps_frames >= 8 {
            fps = fps_frames as f32 / window_secs.max(0.000_1);
        }
        if window_secs >= 1.0 {
            fps = fps_frames as f32 / window_secs;
            println!(
                "fps {fps:.1}  frame {frame_ms:.2} ms  sim {:.3} ms  drawn {}  lod {}  ships {}",
                snapshot.step_ms,
                hud.drawn,
                hud.lod,
                snapshot.ships.len()
            );
            fps_frames = 0;
            fps_window = Instant::now();
        }

        if config.frames.is_some_and(|limit| presented >= limit) {
            break;
        }
    }

    let total = started.elapsed().as_secs_f32().max(0.000_1);
    let avg_fps = presented as f32 / total;
    let avg_sim = if sim_samples == 0 {
        0.0
    } else {
        sim_ms_sum / f64::from(sim_samples)
    };
    println!(
        "summary frames {presented}  avg_fps {avg_fps:.1}  avg_sim_ms {avg_sim:.3}  max_sim_ms {max_sim_ms:.3}  renderer {}",
        canvas.info().name
    );

    // Join the sim thread on every exit, including a draw or screenshot error.
    let stopped = sim_handle.stop();
    match (failure, stopped) {
        (Some(err), _) => Err(err),
        (None, stopped) => stopped,
    }
}

fn empty_hud() -> Hud {
    Hud {
        own_ship: sdl2::rect::Rect::new(0, 0, 0, 0),
        objective: sdl2::rect::Rect::new(0, 0, 0, 0),
        drawn: 0,
        lod: "ICON",
    }
}

fn ship_hex(snapshot: &Snapshot, index: usize) -> Hex {
    let ship = &snapshot.ships[index];
    hex::round_axial(ship.q, ship.r)
}

fn pan_from_keys(keys: &sdl2::keyboard::KeyboardState<'_>, camera: &mut Camera, dt: f32) {
    let (x, y) = arrow_pan_direction(
        keys.is_scancode_pressed(Scancode::Left),
        keys.is_scancode_pressed(Scancode::Right),
        keys.is_scancode_pressed(Scancode::Up),
        keys.is_scancode_pressed(Scancode::Down),
    );
    if x == 0.0 && y == 0.0 {
        return;
    }
    let len = (x * x + y * y).sqrt();
    camera.pan_screen(x / len * PAN_PX_PER_SEC * dt, y / len * PAN_PX_PER_SEC * dt);
}

fn zoom_view(
    camera: &mut Camera,
    canvas: &Canvas<Window>,
    factor: f32,
    cursor: Option<(i32, i32)>,
) {
    let (width, height) = canvas.output_size().unwrap_or((1, 1));
    let layout = Layout::from_window(width, height);
    let (sx, sy) = cursor.unwrap_or((layout.view_x + layout.view_w / 2, layout.view_h / 2));
    camera.zoom_at(
        factor,
        (sx - layout.view_x) as f32,
        sy as f32,
        layout.view_w as f32,
        layout.view_h as f32,
    );
}

fn in_view(canvas: &Canvas<Window>, x: i32, y: i32) -> bool {
    let (width, height) = canvas.output_size().unwrap_or((1, 1));
    let layout = Layout::from_window(width, height);
    x >= layout.view_x && x < layout.view_x + layout.view_w && y >= 0 && y < layout.view_h
}

fn to_drawable(canvas: &Canvas<Window>, x: i32, y: i32) -> (i32, i32) {
    let (sx, sy) = drawable_scale(canvas);
    ((x as f32 * sx) as i32, (y as f32 * sy) as i32)
}

fn to_drawable_delta(canvas: &Canvas<Window>, xrel: i32, yrel: i32) -> (f32, f32) {
    let (sx, sy) = drawable_scale(canvas);
    (xrel as f32 * sx, yrel as f32 * sy)
}

fn drawable_scale(canvas: &Canvas<Window>) -> (f32, f32) {
    let (dw, dh) = canvas.output_size().unwrap_or((1, 1));
    let (ww, wh) = canvas.window().size();
    (dw as f32 / ww.max(1) as f32, dh as f32 / wh.max(1) as f32)
}

fn open_canvas(video: &VideoSubsystem) -> Result<Canvas<Window>, String> {
    match make_window(video)?
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
    {
        Ok(canvas) => Ok(canvas),
        Err(err) => {
            eprintln!("accelerated renderer unavailable ({err}); using software");
            match make_window(video)?
                .into_canvas()
                .software()
                .present_vsync()
                .build()
            {
                Ok(canvas) => Ok(canvas),
                Err(err) => {
                    eprintln!("software vsync unavailable ({err}); using software without vsync");
                    make_window(video)?
                        .into_canvas()
                        .software()
                        .build()
                        .map_err(|err| err.to_string())
                }
            }
        }
    }
}

fn make_window(video: &VideoSubsystem) -> Result<Window, String> {
    WindowBuilder::new(video, "Battles of Galactic Armadas", 1280, 720)
        .position_centered()
        .resizable()
        .build()
        .map_err(|err| err.to_string())
}

fn save_screenshot(canvas: &Canvas<Window>, path: &PathBuf) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    let pixels = canvas.read_pixels(None, PixelFormatEnum::RGB24)?;
    write_bmp(path, width, height, &pixels)
}

fn write_bmp(path: &PathBuf, width: u32, height: u32, rgb: &[u8]) -> Result<(), String> {
    let width = width as usize;
    let height = height as usize;
    let src_stride = width * 3;
    if rgb.len() < src_stride * height {
        return Err(format!(
            "renderer returned {} bytes, expected at least {}",
            rgb.len(),
            src_stride * height
        ));
    }
    let row_stride = (src_stride + 3) & !3;
    let pixel_bytes = row_stride * height;
    let file_size = 54 + pixel_bytes as u32;
    let mut file = File::create(path).map_err(|err| err.to_string())?;
    let mut header = [0u8; 54];
    header[0] = b'B';
    header[1] = b'M';
    header[2..6].copy_from_slice(&file_size.to_le_bytes());
    header[10..14].copy_from_slice(&54u32.to_le_bytes());
    header[14..18].copy_from_slice(&40u32.to_le_bytes());
    header[18..22].copy_from_slice(&(width as u32).to_le_bytes());
    header[22..26].copy_from_slice(&(height as u32).to_le_bytes());
    header[26..28].copy_from_slice(&1u16.to_le_bytes());
    header[28..30].copy_from_slice(&24u16.to_le_bytes());
    file.write_all(&header).map_err(|err| err.to_string())?;
    let mut row = vec![0u8; row_stride];
    for y in (0..height).rev() {
        let src = &rgb[y * src_stride..(y + 1) * src_stride];
        for x in 0..width {
            // BMP stores blue, green, red.
            row[x * 3] = src[x * 3 + 2];
            row[x * 3 + 1] = src[x * 3 + 1];
            row[x * 3 + 2] = src[x * 3];
        }
        file.write_all(&row).map_err(|err| err.to_string())?;
    }
    Ok(())
}

impl Config {
    fn from_args() -> Result<Self, String> {
        let mut frames = None;
        let mut zoom = None;
        let mut screenshot = None;
        let mut look_at_objective = false;
        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--frames" => frames = Some(parse_next(&mut args, "--frames")?),
                "--zoom" => {
                    let value: f32 = parse_next(&mut args, "--zoom")?;
                    if value <= 0.0 {
                        return Err("--zoom must be greater than 0".into());
                    }
                    zoom = Some(value);
                }
                "--screenshot" => {
                    screenshot = Some(PathBuf::from(next_arg(&mut args, "--screenshot")?))
                }
                "--look" => {
                    let which = next_arg(&mut args, "--look")?;
                    look_at_objective = match which.as_str() {
                        "player" => false,
                        "objective" => true,
                        other => {
                            return Err(format!("--look must be player or objective, not {other}"));
                        }
                    };
                }
                "--help" | "-h" => {
                    println!(
                        "\
Battles of Galactic Armadas — scale scene

Usage: galactic-armadas [--frames N] [--zoom Z] [--look TARGET] [--screenshot FILE]

  --frames N         Exit after N presented frames.
  --zoom Z           Starting zoom. 1 is the opening view.
  --look TARGET      player (default) or objective.
  --screenshot FILE  Write a BMP of the last frame. Exits after 30 frames
                     unless --frames is set.
"
                    );
                    process::exit(0);
                }
                other => return Err(format!("unknown argument {other} (try --help)")),
            }
        }
        if screenshot.is_some() && frames.is_none() {
            frames = Some(30);
        }
        Ok(Self {
            frames,
            zoom,
            screenshot,
            look_at_objective,
        })
    }
}

fn next_arg(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    args.next().ok_or_else(|| format!("{flag} needs a value"))
}

fn parse_next<T: std::str::FromStr>(
    args: &mut impl Iterator<Item = String>,
    flag: &str,
) -> Result<T, String> {
    let value = next_arg(args, flag)?;
    value
        .parse::<T>()
        .map_err(|_| format!("{flag} could not parse {value}"))
}
