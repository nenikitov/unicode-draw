pub trait CopyFrom<T = Self> {
    fn copy_from(&mut self, rhs: &T);
}
