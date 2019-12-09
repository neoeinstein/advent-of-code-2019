use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_of_code_2019::day07;

pub fn part_1_solution(c: &mut Criterion) {
    let memory = intcode::Memory::from_str(include_str!("../inputs/input-07")).unwrap();

    c.bench_function("day07 part1", |b| b.iter(|| {
        day07::permute(black_box(&memory), black_box([0, 1, 2, 3, 4]), black_box(0))
    }));
}

pub fn part_2_solution(c: &mut Criterion) {
    let memory = intcode::Memory::from_str(include_str!("../inputs/input-07")).unwrap();

    c.bench_function("day07 part2", |b| b.iter(|| {
        day07::permute(black_box(&memory), black_box([5, 6, 7, 8, 9]), black_box(0))
    }));
}

criterion_group!(day07, part_1_solution, part_2_solution);
criterion_main!(day07);