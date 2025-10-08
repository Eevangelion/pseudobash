use pseudobash::{
    cli::CLI,
    parser::{
        CLIParser,
        arg_builder::ArgBuilder,
        pipeline_builder::{PipelineBuilder, pipeline::Pipeline},
        program_builder::ProgramBuilder,
        token::Token,
    },
};

fn main() {
    println!("Welcome to Pseudobash v{}!\n", env!("CARGO_PKG_VERSION"));

    let mut cli: CLI<
        Pipeline,
        CLIParser<Pipeline, PipelineBuilder<ProgramBuilder<ArgBuilder<Token>>>>,
    > = CLI::default();
    cli.start();
}
