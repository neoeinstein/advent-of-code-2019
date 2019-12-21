use advent_of_code_2019::*;
use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    day: Option<u8>,
    #[structopt(long)]
    all: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();

    if opt.all {
        for i in 1..=16 {
            run_day(i)?;
        }

        Ok(())
    } else {
        run_day(opt.day.unwrap_or_default())
    }
}

fn run_day(day: u8) -> Result<()> {
    match day {
        1 => day01::run(),
        2 => day02::run()?,
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run()?,
        6 => day06::run()?,
        7 => {
            //day07::run()?
            tokio::runtime::Runtime::new()?.block_on(day07::run_async())?
        }
        8 => day08::run()?,
        9 => day09::run()?,
        10 => day10::run()?,
        11 => day11::run()?,
        12 => day12::run()?,
        13 => day13::run()?,
        14 => day14::run()?,
        15 => day15::run()?,
        16 => day16::run()?,
        17 => day17::run()?,
        18 => day18::run()?,
        19 => day19::run()?,
        20 => day20::run()?,
        _ => day21::run()?,
    }

    Ok(())
}
