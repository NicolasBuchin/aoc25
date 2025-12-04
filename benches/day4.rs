use aoc25::day4;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

fn bench_day4(c: &mut Criterion) {
    let input = read_to_string("inputs/day4/input.txt").unwrap();

    c.bench_function("day4_part1", |b| b.iter(|| day4::forklift(&input)));

    c.bench_function("day4_part2", |b| b.iter(|| day4::forklift2(&input)));
}

criterion_group!(benches, bench_day4);
criterion_main!(benches);
