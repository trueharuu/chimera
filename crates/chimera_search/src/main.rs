use chimera_core::{board::Board, piece::Piece, render::render, spin::Spins};
use chimera_nav::{buffer::MoveBuffer, global::movegen};
use chimera_search::score::{ScoreConfig, ScoringEvent};

pub fn main() {
    let mut board = Board::EMPTY;

    board.set_many(&[
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
        (9, 0),
        (0, 1),
        (1, 1),
        (4, 1),
        (5, 1),
        (7, 1),
        (8, 1),
        (9, 1),
        (0, 2),
        (8, 2),
        (9, 2),
        (0, 3),
        (7, 3),
        (8, 3),
        (9, 3),
    ]);

    let mut buf = MoveBuffer::new();

    movegen(board, Piece::I, Spins::T, &mut buf);

    buf.retain(|x| x.cells().iter().all(|&(_, y)| y < 4));

    for m in buf.as_slice() {
        let cfg = ScoreConfig::blitz();
        let score = ScoringEvent {
            board,
            placement: *m,
            b2b: 0,
            combo: 0,
            config: cfg,
            level: 1,
        };
        render(&board, Some(*m));
        println!("{:?} scores {}", m, score.score());
    }
}
