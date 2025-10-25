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
        utils.insert(
            "cd".to_string(),
            cd as fn(args: &Vec<String>, env: &mut Environment) -> ProgramOutput,
        );
        Self { utils }
    }
}

fn exit(_: &Vec<String>, _: &mut Environment) -> ProgramOutput {
    std::process::exit(0)
}

fn cd(args: &Vec<String>, env: &mut Environment) -> ProgramOutput {
    let target_dir = if args.len() < 2 {
        std::env::var("HOME").unwrap_or_else(|_| {
            eprintln!("cd: HOME not set");
            return String::new();
        })
    } else {
        args[1].clone()
    };

    if let Err(e) = std::env::set_current_dir(&target_dir) {
        eprintln!("cd: {}: {}", e, target_dir);
        return ProgramOutput::new(1, vec![], vec![]);
    }

    ProgramOutput::new(0, vec![], vec![])
}