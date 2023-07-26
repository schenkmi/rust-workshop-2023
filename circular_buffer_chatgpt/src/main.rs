use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

struct CircularBuffer<T> {
    buffer: Mutex<VecDeque<T>>,
    max_size: usize,
}

impl<T> CircularBuffer<T> {
    // Create a new CircularBuffer with a given maximum size
    fn new(max_size: usize) -> Self {
        CircularBuffer {
            buffer: Mutex::new(VecDeque::with_capacity(max_size)),
            max_size,
        }
    }

    // Add an item to the buffer's back
    fn push_back(&self, item: T) {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.len() == self.max_size {
            buffer.pop_front();
        }
        buffer.push_back(item);
    }

    // Add an item to the buffer's front
    fn push_front(&self, item: T) {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.len() == self.max_size {
            buffer.pop_back();
        }
        buffer.push_front(item);
    }

    // Remove and return the item at the buffer's front
    fn pop_front(&self) -> Option<T> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.pop_front()
    }

    // Remove and return the item at the buffer's back
    fn pop_back(&self) -> Option<T> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.pop_back()
    }

    // Get the number of items currently in the buffer
    fn len(&self) -> usize {
        let buffer = self.buffer.lock().unwrap();
        buffer.len()
    }

    // Check if the buffer is empty
    fn is_empty(&self) -> bool {
        let buffer = self.buffer.lock().unwrap();
        buffer.is_empty()
    }

    // Check if the buffer is full
    fn is_full(&self) -> bool {
        let buffer = self.buffer.lock().unwrap();
        buffer.len() == self.max_size
    }

    // Get the maximum size of the buffer
    fn max_size(&self) -> usize {
        self.max_size
    }
}

fn main() {
    let circular_buffer = Arc::new(CircularBuffer::new(5));

    let mut handles = vec![];

    for i in 0..10 {
        let buffer_clone = circular_buffer.clone();
        let handle = std::thread::spawn(move || {
            buffer_clone.push_back(i);
            println!("Added item: {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    while let Some(item) = circular_buffer.pop_front() {
        println!("Popped item: {}", item);
    }
}
