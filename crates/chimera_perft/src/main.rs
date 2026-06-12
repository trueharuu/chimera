use std::{
    hint::black_box,
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use chimera_core::{board::Board, piece::Piece, queue::Queue, spin::Spins};
use chimera_nav::{buffer::MoveBuffer, global::movegen};
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn perft(board: Board, queue: Queue, depth: usize) -> usize {
    if depth == 0 || queue.is_empty() {
        return 0;
    }

    let zz = AtomicUsize::new(0);

    let mut out = MoveBuffer::new();
    let piece = queue.get(0).unwrap();

    movegen(board, piece, Spins::T, &mut out);

    zz.fetch_add(out.len(), Ordering::Relaxed);

    out.iter().par_bridge().for_each(|i| {
        let mut cpy = board;

        // if on {
        // render(&cpy, Some(*i));
        // println!("{i:?}");
        // }

        cpy.apply(*i);
        let mask = cpy.filled_rows();
        cpy.clear(mask);
        zz.fetch_add(
            perft(cpy, queue.slice(1, queue.len()), depth - 1),
            Ordering::Relaxed,
        );
    });

    zz.load(Ordering::Relaxed)
}
fn main() {
    let board = Board::EMPTY;

    let depth = 7;
    let queue = Queue::from_slice(&Piece::ALL);

    let start = Instant::now();
    let nodes = black_box(perft(board, queue, depth));
    let elapsed = start.elapsed();

    println!(
        "perft({depth}) = \x1b[34m{nodes}\x1b[0m in {elapsed:?} (\x1b[33m{}\x1b[0m nodes/s)",
        suffixize(nodes as f64 / elapsed.as_secs_f64())
    );
}

pub fn suffixize(t: f64) -> String {
    if t > 1_000_000_000.0 {
        format!("{:.1}B", t / 1_000_000_000.0)
    } else if t > 1_000_000.0 {
        format!("{:.1}M", t / 1_000_000.0)
    } else if t > 1_000.0 {
        format!("{:.1}K", t / 1_000.0)
    } else {
        t.to_string()
    }
}
