pub trait Singleton {
    fn get() -> &'static Self;
}