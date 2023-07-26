struct CircularBuffer<'a, T> {
    data: &'a mut [T],
    read_index: usize,
    write_index: usize,
}

#[derive(Debug, PartialEq)]
enum PushError {
    Full,
}

impl<'a, T: Clone + Default> CircularBuffer<'a, T> {
    pub fn new(buffer: &'a mut [T]) -> Self {
        Self {
            data: buffer,
            read_index: 0,
            write_index: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), PushError> {
        let next_write_index = (self.write_index + 1) % self.data.len();
        if next_write_index != self.read_index {
            self.data[self.write_index] = value;
            self.write_index = next_write_index;
            Ok(())
        } else {
            Err(PushError::Full)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
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

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn pop_all<T: Clone + Default>(buffer: &mut CircularBuffer<T>) -> Vec<T> {
        let mut items: Vec<T> = vec![];
        loop {
            if let Some(item) = buffer.pop() {
                items.push(item);
            } else {
                break;
            }
        }
        items
    }

    #[test]
    fn test_new() {
        let mut raw = vec![0i32; 1]; // usable length: 0
        let mut buffer = CircularBuffer::new(&mut raw);
        assert_eq!(buffer.push(111), Err(PushError::Full));
        assert_eq!(pop_all(&mut buffer), vec![]);

        let mut raw = vec![0i32; 2]; // usable length: 1
        let mut buffer = CircularBuffer::new(&mut raw);
        assert_eq!(buffer.push(111), Ok(()));
        assert_eq!(buffer.push(222), Err(PushError::Full));
        assert_eq!(pop_all(&mut buffer), vec![111]);

        let mut raw = vec![0i32; 3]; // usable length: 2
        let mut buffer = CircularBuffer::new(&mut raw);
        assert_eq!(buffer.push(111), Ok(()));
        assert_eq!(buffer.push(222), Ok(()));
        assert_eq!(buffer.push(333), Err(PushError::Full));
        assert_eq!(pop_all(&mut buffer), vec![111, 222]);
    }

    #[test]
    fn test_is_empty() {
        let mut raw = vec![(); 4]; // usable length: 3
        let mut buffer = CircularBuffer::new(&mut raw);
        assert!(buffer.is_empty());
        buffer.push(()).unwrap();
        assert!(!buffer.is_empty());
        buffer.push(()).unwrap();
        buffer.push(()).unwrap();
        assert!(!buffer.is_empty());
        buffer.pop();
        buffer.pop();
        assert!(!buffer.is_empty());
        buffer.pop();
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_push_pop() {
        let mut raw = vec![0i32; 5]; // usable length: 4
        let mut buffer = CircularBuffer::new(&mut raw);
        assert_eq!(buffer.pop(), None);
        assert_eq!(buffer.push(111), Ok(()));
        assert_eq!(buffer.push(222), Ok(()));
        assert_eq!(buffer.push(333), Ok(()));
        assert_eq!(buffer.pop(), Some(111));
        assert_eq!(buffer.pop(), Some(222));
        assert_eq!(buffer.push(444), Ok(()));
        assert_eq!(buffer.push(555), Ok(()));
        assert_eq!(buffer.push(666), Ok(()));
        assert_eq!(buffer.push(777), Err(PushError::Full));
        assert_eq!(buffer.push(777), Err(PushError::Full));
        assert_eq!(buffer.pop(), Some(333));
        assert_eq!(buffer.pop(), Some(444));
        assert_eq!(buffer.pop(), Some(555));
        assert_eq!(buffer.pop(), Some(666));
        assert_eq!(buffer.pop(), None);
        assert_eq!(buffer.pop(), None);
    }
}
