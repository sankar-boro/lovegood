use lovegood::block_on;

async fn hello() -> String {
    println!("Hello World!");
    return String::from("Hello");
}

fn main() {
    block_on(hello());
}