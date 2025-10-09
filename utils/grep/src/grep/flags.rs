#[derive(Default, Clone, Copy)]
pub struct Flags {
    pub full_matching: bool,
    pub case_insensitive: bool,
    pub trace: usize,
}
