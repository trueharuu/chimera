use chimera_core::{board::Board, header::{COLS, ROWS}, piece::Piece, render::draw_cell, rotation::Rotation};

use crate::collision_map::CollisionMap;

pub fn render_collision(
    board_a: &Board,
    collision_map: &CollisionMap,
    rot: Rotation,
    piece: Piece,
) {
    println!("\u{250c}{}\u{2510}", "\u{2500}".repeat(20),);
    for y in (0..ROWS).rev() {
        print!("\u{2502}");
        for x in 0..COLS {
            let cell = board_a.get(x, y);
            if collision_map.landed(x, y, rot) {
                print!("{}", draw_cell(piece, false));
            } else if cell {
                print!("\x1b[48;2;127;127;127m  \x1b[0m");
            } else {
                print!("  ");
            }
        }
        println!("\u{2502}");
    }
    println!("\u{2514}{}\u{2518}", "\u{2500}".repeat(20));
}
