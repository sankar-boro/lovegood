mod service;
use std::marker::PhantomData;

use service::{ServiceFactory, Service};

/**
 * ServiceFactory implementation 
 */

fn fn_factory<F, Req, Res>(service: F) -> ServiceFactoryContainer<F, Req, Res> 
where F: Service<Req, Res> + Clone + 'static,
{
    ServiceFactoryContainer::new(service)
}

struct ServiceFactoryContainer<F, Req, Res> 
where F: Service<Req, Res>,
{
    service: F,
    _t: PhantomData<(Req, Res)>
}

impl<F, Req, Res> ServiceFactory<F, Req, Res> for ServiceFactoryContainer<F, Req, Res> 
where F: Service<Req, Res> + Clone + 'static,
{
    fn new_service(&self) -> F {
        self.service.clone()
    }
}

impl<F, Req, Res> ServiceFactoryContainer<F, Req, Res> 
where F: Service<Req, Res> + Clone + 'static,
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

fn fn_service<S, Req, Res>(service: S) -> ServiceContainer<S, Req, Res>
where S: Fn(Req)-> Res + Clone + 'static
{
    ServiceContainer::new(service)
}

struct ServiceContainer<S, Req, Res> 
where S: Fn(Req)-> Res + Clone + 'static {
    service: S, 
    _t: PhantomData<(Req, Res)>
}

impl<S, Req, Res> Clone for ServiceContainer<S, Req, Res> 
where S: Fn(Req)-> Res + Clone + 'static 
{
    fn clone(&self) -> Self { 
        ServiceContainer { service: self.service.clone(), _t: PhantomData }
    }
}

impl<F, Req, Res> ServiceContainer<F, Req, Res> 
where F: Fn(Req)-> Res + Clone + 'static
{
    fn new(service: F) -> Self {
        ServiceContainer {
            service,
            _t: PhantomData,
        }
    }
}

impl<F, Req, Res> Service<Req, Res> for ServiceContainer<F, Req, Res> 
where F: Fn(Req) -> Res + Clone + 'static,
{
    fn call(&self, param: Req) -> Res {
        (self.service)(param)
    }
}

fn index(param: String) -> String {
    format!("{}: Sankar boro", param)
}

fn main() {
    let service = fn_service(index);
    let factory = fn_factory(service);
    let a = factory.new_service();
    let b = a.call(String::from("Hello"));
    println!("{}", b);
}
