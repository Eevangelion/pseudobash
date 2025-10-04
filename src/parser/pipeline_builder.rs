pub mod pipeline;

use crate::parser::{
    arg_builder::ArgBuilderState, builder::Builder, pipeline_builder::pipeline::Pipeline,
    program_builder::program::Program,
};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct PipelineBuilder<T: Default + Builder<Program>> {
    current_pipeline: Pipeline,
    program_builder: T,
}

impl<T: Default + Builder<Program>> Builder<Pipeline> for PipelineBuilder<T> {
    fn apply(
        &mut self,
        byte: u8,
        context: &mut super::context::Context,
    ) -> anyhow::Result<Option<Pipeline>> {
        match byte {
            b';' => match context.arg_builder_state {
                ArgBuilderState::Default => return self.finish(context),
                ArgBuilderState::WeakSep | ArgBuilderState::StrongSep => {}
            },
            _ => {}
        }

        self.program_builder.apply(byte, context)?.map(|program| {
            self.current_pipeline.push(program);
        });
        Ok(None)
    }

    fn finish(
        &mut self,
        context: &mut super::context::Context,
    ) -> anyhow::Result<Option<Pipeline>> {
        self.program_builder.finish(context)?.map(|program| {
            self.current_pipeline.push(program);
        });
        Ok(self.return_if_not_empty())
    }
}

impl<T: Default + Builder<Program>> PipelineBuilder<T> {
    fn return_if_not_empty(&mut self) -> Option<Pipeline> {
        if self.current_pipeline.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut self.current_pipeline))
        }
    }
}
