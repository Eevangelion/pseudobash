pub trait Builder<T, C> {
    fn apply(&mut self, byte: u8, context: &mut C) -> anyhow::Result<Option<T>>;
    fn finish(&mut self, context: &mut C) -> anyhow::Result<Option<T>>;
}
