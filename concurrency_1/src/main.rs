use std::sync::*;
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(String::new()));
    let mut children = vec![];
    // Create threads.
    for _ in 0..8 {
        let data = Arc::clone(&data);
        children.push(thread::spawn(move || {
            // Lock blocks until the mutex is available.
            let mut data = data.lock().unwrap();
            // Generate a string.
            let number = 100;
            let result = "Data ".to_string() + &number.to_string();
            // Store string in mutex.
            *data = result;
        }));
    }
    
    // Join all threads.
    for child in children {
        let _result = child.join();
    }
    
    // Print shared string.
    let result = data.lock().unwrap();
    println!("{}", result);
}
