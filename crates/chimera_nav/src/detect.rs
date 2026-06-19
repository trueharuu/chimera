use chimera_core::{
    board::Board, header::COLS,
    rotation::Rotation, spin::Spin,
};

use crate::collision_map::CollisionMap;

#[inline]
pub const fn spin_max(a: Spin, b: Spin) -> Spin {
    match (a, b) {
        (Spin::Full, _) | (_, Spin::Full) => Spin::Full,
        (Spin::Mini, _) | (_, Spin::Mini) => Spin::Mini,
        _ => Spin::None,
    }
}

#[inline]
pub fn is_immobile<const PIECE: u8>(cm: &CollisionMap, rot: Rotation, x: usize, y: usize) -> bool {
    // Left
    if x > 0 && !cm.collides(x - 1, y, rot) {
        return false;
    }

    // Right
    if x + 1 < COLS && !cm.collides(x + 1, y, rot) {
        return false;
    }

    // Down (y - 1); y == 0 means floor → blocked
    if y > 0 && !cm.collides(x, y - 1, rot) {
        return false;
    }

    // Up (y + 1); if bit y+1 is free, piece can shift up
    if y + 1 < 8 && !cm.collides(x, y + 1, rot) {
        return false;
    }

    true
}

pub const T_CORNERS: [(i8, i8); 4] = [(-1, 1), (1, 1), (-1, -1), (1, -1)];

pub const T_FRONT_CORNERS: [[(i8, i8); 2]; Rotation::NB] = [
    [(-1, 1), (1, 1)],
    [(1, 1), (1, -1)],
    [(-1, -1), (1, -1)],
    [(-1, 1), (-1, -1)],
];

#[inline]
pub fn t_spin_from_corners(board: Board, x: usize, y: usize, rot: Rotation, is_tst: bool) -> Spin {
    let corners = T_CORNERS;
    let front_corners = T_FRONT_CORNERS[rot as usize];
    let mut front_count = 0;
    let mut total = 0;
    for &(cx, cy) in &corners {
        let cell = cell_occupied(board, x as i8 + cx, y as i8 + cy);
        if cell {
            total += 1;
        }
    }

    for &(cx, cy) in &front_corners {
        if cell_occupied(board, x as i8 + cx, y as i8 + cy) {
            front_count += 1;
        }
    }

    if total < 3 {
        return Spin::None;
    }

    // TST kick always upgrades to Full regardless of corner layout.
    if is_tst {
        return Spin::Full;
    }

    // both front corners filled are Full; one front + two back are Mini
    if front_count == 2 {
        Spin::Full
    } else {
        Spin::Mini
    }
}

#[inline]
pub fn cell_occupied(board: Board, cx: i8, cy: i8) -> bool {
    if cx < 0 || cx >= COLS as i8 || cy < 0 {
        return true;
    }

    // rows >= board height are open sky and not occupied
    board.get(cx as usize, cy as usize)
}
