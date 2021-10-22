use std::future::Future;
/**
 * Service is a trait that takes in a Request and returns Response.
 */
pub trait Service<Req, Res, E> {
    type Request;
    type Response;
    type Error;
    type Future: Future<Output=Result<Self::Response, Self::Error>>;

    fn call(&self, param: Req) -> Self::Future;
}

/**
 * ServiceFactory is a trait that creates new service to transform Request to Response.
 */
pub trait ServiceFactory<S: Service<Req, Res, E>, Req, Res, E> {
    type Request;
    type Response;
    type Error;
    type Service: Service<Req, Res, E>;
    type Future: Future<Output=Result<Self::Service, Self::Error>>;

    fn new_service(&self) -> Self::Future;
}