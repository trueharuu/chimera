use chimera_core::{board::Board, piece::Piece};
use chimera_nav::collision_map::CollisionMap;
use criterion::{Criterion, criterion_group, criterion_main};
fn collision_map_bench(c: &mut Criterion) {
    let board = Board::EMPTY;

    c.bench_function("collision_map_t", |b| {
        b.iter(|| CollisionMap::new::<{ Piece::T as u8 }>(board))
    });
    c.bench_function("collision_map_i", |b| {
        b.iter(|| CollisionMap::new::<{ Piece::I as u8 }>(board))
    });
    c.bench_function("collision_map_j", |b| {
        b.iter(|| CollisionMap::new::<{ Piece::J as u8 }>(board))
    });
    c.bench_function("collision_map_l", |b| {
        b.iter(|| CollisionMap::new::<{ Piece::L as u8 }>(board))
    });
    c.bench_function("collision_map_o", |b| {
        b.iter(|| CollisionMap::new::<{ Piece::O as u8 }>(board))
    });
    c.bench_function("collision_map_s", |b| {
        b.iter(|| CollisionMap::new::<{ Piece::S as u8 }>(board))
    });
    c.bench_function("collision_map_z", |b| {
        b.iter(|| CollisionMap::new::<{ Piece::Z as u8 }>(board))
    });
}

criterion_group!(benches, collision_map_bench);
criterion_main!(benches);
