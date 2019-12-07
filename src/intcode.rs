//! Intcode emulation facilities
//!
//! ## Example
//!
//! ```
//! use advent_of_code_2019::intcode::{Address, Executable, Program};
//!
//! const PROGRAM_DATA: &str = "1,1,1,4,99,5,6,0,99";
//! let program = Program::from_str(PROGRAM_DATA).expect("valid program");
//!
//! let mut exe = Executable::from(program);
//!
//! exe.execute().expect("successful execution");
//!
//! assert_eq!(Some(30), exe.memory().try_read(Address::new(0)));
//! ```

mod address;
mod decoder;
mod executor;
mod program;

pub use address::Address;
pub use executor::{Executable, ExecutionError};
pub use program::Program;

/// The quantum of data in program memory
pub type ProgramValue = i32;

#[cfg(test)]
mod tests {
    use super::{Executable, Program, ProgramValue};
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    const PUZ_2_EXAMPLE_1: &str = "1,9,10,3,2,3,11,0,99,30,40,50";
    const PUZ_2_EXAMPLE_1_EXPECTED: &str = "3500,9,10,70,2,3,11,0,99,30,40,50";
    const PUZ_2_EXAMPLE_2: &str = "1,0,0,0,99";
    const PUZ_2_EXAMPLE_2_EXPECTED: &str = "2,0,0,0,99";
    const PUZ_2_EXAMPLE_3: &str = "2,3,0,3,99";
    const PUZ_2_EXAMPLE_3_EXPECTED: &str = "2,3,0,6,99";
    const PUZ_2_EXAMPLE_4: &str = "2,4,4,5,99,0";
    const PUZ_2_EXAMPLE_4_EXPECTED: &str = "2,4,4,5,99,9801";
    const PUZ_2_EXAMPLE_5: &str = "1,1,1,4,99,5,6,0,99";
    const PUZ_2_EXAMPLE_5_EXPECTED: &str = "30,1,1,4,2,5,6,0,99";

    #[test]
    fn add_mul_halt_example_1() -> Result<()> {
        verify_final_state_test(PUZ_2_EXAMPLE_1, PUZ_2_EXAMPLE_1_EXPECTED)
    }

    #[test]
    fn add_mul_halt_example_2() -> Result<()> {
        verify_final_state_test(PUZ_2_EXAMPLE_2, PUZ_2_EXAMPLE_2_EXPECTED)
    }

    #[test]
    fn add_mul_halt_example_3() -> Result<()> {
        verify_final_state_test(PUZ_2_EXAMPLE_3, PUZ_2_EXAMPLE_3_EXPECTED)
    }

    #[test]
    fn add_mul_halt_example_4() -> Result<()> {
        verify_final_state_test(PUZ_2_EXAMPLE_4, PUZ_2_EXAMPLE_4_EXPECTED)
    }

    #[test]
    fn add_mul_halt_example_5() -> Result<()> {
        verify_final_state_test(PUZ_2_EXAMPLE_5, PUZ_2_EXAMPLE_5_EXPECTED)
    }

    fn verify_final_state_test(initial: &str, expected: &str) -> Result<()> {
        let program = Program::from_str(initial)?;
        let expected = Program::from_str(expected)?;

        let mut exe = Executable::from(program);

        exe.execute()?;

        assert_eq!(&expected, exe.memory());

        Ok(())
    }

    const POS_IS_INPUT_EQUAL_TO_8: &str = "3,9,8,9,10,9,4,9,99,-1,8";
    const POS_IS_INPUT_LESS_THAN_8: &str = "3,9,7,9,10,9,4,9,99,-1,8";
    const IMM_IS_INPUT_EQUAL_TO_8: &str = "3,3,1108,-1,8,3,4,3,99";
    const IMM_IS_INPUT_LESS_THAN_8: &str = "3,3,1107,-1,8,3,4,3,99";

    const TRUE: &[ProgramValue] = &[1];
    const ONE: &[ProgramValue] = &[1];
    const FALSE: &[ProgramValue] = &[0];
    const ZERO: &[ProgramValue] = &[0];

    #[test]
    fn position_mode_1_is_equal_to_8() -> Result<()> {
        let input = 1;

        run_program_test(POS_IS_INPUT_EQUAL_TO_8, input, FALSE)
    }

    #[test]
    fn position_mode_8_is_equal_to_8() -> Result<()> {
        let input = 8;

        run_program_test(POS_IS_INPUT_EQUAL_TO_8, input, TRUE)
    }

    #[test]
    fn position_mode_1_is_less_than_8() -> Result<()> {
        let input = 1;

        run_program_test(POS_IS_INPUT_LESS_THAN_8, input, TRUE)
    }

    #[test]
    fn position_mode_8_is_less_than_8() -> Result<()> {
        let input = 8;

        run_program_test(POS_IS_INPUT_LESS_THAN_8, input, FALSE)
    }

    #[test]
    fn immediate_mode_1_is_equal_to_8() -> Result<()> {
        let input = 1;

        run_program_test(IMM_IS_INPUT_EQUAL_TO_8, input, FALSE)
    }

    #[test]
    fn immediate_mode_8_is_equal_to_8() -> Result<()> {
        let input = 8;

        run_program_test(IMM_IS_INPUT_EQUAL_TO_8, input, TRUE)
    }

    #[test]
    fn immediate_mode_1_is_less_than_8() -> Result<()> {
        let input = 1;

        run_program_test(IMM_IS_INPUT_LESS_THAN_8, input, TRUE)
    }

    #[test]
    fn immediate_mode_8_is_less_than_8() -> Result<()> {
        let input = 8;

        run_program_test(IMM_IS_INPUT_LESS_THAN_8, input, FALSE)
    }

    const POS_JUMP_INPUT_WAS_ZERO: &str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    const IMM_JUMP_INPUT_WAS_ZERO: &str = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";

    #[test]
    fn position_mode_jump_if_0_is_0() -> Result<()> {
        let input = 0;

        run_program_test(POS_JUMP_INPUT_WAS_ZERO, input, ZERO)
    }

    #[test]
    fn position_mode_jump_if_1_is_0() -> Result<()> {
        let input = 1;

        run_program_test(POS_JUMP_INPUT_WAS_ZERO, input, ONE)
    }

    #[test]
    fn immediate_mode_jump_if_0_is_0() -> Result<()> {
        let input = 0;

        run_program_test(IMM_JUMP_INPUT_WAS_ZERO, input, ZERO)
    }

    #[test]
    fn immediate_mode_jump_if_1_is_0() -> Result<()> {
        let input = 1;

        run_program_test(IMM_JUMP_INPUT_WAS_ZERO, input, ONE)
    }

    const PUZ_5_PART_2_EXAMPLE: &str = "
        3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

    #[test]
    fn compare_neg2_against_8() -> Result<()> {
        let input = -2;

        run_program_test(PUZ_5_PART_2_EXAMPLE, input, &[999])
    }

    #[test]
    fn compare_8_against_8() -> Result<()> {
        let input = 8;

        run_program_test(PUZ_5_PART_2_EXAMPLE, input, &[1000])
    }

    #[test]
    fn compare_99_against_8() -> Result<()> {
        let input = 99;

        run_program_test(PUZ_5_PART_2_EXAMPLE, input, &[1001])
    }

    #[test]
    fn run_system_1_diagnostics() -> Result<()> {
        let outputs = run_diagnostics(1)?;

        assert!(outputs.iter().rev().skip(1).copied().all(|i| i == 0i32));

        Ok(())
    }

    #[test]
    fn run_system_5_diagnostics() -> Result<()> {
        let outputs = run_diagnostics(5)?;

        assert_eq!(1, outputs.len());

        Ok(())
    }

    fn run_diagnostics(system: ProgramValue) -> Result<Vec<ProgramValue>> {
        const PROGRAM: &str = include_str!("../inputs/input-05");

        let program = Program::from_str(PROGRAM)?;

        let mut exe = Executable::from(program);

        exe.single_input(system);
        let drain = exe.drain();

        exe.execute()?;

        let outputs = drain.to_vec();

        println!(
            "system {} diagnostic code = {}",
            system,
            outputs[outputs.len() - 1]
        );

        Ok(outputs)
    }

    fn run_program_test(
        program_data: &str,
        input: ProgramValue,
        expected: &[ProgramValue],
    ) -> Result<()> {
        let program = Program::from_str(program_data)?;

        let mut exe = Executable::from(program);

        exe.single_input(input);
        let drain = exe.drain();

        exe.execute()?;

        assert_eq!(expected, &drain.to_vec()[..]);

        Ok(())
    }
}
