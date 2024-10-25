const FACTOR: u32 = 3; 
static mut REQUEST_COUNTER: u8 = 0;
static mut RUN_COUNT: u32 = 0;

pub fn due_activation(param: u8) -> bool {
    unsafe {
        REQUEST_COUNTER = (REQUEST_COUNTER + 1) % 5;
        REQUEST_COUNTER == param
    }
}

pub fn check_due() -> bool {
    unsafe {
        RUN_COUNT = (RUN_COUNT + 1) % 1_000;
        let divisor = RUN_COUNT / FACTOR;
        (divisor * FACTOR) == RUN_COUNT
    }
}

