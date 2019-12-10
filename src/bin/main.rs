use anyhow::Result;
use structopt::StructOpt;
use advent_of_code_2019::*;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    day: Option<u8>,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();

    match opt.day.unwrap_or_default() {
        1 => day01::run(),
        2 => day02::run()?,
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run()?,
        6 => day06::run()?,
        7 => day07::run()?,
        8 => day08::run()?,
        _ => day09::run()?,
    }

    Ok(())
}