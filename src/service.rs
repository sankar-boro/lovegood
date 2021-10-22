pub trait Service<Req, Res> {
    fn call(&self, param: Req) -> Res;
}

pub trait ServiceFactory<S: Service<Req, Res>, Req, Res> {
    fn new_service(&self) -> S;
}