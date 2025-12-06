use aoc25::day6;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

fn bench_day6(c: &mut Criterion) {
    let input = read_to_string("inputs/day6/input.txt").unwrap();

    c.bench_function("day6_part1", |b| b.iter(|| day6::cephalopod_math(&input)));

    c.bench_function("day6_part2", |b| b.iter(|| day6::cephalopod_math2(&input)));
}

criterion_group!(benches, bench_day6);
criterion_main!(benches);
