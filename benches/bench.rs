
mod day07 {
    use criterion::{black_box, criterion_group, Criterion};
    use advent_of_code_2019::day07;

    const PROGRAM: &str = include_str!("../inputs/input-07");

    pub fn part_1(c: &mut Criterion) {
        let memory = intcode::Memory::from_str(PROGRAM).unwrap();

        c.bench_function("day07 part1", |b| b.iter(|| {
            day07::permute(black_box(&memory), black_box([0, 1, 2, 3, 4]), black_box(0))
        }));
    }

    pub fn part_2(c: &mut Criterion) {
        let memory = intcode::Memory::from_str(PROGRAM).unwrap();

        c.bench_function("day07 part2", |b| b.iter(|| {
            day07::permute(black_box(&memory), black_box([5, 6, 7, 8, 9]), black_box(0))
        }));
    }

    criterion_group!(solutions, part_1, part_2);
}

mod day09 {
    use criterion::{black_box, criterion_group, Criterion};
    use advent_of_code_2019::day09;

    const PROGRAM: &str = include_str!("../inputs/input-09");

    pub fn part_1(c: &mut Criterion) {
        let memory = intcode::Memory::from_str(PROGRAM).unwrap();

        c.bench_function("day09 part1", |b| b.iter(|| {
            let memory = memory.clone();
            day09::run_diagnostic(black_box(memory))
        }));
    }

    pub fn part_2(c: &mut Criterion) {
        let memory = intcode::Memory::from_str(PROGRAM).unwrap();

        c.bench_function("day09 part2", |b| b.iter(|| {
            let memory = memory.clone();
            day09::run_boost(black_box(memory))
        }));
    }
    criterion_group!(solutions, part_1, part_2);
}

criterion::criterion_main!(day07::solutions, day09::solutions);