use crate::production_worklad;

const ACTIVATION_LOG_READER_WORKLOAD: usize = 139;

pub fn do_work() {
    let _ = production_worklad::small_whetstone(ACTIVATION_LOG_READER_WORKLOAD);
  
}
