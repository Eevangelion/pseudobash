use {
    crate::grep::LineIterator,
    std::{
        fs::File,
        io::{BufRead, BufReader, stdin},
    },
};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The flag for a full match
    #[arg(short = 'w', long, default_value_t = false)]
    pub full_matching: bool,

    /// The needle
    //#[arg(short, long)]
    pub needle: String,

    /// The path to the haystack file
    //#[arg(short, long)]
    pub file_path: Option<std::path::PathBuf>,
}

impl Args {
    pub fn get_haystack(&self) -> anyhow::Result<LineIterator> {
        match &self.file_path {
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
