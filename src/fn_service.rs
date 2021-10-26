use std::{future::{Future}, marker::PhantomData};
use service::{ServiceFactory, Service};
use ready::Ready;

/**
 * ServiceFactory implementation 
 */

fn fn_factory<F, Req, Res, E>(service: F) -> ServiceFactoryContainer<F, Req, Res, E> 
where F: Service<Req, Res, E> + Clone + 'static
{
    ServiceFactoryContainer::new(service)
}

struct ServiceFactoryContainer<F, Req, Res, E> 
where F: Service<Req, Res, E>,
{
    service: F,
    _t: PhantomData<(Req, Res, E)>
}

impl<F, Req, Res, E> ServiceFactory<F, Req, Res, E> for ServiceFactoryContainer<F, Req, Res, E> 
where F: Service<Req, Res, E> + Clone + 'static
{
    type Request = Req;

    type Response = Res;

    type Error = E;

    type Service = F;

    type Future = Ready<Self::Service, Self::Error>;

    fn new_service(&self) -> Self::Future {
        Ready::Ok(self.service.clone())
    }
}

impl<F, Req, Res, E> ServiceFactoryContainer<F, Req, Res, E> 
where F: Service<Req, Res, E> + Clone + 'static
{
    fn new(service: F) -> Self {
        Self {
            service,
            _t: PhantomData,
        }
    }
}

/**
 * Service implementation 
 */
fn fn_service<S, Fut, Req, Res, E>(service: S) -> ServiceContainer<S, Fut, Req, Res, E>
where S: Fn(Req)-> Fut + Clone + 'static,
Fut: Future<Output=Result<Res, E>>
{
    ServiceContainer::new(service)
}

struct ServiceContainer<S, Fut, Req, Res, E> 
where S: Fn(Req)-> Fut + Clone + 'static,
Fut: Future<Output=Result<Res, E>>
{
    service: S, 
    _t: PhantomData<(Req, Res)>
}

impl<S, Fut, Req, Res, E> Clone for ServiceContainer<S, Fut, Req, Res, E> 
where S: Fn(Req)-> Fut + Clone + 'static,
Fut: Future<Output=Result<Res, E>> 
{
    fn clone(&self) -> Self { 
        ServiceContainer { service: self.service.clone(), _t: PhantomData }
    }
}

impl<F, Fut, Req, Res, E> ServiceContainer<F, Fut, Req, Res, E> 
where F: Fn(Req)-> Fut + Clone + 'static,
Fut: Future<Output=Result<Res, E>>
{
    fn new(service: F) -> Self {
        ServiceContainer {
            service,
            _t: PhantomData,
        }
    }
}

impl<F, Fut, Req, Res, E> Service<Req, Res, E> for ServiceContainer<F, Fut, Req, Res, E> 
where F: Fn(Req) -> Fut + Clone + 'static,
Fut: Future<Output=Result<Res, E>>
{

    type Request = Req;

    type Response = Res;

    type Error = E;

    type Future = Fut;

    fn call(&self, param: Req) -> Self::Future {
        (self.service)(param)
    }
}

async fn index(param: String) -> Result<String, ()> {
    Ok(format!("{}: Sankar boro", param))
}

#[async_std::main]
async fn main() {
    let _service = fn_service(index);
    let factory = fn_factory(_service);
    let a = factory.new_service();
    let b = a.await.unwrap();
    let c = b.call(String::from("Hello"));
    let d = c.await.unwrap();
    println!("{}", d);
}
