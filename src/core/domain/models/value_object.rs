pub trait ValueObject<T> {
    fn new(value: T) -> Self;
    fn get_value(&self) -> &T;
    fn is_equal(&self, other: &Self) -> bool;
}
