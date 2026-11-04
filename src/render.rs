//! Drawing the holotable. Runs on the SDL thread and only reads a snapshot.
//!
//! Level of detail follows the on-screen hex size: a facing chevron up close,
//! a diamond further out, and a point when the whole armada is in view.
//! Grid lines thin out the same way, so a zoomed-out frame is not 20,000 outlines.

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Vertex, VertexIndices};
use sdl2::video::Window;

use crate::camera::{Camera, BASE_HEX_SIZE};
use crate::font;
use crate::hex::{self, Hex};
use crate::ship::{Faction, Role, ShipView};
use crate::sim::{Snapshot, MAP_RADIUS};

const HOLO_BG: Color = Color {
    r: 8,
    g: 16,
    b: 24,
    a: 255,
};
const GRID: Color = Color {
    r: 36,
    g: 78,
    b: 92,
    a: 255,
};
const SIDEBAR_BG: Color = Color {
    r: 10,
    g: 14,
    b: 20,
    a: 255,
};
const SIDEBAR_EDGE: Color = Color {
    r: 48,
    g: 82,
    b: 94,
    a: 255,
};
const TEXT: Color = Color {
    r: 186,
    g: 220,
    b: 208,
    a: 255,
};
const TEXT_DIM: Color = Color {
    r: 116,
    g: 150,
    b: 146,
    a: 255,
};
const GOLD: Color = Color {
    r: 232,
    g: 196,
    b: 110,
    a: 255,
};
const AMBER: Color = Color {
    r: 232,
    g: 164,
    b: 64,
    a: 255,
};
const BUTTON_BG: Color = Color {
    r: 18,
    g: 34,
    b: 42,
    a: 255,
};
const BUTTON_EDGE: Color = Color {
    r: 86,
    g: 150,
    b: 158,
    a: 255,
};
const OVERLAY_BG: Color = Color {
    r: 6,
    g: 10,
    b: 14,
    a: 255,
};

const SIDEBAR_MAX: i32 = 250;

pub struct Layout {
    pub side: i32,
    pub view_x: i32,
    pub view_w: i32,
    pub view_h: i32,
}

impl Layout {
    pub fn from_window(width: u32, height: u32) -> Self {
        let side = SIDEBAR_MAX.min(width as i32 / 5).max(150);
        Self {
            side,
            view_x: side,
            view_w: (width as i32 - side * 2).max(1),
            view_h: height as i32,
        }
    }
}

pub struct Hud {
    pub own_ship: Rect,
    pub objective: Rect,
    pub drawn: usize,
    pub lod: &'static str,
}

pub struct FrameInfo<'a> {
    pub camera: &'a Camera,
    pub snapshot: &'a Snapshot,
    pub fps: f32,
    pub frame_ms: f32,
}

pub fn draw(
    canvas: &mut Canvas<Window>,
    info: FrameInfo<'_>,
    vertices: &mut Vec<Vertex>,
) -> Result<Hud, String> {
    let (width, height) = canvas.output_size()?;
    let layout = Layout::from_window(width, height);

    canvas.set_draw_color(HOLO_BG);
    canvas.clear();

    let view = Rect::new(layout.view_x, 0, layout.view_w as u32, layout.view_h as u32);
    canvas.set_clip_rect(view);
    let lod = draw_grid(canvas, info.camera, &layout);
    let drawn = draw_ships(canvas, info.camera, info.snapshot, &layout, vertices);
    canvas.set_clip_rect(None);

    let hud_buttons = draw_sidebars(canvas, &layout, &info, drawn, lod);
    draw_overlay(canvas, &layout, &info, lod);

    Ok(Hud {
        own_ship: hud_buttons.0,
        objective: hud_buttons.1,
        drawn,
        lod,
    })
}

fn draw_grid(canvas: &mut Canvas<Window>, camera: &Camera, layout: &Layout) -> &'static str {
    let hex_px = camera.hex_px();
    let lod = lod_for(hex_px);
    // Outlines are the expensive part of a zoomed-out frame. Points carry
    // the picture once a hex is only a few pixels across.
    let stride = if hex_px >= 8.0 {
        1
    } else if hex_px >= 3.5 {
        5
    } else {
        return lod;
    };

    canvas.set_draw_color(GRID);
    let (q_min, q_max, r_min, r_max) = visible_axial(camera, layout);
    let origin = Hex::new(0, 0);
    for q in q_min..=q_max {
        for r in r_min..=r_max {
            if stride > 1 && (q.rem_euclid(stride) != 0 || r.rem_euclid(stride) != 0) {
                continue;
            }
            let hex = Hex::new(q, r);
            if hex::distance(hex, origin) > MAP_RADIUS {
                continue;
            }
            let (wx, wy) = hex::to_pixel(hex, BASE_HEX_SIZE);
            let (sx, sy) =
                camera.world_to_screen(wx, wy, layout.view_w as f32, layout.view_h as f32);
            stroke_hex(canvas, layout.view_x as f32 + sx, sy, hex_px);
        }
    }
    lod
}

fn draw_ships(
    canvas: &mut Canvas<Window>,
    camera: &Camera,
    snapshot: &Snapshot,
    layout: &Layout,
    vertices: &mut Vec<Vertex>,
) -> usize {
    vertices.clear();
    let hex_px = camera.hex_px();
    let view_w = layout.view_w as f32;
    let view_h = layout.view_h as f32;
    let mut drawn = 0;
    // Fleet first, then the two marked ships, so the markers sit on top.
    for pass in [false, true] {
        for ship in &snapshot.ships {
            let marked = ship.role != Role::Fleet;
            if marked != pass {
                continue;
            }
            let (wx, wy) = hex::axial_to_pixel(ship.q, ship.r, BASE_HEX_SIZE);
            let (sx, sy) = camera.world_to_screen(wx, wy, view_w, view_h);
            if sx < -24.0 || sy < -24.0 || sx > view_w + 24.0 || sy > view_h + 24.0 {
                continue;
            }
            push_ship(vertices, ship, layout.view_x as f32 + sx, sy, hex_px);
            drawn += 1;
        }
    }
    flush_geometry(canvas, vertices);
    drawn
}

fn draw_sidebars(
    canvas: &mut Canvas<Window>,
    layout: &Layout,
    info: &FrameInfo<'_>,
    drawn: usize,
    lod: &str,
) -> (Rect, Rect) {
    let height = layout.view_h;
    let left = Rect::new(0, 0, layout.side as u32, height as u32);
    let right = Rect::new(
        layout.view_x + layout.view_w,
        0,
        layout.side as u32,
        height as u32,
    );
    canvas.set_draw_color(SIDEBAR_BG);
    let _ = canvas.fill_rect(left);
    let _ = canvas.fill_rect(right);
    canvas.set_draw_color(SIDEBAR_EDGE);
    let _ = canvas.draw_line(Point::new(layout.side, 0), Point::new(layout.side, height));
    let right_edge = layout.view_x + layout.view_w;
    let _ = canvas.draw_line(Point::new(right_edge, 0), Point::new(right_edge, height));

    let mut y = 16;
    let x = 14;
    y = label(canvas, x, y, 2, TEXT, "SCALE SCENE");
    y += 8;
    y = label(canvas, x, y, 2, TEXT_DIM, "ORDER");
    y = label(canvas, x, y, 2, TEXT, "DEVELOPMENT");
    y = label(canvas, x, y, 2, TEXT, "PICTURE OF");
    y = label(canvas, x, y, 2, TEXT, "THE ARMADA");
    y += 8;
    y = label(canvas, x, y, 2, TEXT_DIM, "HARD OBJECTIVE");
    y = label(canvas, x, y, 2, TEXT, "MARKED CONTACT");
    y += 10;

    let button_w = layout.side - 28;
    let own_ship = button(canvas, x, y, button_w, "HOME  OWN SHIP");
    y += 30;
    let objective = button(canvas, x, y, button_w, "END  OBJECTIVE");
    y += 36;

    y = label(canvas, x, y, 2, TEXT_DIM, "FLEET");
    y = label(
        canvas,
        x,
        y,
        2,
        TEXT,
        &format!("SHIPS {}", info.snapshot.ships.len()),
    );
    y = label(
        canvas,
        x,
        y,
        2,
        TEXT,
        &format!("NO AP {}", info.snapshot.idle_ships),
    );
    y = label(
        canvas,
        x,
        y,
        2,
        TEXT,
        &format!("DECIDED {}", info.snapshot.decisions),
    );
    y = label(
        canvas,
        x,
        y,
        2,
        TEXT,
        &format!("SIM {:.2} MS", info.snapshot.step_ms),
    );
    let _ = label(
        canvas,
        x,
        y,
        2,
        TEXT_DIM,
        &format!("TICK {}", info.snapshot.tick),
    );

    let mut y = 16;
    let x = right_edge + 14;
    y = label(canvas, x, y, 2, TEXT_DIM, "FACTIONS");
    y = swatch(canvas, x, y, Faction::Imperial, &["IMPERIAL", "ROYAL NAVY"]);
    y += 6;
    y = swatch(
        canvas,
        x,
        y,
        Faction::Alliance,
        &["ALLIED", "PEOPLE'S NAVAL", "FORCES"],
    );
    y += 10;
    y = label(canvas, x, y, 2, TEXT_DIM, "CAMERA");
    y = label(canvas, x, y, 2, TEXT, "PAN ARROWS");
    y = label(canvas, x, y, 2, TEXT, "OR DRAG THE MAP");
    y = label(canvas, x, y, 2, TEXT, "ZOOM - = WHEEL");
    y = label(canvas, x, y, 2, TEXT, "HOME OWN SHIP");
    y = label(canvas, x, y, 2, TEXT, "END OBJECTIVE");
    y = label(canvas, x, y, 2, TEXT, "ESC QUIT");
    y += 8;
    y = label(canvas, x, y, 2, TEXT_DIM, "VIEW");
    y = label(
        canvas,
        x,
        y,
        2,
        TEXT,
        &format!("ZOOM {:.2}", info.camera.zoom),
    );
    y = label(canvas, x, y, 2, TEXT, &format!("LOD {lod}"));
    let _ = label(canvas, x, y, 2, TEXT, &format!("DRAWN {drawn}"));

    (own_ship, objective)
}

fn draw_overlay(canvas: &mut Canvas<Window>, layout: &Layout, info: &FrameInfo<'_>, lod: &str) {
    let x = layout.view_x + 10;
    let y = 10;
    let panel = Rect::new(x - 4, y - 4, 132, 40);
    canvas.set_draw_color(OVERLAY_BG);
    let _ = canvas.fill_rect(panel);
    let fps = format!("FPS {:.0}", info.fps);
    let frame = format!("{:.1} MS  {lod}", info.frame_ms);
    label(canvas, x, y, 2, TEXT, &fps);
    label(canvas, x, y + font::line_height(2), 2, TEXT_DIM, &frame);
}

fn swatch(canvas: &mut Canvas<Window>, x: i32, y: i32, faction: Faction, lines: &[&str]) -> i32 {
    let (r, g, b) = faction.rgb();
    canvas.set_draw_color(Color::RGB(r, g, b));
    let _ = canvas.fill_rect(Rect::new(x, y + 2, 12, 12));
    let mut y = y;
    for (index, line) in lines.iter().enumerate() {
        let text_x = if index == 0 { x + 18 } else { x };
        y = label(canvas, text_x, y, 2, TEXT, line);
    }
    y
}

fn button(canvas: &mut Canvas<Window>, x: i32, y: i32, width: i32, text: &str) -> Rect {
    let rect = Rect::new(x, y, width as u32, 24);
    canvas.set_draw_color(BUTTON_BG);
    let _ = canvas.fill_rect(rect);
    canvas.set_draw_color(BUTTON_EDGE);
    let _ = canvas.draw_rect(rect);
    font::draw_text(canvas, x + 8, y + 6, 2, TEXT, text);
    rect
}

fn label(canvas: &mut Canvas<Window>, x: i32, y: i32, scale: i32, color: Color, text: &str) -> i32 {
    font::draw_text(canvas, x, y, scale, color, text);
    y + font::line_height(scale)
}

fn lod_for(hex_px: f32) -> &'static str {
    if hex_px >= 14.0 {
        "ICON"
    } else if hex_px >= 5.0 {
        "MARK"
    } else {
        "POINT"
    }
}

fn visible_axial(camera: &Camera, layout: &Layout) -> (i32, i32, i32, i32) {
    let view_w = layout.view_w as f32;
    let view_h = layout.view_h as f32;
    let mut q_min = i32::MAX;
    let mut q_max = i32::MIN;
    let mut r_min = i32::MAX;
    let mut r_max = i32::MIN;
    for (sx, sy) in [(0.0, 0.0), (view_w, 0.0), (0.0, view_h), (view_w, view_h)] {
        let (wx, wy) = camera.screen_to_world(sx, sy, view_w, view_h);
        let (q, r) = hex::pixel_to_axial(wx, wy, BASE_HEX_SIZE);
        q_min = q_min.min(q.floor() as i32);
        q_max = q_max.max(q.ceil() as i32);
        r_min = r_min.min(r.floor() as i32);
        r_max = r_max.max(r.ceil() as i32);
    }
    (q_min - 1, q_max + 1, r_min - 1, r_max + 1)
}

fn stroke_hex(canvas: &mut Canvas<Window>, cx: f32, cy: f32, size: f32) {
    let mut corners = [(0.0f32, 0.0f32); 6];
    for i in 0..6 {
        // Pointy top: the first corner is 30 degrees above east.
        let angle = std::f32::consts::FRAC_PI_3 * i as f32 - std::f32::consts::FRAC_PI_6;
        corners[i] = (cx + size * angle.cos(), cy + size * angle.sin());
    }
    for i in 0..6 {
        let (x0, y0) = corners[i];
        let (x1, y1) = corners[(i + 1) % 6];
        let _ = canvas.draw_line(
            Point::new(x0.round() as i32, y0.round() as i32),
            Point::new(x1.round() as i32, y1.round() as i32),
        );
    }
}

fn push_ship(vertices: &mut Vec<Vertex>, ship: &ShipView, x: f32, y: f32, hex_px: f32) {
    let (r, g, b) = ship.faction.rgb();
    let color = Color::RGB(r, g, b);
    if hex_px >= 14.0 {
        if ship.role == Role::Player {
            push_chevron(vertices, x, y, ship.facing, hex_px * 0.78, GOLD);
        } else if ship.role == Role::Objective {
            push_diamond(vertices, x, y, hex_px * 0.72, AMBER);
        }
        push_chevron(vertices, x, y, ship.facing, hex_px * 0.46, color);
    } else if hex_px >= 5.0 {
        let radius = (hex_px * 0.30).max(2.4);
        if ship.role == Role::Player {
            push_diamond(vertices, x, y, radius + 2.5, GOLD);
        } else if ship.role == Role::Objective {
            push_diamond(vertices, x, y, radius + 2.5, AMBER);
        }
        push_diamond(vertices, x, y, radius, color);
    } else {
        if ship.role == Role::Player {
            push_diamond(vertices, x, y, 4.5, GOLD);
        } else if ship.role == Role::Objective {
            push_diamond(vertices, x, y, 4.5, AMBER);
        }
        push_point(vertices, x, y, 1.8, color);
    }
}

fn push_chevron(vertices: &mut Vec<Vertex>, x: f32, y: f32, facing: u8, radius: f32, color: Color) {
    // Facing 0 is east. Screen y grows downward, matching the hex pixel math.
    let angle = -(facing as f32) * std::f32::consts::FRAC_PI_3;
    let nose = (x + angle.cos() * radius, y + angle.sin() * radius);
    let left = angle + 2.45;
    let right = angle - 2.45;
    let back = radius * 0.78;
    let wing_l = (x + left.cos() * back, y + left.sin() * back);
    let wing_r = (x + right.cos() * back, y + right.sin() * back);
    push_tri(vertices, nose, wing_l, wing_r, color);
}

fn push_diamond(vertices: &mut Vec<Vertex>, x: f32, y: f32, radius: f32, color: Color) {
    let top = (x, y - radius);
    let right = (x + radius, y);
    let bottom = (x, y + radius);
    let left = (x - radius, y);
    push_tri(vertices, top, right, bottom, color);
    push_tri(vertices, top, bottom, left, color);
}

fn push_point(vertices: &mut Vec<Vertex>, x: f32, y: f32, radius: f32, color: Color) {
    push_tri(
        vertices,
        (x, y - radius),
        (x + radius, y + radius),
        (x - radius, y + radius),
        color,
    );
}

fn push_tri(vertices: &mut Vec<Vertex>, a: (f32, f32), b: (f32, f32), c: (f32, f32), color: Color) {
    vertices.push(vertex(a.0, a.1, color));
    vertices.push(vertex(b.0, b.1, color));
    vertices.push(vertex(c.0, c.1, color));
}

fn vertex(x: f32, y: f32, color: Color) -> Vertex {
    Vertex {
        position: sdl2::rect::FPoint::new(x, y),
        color,
        tex_coord: sdl2::rect::FPoint::new(0.0, 0.0),
    }
}

fn flush_geometry(canvas: &mut Canvas<Window>, vertices: &mut Vec<Vertex>) {
    const CHUNK: usize = 60_000;
    let mut start = 0;
    while start + 3 <= vertices.len() {
        let mut end = (start + CHUNK).min(vertices.len());
        end = start + ((end - start) / 3) * 3;
        if canvas
            .render_geometry(&vertices[start..end], None, VertexIndices::Sequential)
            .is_err()
        {
            break;
        }
        start = end;
    }
    vertices.clear();
}
