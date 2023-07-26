
struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    read_index: usize,
    write_index: usize,
    max_size: usize,
}

impl<T: Default + Clone> CircularBuffer<T> {
    // Create a new CircularBuffer with a given maximum size
    fn new(max_size: usize) -> Self {
        CircularBuffer {
            buffer: vec![None; max_size],
            read_index: 0,
            write_index: 0,
            max_size,
        }
    }

    // Add an item to the buffer (overwriting if the buffer is full)
    fn push(&mut self, item: T) {
        if self.buffer[self.write_index].is_some() {
            self.read_index = (self.read_index + 1) % self.max_size;
        }

        self.buffer[self.write_index] = Some(item);
        self.write_index = (self.write_index + 1) % self.max_size;
    }

    // Remove and return the next item from the buffer
    fn pop(&mut self) -> Option<T> {
        let item = self.buffer[self.read_index].take();
        self.read_index = (self.read_index + 1) % self.max_size;
        item
    }

    // Get the number of items currently in the buffer
    fn len(&self) -> usize {
        let occupied = self.buffer.iter().filter(|item| item.is_some()).count();
        if occupied > 0 {
            occupied
        } else {
            0
        }
    }

    // Check if the buffer is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Check if the buffer is full
    fn is_full(&self) -> bool {
        self.len() == self.max_size
    }

    // Get the maximum size of the buffer
    fn max_size(&self) -> usize {
        self.max_size
    }
}

fn main() {
    let mut circular_buffer = CircularBuffer::new(5);

    circular_buffer.push(1);
    circular_buffer.push(2);
    circular_buffer.push(3);
    circular_buffer.push(4);
    circular_buffer.push(5);

    println!("Buffer size: {}", circular_buffer.len());
    println!("Is buffer empty? {}", circular_buffer.is_empty());
    println!("Is buffer full? {}", circular_buffer.is_full());

    // Adding more items to the full buffer (overwriting)
    circular_buffer.push(6);
    circular_buffer.push(7);
    circular_buffer.push(8);

    while let Some(item) = circular_buffer.pop() {
        println!("Popped item: {}", item);
    }
}
