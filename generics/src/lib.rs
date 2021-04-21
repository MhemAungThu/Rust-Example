// defined two structs which one is implemented by `Debug` but other is not.
#[derive(Debug)]
pub struct Empty;

pub struct Null {
    pub s: String,
}

// A trait generic over `T`
pub trait DoubleDrop<T> {
    // Defined a method on the caller type which takes an
    // additional single parameter `T` and does nothing
    fn double_drop(self, _: T);
}

// implement `DoubleDrop<T>` for any generic parameter `T` and
// caller 'U`
impl<T, U> DoubleDrop<T> for U
where
    U: Clone + Copy,
{
    // method takes ownership of both pass args, dealloc both
    fn double_drop(self, _: T){}
}
