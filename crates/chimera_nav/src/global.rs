use chimera_core::{
    board::Board,
    data::kicks_for,
    header::{COL_MASK, COLS},
    piece::Piece,
    placement::Move,
    rotation::Rotation,
    spin::{Spin, Spins},
};
use crate::{buffer::MoveBuffer, collision_map::CollisionMap};
// no heap allocations for dedupe

/// 'Global' mode movegen. No restrictions are made other than SRS.
///
/// Spin detection is dependent on the value of `spins`.
///
/// If [`Spins::None`], nothing is a spin.
///
/// If [`Spins::T`], a piece is a spin if:
/// - The piece is [`Piece::T`].
/// - The last input is a rotation.
/// - 3 of the 4 squares diagonally adjacent to the piece's center are occupied.
/// - A [`Spin::Full`] is emitted if there are two cells in the "front" corners.
/// - Otherwise, if there is only one cell in the front and two in the back, a [`Spin::Mini`] is emitted.
/// - Any spin that is the result of the last kick (TST kick) in a (counter)clockwise rotation is upgraded to a [`Spin::Full`].
///
/// If [`Spins::All`], then a piece is a spin if it is either:
/// - Immobile, and cannot move in any of the 4 directions
/// - A case where a [`Spin::Mini`] would be emitted under `A = 1`.
///
/// The "front" corners of a T piece are the ones adjacent to the "sticking out" mino of the piece.
/// The floor and walls of the board are considered to be occupied.
///
/// Movegen should output both sets of placements if available. If the same position is reachable with both a spin and without (i.e. T placement vs. T-spin-mini),
/// then they are considered distinct.
pub fn movegen(board: Board, piece: Piece, spins: Spins, out: &mut MoveBuffer) {
    match piece {
        Piece::T => movegen_inner::<{ Piece::T as u8 }>(board, spins, out),
        Piece::I => movegen_inner::<{ Piece::I as u8 }>(board, spins, out),
        Piece::J => movegen_inner::<{ Piece::J as u8 }>(board, spins, out),
        Piece::L => movegen_inner::<{ Piece::L as u8 }>(board, spins, out),
        Piece::O => movegen_inner::<{ Piece::O as u8 }>(board, spins, out),
        Piece::S => movegen_inner::<{ Piece::S as u8 }>(board, spins, out),
        Piece::Z => movegen_inner::<{ Piece::Z as u8 }>(board, spins, out),
    }
}
pub fn movegen_inner<const PIECE: u8>(board: Board, spins: Spins, out: &mut MoveBuffer) {
    out.clear();

    
}
