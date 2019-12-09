use anyhow::Result;

pub fn run_diagnostic(program: intcode::Memory) -> Result<intcode::Word, intcode::ExecutionError> {
    super::run_intcode_program_single_in_single_out(program, 1)
}

pub fn run_boost(program: intcode::Memory) -> Result<intcode::Word, intcode::ExecutionError> {
    super::run_intcode_program_single_in_single_out(program, 2)
}
