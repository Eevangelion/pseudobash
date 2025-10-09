use {
    crate::{
        executor::{Executor, execute::Execute},
        global_state::GlobalState,
        listener::Listener,
        parser::{DefaultExecutable, DefaultParser, Parser},
        program_output::ProgramOutput,
    },
    std::{io::Write, marker::PhantomData},
};

pub type DefaultCLI = CLI<DefaultExecutable, DefaultParser>;

#[derive(Default)]
pub struct CLI<E: Execute, P: Default + Parser<E>> {
    listener: Listener,
    parser: P,
    executor: Executor,
    global_state: GlobalState,

    phantom_data: PhantomData<E>,
}

impl<E: Execute, P: Default + Parser<E>> CLI<E, P> {
    pub fn start(&mut self) {
        loop {
            print!("{} ", self.global_state.settings.get_invitation_input());
            std::io::stdout().flush().unwrap();

            self.parser
                .set_input(&mut self.listener.listen().into_bytes());

            for pipeline in &mut self.parser {
                match pipeline {
                    Ok(executable) => Self::print_output(
                        self.executor.execute(executable, &mut self.global_state),
                    ),
                    Err(e) => eprintln!("Parser error: {}", e),
                }
            }
        }
    }

    fn print_output(output: anyhow::Result<ProgramOutput>) {
        match output {
            Ok(program_output) => match program_output.code {
                0 => print!("{}", program_output),
                _ => eprintln!("{}", program_output),
            },
            Err(e) => eprintln!("Executing error: {}", e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        cli::{CLI, DefaultCLI},
        parser::Parser,
        program_output::ProgramOutput,
    };

    #[test]
    fn check_var_setter() {
        let mut cli: DefaultCLI = CLI::default();

        cli.parser.set_input(&mut b"  qwe=1278\n".to_vec());
        let output: Vec<ProgramOutput> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
                    .unwrap()
            })
            .collect();
        let mut var = "qwe".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, "1278".as_bytes().to_vec());
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        cli.parser.set_input(&mut b"  qwe==10\n".to_vec());
        let output: Vec<ProgramOutput> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
                    .unwrap()
            })
            .collect();
        let mut var = "qwe".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, "=10".as_bytes().to_vec());
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        cli.parser.set_input(&mut b"  qwe=qwe\n".to_vec());
        let output: Vec<ProgramOutput> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
                    .unwrap()
            })
            .collect();
        let mut var = "qwe".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, "qwe".as_bytes().to_vec());
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        cli.parser.set_input(&mut b"  qwe=\n".to_vec());
        let output: Vec<ProgramOutput> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
                    .unwrap()
            })
            .collect();
        let mut var = "qwe".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, "".as_bytes().to_vec());
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        cli.parser.set_input(&mut b"  qwe='10$10'\n".to_vec());
        let output: Vec<ProgramOutput> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
                    .unwrap()
            })
            .collect();
        let mut var = "qwe".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, "10$10".as_bytes().to_vec());
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        cli.parser.set_input(&mut b"x=$PWD\n".to_vec());
        let output: Vec<ProgramOutput> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
                    .unwrap()
            })
            .collect();
        let mut x = "x".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut x);
        let mut var = "PWD".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, x);
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        cli.parser.set_input(&mut b"x=$PWD:9\n".to_vec());
        let output: Vec<ProgramOutput> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
                    .unwrap()
            })
            .collect();
        let mut x = "x".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut x);
        let mut var = "PWD".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        var.push(b':');
        var.push(b'9');
        assert_eq!(var, x);
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);
    }

    #[test]
    fn check_var_getter() {
        let mut cli: DefaultCLI = CLI::default();

        cli.parser.set_input(&mut b" echo $PWD\n".to_vec());
        let output: Vec<ProgramOutput> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
                    .unwrap()
            })
            .collect();
        let mut var = "PWD".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        var.push(b'\n');
        assert_eq!(output, vec![ProgramOutput::new(0, var, vec![])]);

        cli.parser.set_input(&mut b" echo $PWD $PWD\n".to_vec());
        let output: Vec<ProgramOutput> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
                    .unwrap()
            })
            .collect();
        let mut var = "PWD".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        var.push(b' ');
        var.append(&mut var.clone());
        var.last_mut().map(|byte| *byte = b'\n');
        assert_eq!(output, vec![ProgramOutput::new(0, var, vec![])]);

        cli.parser.set_input(&mut b" echo $PWD$PWD\n".to_vec());
        let output: Vec<ProgramOutput> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
                    .unwrap()
            })
            .collect();
        let mut var = "PWD".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        var.append(&mut var.clone());
        var.push(b'\n');
        assert_eq!(output, vec![ProgramOutput::new(0, var, vec![])]);

        cli.parser.set_input(&mut b" echo $PWDPWD\n".to_vec());
        let output: Vec<ProgramOutput> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
                    .unwrap()
            })
            .collect();
        assert_eq!(
            output,
            vec![ProgramOutput::new(0, "\n".as_bytes().to_vec(), vec![])]
        );
    }

    #[test]
    fn check_error() {
        let mut cli: DefaultCLI = CLI::default();

        cli.parser.set_input(&mut b"  '1'\n".to_vec());
        let output: Vec<anyhow::Result<ProgramOutput>> = (&mut cli.parser)
            .into_iter()
            .map(|pipeline| {
                cli.executor
                    .execute(pipeline.unwrap(), &mut cli.global_state)
            })
            .collect();
        assert!(output.iter().all(|res| res.is_err()));
    }
}
