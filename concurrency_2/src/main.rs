use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, thread::sleep};

#[derive(Debug)]
struct User {
    name: String,
}

fn main() {
    let user_original = Arc::new(Mutex::new(User {
        name: String::from("Michael"),
    }));

    let user = user_original.clone();
    let t1 = thread::spawn(move || {
        let mut locked_user = user.lock().unwrap();
        locked_user.name = String::from("Gugus");
        // after locked_user goes out of scope, mutex will be unlocked again,
        // but you can also explicitly unlock it with:
        // drop(locked_user);
    });

    let user = user_original.clone();
    let t2 = thread::spawn(move || {
        sleep(Duration::from_millis(10));

        // it will print: Hello Gugus
        println!("Hello {}", user.lock().unwrap().name);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
