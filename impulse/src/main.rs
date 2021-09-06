//! Impulse Rocket Simulation Tool

#![deny(unsafe_code)]

use std::thread;

use crossfire::mpmc;
use iced::{window, Application, Settings};
use sim::simulation_thread;
use tracing::{info, warn};
use ui::{Counter, UIChannels};

mod model;
mod sim;
mod ui;

fn main() -> iced::Result {
    tracing_subscriber::fmt().init();

    let (to_sim, from_ui) = mpmc::bounded_tx_future_rx_blocking(10);
    let (to_ui, from_sim) = mpmc::bounded_tx_blocking_rx_future(10);

    thread::spawn(move || {
        simulation_thread(to_ui, from_ui);
    });

    Counter::run(Settings {
        window: window::Settings {
            size: (750, 500),
            ..Default::default()
        },
        antialiasing: true,
        flags: UIChannels { to_sim, from_sim },
        default_font: Settings::<()>::default().default_font,
        default_text_size: Settings::<()>::default().default_text_size,
        exit_on_close_request: Settings::<()>::default().exit_on_close_request,
    })
}
