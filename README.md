# Advent of Code 2019 Solutions

Yep. I'm actually trying to do the Advent of Code this year. Inputs committed
are the inputs for my advent puzzles. Most of these are quick and dirty, so
aren't really fleshed out with tests (though I should get better about doing
that as the month progresses.)

## Execution

Generally, the completed puzzles provide the solution for parts 1 & 2. The day
to run can be specified with the `-d` or `--day` option. If none is specified,
then the latest day is run.

```bash
cargo run -d 12
```

Benchmarks are also provided and can be run with `cargo bench`. Tests can be
run, including validating the benchmarks, with:

```bash
cargo test --benches --all
```
