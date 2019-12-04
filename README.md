# Advent of Code 2019 Solutions

Yep. I'm actually trying to do the Advent of Code this year. Inputs committed
are the inputs for my advent puzzles. Most of these are quick and dirty, so
aren't really fleshed out with tests (though I should get better about doing
that as the month progresses.)

## Execution

I've tried to ensure that every puzzle solution will read from a file specified
on the command line. If no file is provided, then the program will default to
reading from STDIN.

Puzzle solutions can be built and run in one command.

Reading from a file:

```bash
cargo run --bin puzzle-03 -- inputs/input-03
```

Reading from STDIN:

```bash
echo "171309-643603" | cargo run --bin puzzle-04
```

Generally, the completed puzzles provide the solution for part 2. For some
puzzles, the solution for part 1 is still incorporated into the code and can
be accessed using the `part-1` feature flag:

```bash
cargo run --bin puzzle-03 --features part-1 -- inputs/input-03
```
