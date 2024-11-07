#![no_std]
#![no_main]

use rtic::app;

mod activation_log;
mod activation_log_reader;
mod activation_manager;
mod auxiliary;
mod external_event_server;
mod forced_interrupt;
mod on_call_producer;
mod production_worklad;
mod regular_producer;

#[app(device=esp32c3, dispatchers=[FROM_CPU_INTR0, FROM_CPU_INTR1, FROM_CPU_INTR2, FROM_CPU_INTR3])]
mod app {

    use rtic_sync::{channel::*, make_channel};

    use esp_backtrace as _;
    use esp_hal::{self as _, timer::timg::TimerGroup};

    use esp_println::println;

    use rtic_monotonics::esp32c3::prelude::*;
    esp32c3_systimer_monotonic!(Mono);

    use crate::{
        activation_log::ActivationLog, activation_log_reader, activation_manager,
        external_event_server, forced_interrupt, on_call_producer, regular_producer,
    };

    #[shared]
    struct Shared {
        activation_log: ActivationLog,
    }

    #[local]
    struct Local {}

    const CAPACITY: usize = 5;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        println!("Init !");

        let peripherals = esp_hal::init(esp_hal::Config::default());
        let mut timg0 = TimerGroup::new(peripherals.TIMG0);
        timg0.wdt.disable();

        let peripherals = cx.device;
        Mono::start(peripherals.SYSTIMER);

        let (alr_s, alr_r) = make_channel!((), CAPACITY);
        let (ocp_s, ocp_r) = make_channel!(usize, CAPACITY);
        let (ees_s, ees_r) = make_channel!((), CAPACITY);

        regular_producer_task::spawn(ocp_s, alr_s).unwrap();
        on_call_producer_task::spawn(ocp_r).unwrap();
        forced_interrupt_task::spawn(ees_s).unwrap();
        external_event_server_task::spawn(ees_r).unwrap();
        activation_log_reader_task::spawn(alr_r).unwrap();

        (
            Shared {
                activation_log: ActivationLog::new(),
            },
            Local {},
        )
    }

    #[task(priority = 7)]
    async fn regular_producer_task(
        _cx: regular_producer_task::Context,
        mut ocp_sender: Sender<'static, usize, CAPACITY>,
        mut alr_sender: Sender<'static, (), CAPACITY>,
    ) {
        let period = regular_producer::PERIOD;
        let mut next_activation = activation_manager::activation_cyclic().await;

        loop {
            let (activate_on_call_prod, activation_param, activate_log_reader) =
                regular_producer::do_work();

            if activate_on_call_prod {
                if let Err(_err) = ocp_sender.send(activation_param).await {
                    println!("Failed sporadic activation");
                }
            }

            if activate_log_reader {
                _ = alr_sender.send(()).await;
            }

            let deadline = next_activation
                .checked_add_duration(regular_producer::DEADLINE)
                .unwrap();

            if Mono::now() > deadline {
                println!("RP: Deadline MISS!");
            }

            println!("End of periodic activation");

            next_activation = next_activation
                .checked_add_duration(period)
                .expect("Time error");
            Mono::delay_until(next_activation).await;
        }
    }

    #[task(priority = 5)]
    async fn on_call_producer_task(
        _cx: on_call_producer_task::Context,
        mut receiver: Receiver<'static, usize, CAPACITY>,
    ) {
        _ = activation_manager::activation_sporadic().await;
        loop {
            let workload = receiver.recv().await.unwrap();

            let activation = Mono::now();

            on_call_producer::do_work(workload);

            let deadline = activation
                .checked_add_duration(on_call_producer::DEADLINE)
                .unwrap();
            if Mono::now() > deadline {
                println!("OCP DEADLINE MISS!");
            }

            println!("End of sporradic activation");
        }
    }

    #[task(priority = 11)]
    async fn forced_interrupt_task(
        _cx: forced_interrupt_task::Context,
        mut sender: Sender<'static, (), CAPACITY>,
    ) {
        let period = forced_interrupt::PERIOD;

        let mut next_activation = activation_manager::activation_cyclic().await;
        let mut deadline = next_activation
            .checked_add_duration(forced_interrupt::DEADLINE)
            .unwrap();

        loop {
            next_activation = next_activation
                .checked_add_duration(period)
                .expect("Time error");

            _ = sender.send(()).await;

            if Mono::now() > deadline {
                println!("FI DEADLINE MISS!");
            }
            deadline = next_activation
                .checked_add_duration(forced_interrupt::DEADLINE)
                .unwrap();

            Mono::delay_until(next_activation).await;
        }
    }

    #[task(priority = 11, shared = [activation_log])]
    async fn external_event_server_task(
        mut cx: external_event_server_task::Context,
        mut receiver: Receiver<'static, (), CAPACITY>,
    ) {
        activation_manager::activation_sporadic().await;
        loop {
            _ = receiver.recv().await;

            let activation = Mono::now();

            cx.shared.activation_log.lock(|activation_log| {
                activation_log.write();
            });

            let deadline = activation
                .checked_add_duration(external_event_server::DEADLINE)
                .unwrap();
            if Mono::now() > deadline {
                println!("EES DEADLINE MISS!",);
            }
        }
    }

    #[task(priority = 3, shared = [activation_log])]
    async fn activation_log_reader_task(
        mut cx: activation_log_reader_task::Context,
        mut receiver: Receiver<'static, (), CAPACITY>,
    ) {
        activation_manager::activation_sporadic().await;
        loop {
            _ = receiver.recv().await.unwrap();

            let activation_time = Mono::now();

            activation_log_reader::do_work();

            cx.shared.activation_log.lock(|activation_log| {
                let (_interrupt_arrival_counter, _interrupt_arrival_instantt) =
                    activation_log.read();
            });

            let deadline = activation_time
                .checked_add_duration(activation_log_reader::DEADLINE)
                .unwrap();
            if Mono::now() > deadline {
                println!("ALR DEADLINE MISS!");
            }
            println!("End of parameterless sporadic activation.");
        }
    }
}
