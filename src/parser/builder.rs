use crate::parser::context::Context;

pub trait Builder<T> {
    fn apply(&mut self, byte: u8, context: &mut Context) -> anyhow::Result<Option<T>>;

    fn finish(&mut self, context: &mut Context) -> anyhow::Result<Option<T>>;
}
