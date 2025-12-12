use aoc25::day12;
use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs::read_to_string, hint::black_box};

fn bench_day12(c: &mut Criterion) {
    let input = read_to_string("inputs/day12/input.txt").unwrap();

    c.bench_function("day12", |b| b.iter(|| day12::presents(black_box(&input))));
}

criterion_group!(benches, bench_day12);
criterion_main!(benches);
