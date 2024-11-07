use crate::{auxiliary, production_worklad};

use fugit::Duration;

const REGULAR_PRODUCER_WORKLOAD: usize = 60 * 22 + 17;
pub const ON_CALL_PRODUCER_WORKLOAD: usize = 20 * 66 + 19;

pub const PERIOD: Duration<u64, 1, 1> = Duration::<u64, 1, 1>::from_ticks(1); //1s

pub const DEADLINE: Duration<u64, 1, 10> = Duration::<u64, 1, 10>::from_ticks(5); //500ms

pub fn do_work() -> (bool, usize, bool) {
    let _ = production_worklad::small_whetstone(REGULAR_PRODUCER_WORKLOAD);

    (
        auxiliary::due_activation(2),
        ON_CALL_PRODUCER_WORKLOAD,
        auxiliary::check_due(),
    )
}
