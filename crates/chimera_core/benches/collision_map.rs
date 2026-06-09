use chimera_core::{board::Board, collision_map::CollisionMap, piece::Piece};
use criterion::{Criterion, criterion_group, criterion_main};

fn collision_map_bench(c: &mut Criterion) {
    let board = Board::EMPTY;

    c.bench_function("collision_map_t", |b| {
        b.iter(|| CollisionMap::new(board, Piece::T))
    });
    c.bench_function("collision_map_i", |b| {
        b.iter(|| CollisionMap::new(board, Piece::I))
    });
    c.bench_function("collision_map_j", |b| {
        b.iter(|| CollisionMap::new(board, Piece::J))
    });
    c.bench_function("collision_map_l", |b| {
        b.iter(|| CollisionMap::new(board, Piece::L))
    });
    c.bench_function("collision_map_o", |b| {
        b.iter(|| CollisionMap::new(board, Piece::O))
    });
    c.bench_function("collision_map_s", |b| {
        b.iter(|| CollisionMap::new(board, Piece::S))
    });
    c.bench_function("collision_map_z", |b| {
        b.iter(|| CollisionMap::new(board, Piece::Z))
    });
}

criterion_group!(benches, collision_map_bench);
criterion_main!(benches);
