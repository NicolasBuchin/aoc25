use aoc25::day11;
use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs::read_to_string, hint::black_box};

fn bench_day11(c: &mut Criterion) {
    let input = read_to_string("inputs/day11/input.txt").unwrap();

    c.bench_function("day11_part1", |b| {
        b.iter(|| day11::reactor(black_box(&input)))
    });

    c.bench_function("day11_part2", |b| {
        b.iter(|| day11::reactor2(black_box(&input)))
    });
}

criterion_group!(benches, bench_day11);
criterion_main!(benches);
