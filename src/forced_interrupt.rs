use fugit::Duration;

pub const PERIOD: Duration<u64, 1, 1> = Duration::<u64, 1, 1>::from_ticks(5); //5s

pub const DEADLINE: Duration<u64, 1, 10> = Duration::<u64, 1, 10>::from_ticks(1); //100ms
