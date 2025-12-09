use aoc25::day9;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

fn bench_day9(c: &mut Criterion) {
    let input = read_to_string("inputs/day9/input.txt").unwrap();

    c.bench_function("day9_part1", |b| b.iter(|| day9::red_rectangle(&input)));

    c.bench_function("day9_part2", |b| b.iter(|| day9::red_rectangle2(&input)));
}

criterion_group!(benches, bench_day9);
criterion_main!(benches);
