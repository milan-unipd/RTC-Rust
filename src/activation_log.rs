use rtic_monotonics::Monotonic;
use crate::app::Mono;
use fugit::Instant;

pub struct ActivationLog {
    activation_counter: u8,
    activation_time: Instant<u64, 1, 16000000>
}

impl ActivationLog {
    pub fn new() -> Self {
        Self {
            activation_counter: 0,
            activation_time: Mono::now()
            
        }
    }

    pub fn read(&mut self)-> (u8, Instant<u64,1 , 16000000>) {
        (self.activation_counter, self.activation_time)
    }

    pub fn write(&mut self) {
        self.activation_counter = (self.activation_counter + 1) % 100;
        self.activation_time = Mono::now();
    }
}

// impl RequestBuffer {

//     pub fn new() -> Self {
//         Self {
//             buffer: [0; BUFFER_SIZE],
//             head: 0,
//             tail: 0,
//             count: 0,

//         }
//     }

//     pub fn deposit(&mut self, item: usize) -> bool {
//         if self.count == BUFFER_SIZE {
//             return false;
//         }

//         self.buffer[self.tail] = item;
//         self.tail = (self.tail + 1) % BUFFER_SIZE;
//         self.count += 1;

//         true
//     }

//     pub fn extract(&mut self) -> Option<usize> {

//             if self.count == 0 {
//                 return None;
//             }

//             let item = self.buffer[self.head];
//             self.head = (self.head + 1) % BUFFER_SIZE;
//             self.count -= 1;

//             Some(item)
//     }

//     pub fn is_full(&self) -> bool {
//         self.count == BUFFER_SIZE
//     }

//     pub fn is_empty(&self) -> bool {
//         self.count == 0
//     }

// }
