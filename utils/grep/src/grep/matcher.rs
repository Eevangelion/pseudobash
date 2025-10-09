use {
    crate::grep::flags::Flags,
    regex::{Regex, RegexBuilder, escape},
    std::ops::Range,
};

pub type RangeIterator<'a> = Box<dyn Iterator<Item = Range<usize>> + 'a>;

pub trait Matcher: Sized {
    fn new(needle: &str, flags: Flags) -> anyhow::Result<Self>;
    fn matches<'a>(&'a self, haystack: &'a str) -> RangeIterator<'a>;
}

pub struct GrepMatcher {
    regex: Regex,

    _needle: String,
    _flags: Flags,
}

impl Matcher for GrepMatcher {
    fn new(needle: &str, flags: Flags) -> anyhow::Result<Self> {
        let needle = match (flags.full_matching, Regex::new(needle).is_ok()) {
            (true, true) => format!(r"\b{}\b", needle),
            (true, false) => format!(r"\b{}\b", escape(needle)),
            (false, true) => format!(r"{}", needle),
            (false, false) => format!(r"{}", escape(needle)),
        };

        RegexBuilder::new(&needle)
            .case_insensitive(flags.case_insensitive)
            .build()
            .map_err(|e| anyhow::Error::new(e))
            .map(|regex| GrepMatcher {
                _needle: needle,
                regex,
                _flags: flags,
            })
    }

    fn matches<'a>(&'a self, haystack: &'a str) -> RangeIterator<'a> {
        Box::new(self.regex.find_iter(haystack).map(|m| m.range()))
    }
}
