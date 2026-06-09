use chimera_core::{
    board::Board, collision_map::CollisionMap, piece::Piece, rotation::Rotation, spin::Spins,
};

use crate::buffer::MoveBuffer;

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
pub fn movegen(board: Board, piece: Piece, _spins: Spins, _out: &mut MoveBuffer) {
    let mut usable_map = CollisionMap::new(board, piece);
    let mut landable_map = usable_map.landable();

    let mut search = [Board::EMPTY; Rotation::NB];

    
}
