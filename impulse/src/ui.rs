use std::fmt::Debug;

use crossfire::mpmc::{RxFuture, SharedSenderBRecvF, SharedSenderFRecvB, TxFuture};
use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Container, Element,
    Length, Subscription, Text,
};

use crate::{
    model::{InterfaceEvent, SimulationEvent, SimulationStatus},
    sim,
};

pub struct Counter {
    simulation_status: SimulationStatus,

    channels: UIChannels,

    button_control_sim: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    StartSimulation,
    StopSimulation,
    DisableSimulationButton,
    SimulationEvent(SimulationEvent),
}

pub struct UIChannels {
    pub to_sim: TxFuture<InterfaceEvent, SharedSenderFRecvB>,
    pub from_sim: RxFuture<SimulationEvent, SharedSenderBRecvF>,
}

impl Application for Counter {
    type Message = Message;

    type Executor = executor::Default;

    type Flags = UIChannels;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                channels: flags,

                simulation_status: SimulationStatus::Idle,

                button_control_sim: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Impulse Rocket Simulator ({})", self.simulation_status)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        sim::subscribe(self.channels.from_sim.clone()).map(Message::SimulationEvent)
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        let to_sim = self.channels.to_sim.clone();

        match message {
            Message::StartSimulation => {
                Command::perform(
                    async move { to_sim.send(InterfaceEvent::StartSimulation).await },
                    |_| Message::DisableSimulationButton, /* FIXME: */
                )
            }
            Message::StopSimulation => todo!(),
            Message::DisableSimulationButton => {
                self.simulation_status = SimulationStatus::Pending;

                Command::none()
            }
            Message::SimulationEvent(e) => match e {
                SimulationEvent::StatusUpdate(status) => {
                    self.simulation_status = status;

                    Command::none()
                }
            },
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let button_properties = match self.simulation_status {
            SimulationStatus::Idle | SimulationStatus::Complete | SimulationStatus::Cancelled => {
                Some(("Start Simulation", Message::StartSimulation))
            }
            SimulationStatus::Running => Some(("Stop Simulation", Message::StopSimulation)),
            SimulationStatus::Pending => None,
        };

        let content = Column::new()
            .align_items(Align::Center)
            .push(Text::new(format!(
                "Simulation Status: {}",
                self.simulation_status
            )));

        let content = if let Some((button_label, button_message)) = button_properties {
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
