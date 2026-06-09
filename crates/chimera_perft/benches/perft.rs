use std::hint::black_box;

use chimera_core::{board::Board, piece::Piece, spin::Spins};
use chimera_nav::{buffer::MoveBuffer, global::movegen};
use criterion::{Criterion, criterion_group, criterion_main};
#[allow(clippy::unit_arg)]
fn perft_bench(c: &mut Criterion) {
    let board = Board::EMPTY;

    c.bench_function("perft_1_t", |b| {
        let mut out = MoveBuffer::new();
        b.iter(|| {
            out.clear();
            black_box(movegen(board, Piece::T, Spins::None, &mut out));
        })
    });

    c.bench_function("perft_1_i", |b| {
        let mut out = MoveBuffer::new();
        b.iter(|| {
            out.clear();
            black_box(movegen(board, Piece::I, Spins::None, &mut out));
        })
    });

    c.bench_function("perft_1_j", |b| {
        let mut out = MoveBuffer::new();
        b.iter(|| {
            out.clear();
            black_box(movegen(board, Piece::J, Spins::None, &mut out));
        })
    });

    c.bench_function("perft_1_l", |b| {
        let mut out = MoveBuffer::new();
        b.iter(|| {
            out.clear();
            black_box(movegen(board, Piece::L, Spins::None, &mut out));
        })
    });

    c.bench_function("perft_1_o", |b| {
        let mut out = MoveBuffer::new();
        b.iter(|| {
            out.clear();
            black_box(movegen(board, Piece::O, Spins::None, &mut out));
        })
    });

    c.bench_function("perft_1_s", |b| {
        let mut out = MoveBuffer::new();
        b.iter(|| {
            out.clear();
            black_box(movegen(board, Piece::S, Spins::None, &mut out));
        })
    });

    c.bench_function("perft_1_z", |b| {
        let mut out = MoveBuffer::new();
        b.iter(|| {
            out.clear();
            black_box(movegen(board, Piece::Z, Spins::None, &mut out));
        })
    });
}

criterion_group!(benches, perft_bench);
criterion_main!(benches);
