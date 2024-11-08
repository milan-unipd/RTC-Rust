use fugit::Duration;

use crate::production_worklad;

const ACTIVATION_LOG_READER_WORKLOAD: usize = 10;

pub const DEADLINE: Duration<u64, 1, 1> = Duration::<u64, 1, 1>::from_ticks(1); //1s

pub fn do_work() {
    let _ = production_worklad::small_whetstone(ACTIVATION_LOG_READER_WORKLOAD);
}
