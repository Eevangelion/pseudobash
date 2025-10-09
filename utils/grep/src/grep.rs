pub mod flags;
pub mod line_iterator;
pub mod matcher;

use crate::{
    builder::Builder,
    grep::{flags::Flags, line_iterator::HaystackIterator, matcher::Matcher},
};

pub trait GrepBuilder<'a, M: Matcher>:
    Builder<anyhow::Result<HaystackIterator<'a>>, Flags>
    + Builder<anyhow::Result<M>, Flags>
    + Builder<anyhow::Result<Flags>, ()>
{
}

pub struct Grep<'a, M: Matcher> {
    haystack_it: HaystackIterator<'a>,
    matcher: M,
    flags: Flags,
    current_trace: usize,
}

impl<'a, M: Matcher> Grep<'a, M> {
    pub fn new<T: GrepBuilder<'a, M>>(value: T) -> anyhow::Result<Self> {
        let flags = <T as Builder<anyhow::Result<Flags>, ()>>::build(&value, ())?;
        Ok(Self {
            haystack_it: <T as Builder<anyhow::Result<HaystackIterator<'a>>, Flags>>::build(
                &value, flags,
            )?,
            matcher: <T as Builder<anyhow::Result<M>, Flags>>::build(&value, flags)?,
            flags,
            current_trace: 0,
        })
    }

    pub fn run(&mut self) {
        for line in &mut self.haystack_it {
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
                println!("{}", &line[last_end..]);
                self.current_trace = self.flags.trace;
            } else {
                if self.current_trace > 0 {
                    if line.len() == 0 {
                        println!("\x1b[0;1;32m~\x1b[0m");
                    } else {
                        println!("{}", &line);
                    }
                    self.current_trace -= 1;
                }
            }
        }
        if self.current_trace > 0 {
            println!("\x1b[0;1;32m~\x1b[0m");
            self.current_trace = 0;
        }
    }
}
