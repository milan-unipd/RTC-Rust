use crate::constants::{TICKS_PER_SECOND, TICKS_PER_SECOND_64};
use crate::{auxiliary, production_worklad};
use fugit::Duration;

const REGULAR_PRODUCER_WORKLOAD: usize = 756;
const ON_CALL_PRODUCER_WORKLOAD: usize = 278;

pub const PERIOD: Duration<u64, 1, TICKS_PER_SECOND> =
    Duration::<u64, 1, TICKS_PER_SECOND>::from_ticks(TICKS_PER_SECOND_64);

pub fn do_work() -> (bool, usize, bool) {
    let _ = production_worklad::small_whetstone(REGULAR_PRODUCER_WORKLOAD);

    (
        auxiliary::due_activation(2),
        ON_CALL_PRODUCER_WORKLOAD,
        auxiliary::check_due(),
    )
    
}
