use std::cell::RefCell;
use std::thread;

thread_local! {
    static FOO: RefCell<f32> = RefCell::new(1.0);
}

fn main() {

    FOO.with(|foo| {
        *foo.borrow_mut() = 3.0;
    });

    thread::spawn(move|| {
        // FOO value should be 1.0
        FOO.with(|foo| {
            println!("inner: {}", *foo.borrow());
        });
    }).join().unwrap();

    // FOO value should be 3.0
    FOO.with(|foo| {
        println!("main: {}", *foo.borrow());
    });
}