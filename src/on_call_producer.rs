use fugit::Duration;

use crate::production_worklad::small_whetstone;

pub const DEADLINE: Duration<u64, 1, 10> = Duration::<u64, 1, 10>::from_ticks(8); //800ms

pub fn do_work(workload: usize) {
    let _ = small_whetstone(workload);
}
