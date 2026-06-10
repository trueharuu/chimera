use crate::{
    board::Board,
    collision_map::CollisionMap,
    data::PIECE_CELLS,
    header::{COLS, ROWS},
    piece::Piece,
    placement::Move,
    rotation::Rotation,
};

pub fn render(board_a: &Board, placement_a: Option<Move>) {
    let pad = ROWS.ilog10() as usize + 1;
    eprintln!(" {}\u{250c}{}\u{2510}", " ".repeat(pad), "\u{2500}".repeat(20));
    for y in (0..ROWS).rev() {
        eprint!("{y: >pad$} \u{2502}");
        for x in 0..COLS {
            let cell = board_a.get(x, y);
            if cell {
                eprint!("\x1b[48;2;127;127;127m  \x1b[0m");
            } else if let Some(p) = placement_a {
                let cells = PIECE_CELLS[p.piece() as usize][p.rot() as usize];
                if cells
                    .iter()
                    .any(|(dx, dy)| p.x() as i8 + dx == x as i8 && p.y() as i8 + dy == y as i8)
                {
                    let is_center = p.x() as i8 == x as i8 && p.y() as i8 == y as i8;
                    eprint!("{}", draw_cell(p.piece(), is_center));
                } else {
                    eprint!("  ");
                }
            } else {
                eprint!("  ");
            }
        }
        eprintln!("\u{2502}");
    }
    eprintln!(" {}\u{2514}{}\u{2518}", " ".repeat(pad), "\u{2500}".repeat(20));
}

pub fn draw_cell(piece: Piece, is_center: bool) -> String {
    let s = if is_center { "<>" } else { "  " };
    let l = match piece {
        Piece::Z => "\x1b[48;2;255;127;127m",
        Piece::L => "\x1b[48;2;255;192;127m",
        Piece::O => "\x1b[48;2;255;255;127m",
        Piece::S => "\x1b[48;2;127;255;127m",
        Piece::I => "\x1b[48;2;127;255;255m",
        Piece::J => "\x1b[48;2;127;127;255m",
        Piece::T => "\x1b[48;2;255;127;255m",
    };
    format!("{l}{s}\x1b[0m")
}

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
