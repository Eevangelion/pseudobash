pub trait Builder<T, H> {
    fn build(&self, helper: H) -> T;
}
