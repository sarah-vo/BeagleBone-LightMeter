mod circular_buffer {


    pub struct CircularBuffer<T>{
        pub buffer: Vec<Option<T>>,
        /// Index to be written (or overwritten)
        head: usize,
        /// Index of oldest element
        tail: usize,
        /// Current elements in the buffer
        pub(crate) size: usize,
        /// Maximum amount of elements buffer can store
        capacity: usize
    }

    impl <T> CircularBuffer<T> where T:Clone{
        pub fn new(size:usize) -> CircularBuffer<T>{
            CircularBuffer{
                buffer: vec![None; size],
                head: 0,
                tail: 0,
                size: 0,
                capacity: size
            }
        }
        pub fn is_full(&self) -> bool{
            self.size == self.capacity
        }
        pub fn is_empty(&self) -> bool{
            self.size == 0
        }

        /// Pushes an element to the buffer
        /// If the buffer is full, the oldest element will be overwritten
        pub fn push(&mut self,ele : T){
            self.buffer[self.head] = Some(ele);
            self.head = (self.head + 1) % self.capacity;

            if self.is_full(){
                self.tail = (self.tail + 1) % self.capacity;
            } else {
                self.size += 1;
            }
        }

        pub fn pop(&mut self) -> Option<T>{
            if self.is_empty(){
                return None;
            }
            let ele = self.buffer[self.tail].clone();
            self.buffer[self.tail] = None;
            self.tail = (self.tail + 1) % self.capacity;
            self.size -= 1;
            return ele;
        }

        pub fn get_latest_samples(&self, size: usize) -> Vec<T>{
            if size > self.size{
                println!("Error: Size is larger than buffer size");
            }
            let mut samples = Vec::with_capacity(size);
            let mut index = self.tail + self.size - 1 - size;
            for _ in 0..size{
                index = (index + 1) % self.capacity;
                samples.push(self.buffer[index].clone().unwrap());
            }
            return samples;

        }

        pub fn resize(&mut self, new_size: usize){
            let mut new_buffer : CircularBuffer<T> = CircularBuffer::new(new_size);
            let size_to_copy = std::cmp::min(self.size, new_size);
            new_buffer.buffer = self.get_latest_samples(size_to_copy).into_iter().map(|x| Some(x)).collect();
            new_buffer.tail = 0;
            new_buffer.head = (size_to_copy + 1) % size_to_copy;
            new_buffer.size = size_to_copy;

            *self = new_buffer;
        }
    }



}

#[cfg(test)]
mod tests {
    use crate::circular_buffer::circular_buffer::CircularBuffer;

    #[test]
    fn test_push() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert_eq!(buffer.buffer, vec![Some(1), Some(2), Some(3)]);
        buffer.push(4);
        assert_eq!(buffer.buffer, vec![Some(4), Some(2), Some(3)]);
    }

    #[test]
    fn test_pop() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.buffer, vec![None, Some(2), Some(3)]);
    }

    #[test]
    fn test_is_full() {
        let mut buffer = CircularBuffer::new(2);
        buffer.push(1);
        buffer.push(2);
        assert_eq!(buffer.is_full(), true);
    }

    #[test]
    fn test_is_empty() {
        let mut buffer = CircularBuffer::new(2);
        assert_eq!(buffer.is_empty(), true);
        buffer.push(1);
        assert_eq!(buffer.is_empty(), false);
    }

    #[test]
    fn test_get_latest_samples() {
        let mut buffer = CircularBuffer::new(4);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);
        assert_eq!(buffer.get_latest_samples(2), vec![3, 4]);
        for i in 5..=10{
            buffer.push(i);
        }
        assert_eq!(buffer.get_latest_samples(4), vec![7, 8, 9, 10]);
    }

    #[test]
    fn test_resize() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.resize(2);
        assert_eq!(buffer.buffer, vec![Some(2), Some(3)]);
        assert_eq!(buffer.size, 2);
    }
}
