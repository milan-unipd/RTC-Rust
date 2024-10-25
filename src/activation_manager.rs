use rtic_monotonics::Monotonic;
use crate::app::Mono;
use fugit::{Duration, Instant};
use crate::constants::*;

pub async fn activation_cyclic() -> Instant<u64, 1, 16000000>  {
    let activation_time 
        = Mono::now()
            .checked_add_duration( Duration::<u64, 1,TICKS_PER_SECOND>::from_ticks(TICKS_PER_SECOND_64 *2 ))
            .expect("Time error");
    
    Mono::delay_until(activation_time).await;
    activation_time
}