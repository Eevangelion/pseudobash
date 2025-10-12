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
    let new_path = match args.len() {
        1 => match std::env::home_dir() {
            Some(path) => path,
            None => {
                return ProgramOutput::new(
                    -1,
                    vec![],
                    "Failed to get home dir".as_bytes().to_vec(),
                );
            }
        },
        2 => match std::env::current_dir().and_then(|path| path.join(&args[1]).canonicalize()) {
            Ok(npath) => npath,
            Err(e) => {
                return ProgramOutput::new(-1, vec![], format!("{}", e).as_bytes().to_vec());
            }
        },
        _ => {
            return ProgramOutput::new(
                -1,
                vec![],
                format!("Unexpected number of arguments in: '{}'", args.join(" "))
                    .as_bytes()
                    .to_vec(),
            );
        }
    };
    match std::env::set_current_dir(&new_path) {
        Ok(_) => {}
        Err(e) => {
            return ProgramOutput::new(-1, vec![], format!("{}", e).as_bytes().to_vec());
        }
    }
    let mut pwd = "PWD=".as_bytes().to_vec();
    pwd.append(&mut new_path.to_string_lossy().as_bytes().to_vec());
    env.set_var(pwd);

    ProgramOutput::default()
}
