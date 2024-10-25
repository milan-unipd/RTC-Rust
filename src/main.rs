#![no_std]
#![no_main]

use rtic::app;

mod activation_log;
mod activation_log_reader;
mod activation_manager;
mod auxiliary;
mod constants;
mod forced_interrupt;
mod on_call_producer;
mod production_worklad;
mod regular_producer;
mod request_buffer;

#[app(device=esp32c3, dispatchers=[FROM_CPU_INTR0, FROM_CPU_INTR1, FROM_CPU_INTR2, FROM_CPU_INTR3])]
mod app {
    use esp_backtrace as _;
    use esp_hal::{self as _};
    use esp_println::println;

    use rtic_monotonics::esp32c3::prelude::*;
    esp32c3_systimer_monotonic!(Mono);

    use crate::{
        activation_log::ActivationLog, activation_log_reader, activation_manager, forced_interrupt,
        on_call_producer, regular_producer, request_buffer::RequestBuffer,
    };

    #[shared]
    struct Shared {
        request_buffer: RequestBuffer,
        activation_log: ActivationLog,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        println!("Init");

        let timer = cx.device.SYSTIMER;
        Mono::start(timer);

        regular_producer_task::spawn().unwrap();

        (
            Shared {
                request_buffer: RequestBuffer::new(),
                activation_log: ActivationLog::new(),
            },
            Local {},
        )
    }

    #[task(priority = 7, shared = [request_buffer])]
    async fn regular_producer_task(mut cx: regular_producer_task::Context) {
        let period = regular_producer::PERIOD;
        let mut next_activation = activation_manager::activation_cyclic().await;

        loop {
            next_activation = next_activation
                .checked_add_duration(period)
                .expect("Time error");

            let (activate_on_call_prod, activation_param, activate_log_reader) =
                regular_producer::do_work();

            if activate_on_call_prod {
                cx.shared.request_buffer.lock(|request_buffer| {
                    if request_buffer.deposit(activation_param) {
                        if let Err(err) = on_call_producer_task::spawn() {
                            println!("Failed to spawn on_call_producer_task: {:?}", err);
                        }
                    } else {
                        println!("Failed sporadic activation");
                    }
                });
            }

            if activate_log_reader {
                if let Err(err) = activation_log_reader_task::spawn() {
                    println!("Failed to spawn activation_log_reader: {:?}", err);
                }
            }

            println!("End of periodic activation");
            Mono::delay_until(next_activation).await;
        }
    }

    #[task(priority = 5, shared = [request_buffer])]
    async fn on_call_producer_task(mut cx: on_call_producer_task::Context) {
        loop {
            let buffer_item = cx
                .shared
                .request_buffer
                .lock(|request_buffer| request_buffer.extract());
            if let Some(workload) = buffer_item {
                on_call_producer::do_work(workload);
                println!("End of sporradic activation");
            } else {
                return;
            }
        }
    }

    #[task]
    async fn forced_interrupt_task(_cx: forced_interrupt_task::Context) {
        let period = forced_interrupt::PERIOD;
        let mut next_activation = activation_manager::activation_cyclic().await;
        loop {
            next_activation = next_activation
                .checked_add_duration(period)
                .expect("Time error");

            if let Err(err) = external_event_server_task::spawn() {
                println!("Failed to spawn external_event_server: {:?}", err);
            }

            Mono::delay_until(next_activation).await;
        }
    }

    #[task(priority = 11, shared = [activation_log])]
    async fn external_event_server_task(mut cx: external_event_server_task::Context) {
        cx.shared.activation_log.lock(|activation_log| {
            activation_log.write();
        });
    }

    #[task(priority = 3, shared = [activation_log])]
    async fn activation_log_reader_task(mut cx: activation_log_reader_task::Context) {
        activation_log_reader::do_work();
        cx.shared.activation_log.lock(|activation_log| {
            let (_interrupt_arrival_counter, _interrupt_arrival_instantt) = activation_log.read();
        });
        println!("End of parameterless sporadic activation.");
    }
}
