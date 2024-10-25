const BUFFER_SIZE: usize = 16;


pub struct RequestBuffer {
    buffer: [usize; BUFFER_SIZE],
    head: usize,
    tail: usize,
    count: usize,
 
}

impl RequestBuffer {
    
    pub fn new() -> Self {
        Self {
            buffer: [0; BUFFER_SIZE],
            head: 0,
            tail: 0,
            count: 0,
 
        }
    }

    
    pub fn deposit(&mut self, item: usize) -> bool {
        if self.count == BUFFER_SIZE {
            return false;
        }

        self.buffer[self.tail] = item;
        self.tail = (self.tail + 1) % BUFFER_SIZE;
        self.count += 1;

        true
    }

    
    pub fn extract(&mut self) -> Option<usize> {
 
            if self.count == 0 {
                return None;
            }

            let item = self.buffer[self.head];
            self.head = (self.head + 1) % BUFFER_SIZE;
            self.count -= 1;

            Some(item)
    }

    pub fn is_full(&self) -> bool {
        self.count == BUFFER_SIZE
    }
    
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }


}