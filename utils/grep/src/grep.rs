pub mod flags;
pub mod haystack_iterator;
pub mod matcher;

use crate::{
    builder::Builder,
    grep::{flags::Flags, haystack_iterator::HaystackIterator, matcher::Matcher},
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
            match (
                self.matcher.matches(&line).next().is_some(),
                self.current_trace > 0,
            ) {
                (true, _) => {
                    println!("{}", &line);
                    self.current_trace = self.flags.trace;
                }
                (false, true) => {
                    println!("{}", &line);
                    self.current_trace -= 1;
                }
                (false, false) => {}
            }
        }
    }
}
