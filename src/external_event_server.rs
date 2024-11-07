use fugit::Duration;

pub const DEADLINE: Duration<u64, 1, 10> = Duration::<u64, 1, 10>::from_ticks(1); //100ms
