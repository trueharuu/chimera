use std::time::Instant;

use chimera_core::{
    board::Board,
    piece::Piece,
    queue::Queue,
};
use chimera_nav::{buffer::MoveBuffer, global::movegen};

pub fn perft(board: Board, queue: Queue, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut total = 0;

    let mut out = MoveBuffer::new();
    let piece = queue.get(0).unwrap();
    movegen(board, piece, &mut out);

    total += out.len();

    for i in out.iter() {
        let mut cpy = board;
        cpy.apply(*i);

        total += perft(cpy, queue.slice(1, queue.len()), depth - 1);
    }

    total
}
fn main() {
    let board = Board::EMPTY;
    let depth = 7;
    let queue = Queue::from_slice(&Piece::ALL);

    let i = Instant::now();
    let r = perft(board, queue, depth);
    let e = i.elapsed();

    println!("perft {depth} {queue} = {r} in {e:?}",)
}
