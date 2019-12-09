use anyhow::Result;

fn run_amplifier_sequence(
    memory: &intcode::Memory,
    phase_sequence: [intcode::Word; 5],
) -> Result<intcode::Word> {
    let mut amp_a = intcode::Executable::from(memory.clone());
    let mut amp_b = intcode::Executable::from(memory.clone());
    let mut amp_c = intcode::Executable::from(memory.clone());
    let mut amp_d = intcode::Executable::from(memory.clone());
    let mut amp_e = intcode::Executable::from(memory.clone());

    let buffer = amp_e.buffer_to(&mut amp_a);
    let a_in = buffer.injector();
    let b_in = amp_a.pipe_to(&mut amp_b);
    let c_in = amp_b.pipe_to(&mut amp_c);
    let d_in = amp_c.pipe_to(&mut amp_d);
    let e_in = amp_d.pipe_to(&mut amp_e);

    let exec_a = amp_a.execute_in_thread();
    let exec_b = amp_b.execute_in_thread();
    let exec_c = amp_c.execute_in_thread();
    let exec_d = amp_d.execute_in_thread();
    let exec_e = amp_e.execute_in_thread();
    let exec_buf = buffer.execute_in_thread();

    e_in.send(phase_sequence[4])?;
    d_in.send(phase_sequence[3])?;
    c_in.send(phase_sequence[2])?;
    b_in.send(phase_sequence[1])?;
    a_in.send(phase_sequence[0])?;
    a_in.send(0)?;

    exec_a.join().expect("thread A panicked")?;
    exec_b.join().expect("thread B panicked")?;
    exec_c.join().expect("thread C panicked")?;
    exec_d.join().expect("thread D panicked")?;
    exec_e.join().expect("thread E panicked")?;
    let result = exec_buf.join().expect("thread Buf panicked");

    Ok(result.expect("amplifier sequence did not produce a value"))
}

pub fn permute(
    memory: &intcode::Memory,
    mut phase_sequence: [intcode::Word; 5],
    start: usize,
) -> Result<([intcode::Word; 5], intcode::Word)> {
    if start == 5 {
        let result = run_amplifier_sequence(memory, phase_sequence)?;
        // println!("With sequence {:?}, gives {}", phase_sequence, result);
        Ok((phase_sequence, result))
    } else {
        let mut best_sequence = phase_sequence;
        let mut max = 0;
        for i in start..=4 {
            phase_sequence.swap(i, start);
            let (best_seq, seq_max) = permute(memory, phase_sequence, start + 1)?;
            if seq_max > max {
                best_sequence = best_seq;
                max = seq_max;
            }
        }
        Ok((best_sequence, max))
    }
}

#[cfg(test)]
mod tests {
    use super::run_amplifier_sequence;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part_1_example_1() -> Result<()> {
        init_logging();
        const PROGRAM: &str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        const PHASES: &[intcode::Word; 5] = &[4, 3, 2, 1, 0];

        let memory = intcode::Memory::from_str(PROGRAM)?;

        let actual = run_amplifier_sequence(&memory, *PHASES)?;
        const EXPECTED: intcode::Word = 43210;

        assert_eq!(EXPECTED, actual);

        Ok(())
    }

    #[test]
    fn part_1_example_2() -> Result<()> {
        init_logging();
        const PROGRAM: &str = "3,23,3,24,1002,24,10,24,1002,23,-1,23,\
                               101,5,23,23,1,24,23,23,4,23,99,0,0";
        const PHASES: &[intcode::Word; 5] = &[0, 1, 2, 3, 4];

        let memory = intcode::Memory::from_str(PROGRAM)?;

        let actual = run_amplifier_sequence(&memory, *PHASES)?;
        const EXPECTED: intcode::Word = 54321;

        assert_eq!(EXPECTED, actual);

        Ok(())
    }

    #[test]
    fn part_1_example_3() -> Result<()> {
        init_logging();
        const PROGRAM: &str = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
                               1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        const PHASES: &[intcode::Word; 5] = &[1, 0, 4, 3, 2];

        let memory = intcode::Memory::from_str(PROGRAM)?;

        let actual = run_amplifier_sequence(&memory, *PHASES)?;
        const EXPECTED: intcode::Word = 65210;

        assert_eq!(EXPECTED, actual);

        Ok(())
    }

    #[test]
    fn part_2_example_1() -> Result<()> {
        init_logging();
        const PROGRAM: &str = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
                               27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        const PHASES: [intcode::Word; 5] = [9, 8, 7, 6, 5];

        let memory = intcode::Memory::from_str(PROGRAM)?;

        let actual = run_amplifier_sequence(&memory, PHASES)?;
        const EXPECTED: intcode::Word = 139629729;

        assert_eq!(EXPECTED, actual);

        Ok(())
    }

    #[test]
    fn part_2_example_2() -> Result<()> {
        init_logging();
        const PROGRAM: &str = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
                               -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
                               53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        const PHASES: [intcode::Word; 5] = [9, 7, 8, 5, 6];

        let memory = intcode::Memory::from_str(PROGRAM)?;

        let actual = run_amplifier_sequence(&memory, PHASES)?;
        const EXPECTED: intcode::Word = 18216;

        assert_eq!(EXPECTED, actual);

        Ok(())
    }
}
