use std::sync::{Arc, Mutex};

struct A {
    value: i32,
}

fn main() {
    let a = Arc::new(Mutex::new(A { value: 0 }));

    let mut threads = vec![];
    for thread_id in 0..5 {
        let a_cloned = a.clone();
        let thread = std::thread::spawn(move || {
            let mut a_unlocked = a_cloned.lock().unwrap();
            a_unlocked.value += 1;
            println!("Thread {}, value {}", thread_id, a_unlocked.value);
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
