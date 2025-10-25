use {
    crate::{global_state::environment::Environment, program_output::ProgramOutput},
    std::collections::HashMap,
};

pub struct Utils {
    utils: HashMap<String, fn(args: &Vec<String>, env: &mut Environment) -> ProgramOutput>,
}

impl Utils {
    pub fn try_exec(&self, program: &Vec<String>, env: &mut Environment) -> Option<ProgramOutput> {
        self.utils
            .get(&program[0])
            .and_then(|util| Some(util(program, env)))
    }
}

impl Default for Utils {
    fn default() -> Self {
        let mut utils = HashMap::default();
        utils.insert(
            "exit".to_string(),
            exit as fn(args: &Vec<String>, env: &mut Environment) -> ProgramOutput,
        );
        Self { utils }
    }
}

fn exit(_: &Vec<String>, _: &mut Environment) -> ProgramOutput {
    std::process::exit(0)
}