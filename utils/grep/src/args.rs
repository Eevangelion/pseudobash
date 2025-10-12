use {
    crate::{
        builder::Builder,
        grep::{GrepBuilder, flags::Flags, haystack_iterator::HaystackIterator, matcher::Matcher},
    },
    std::{
        fs::File,
        io::{BufRead, BufReader, stdin},
    },
};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Search for full matches only
    #[arg(short = 'w', long, default_value_t = false)]
    full_matching: bool,

    /// Perform case insensitive search
    #[arg(short = 'i', long, default_value_t = false)]
    case_insensitive: bool,

    /// Lines of trailing context to show after matches
    #[arg(short = 'A', long, default_value_t = 0)]
    trace: usize,

    /// The search pattern (needle)
    needle: String,

    /// The file to search through (haystack)
    haystack_file_path: Option<std::path::PathBuf>,
}

impl<'a> Builder<anyhow::Result<HaystackIterator<'a>>, Flags> for &Args {
    fn build(&self, _: Flags) -> anyhow::Result<HaystackIterator<'a>> {
        match &self.haystack_file_path {
            Some(path) => Ok(Box::new(
                BufReader::new(File::open(path)?)
                    .lines()
                    .map(|line| line.expect("Failed to read line")),
            )),
            None => Ok(Box::new(
                stdin()
                    .lines()
                    .map(|line| line.expect("Failed to read line")),
            )),
        }
    }
}

impl<M: Matcher> Builder<anyhow::Result<M>, Flags> for &Args {
    fn build(&self, helper: Flags) -> anyhow::Result<M> {
        M::new(&self.needle, helper)
    }
}

impl Builder<anyhow::Result<Flags>, ()> for &Args {
    fn build(&self, _: ()) -> anyhow::Result<Flags> {
        let mut flags = Flags::default();
        flags.full_matching = self.full_matching;
        flags.case_insensitive = self.case_insensitive;
        flags.trace = self.trace;
        Ok(flags)
    }
}

impl<'a, M: Matcher> GrepBuilder<'a, M> for &Args {}
