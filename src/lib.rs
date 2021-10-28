use learn_macros::{
    new_factory
};

fn do_something() {
    let x = String::from("sankar boro");
    new_factory!({ x });
}