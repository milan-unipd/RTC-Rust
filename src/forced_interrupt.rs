use crate::constants::{TICKS_PER_SECOND, TICKS_PER_SECOND_64};
use fugit::Duration;

pub const PERIOD: Duration<u64, 1, TICKS_PER_SECOND> =
    Duration::<u64, 1, TICKS_PER_SECOND>::from_ticks(5 * TICKS_PER_SECOND_64);

