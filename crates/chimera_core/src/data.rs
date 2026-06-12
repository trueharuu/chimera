use crate::{piece::Piece, rotation::Rotation};

/// `(dx, dy)` offsets of each mino relative to the SRS pivot point.
/// `dy > 0` is upward.
pub type Cells = [(i8, i8); 4];

/// Cell offsets for `(piece, rotation)`.
pub const PIECE_CELLS: [[Cells; Rotation::NB]; Piece::NB] = [
    // T
    [
        [(0, 0), (-1, 0), (0, 1), (1, 0)],
        [(0, 0), (0, 1), (0, -1), (1, 0)],
        [(0, 0), (-1, 0), (0, -1), (1, 0)],
        [(0, 0), (0, 1), (0, -1), (-1, 0)],
    ],
    // I
    [
        [(0, 0), (-1, 0), (1, 0), (2, 0)],
        [(0, 0), (0, 1), (0, -1), (0, -2)],
        [(0, 0), (-1, 0), (1, 0), (-2, 0)],
        [(0, 0), (0, -1), (0, 1), (0, 2)],
    ],
    // J
    [
        [(0, 0), (-1, 0), (-1, 1), (1, 0)],
        [(0, 0), (0, 1), (1, 1), (0, -1)],
        [(0, 0), (-1, 0), (1, 0), (1, -1)],
        [(0, 0), (0, 1), (-1, -1), (0, -1)],
    ],
    // L
    [
        [(0, 0), (-1, 0), (1, 1), (1, 0)],
        [(0, 0), (0, 1), (1, -1), (0, -1)],
        [(0, 0), (-1, 0), (1, 0), (-1, -1)],
        [(0, 0), (0, 1), (-1, 1), (0, -1)],
    ],
    // O
    [
        [(0, 0), (1, 0), (0, 1), (1, 1)],
        [(0, 0), (1, 0), (0, -1), (1, -1)],
        [(0, 0), (-1, 0), (0, -1), (-1, -1)],
        [(0, 0), (-1, 0), (0, 1), (-1, 1)],
    ],
    // S
    [
        [(0, 0), (-1, 0), (0, 1), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, -1)],
        [(0, 0), (0, -1), (-1, -1), (1, 0)],
        [(0, 0), (-1, 0), (-1, 1), (0, -1)],
    ],
    // Z
    [
        [(0, 0), (0, 1), (-1, 1), (1, 0)],
        [(0, 0), (1, 0), (1, 1), (0, -1)],
        [(0, 0), (-1, 0), (0, -1), (1, -1)],
        [(0, 0), (-1, 0), (-1, -1), (0, 1)],
    ],
];

#[warn(clippy::match_same_arms)]
/// Minimum x offset so the leftmost cell is at column 0, and the maximum x offset so the rightmost cell is at column 9.
pub const fn x_range(piece: Piece, rot: Rotation) -> (i8, i8) {
    match (piece, rot) {
        (Piece::I, Rotation::North) => (1, 7),
        (Piece::I, Rotation::East) | (Piece::I, Rotation::West) => (0, 9),
        (Piece::I, Rotation::South) => (2, 8),
        (Piece::T, Rotation::East)
        | (Piece::J, Rotation::East)
        | (Piece::L, Rotation::East)
        | (Piece::O, Rotation::North)
        | (Piece::O, Rotation::East)
        | (Piece::S, Rotation::East)
        | (Piece::Z, Rotation::East) => (0, 8),
        (Piece::T, Rotation::North)
        | (Piece::T, Rotation::South)
        | (Piece::J, Rotation::North)
        | (Piece::J, Rotation::South)
        | (Piece::L, Rotation::North)
        | (Piece::L, Rotation::South)
        | (Piece::S, Rotation::North)
        | (Piece::S, Rotation::South)
        | (Piece::Z, Rotation::North)
        | (Piece::Z, Rotation::South) => (1, 8),
        (Piece::T, Rotation::West)
        | (Piece::J, Rotation::West)
        | (Piece::L, Rotation::West)
        | (Piece::O, Rotation::South)
        | (Piece::O, Rotation::West)
        | (Piece::S, Rotation::West)
        | (Piece::Z, Rotation::West) => (1, 9),
    }
}

#[inline(always)]
#[warn(clippy::match_same_arms)]
pub const fn kicks_for(piece: Piece, from: Rotation, to: Rotation) -> &'static [(i8, i8)] {
    match piece {
        Piece::T | Piece::J | Piece::L | Piece::S | Piece::Z => match (from, to) {
            // L.NE=(0,0)(-1,0)(-1,1)(0,-2)(-1,-2)
            // L.ES=(0,0)(1,0)(1,-1)(0,2)(1,2)
            // L.SW=(0,0)(1,0)(1,1)(0,-2)(1,-2)
            // L.WN=(0,0)(-1,0)(-1,-1)(0,2)(-1,2)
            // L.NW=(0,0)(1,0)(1,1)(0,-2)(1,-2)
            // L.WS=(0,0)(-1,0)(-1,-1)(0,2)(-1,2)
            // L.SE=(0,0)(-1,0)(-1,1)(0,-2)(-1,-2)
            // L.EN=(0,0)(1,0)(1,-1)(0,2)(1,2)
            // L.NS=(0,0)(0,1)
            // L.EW=(0,0)(1,0)
            // L.SN=(0,0)(0,-1)
            // L.WE=(0,0)(-1,0)
            (Rotation::South, Rotation::West) | (Rotation::North, Rotation::West) => {
                &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
            }
            (Rotation::North, Rotation::East) | (Rotation::West, Rotation::South) => {
                &[(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
            }
            (Rotation::East, Rotation::South) | (Rotation::South, Rotation::East) => {
                &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]
            }
            (Rotation::West, Rotation::North) | (Rotation::East, Rotation::North) => {
                &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
            }
            (Rotation::North, Rotation::South) => &[(0, 0), (0, 1)],
            (Rotation::East, Rotation::West) => &[(0, 0), (1, 0)],
            (Rotation::South, Rotation::North) => &[(0, 0), (0, -1)],
            (Rotation::West, Rotation::East) => &[(0, 0), (-1, 0)],
            _ => &[],
        },

        // I.NE=(1,0)(-1,0)(2,0)(-1,-1)(2,2)
        // I.ES=(0,-1)(-1,-1)(2,-1)(-1,1)(2,-2)
        // I.SW=(-1,0)(1,0)(-2,0)(1,1)(-2,-2)
        // I.WN=(0,1)(1,1)(-2,1)(1,-1)(-2,2)
        // I.NW=(0,-1)(-1,-1)(2,-1)(-1,1)(2,-2)
        // I.WS=(1,0)(-1,0)(2,0)(-1,-1)(2,2)
        // I.SE=(0,1)(1,1)(-2,1)(1,-1)(-2,2)
        // I.EN=(-1,0)(1,0)(-2,0)(1,1)(-2,-2)
        // I.NS=(1,-1)(1,0)
        // I.EW=(-1,-1)(0,-1)
        // I.SN=(-1,1)(-1,0)
        // I.WE=(1,1)(0,1)
        Piece::I => match (from, to) {
            (Rotation::South, Rotation::West) => &[(-1, 0), (1, 0), (-2, 0), (1, 1), (-2, -2)],
            (Rotation::West, Rotation::North) | (Rotation::East, Rotation::North) => {
                &[(0, 1), (1, 1), (-2, 1), (1, -1), (-2, 2)]
            }
            (Rotation::North, Rotation::East) | (Rotation::West, Rotation::South) => {
                &[(1, 0), (-1, 0), (2, 0), (-1, -1), (2, 2)]
            }
            (Rotation::East, Rotation::South)
            | (Rotation::North, Rotation::West)
            | (Rotation::South, Rotation::East) => &[(0, -1), (-1, -1), (2, -1), (-1, 1), (2, -2)],
            (Rotation::North, Rotation::South) => &[(1, -1), (1, 0)],
            (Rotation::East, Rotation::West) => &[(-1, -1), (0, -1)],
            (Rotation::South, Rotation::North) => &[(-1, 1), (-1, 0)],
            (Rotation::West, Rotation::East) => &[(1, 1), (0, 1)],
            _ => &[],
        },

        // O.NE=(0,1)
        // O.ES=(1,0)
        // O.SW=(0,-1)
        // O.WN=(-1,0)
        // O.NW=(1,0)
        // O.WS=(0,1)
        // O.SE=(-1,0)
        // O.EN=(0,-1)
        // O.NS=(1,1)
        // O.EW=(1,-1)
        // O.SN=(-1,-1)
        // O.WE=(-1,1)
        Piece::O => match (from, to) {
            (Rotation::East, Rotation::South) | (Rotation::North, Rotation::West) => &[(1, 0)],
            (Rotation::North, Rotation::East) | (Rotation::West, Rotation::South) => &[(0, 1)],
            (Rotation::West, Rotation::North) | (Rotation::South, Rotation::East) => &[(-1, 0)],
            (Rotation::South, Rotation::West) | (Rotation::East, Rotation::North) => &[(0, -1)],
            (Rotation::North, Rotation::South) => &[(1, 1)],
            (Rotation::East, Rotation::West) => &[(1, -1)],
            (Rotation::South, Rotation::North) => &[(-1, -1)],
            (Rotation::West, Rotation::East) => &[(-1, 1)],
            _ => &[],
        },
    }
}
