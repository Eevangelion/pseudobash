use {
    clap::Parser,
    grep::{
        args::Args,
        grep::{Grep, matcher::GrepMatcher},
    },
};

fn main() {
    match Grep::<GrepMatcher>::new(&Args::parse()) {
        Ok(mut grep) => grep.run(),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(-1);
        }
    }
}
