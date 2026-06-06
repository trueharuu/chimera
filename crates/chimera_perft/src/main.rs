use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use chimera_core::{board::Board, piece::Piece, queue::Queue};
use chimera_nav::{buffer::MoveBuffer, global::movegen};
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn perft(board: Board, queue: Queue, depth: usize) -> usize {
    if depth == 0 {
        return 0;
    }

    let total = AtomicUsize::new(0);

    let mut out = MoveBuffer::new();
    let piece = queue.get(0).unwrap();

    movegen(board, piece, &mut out);

    total.fetch_add(out.len(), Ordering::Relaxed);

    out.iter().par_bridge().for_each(|i| {
        let mut cpy = board;
        cpy.apply(*i);

        total.fetch_add(
            perft(cpy, queue.slice(1, queue.len()), depth - 1),
            Ordering::Relaxed,
        );
    });

    total.load(Ordering::Relaxed)
}
fn main() {
    let board = Board::EMPTY;

    let depth = 7;
    let queue = Queue::from_slice(&Piece::ALL);

    let i = Instant::now();
    let r = std::hint::black_box(perft(board, queue, depth));
    let e = i.elapsed();

    println!(
        "perft {depth} {queue} = {r} in {e:?} ({} nodes/s)",
        suffixize(r as f64 / e.as_secs_f64())
    );
}

fn suffixize(t: f64) -> String {
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
