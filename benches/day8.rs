use aoc25::day8;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

fn bench_day8(c: &mut Criterion) {
    let input = read_to_string("inputs/day8/input.txt").unwrap();

    c.bench_function("day8_part1", |b| {
        b.iter(|| day8::join_circuits(&input, 1000))
    });

    c.bench_function("day8_part2", |b| b.iter(|| day8::join_circuits2(&input)));
}

criterion_group!(benches, bench_day8);
criterion_main!(benches);
