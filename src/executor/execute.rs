use crate::{global_state::GlobalState, program_output::ProgramOutput};

pub trait Execute {
    fn execute(self, gs: &mut GlobalState) -> anyhow::Result<ProgramOutput>;
}
