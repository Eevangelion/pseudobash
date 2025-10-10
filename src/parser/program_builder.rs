pub mod program;

use crate::parser::{
    arg_builder::{ArgBuilderState, DefaultArgBuilder, arg::Arg},
    builder::Builder,
    context::Context,
    program_builder::program::Program,
};

pub type DefaultProgramBuilder = ProgramBuilder<DefaultArgBuilder>;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ProgramBuilder<T: Default + Builder<Arg, Context>> {
    current_program: Program,
    arg_builder: T,
}

impl<T: Default + Builder<Arg, Context>> Builder<Program, Context> for ProgramBuilder<T> {
    fn apply(&mut self, byte: u8, context: &mut Context) -> anyhow::Result<Option<Program>> {
        match byte {
            b'|' => match context.arg_builder_state {
                ArgBuilderState::Default => return self.finish(context),
                ArgBuilderState::WeakSep | ArgBuilderState::StrongSep => {}
            },
            _ => {}
        }

        self.arg_builder.apply(byte, context)?.map(|arg| {
            self.current_program.push(arg);
        });
        Ok(None)
    }

    fn finish(&mut self, context: &mut Context) -> anyhow::Result<Option<Program>> {
        self.arg_builder.finish(context)?.map(|arg| {
            self.current_program.push(arg);
        });
        Ok(self.return_if_not_empty())
    }
}

impl<T: Default + Builder<Arg, Context>> ProgramBuilder<T> {
    fn return_if_not_empty(&mut self) -> Option<Program> {
        if self.current_program.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut self.current_program))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        arg_builder::{ArgBuilder, arg::Arg},
        builder::Builder,
        context::Context,
        program_builder::{ProgramBuilder, program::Program},
        token::Token,
    };

    #[test]
    fn check_program_builder_apply() {
        let mut program_builder = ProgramBuilder::<ArgBuilder<Token>>::default();
        let mut context = Context::default();

        let mut result: Vec<Program> = "echo 100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new_default(vec![Token::new_default("echo")]),
                Arg::new_default(vec![Token::new_default("100")])
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "x=100 echo 100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new_var_setter(vec![Token::new_default("x=100")]),
                Arg::new_default(vec![Token::new_default("echo")]),
                Arg::new_default(vec![Token::new_default("100")])
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "echo 100 200"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new_default(vec![Token::new_default("echo")]),
                Arg::new_default(vec![Token::new_default("100")]),
                Arg::new_default(vec![Token::new_default("200")])
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "echo 100 200"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new_default(vec![Token::new_default("echo")]),
                Arg::new_default(vec![Token::new_default("100")]),
                Arg::new_default(vec![Token::new_default("200")])
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "echo $x"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new_default(vec![Token::new_default("echo")]),
                Arg::new_default(vec![Token::new_var_getter("x")]),
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "x=100 x=100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new_var_setter(vec![Token::new_default("x=100")]),
                Arg::new_var_setter(vec![Token::new_default("x=100")]),
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());
    }
}
