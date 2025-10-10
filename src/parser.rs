mod arg_builder;
mod builder;
mod context;
mod pipeline_builder;
mod program_builder;
mod token;

use {
    crate::{
        executor::execute::Execute,
        parser::{
            builder::Builder,
            context::Context,
            pipeline_builder::{DefaultPipelineBuilder, pipeline::Pipeline},
            program_builder::{DefaultProgramBuilder, program::Program},
        },
    },
    std::marker::PhantomData,
};

pub type DefaultExecutable = Pipeline;
pub type DefaultBuilder = DefaultPipelineBuilder;
pub type DefaultContext = Context;

pub type DefaultParser = CLIParser<DefaultContext, DefaultExecutable, DefaultBuilder>;
pub type _DefaultProgramParser = CLIParser<DefaultContext, Program, DefaultProgramBuilder>;

pub trait Parser<I: Execute>: Iterator<Item = anyhow::Result<I>> {
    fn set_input(&mut self, input: &mut Vec<u8>);
}

#[derive(Debug, Default, PartialEq)]
pub struct CLIParser<C: Default, E: Execute, B: Default + Builder<E, C>> {
    builder: B,
    context: C,

    input: Vec<u8>,
    current_index: usize,
    finished: bool,

    phantom_data: PhantomData<E>,
}

impl<C: Default, E: Execute, B: Default + Builder<E, C>> Parser<E> for CLIParser<C, E, B> {
    fn set_input(&mut self, input: &mut Vec<u8>) {
        std::mem::swap(&mut self.input, input);
        self.current_index = 0;
        self.finished = false
    }
}

impl<C: Default, E: Execute, B: Default + Builder<E, C>> Iterator for CLIParser<C, E, B> {
    type Item = anyhow::Result<E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_index == self.input.len() {
                match self.finished {
                    true => return None,
                    false => {
                        self.finished = false;
                        match self.builder.finish(&mut self.context) {
                            Ok(Some(executable)) => return Some(Ok(executable)),
                            Ok(None) => return None,
                            Err(e) => {
                                std::mem::take(&mut self.builder);
                                std::mem::take(&mut self.context);
                                return Some(Err(e));
                            }
                        }
                    }
                }
            }

            match self
                .builder
                .apply(self.input[self.current_index], &mut self.context)
            {
                Ok(Some(pipeline)) => {
                    self.current_index += 1;
                    return Some(Ok(pipeline));
                }
                Ok(None) => {}
                Err(e) => {
                    while self.current_index < self.input.len()
                        && self.input[self.current_index] != b';'
                    {
                        self.current_index += 1;
                    }
                    std::mem::take(&mut self.builder);
                    std::mem::take(&mut self.context);
                    return Some(Err(e));
                }
            }

            self.current_index += 1;
        }
    }
}
