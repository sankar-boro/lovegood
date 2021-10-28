pub trait NewFactory<P> {
    fn call(self, param: P);
}

pub struct User {
    pub name: String,
}

#[macro_export]
macro_rules! new_factory {
        ({}) => {
    
        };
        ({ $($param:expr)* }) => {
            println!("{}", stringify!($param));
        };
}

// new_factory!{}
// new_factory!{ A }