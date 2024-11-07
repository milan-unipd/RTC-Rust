use crate::app::Mono;
use fugit::Instant;
use rtic_monotonics::Monotonic;

pub struct ActivationLog {
    activation_counter: u8,
    activation_time: Instant<u64, 1, 16000000>,
}

impl ActivationLog {
    pub fn new() -> Self {
        Self {
            activation_counter: 0,
            activation_time: Mono::now(),
        }
    }

    pub fn read(&mut self) -> (u8, Instant<u64, 1, 16000000>) {
        (self.activation_counter, self.activation_time)
    }

    pub fn write(&mut self) {
        self.activation_counter = (self.activation_counter + 1) % 100;
        self.activation_time = Mono::now();
    }
}