use chimera_core::{board::Board, piece::Piece};

use crate::buffer::MoveBuffer;

/// 'Instant' mode move generation. Only generates placements reachable with infinite gravity applied after every movement.
/// Additionally, all pieces spawn at the lowest possible point.
pub fn movegen(board: Board, piece: Piece, out: &mut MoveBuffer) {
    let _ = (board, piece, out);
    
}