//! # Day 7: Amplification Circuit
//!
//! Based on the navigational maps, you're going to need to send more power to
//! your ship's thrusters to reach Santa in time. To do this, you'll need to
//! configure a series of amplifiers already installed on the ship.
//!
//! There are five amplifiers connected in series; each one receives an input
//! signal and produces an output signal. They are connected such that the first
//! amplifier's output leads to the second amplifier's input, the second
//! amplifier's output leads to the third amplifier's input, and so on. The
//! first amplifier's input value is 0, and the last amplifier's output leads to
//! your ship's thrusters.
//!
//! ```text
//!     O-------O  O-------O  O-------O  O-------O  O-------O
//! 0 ->| Amp A |->| Amp B |->| Amp C |->| Amp D |->| Amp E |-> (to thrusters)
//!     O-------O  O-------O  O-------O  O-------O  O-------O
//! ```
//!
//! The Elves have sent you some Amplifier Controller Software (your puzzle
//! input), a program that should run on your existing Intcode computer. Each
//! amplifier will need to run a copy of the program.
//!
//! When a copy of the program starts running on an amplifier, it will first use
//! an input instruction to ask the amplifier for its current phase setting (an
//! integer from 0 to 4). Each phase setting is used exactly once, but the Elves
//! can't remember which amplifier needs which phase setting.
//!
//! The program will then call another input instruction to get the amplifier's
//! input signal, compute the correct output signal, and supply it back to the
//! amplifier with an output instruction. (If the amplifier has not yet received
//! an input signal, it waits until one arrives.)
//!
//! Your job is to find the largest output signal that can be sent to the
//! thrusters by trying every possible combination of phase settings on the
//! amplifiers. Make sure that memory is not shared or reused between copies of
//! the program.
//!
//! For example, suppose you want to try the phase setting sequence `3,1,2,4,0`,
//! which would mean setting amplifier A to phase setting 3, amplifier B to
//! setting 1, C to 2, D to 4, and E to 0. Then, you could determine the output
//! signal that gets sent from amplifier E to the thrusters with the following
//! steps:
//!
//! * Start the copy of the amplifier controller software that will run on
//!   amplifier A. At its first input instruction, provide it the amplifier's
//!   phase setting, 3. At its second input instruction, provide it the input
//!   signal, 0. After some calculations, it will use an output instruction to
//!   indicate the amplifier's output signal.
//! * Start the software for amplifier B. Provide it the phase setting (1) and
//!   then whatever output signal was produced from amplifier A. It will then
//!   produce a new output signal destined for amplifier C.
//! * Start the software for amplifier C, provide the phase setting (2) and the
//!   value from amplifier B, then collect its output signal.
//! * Run amplifier D's software, provide the phase setting (4) and input value,
//!   and collect its output signal.
//! * Run amplifier E's software, provide the phase setting (0) and input value,
//!   and collect its output signal.
//!
//! The final output signal from amplifier E would be sent to the thrusters.
//! However, this phase setting sequence may not have been the best one; another
//! sequence might have sent a higher signal to the thrusters.
//!
//! Here are some example programs:
//!
//! * Max thruster signal 43210 (from phase setting sequence `4,3,2,1,0`):
//!
//! ```text
//! 3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0
//! ```
//!
//! * Max thruster signal 54321 (from phase setting sequence `0,1,2,3,4`):
//!
//! ```text
//! 3,23,3,24,1002,24,10,24,1002,23,-1,23,
//! 101,5,23,23,1,24,23,23,4,23,99,0,0
//! ```
//!
//!     Max thruster signal 65210 (from phase setting sequence `1,0,4,3,2`):
//!
//! ```text
//! 3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
//! 1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
//! ```
//!
//! Try every combination of phase settings on the amplifiers. What is the
//! highest signal that can be sent to the thrusters?

use advent_of_code_2019::{get_input_reader, intcode};
use anyhow::Result;
use std::{sync::mpsc::channel, thread};

fn run_amplifier_sequence(
    program: &intcode::Program,
    phase_sequence: [intcode::ProgramValue; 5],
) -> Result<intcode::ProgramValue> {
    let mut amp_a = intcode::Executable::from(program.clone());
    let mut amp_b = intcode::Executable::from(program.clone());
    let mut amp_c = intcode::Executable::from(program.clone());
    let mut amp_d = intcode::Executable::from(program.clone());
    let mut amp_e = intcode::Executable::from(program.clone());

    let (a_in, init) = channel();
    let (b_in, a_out) = channel();
    let (c_in, b_out) = channel();
    let (d_in, c_out) = channel();
    let (e_in, d_out) = channel();

    amp_a.pipe_inputs_from(init);
    amp_a.pipe_outputs_to(b_in.clone());
    amp_b.pipe_inputs_from(a_out);
    amp_b.pipe_outputs_to(c_in.clone());
    amp_c.pipe_inputs_from(b_out);
    amp_c.pipe_outputs_to(d_in.clone());
    amp_d.pipe_inputs_from(c_out);
    amp_d.pipe_outputs_to(e_in.clone());
    amp_e.pipe_inputs_from(d_out);
    let drain = amp_e.drain();

    let exec_a = thread::spawn(move || amp_a.execute());
    let exec_b = thread::spawn(move || amp_b.execute());
    let exec_c = thread::spawn(move || amp_c.execute());
    let exec_d = thread::spawn(move || amp_d.execute());
    let exec_e = thread::spawn(move || amp_e.execute());

    a_in.send(phase_sequence[0])?;
    b_in.send(phase_sequence[1])?;
    c_in.send(phase_sequence[2])?;
    d_in.send(phase_sequence[3])?;
    e_in.send(phase_sequence[4])?;
    a_in.send(0)?;

    exec_a.join().expect("thread A panicked")?;
    exec_b.join().expect("thread B panicked")?;
    exec_c.join().expect("thread C panicked")?;
    exec_d.join().expect("thread D panicked")?;
    exec_e.join().expect("thread E panicked")?;

    Ok(drain.to_vec()[0])
}

fn permute(
    program: &intcode::Program,
    mut phase_sequence: [intcode::ProgramValue; 5],
    start: usize,
) -> Result<([intcode::ProgramValue; 5], intcode::ProgramValue)> {
    if start == 5 {
        let result = run_amplifier_sequence(program, phase_sequence)?;
        println!("With sequence {:?}, gives {}", phase_sequence, result);
        Ok((phase_sequence, result))
    } else {
        let mut best_sequence = phase_sequence;
        let mut max = 0;
        for i in start..=4 {
            phase_sequence.swap(i, start);
            let (best_seq, seq_max) = permute(program, phase_sequence, start + 1)?;
            if seq_max > max {
                best_sequence = best_seq;
                max = seq_max;
            }
        }
        Ok((best_sequence, max))
    }
}

fn main() -> Result<()> {
    let program = intcode::Program::from_buf_reader(&mut get_input_reader())?;

    let (best_sequence, max) = permute(&program, [0, 1, 2, 3, 4], 0)?;

    println!("Best sequence: {:?}, end value = {}", best_sequence, max);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run_amplifier_sequence;
    use advent_of_code_2019::intcode;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn example_1() -> Result<()> {
        const PROGRAM: &str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        const PHASES: &[intcode::ProgramValue; 5] = &[4, 3, 2, 1, 0];

        let program = intcode::Program::from_str(PROGRAM)?;

        let actual = run_amplifier_sequence(&program, *PHASES)?;
        const EXPECTED: intcode::ProgramValue = 43210;

        assert_eq!(EXPECTED, actual);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        const PROGRAM: &str = "\
                               3,23,3,24,1002,24,10,24,1002,23,-1,23,\
                               101,5,23,23,1,24,23,23,4,23,99,0,0";
        const PHASES: &[intcode::ProgramValue; 5] = &[0, 1, 2, 3, 4];

        let program = intcode::Program::from_str(PROGRAM)?;

        let actual = run_amplifier_sequence(&program, *PHASES)?;
        const EXPECTED: intcode::ProgramValue = 54321;

        assert_eq!(EXPECTED, actual);

        Ok(())
    }

    #[test]
    fn example_3() -> Result<()> {
        const PROGRAM: &str = "\
                               3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
                               1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        const PHASES: &[intcode::ProgramValue; 5] = &[1, 0, 4, 3, 2];

        let program = intcode::Program::from_str(PROGRAM)?;

        let actual = run_amplifier_sequence(&program, *PHASES)?;
        const EXPECTED: intcode::ProgramValue = 65210;

        assert_eq!(EXPECTED, actual);

        Ok(())
    }
}
