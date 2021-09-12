use std::{fmt::Debug, thread::Thread};

use crossfire::mpmc::{RxFuture, SharedSenderBRecvF, SharedSenderFRecvB, TxFuture};
use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Container, Element,
    Length, Subscription, Text,
};
use tracing::trace;

use crate::{
    model::{SimulationControl, SimulationEvent, SimulationStatus},
    sim,
};

pub struct Counter {
    simulation_status: Option<SimulationStatus>,

    simulation: SimulationCommunication,

    button_control_sim: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    StartSimulation,
    StopSimulation,
    PendAction,
    SimulationEvent(SimulationEvent),
}

pub struct SimulationCommunication {
    pub to_sim: TxFuture<SimulationControl, SharedSenderFRecvB>,
    pub from_sim: RxFuture<SimulationEvent, SharedSenderBRecvF>,
    pub sim_thread: Thread,
}

impl Application for Counter {
    type Message = Message;

    type Executor = executor::Default;

    type Flags = SimulationCommunication;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                simulation: flags,

                simulation_status: None,

                button_control_sim: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!(
            "Impulse Rocket Simulator ({})",
            self.simulation_status
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| "Pending".into())
        )
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        sim::subscribe(self.simulation.from_sim.clone()).map(Message::SimulationEvent)
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        let send_to_sim = |control| {
            let to_sim = self.simulation.to_sim.clone();
            let sim_thread = self.simulation.sim_thread.clone();

            trace!(?control, "Sending control signal to sim");

            Command::perform(
                async move {
                    sim_thread.unpark();
                    to_sim.send(control).await
                },
                |_| Message::PendAction,
            )
        };

        match message {
            Message::StartSimulation => send_to_sim(SimulationControl::Start),
            Message::StopSimulation => send_to_sim(SimulationControl::Stop),
            Message::PendAction => {
                self.simulation_status.take();

                Command::none()
            }
            Message::SimulationEvent(e) => match e {
                SimulationEvent::StatusUpdate(status) => {
                    self.simulation_status.replace(status);

                    Command::none()
                }
            },
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Column::new()
            .align_items(Align::Center)
            .push(Text::new(format!(
                "Simulation Status: {}",
                self.simulation_status
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "Pending".into())
            )));

        let content = if let Some(status) = self.simulation_status {
            let (button_label, button_message) = match status {
                SimulationStatus::Idle
                | SimulationStatus::Complete
                | SimulationStatus::Cancelled => ("Start Simulation", Message::StartSimulation),
                SimulationStatus::Running => ("Stop Simulation", Message::StopSimulation),
            };

            content.push(
                Button::new(&mut self.button_control_sim, Text::new(button_label))
                    .on_press(button_message),
            )
        } else {
            content
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }
}
