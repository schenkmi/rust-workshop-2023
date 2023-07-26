struct CircularBuffer {
    data: Vec<i32>,
    read_index: usize,
    write_index: usize,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0; capacity + 1],
            read_index: 0,
            write_index: 0,
        }
    }

    pub fn push(&mut self, value: i32) -> Result<(), &'static str> {
        let next_write_index = (self.write_index + 1) % self.data.len();
        if next_write_index != self.read_index {
            self.data[self.write_index] = value;
            self.write_index = next_write_index;
            Ok(())
        } else {
            Err("Full")
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.read_index != self.write_index {
            let value = self.data[self.read_index].clone();
            self.read_index = (self.read_index + 1) % self.data.len();
            Some(value)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.read_index == self.write_index
    }
}

fn main() {
    let mut circular_buffer = CircularBuffer::new(3);

    circular_buffer.push(1);
    circular_buffer.push(2);
    circular_buffer.push(3);
    circular_buffer.push(4);
    circular_buffer.push(5);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pop_all(buffer: &mut CircularBuffer) -> Vec<i32> {
        let mut items: Vec<i32> = vec![];
        loop {
            if !buffer.is_empty() {
                items.push(buffer.pop());
            } else {
                break;
            }
        }
        items
    }

    #[test]
    fn test_new() {
        let mut buffer = CircularBuffer::new(0);
        buffer.push(111);
        assert_eq!(pop_all(&mut buffer), vec![]);

        let mut buffer = CircularBuffer::new(1);
        buffer.push(111);
        assert_eq!(pop_all(&mut buffer), vec![111]);

        let mut buffer = CircularBuffer::new(2);
        buffer.push(111);
        buffer.push(222);
        buffer.push(333);
        assert_eq!(pop_all(&mut buffer), vec![111, 222]);
    }

    #[test]
    fn test_is_empty() {
        let mut buffer = CircularBuffer::new(3);
        assert!(buffer.is_empty());
        buffer.push(111);
        assert!(!buffer.is_empty());
        buffer.push(222);
        buffer.push(333);
        assert!(!buffer.is_empty());
        buffer.pop();
        buffer.pop();
        assert!(!buffer.is_empty());
        buffer.pop();
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_push_pop() {
        let mut buffer = CircularBuffer::new(4);
        assert_eq!(buffer.pop(), 0);
        buffer.push(111);
        buffer.push(222);
        buffer.push(333);
        assert_eq!(buffer.pop(), 111);
        assert_eq!(buffer.pop(), 222);
        buffer.push(444);
        buffer.push(555);
        buffer.push(666);
        buffer.push(777);
        assert_eq!(buffer.pop(), 333);
        assert_eq!(buffer.pop(), 444);
        assert_eq!(buffer.pop(), 555);
        assert_eq!(buffer.pop(), 666);
        assert_eq!(buffer.pop(), 0);
        assert_eq!(buffer.pop(), 0);
    }
}
