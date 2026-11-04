//! A tiny 3×5 bitmap font, so the overlay does not need SDL_ttf.
//!
//! Glyphs are original and only cover the scale-scene readout. Each row is
//! three bits, with bit 2 on the left.

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Glyph width plus a one-pixel gap, before scaling.
const ADVANCE: i32 = 4;

pub fn line_height(scale: i32) -> i32 {
    6 * scale
}

pub fn draw_text(
    canvas: &mut Canvas<Window>,
    x: i32,
    y: i32,
    scale: i32,
    color: Color,
    text: &str,
) {
    canvas.set_draw_color(color);
    let mut cursor = x;
    for ch in text.chars() {
        let ch = if ch.is_ascii_lowercase() {
            (ch as u8 - 32) as char
        } else {
            ch
        };
        if let Some(rows) = glyph(ch) {
            for (row, bits) in rows.iter().enumerate() {
                for col in 0..3 {
                    if bits & (1 << (2 - col)) == 0 {
                        continue;
                    }
                    let rect = Rect::new(
                        cursor + col * scale,
                        y + row as i32 * scale,
                        scale as u32,
                        scale as u32,
                    );
                    let _ = canvas.fill_rect(rect);
                }
            }
        }
        cursor += ADVANCE * scale;
    }
}

fn glyph(ch: char) -> Option<[u8; 5]> {
    let rows = match ch {
        ' ' => [0, 0, 0, 0, 0],
        'A' => [0b010, 0b101, 0b111, 0b101, 0b101],
        'B' => [0b110, 0b101, 0b110, 0b101, 0b110],
        'C' => [0b011, 0b100, 0b100, 0b100, 0b011],
        'D' => [0b110, 0b101, 0b101, 0b101, 0b110],
        'E' => [0b111, 0b100, 0b110, 0b100, 0b111],
        'F' => [0b111, 0b100, 0b110, 0b100, 0b100],
        'G' => [0b011, 0b100, 0b101, 0b101, 0b011],
        'H' => [0b101, 0b101, 0b111, 0b101, 0b101],
        'I' => [0b111, 0b010, 0b010, 0b010, 0b111],
        'J' => [0b001, 0b001, 0b001, 0b101, 0b010],
        'K' => [0b101, 0b101, 0b110, 0b101, 0b101],
        'L' => [0b100, 0b100, 0b100, 0b100, 0b111],
        'M' => [0b101, 0b111, 0b101, 0b101, 0b101],
        'N' => [0b110, 0b101, 0b101, 0b101, 0b101],
        'O' => [0b010, 0b101, 0b101, 0b101, 0b010],
        'P' => [0b110, 0b101, 0b110, 0b100, 0b100],
        'Q' => [0b010, 0b101, 0b101, 0b111, 0b011],
        'R' => [0b110, 0b101, 0b110, 0b101, 0b101],
        'S' => [0b011, 0b100, 0b010, 0b001, 0b110],
        'T' => [0b111, 0b010, 0b010, 0b010, 0b010],
        'U' => [0b101, 0b101, 0b101, 0b101, 0b010],
        'V' => [0b101, 0b101, 0b101, 0b010, 0b010],
        'W' => [0b101, 0b101, 0b101, 0b111, 0b101],
        'X' => [0b101, 0b101, 0b010, 0b101, 0b101],
        'Y' => [0b101, 0b101, 0b010, 0b010, 0b010],
        'Z' => [0b111, 0b001, 0b010, 0b100, 0b111],
        '0' => [0b111, 0b101, 0b101, 0b101, 0b111],
        '1' => [0b010, 0b110, 0b010, 0b010, 0b111],
        '2' => [0b111, 0b001, 0b111, 0b100, 0b111],
        '3' => [0b111, 0b001, 0b111, 0b001, 0b111],
        '4' => [0b101, 0b101, 0b111, 0b001, 0b001],
        '5' => [0b111, 0b100, 0b111, 0b001, 0b111],
        '6' => [0b111, 0b100, 0b111, 0b101, 0b111],
        '7' => [0b111, 0b001, 0b010, 0b010, 0b010],
        '8' => [0b111, 0b101, 0b111, 0b101, 0b111],
        '9' => [0b111, 0b101, 0b111, 0b001, 0b111],
        '.' => [0, 0, 0, 0, 0b010],
        ':' => [0, 0b010, 0, 0b010, 0],
        '-' => [0, 0, 0b111, 0, 0],
        '+' => [0, 0b010, 0b111, 0b010, 0],
        '=' => [0, 0b111, 0, 0b111, 0],
        '/' => [0b001, 0b001, 0b010, 0b100, 0b100],
        '\'' => [0b010, 0b010, 0, 0, 0],
        '(' | '[' => [0b001, 0b010, 0b010, 0b010, 0b001],
        ')' | ']' => [0b100, 0b010, 0b010, 0b010, 0b100],
        _ => return None,
    };
    Some(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_and_digits_have_ink() {
        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars() {
            let rows = glyph(ch).unwrap_or_else(|| panic!("missing {ch}"));
            let ink: u32 = rows.iter().map(|row| row.count_ones()).sum();
            assert!(ink > 0, "{ch} is blank");
        }
    }
}
