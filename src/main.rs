mod service;
use std::marker::PhantomData;

use service::{Factory, Service};

fn fn_factory<F, S>(service: F) -> ServiceFactoryContainer<F, S> 
where F: Factory<S>,
S: Fn()->String + Clone + 'static,
{
    ServiceFactoryContainer { service, _t: PhantomData }
}

struct ServiceFactoryContainer<F, S> 
where F: Factory<S>,
S: Fn()->String,
{
    service: F,
    _t: PhantomData<S>
}

fn fn_service<S>(service: S) -> ServiceContainer<S>
where S: Fn()->String + Clone + 'static
{
    ServiceContainer::new(service)
}

struct ServiceContainer<S> 
where S: Fn()->String + Clone + 'static {
    service: S
}

impl<S> Clone for ServiceContainer<S> 
where S: Fn()->String + Clone + 'static 
{
    fn clone(&self) -> Self { 
        ServiceContainer { service: self.service.clone() }
    }
}


impl<F, S> Factory<S> for ServiceFactoryContainer<F, S> 
where F: Factory<S>,
S: Fn()->String,
{
    fn new_service(&self) -> S {
        self.service.new_service()
    }
}

impl<S> Factory<S> for ServiceContainer<S> 
where S: Fn()->String + Clone + 'static,
{
    fn new_service(&self) -> S {
        self.service.clone()
    }
}

impl<F> ServiceContainer<F> 
where F: Fn()->String + Clone + 'static
{
    fn new(service: F) -> Self {
        ServiceContainer {
            service,
        }
    }
}

impl<F> Service for ServiceContainer<F> 
where F: Fn()->String + Clone + 'static
{
    fn call(&self) {
        (self.service)();
    }
}

fn index() -> String {
    format!("Sankar boro")
}

fn main() {
    let service = fn_service(index);
    let factory = fn_factory(service);
    let new_service = factory.new_service();
    let data = new_service();
    println!("Hello: {}", data);
}
