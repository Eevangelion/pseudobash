use {
    crate::{
        executor::execute::Execute, global_state::GlobalState, parser::arg_builder::arg::Arg,
        program_output::ProgramOutput,
    },
    std::{io::Write, process::Stdio},
};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Program {
    stdin: Vec<u8>,
    args: Vec<Arg>,
}

impl Program {
    pub fn push(&mut self, arg: Arg) {
        self.args.push(arg);
    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn swap_stdin(&mut self, stdin: &mut Vec<u8>) {
        std::mem::swap(&mut self.stdin, stdin);
    }

    fn prepare(self, gs: &mut GlobalState) -> Vec<String> {
        self.args
            .into_iter()
            .map(|arg| arg.into_string_with_executing(gs))
            .filter(|arg| arg.len() > 0)
            .collect()
    }
}

impl Execute for Program {
    fn execute(mut self, gs: &mut GlobalState) -> anyhow::Result<ProgramOutput> {
        let stdin = std::mem::take(&mut self.stdin);
        let prep_program = self.prepare(gs);
        if prep_program.len() == 0 {
            return Ok(ProgramOutput::default());
        }

        match gs.utils.try_exec(&prep_program, &mut gs.environment) {
            Some(output) => return Ok(output),
            None => {}
        }

        let mut command = std::process::Command::new(&prep_program[0]);
        for (idx, arg) in prep_program.iter().enumerate() {
            if idx > 0 {
                command.arg(arg);
            }
        }

        command.env_clear();
        for (k, v) in gs.environment.vars() {
            command.env(k, v);
        }

        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut proc = command.spawn().map_err(|e| {
            anyhow::Error::msg(format!("{}: '{}'", e.to_string(), prep_program.join(" ")))
        })?;
        proc.stdin
            .as_mut()
            .ok_or(anyhow::Error::msg("Failed to get stdin"))?
            .write_all(&stdin)?;

        Ok(proc
            .wait_with_output()
            .map_err(|e| {
                anyhow::Error::msg(format!("{}: '{}'", e.to_string(), prep_program.join(" ")))
            })?
            .into())
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{arg_builder::arg::Arg, program_builder::program::Program};

    impl Program {
        pub fn new(args: Vec<Arg>) -> Self {
            Self {
                args,
                stdin: Default::default(),
            }
        }
    }
}
