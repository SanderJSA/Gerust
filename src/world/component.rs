use std::any::Any;

pub trait Component: Sized + Any {
    fn new() -> Self;
}
