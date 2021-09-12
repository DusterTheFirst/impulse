//! Impulse Rocket Simulation Tool

#![deny(unsafe_code)]

use std::{panic::catch_unwind, thread};

use crossfire::mpmc;
use iced::{window, Application, Settings};
use sim::simulation_thread;
use tracing::{info, warn};
use tracing_subscriber::{filter::Directive, EnvFilter};
use ui::{Counter, SimulationCommunication};

mod model;
mod sim;
mod ui;

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive(
                if cfg!(debug_assertions) {
                    "impulse=trace"
                } else {
                    "impulse=info"
                }
                .parse()
                .unwrap(),
            ),
        )
        .init();

    let (to_sim, from_ui) = mpmc::bounded_tx_future_rx_blocking(10);
    let (to_ui, from_sim) = mpmc::bounded_tx_blocking_rx_future(10);

    let sim_thread = thread::spawn(move || {
        simulation_thread(to_ui, from_ui);
    });

    Counter::run(Settings {
        window: window::Settings {
            size: (750, 500),
            ..Default::default()
        },
        antialiasing: true,
        flags: SimulationCommunication {
            to_sim,
            from_sim,
            sim_thread: sim_thread.thread().clone(),
        },
        default_font: Settings::<()>::default().default_font,
        default_text_size: Settings::<()>::default().default_text_size,
        exit_on_close_request: Settings::<()>::default().exit_on_close_request,
    })
}
