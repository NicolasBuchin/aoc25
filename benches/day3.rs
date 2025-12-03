use aoc25::day3;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

fn bench_day3(c: &mut Criterion) {
    let input = read_to_string("inputs/day3/input.txt").unwrap();

    c.bench_function("day3_part1", |b| b.iter(|| day3::batteries(&input)));

    c.bench_function("day3_part2", |b| b.iter(|| day3::batteries2(&input)));
}

criterion_group!(benches, bench_day3);
criterion_main!(benches);
