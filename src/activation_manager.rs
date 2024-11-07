use crate::app::Mono;
use fugit::{Duration, Instant};
use rtic_monotonics::Monotonic;

const MONO_FREQ: u32 = 16_000_000;

pub static START: Option<Instant<u64, 1, MONO_FREQ>> =
    Instant::<u64, 1, MONO_FREQ>::from_ticks(0)
        .checked_add_duration(Duration::<u64, 1, 1>::from_ticks(2)); // 2 seconds for elaboration

pub async fn activation_cyclic() -> Instant<u64, 1, 16000000> {
     let activation_time = START.unwrap();
    Mono::delay_until(activation_time).await;
    activation_time
}

pub async fn activation_sporadic() {
    Mono::delay_until(START.unwrap()).await;
}
