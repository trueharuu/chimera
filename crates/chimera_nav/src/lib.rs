#![feature(const_trait_impl, const_index)]
#![allow(clippy::needless_range_loop)]
pub mod buffer;
pub mod detect;
pub mod global;
pub mod instant;
pub mod render;
pub mod collision_map;

#[cfg(test)]
mod tests {
    use crate::buffer::MoveBuffer;
    use crate::global::movegen;
    use chimera_core::{board::Board, piece::Piece, render::render, spin::Spins};

    fn print(b: Board, out: &MoveBuffer) {
        for (i, x) in out.iter().enumerate() {
            eprintln!("{i}: {x:?}");
            render(&b, Some(*x));
        }
    }

    #[test]
    fn d1_t() {
        let board = Board::EMPTY;
        let piece = Piece::T;
        let mut out = MoveBuffer::new();

        movegen(board, piece, Spins::T, &mut out);
        print(board, &out);
        assert_eq!(out.len(), 34);
    }

    #[test]
    fn d1_i() {
        let board = Board::EMPTY;
        let piece = Piece::I;
        let mut out = MoveBuffer::new();

        movegen(board, piece, Spins::T, &mut out);
        print(board, &out);
        assert_eq!(out.len(), 17);
    }

    #[test]
    fn d1_j() {
        let board = Board::EMPTY;
        let piece = Piece::J;
        let mut out = MoveBuffer::new();

        movegen(board, piece, Spins::T, &mut out);
        print(board, &out);
        assert_eq!(out.len(), 34);
    }

    #[test]
    fn d1_l() {
        let board = Board::EMPTY;
        let piece = Piece::L;
        let mut out = MoveBuffer::new();

        movegen(board, piece, Spins::T, &mut out);
        print(board, &out);
        assert_eq!(out.len(), 34);
    }

    #[test]
    fn d1_o() {
        let board = Board::EMPTY;
        let piece = Piece::O;
        let mut out = MoveBuffer::new();

        movegen(board, piece, Spins::T, &mut out);
        print(board, &out);
        assert_eq!(out.len(), 9);
    }

    #[test]
    fn d1_s() {
        let board = Board::EMPTY;
        let piece = Piece::S;
        let mut out = MoveBuffer::new();

        movegen(board, piece, Spins::T, &mut out);
        print(board, &out);
        assert_eq!(out.len(), 17);
    }

    #[test]
    fn d1_z() {
        let board = Board::EMPTY;
        let piece = Piece::Z;
        let mut out = MoveBuffer::new();

        movegen(board, piece, Spins::T, &mut out);
        print(board, &out);
        assert_eq!(out.len(), 17);
    }
}
