mod day02 {
    use advent_of_code_2019::day02;
    use criterion::{black_box, criterion_group, Criterion};

    pub fn part_1(c: &mut Criterion) {
        let memory: intcode::Memory = day02::PUZZLE_INPUT.parse().unwrap();

        c.bench_function("day02::part_1", |b| {
            b.iter(|| {
                day02::run_with_specific_state(
                    black_box(memory.clone()),
                    black_box(12),
                    black_box(2),
                )
            })
        });
    }

    pub fn part_2(c: &mut Criterion) {
        let memory: intcode::Memory = day02::PUZZLE_INPUT.parse().unwrap();

        c.bench_function("day02::part_2", |b| {
            b.iter(|| day02::search_for_noun_and_verb(black_box(memory.clone()), black_box(0)))
        });
    }

    criterion_group!(solutions, part_1, part_2);
}

mod day05 {
    use advent_of_code_2019::day05;
    use criterion::{black_box, criterion_group, Criterion};

    pub fn part_1(c: &mut Criterion) {
        let memory: intcode::Memory = day05::PUZZLE_INPUT.parse().unwrap();

        c.bench_function("day05::part_1", |b| {
            b.iter(|| day05::run_system_1_diagnostic(black_box(memory.clone())))
        });
    }

    pub fn part_2(c: &mut Criterion) {
        let memory: intcode::Memory = day05::PUZZLE_INPUT.parse().unwrap();

        c.bench_function("day05::part_2", |b| {
            b.iter(|| day05::run_system_5_diagnostic(black_box(memory.clone())))
        });
    }

    criterion_group!(solutions, part_1, part_2);
}

mod day07 {
    use advent_of_code_2019::day07;
    use criterion::{black_box, criterion_group, Criterion};

    pub fn part_1(c: &mut Criterion) {
        let mut group = c.benchmark_group("day07::part_1");
        let memory = std::sync::Arc::new(day07::PUZZLE_INPUT.parse().unwrap());

        group.bench_function("sync", |b| {
            b.iter(|| day07::find_best_phase_sequence(black_box(&memory)))
        });

        let mut runtime = tokio::runtime::Builder::new()
            .basic_scheduler()
            .build()
            .expect("initialize runtime");
        group.bench_function("single-async", |b| {
            b.iter(|| {
                runtime.block_on(day07::find_best_phase_sequence_async(black_box(
                    std::sync::Arc::clone(&memory),
                )))
            })
        });

        #[cfg(feature = "threaded-async")]
        {
            let mut threaded_runtime = tokio::runtime::Builder::new()
                .threaded_scheduler()
                .build()
                .expect("initialize runtime");
            group.bench_function("threaded-async", |b| {
                b.iter(|| {
                    threaded_runtime.block_on(day07::find_best_phase_sequence_async(black_box(
                        std::sync::Arc::clone(&memory),
                    )))
                })
            });
        }
    }

    pub fn part_2(c: &mut Criterion) {
        let mut group = c.benchmark_group("day07::part_2");
        let memory = std::sync::Arc::new(day07::PUZZLE_INPUT.parse().unwrap());

        group.bench_function("sync", |b| {
            b.iter(|| day07::find_best_phase_sequence_with_feedback(black_box(&memory)))
        });

        let mut runtime = tokio::runtime::Builder::new()
            .basic_scheduler()
            .build()
            .expect("initialize runtime");
        group.bench_function("single-async", |b| {
            b.iter(|| {
                runtime.block_on(day07::find_best_phase_sequence_with_feedback_async(
                    black_box(std::sync::Arc::clone(&memory)),
                ))
            })
        });

        #[cfg(feature = "threaded-async")]
        {
            let mut threaded_runtime = tokio::runtime::Builder::new()
                .threaded_scheduler()
                .build()
                .expect("initialize runtime");
            group.bench_function("threaded-async", |b| {
                b.iter(|| {
                    threaded_runtime.block_on(day07::find_best_phase_sequence_with_feedback_async(
                        black_box(std::sync::Arc::clone(&memory)),
                    ))
                })
            });
        }
    }

    criterion_group!(solutions, part_1, part_2);
}

mod day09 {
    use advent_of_code_2019::day09;
    use criterion::{black_box, criterion_group, Criterion};

    pub fn part_1(c: &mut Criterion) {
        let memory: intcode::Memory = day09::PUZZLE_INPUT.parse().unwrap();

        c.bench_function("day09::part_1", |b| {
            b.iter(|| {
                let memory = memory.clone();
                day09::run_diagnostic(black_box(memory))
            })
        });
    }

    pub fn part_2(c: &mut Criterion) {
        let memory: intcode::Memory = day09::PUZZLE_INPUT.parse().unwrap();

        c.bench_function("day09::part_2", |b| {
            b.iter(|| {
                let memory = memory.clone();
                day09::run_boost(black_box(memory))
            })
        });
    }
    criterion_group!(solutions, part_1, part_2);
}

criterion::criterion_main!(
    day02::solutions,
    day05::solutions,
    day07::solutions,
    day09::solutions,
);
