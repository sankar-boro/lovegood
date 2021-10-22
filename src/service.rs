pub trait Service {
    fn call(&self);
}

pub trait Factory<S> {
    fn new_service(&self) -> S;
}