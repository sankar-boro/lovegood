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
    ({ $param:expr }) => {
        println!("{}", stringify!($param));
    };
}

macro_rules! thread_local {
    () => { ... };
    ($(#[$attr : meta]) * $vis : vis static $name : ident : $t : ty = const
    { $init : expr } ; $($rest : tt) *) => { ... };
        ($(#[$attr : meta]) * $vis : vis static $name : ident : $t : ty = const
    { $init : expr }) => { ... };
        ($(#[$attr : meta]) * $vis : vis static $name : ident : $t : ty = $init : expr
    ; $($rest : tt) *) => { ... };
        ($(#[$attr : meta]) * $vis : vis static $name : ident : $t : ty = $init :
    expr) => { ... };
}
