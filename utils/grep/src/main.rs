use {
    clap::Parser,
    grep::{args::Args, grep::Grep},
    regex::Regex,
};

fn main() {
    match Grep::<Regex>::new(&Args::parse()) {
        Ok(mut grep) => grep.run(),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(-1);
        }
    }
}
