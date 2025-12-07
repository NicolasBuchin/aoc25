use aoc25::day7;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

fn bench_day7(c: &mut Criterion) {
    let input = read_to_string("inputs/day7/input.txt").unwrap();

    c.bench_function("day7_part1", |b| b.iter(|| day7::tachyon_manifolds(&input)));

    c.bench_function("day7_part2", |b| {
        b.iter(|| day7::tachyon_manifolds2(&input))
    });
}

criterion_group!(benches, bench_day7);
criterion_main!(benches);
