use aoc25::day5;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

fn bench_day5(c: &mut Criterion) {
    let input = read_to_string("inputs/day5/input.txt").unwrap();

    c.bench_function("day5_part1", |b| b.iter(|| day5::spoiled(&input)));

    c.bench_function("day5_part2", |b| b.iter(|| day5::spoiled2(&input)));
}

criterion_group!(benches, bench_day5);
criterion_main!(benches);
