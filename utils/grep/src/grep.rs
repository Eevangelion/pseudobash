use crate::{args::Args, matcher::Matcher};

pub type LineIterator<'a> = Box<dyn Iterator<Item = String> + 'a>;

pub struct Grep<'a, M: Matcher> {
    haystack: LineIterator<'a>,
    matcher: M,
}

impl<'a, M: Matcher> Grep<'a, M> {
    pub fn new(args: &'a Args) -> anyhow::Result<Self> {
        Ok(Self {
            haystack: args.get_haystack()?,
            matcher: M::new(&args.needle)?,
        })
    }

    pub fn run(&mut self) {
        for line in &mut self.haystack {
            let mut last_end = 0;
            for range in self.matcher.matches(&line) {
                let current_end = range.end;
                print!(
                    "{}\x1b[0;1;31m{}\x1b[0m",
                    &line[last_end..range.start],
                    &line[range]
                );
                last_end = current_end;
            }
            if last_end > 0 {
                println!("{}", &line[last_end..])
            }
        }
    }
}
