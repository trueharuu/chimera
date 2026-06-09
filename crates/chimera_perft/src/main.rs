use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Instant,
};

use chimera_core::{
    board::Board, collision_map::CollisionMap, piece::Piece, placement::Move, queue::Queue,
    render::{render, render_collision}, rotation::Rotation, spin::Spins,
};
use chimera_nav::{buffer::MoveBuffer, global::movegen};
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn perft(
    board: Board,
    queue: Queue,
    depth: usize,
    total: Arc<Mutex<Vec<(Board, Move)>>>,
) -> usize {
    if depth == 0 || queue.is_empty() {
        return 0;
    }

    let zz = AtomicUsize::new(0);

    let mut out = MoveBuffer::new();
    let piece = queue.get(0).unwrap();

    movegen(board, piece, Spins::None, &mut out);

    zz.fetch_add(out.len(), Ordering::Relaxed);

    out.iter().par_bridge().for_each(|i| {
        let mut cpy = board;

        total.lock().unwrap().push((cpy, *i));

        // render(&cpy, Some(*i));
        // println!("{i:?}");
        // println!("{:?}", t_spin_from_corners(board, i.x(), i.y(), i.rot(), false));

        cpy.apply(*i);
        zz.fetch_add(
            perft(cpy, queue.slice(1, queue.len()), depth - 1, total.clone()),
            Ordering::Relaxed,
        );
    });

    zz.load(Ordering::Relaxed)
}
fn main() {
    let mut board = Board::EMPTY;

    // board.set_many(
    //     &[
    //         (0, 0),
    //         (0, 1),
    //         (0, 2),
    //         (1, 0),
    //         (3, 0),
    //         (3, 2),
    //         (4, 0),
    //         (4, 1),
    //         (4, 2),
    //         (5, 0),
    //         (5, 1),
    //         (6, 0),
    //         (9, 0),
    //         (9, 1),
    //         (8, 1),
    //         (8, 2),
    //     ],
    //     true,
    // );

    let p = Piece::T;
    let r = Rotation::South;
    let i = Instant::now();
    let cm = CollisionMap::new(board, p);
    let e = i.elapsed();
    println!("generated in \x1b[34m{p:?}, {r:?}\x1b[0m in \x1b[32m{e:?}\x1b[0m");
    render(&board, None);
    render_collision(&board, &cm, r, p);
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
