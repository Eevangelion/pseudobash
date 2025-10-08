use crate::{
    executor::execute::Execute, parser::program_builder::program::Program,
    program_output::ProgramOutput,
};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Pipeline {
    data: Vec<Program>,
}

impl Pipeline {
    pub fn push(&mut self, program: Program) {
        self.data.push(program);
    }

    pub fn is_empty(&mut self) -> bool {
        self.data.is_empty()
    }
}

impl Execute for Pipeline {
    fn execute(
        self,
        gs: &mut crate::global_state::GlobalState,
    ) -> anyhow::Result<crate::program_output::ProgramOutput> {
        let mut last_output = ProgramOutput::default();
        for mut program in self.data {
            match last_output.code {
                0 => {
                    program.swap_stdin(&mut last_output.stdout);
                    std::mem::swap(&mut last_output, &mut program.execute(gs)?);
                }
                _ => {
                    return Ok(last_output);
                }
            }
        }
        Ok(last_output)
    }
}
