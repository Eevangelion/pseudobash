use std::marker::PhantomData;

use crate::{
    executor::execute::Execute,
    parser::{builder::Builder, context::Context},
};

pub mod arg_builder;
pub mod pipeline_builder;
pub mod program_builder;
pub mod token;

mod builder;
mod context;

pub trait Parser<I: Execute>: Iterator<Item = anyhow::Result<I>> {
    fn set_input(&mut self, input: &mut Vec<u8>);
}

#[derive(Debug, Default, PartialEq)]
pub struct CLIParser<I: Execute, T: Default + Builder<I>> {
    pipeline_builder: T,
    input: Vec<u8>,
    current_index: usize,
    finished: bool,
    context: Context,

    phantom_data: PhantomData<I>,
}

impl<I: Execute, T: Default + Builder<I>> Parser<I> for CLIParser<I, T> {
    fn set_input(&mut self, input: &mut Vec<u8>) {
        std::mem::swap(&mut self.input, input);
        self.current_index = 0;
        self.finished = false
    }
}

impl<I: Execute, T: Default + Builder<I>> Iterator for CLIParser<I, T> {
    type Item = anyhow::Result<I>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_index == self.input.len() {
                match self.finished {
                    true => return None,
                    false => {
                        self.finished = false;
                        match self.pipeline_builder.finish(&mut self.context) {
                            Ok(Some(pipeline)) => return Some(Ok(pipeline)),
                            Ok(None) => return None,
                            Err(e) => {
                                std::mem::take(&mut self.pipeline_builder);
                                std::mem::take(&mut self.context);
                                return Some(Err(e));
                            }
                        }
                    }
                }
            }

            match self
                .pipeline_builder
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
                    std::mem::take(&mut self.pipeline_builder);
                    std::mem::take(&mut self.context);
                    return Some(Err(e));
                }
            }

            self.current_index += 1;
        }
    }
}
