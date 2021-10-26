use std::{
    thread,
    sync::{ 
        mpsc::{channel}, 
    },
};

enum Event<T> {
    Close,
    Handle(T)
}

#[derive(Debug)]
struct User {}

fn main() {
    let (tx, rx) = channel::<Event<User>>();

    let handle = thread::spawn(|| {
        for event in rx {
            match event {
                Event::Handle(ev) => println!("{:?}", ev),
                Event::Close => break,
            }
        }
    });
    tx.send(Event::Handle(User{}));
    tx.send(Event::Close);
    handle.join().unwrap();
    handle.join().unwrap();
}