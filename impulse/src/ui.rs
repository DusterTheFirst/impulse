use std::fmt::Debug;

use crossfire::mpsc::{RxFuture, SharedSenderBRecvF, SharedSenderFRecvB, TxFuture};
use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Container, Element,
    Length, Subscription, Text,
};

use crate::model::SimulationStatus;

pub struct Counter {
    simulation_status: SimulationStatus,

    channels: UIChannels,

    button_control_sim: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    None,
    StartSimulation,
    StopStimulation,
}

pub struct UIChannels {
    pub to_sim: TxFuture<(), SharedSenderFRecvB>,
    pub from_sim: RxFuture<(), SharedSenderBRecvF>,
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
        Subscription::none()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::None => Command::none(),
            Message::StartSimulation | Message::StopStimulation => {
                Command::perform(
                    {
                        let to_sim = self.channels.to_sim.clone();

                        async move {
                            match message {
                                Message::StartSimulation => to_sim.send(()).await, // FIXME:
                                Message::StopStimulation => to_sim.send(()).await,
                                _ => unreachable!(),
                            }
                            .unwrap();
                        }
                    },
                    |_| Message::None, /* FIXME: */
                )
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let (button_label, button_message) = match self.simulation_status {
            SimulationStatus::Idle | SimulationStatus::Complete | SimulationStatus::Cancelled => {
                ("Start Simulation", Message::StartSimulation)
            }
            SimulationStatus::Running => ("Stop Simulation", Message::StopStimulation),
        };

        let content = Column::new()
            .align_items(Align::Center)
            .push(Text::new(format!(
                "Simulation Status: {}",
                self.simulation_status
            )))
            .push(
                Button::new(&mut self.button_control_sim, Text::new(button_label))
                    .on_press(button_message),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }
}
