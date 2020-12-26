pub trait Component {
    fn new() -> Self
    where
        Self: Sized;
}
