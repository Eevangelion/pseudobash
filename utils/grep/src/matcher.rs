use {
    regex::Regex,
    std::{ops::Range, str::FromStr},
};

pub type RangeIterator<'a> = Box<dyn Iterator<Item = Range<usize>> + 'a>;

pub trait Matcher: Sized {
    fn new(needle: &str) -> anyhow::Result<Self>;
    fn matches<'a>(&'a self, haystack: &'a str) -> RangeIterator<'a>;
}

impl Matcher for Regex {
    fn new(needle: &str) -> anyhow::Result<Self> {
        Regex::from_str(&needle).map_err(|e| anyhow::Error::new(e))
    }

    fn matches<'a>(&'a self, haystack: &'a str) -> RangeIterator<'a> {
        Box::new(self.find_iter(haystack).map(|m| m.range()))
    }
}
