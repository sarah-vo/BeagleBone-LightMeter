mod circular_buffer {

    struct CircularBuffer<T>{
        buffer: Vec<Option<T>>,
        /// Index to be written (or overwritten)
        head: usize,
        /// Index of oldest element
        tail: usize,
        /// Current elements in the buffer
        size: usize,
        /// Maximum amount of elements buffer can store
        capacity: usize
    }

    impl CircularBuffer<T> {
        ///
        fn new(size:usize) -> CircularBuffer<T>{
            CircularBuffer{
                buffer: vec![None, size],
                head: 0,
                tail: 0,
                size: 0,
                capacity: 0
            }
        }
        fn is_full(&self) -> bool{
            self.size == self.capacity
        }
        fn is_empty(&self) -> bool{
            self.size == 0
        }

        /// Pushes an element to the buffer
        /// If the buffer is full, the oldest element will be overwritten
        fn push(&mut self,ele : T){
            self.buffer[self.head] = ele;
            self.head = (self.head + 1) % self.capacity;

            if self.is_full(){
                self.tail = (self.tail + 1) % self.capacity;
            } else {
                self.size += 1;
            }
        }

        fn pop(&mut self) -> Option<T>{
            if self.is_empty(){
                return None;
            }
            let ele = self.buffer[self.tail];
            self.tail = (self.tail + 1) % self.capacity;
            self.size -= 1;
            return ele;
        }

        fn get_latest_samples(&self, size: usize) -> Vec<T>{
            let mut samples = Vec::with_capacity(size);
            let mut index = self.tail + self.size - 1 - size;
            for _ in 0..size{
                index = (index + 1) % self.capacity;
                samples.push(self.buffer[index]);
            }
            return samples;

        }

        fn resize(&mut self, new_size: usize){
            let mut new_buffer = CircularBuffer::new(new_size);
            let size_to_copy = std::cmp::min(self.size, new_size);
            new_buffer.buffer = self.get_latest_samples(size_to_copy).map(|x| Some(x)).collect();
            new_buffer.tail = 0;
            new_buffer.head = (size_to_copy + 1) % size_to_copy;
            new_buffer.size = size_to_copy;

        }


    }
}