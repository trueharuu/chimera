use std::{
    hint::black_box,
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use chimera_core::{board::Board, collision_map::CollisionMap, piece::Piece, placement::Move, queue::Queue, render::{render, render_collision}, rotation::Rotation, spin::Spins};
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
    // {
    //     let mut board = Board::EMPTY;
    //     board.set_many(&[(0,0),(0,1),(1,1),(1,2),(0,2),(0,3),(1,3),(2,3),(1,0),(2,0),(2,1),(3,1),(4,1),(5,1),(6,0),(6,1),(7,0),(7,1),(7,2),(7,3),(8,0),(8,1),(8,2),(8,3),(9,0),(9,1),(9,2),(9,3)]);
    //     render(&board, None);
    //     let mask = board.filled_rows();
    //     println!("mask: {mask:b}");
    //     board.clear(mask);
    //     render(&board, None);
    // }
    let mut board = Board::EMPTY;

    // if on {
    //     // tki
        // board.set_many(&[
        //     (0, 0),
        //     (0, 1),
        //     (0, 2),
        //     (1, 0),
        //     (3, 0),
        //     (4, 0),
        //     (5, 0),
        //     (6, 0),
        //     (3, 2),
        //     (4, 1),
        //     (4, 2),
        //     (5, 1),
        //     (8, 0),
        //     (8, 1),
        //     (9, 0),
        //     (9, 1),
        //     (7, 0),
        //     (7, 1),
        //     (6, 1),
        //     (6, 2),
        //     (7, 2),
        //     (7, 3),
        //     (8, 2),
        //     (9, 2),
        // ]);
    // }

    let p = Piece::I;
    let start = Instant::now();
    let cm = CollisionMap::new(board, p);
    let elapsed = start.elapsed();
    println!("collision map generated in {elapsed:?}");
    for r in Rotation::ALL {
        render_collision(&board, &cm, r, p);
        for x in 0..10 {
            for y in 0..20 {
                let landed = cm.landed(x, y, r);
                if landed {
                    render(&board, Some(Move::new(x, y, r, p, chimera_core::spin::Spin::None)));
                }
            }
        }
    }

    // let depth = 6;
    // let queue = Queue::from_slice(&Piece::ALL);

    // let start = Instant::now();
    // let nodes = black_box(perft(board, queue, depth));
    // let elapsed = start.elapsed();
    // let _ = (nodes, elapsed);
    // // if on {
    // println!(
    //     "perft({depth}) = \x1b[34m{nodes}\x1b[0m in {elapsed:?} (\x1b[33m{}\x1b[0m nodes/s)",
    //     suffixize(nodes as f64 / elapsed.as_secs_f64())
    // );
    // }
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
