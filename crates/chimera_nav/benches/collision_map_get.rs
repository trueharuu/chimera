use std::hint::black_box;

use chimera_core::{board::Board, piece::Piece, rotation::Rotation};
use chimera_nav::collision_map::CollisionMap;
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_get_landed(c: &mut Criterion) {
    let board = Board::EMPTY;
    let cm_t = CollisionMap::new::<{ Piece::T as u8 }>(board);
    let cm_i = CollisionMap::new::<{ Piece::I as u8 }>(board);

    let mut group = c.benchmark_group("collision_map_get");

    group.bench_function("T_get_3_2_N", |b| {
        b.iter(|| black_box(cm_t.collides(3, 2, Rotation::North)));
    });
    group.bench_function("T_landed_3_2_N", |b| {
        b.iter(|| black_box(cm_t.landed(3, 2, Rotation::North)));
    });

    group.bench_function("I_get_4_1_E", |b| {
        b.iter(|| black_box(cm_i.collides(4, 1, Rotation::East)));
    });
    group.bench_function("I_landed_4_1_E", |b| {
        b.iter(|| black_box(cm_i.landed(4, 1, Rotation::East)));
    });

    group.finish();
}

criterion_group!(benches, bench_get_landed);
criterion_main!(benches);
