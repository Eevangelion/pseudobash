pub mod execute;

use crate::{executor::execute::Execute, global_state::GlobalState, program_output::ProgramOutput};

#[derive(Default)]
pub struct Executor {}

impl Executor {
    pub fn execute<T: Execute>(
        &self,
        executable: T,
        gs: &mut GlobalState,
    ) -> anyhow::Result<ProgramOutput> {
        executable.execute(gs)
    }
}
