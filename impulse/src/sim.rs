use std::{
    any::TypeId,
    hash::{Hash, Hasher},
};

use crossfire::mpmc::{RxBlocking, RxFuture, SharedSenderBRecvF, SharedSenderFRecvB, TxBlocking};
use iced::Subscription;
use iced_futures::{subscription::Recipe, BoxStream};
use tracing::{info, trace, warn};

use crate::model::{InterfaceEvent, SimulationEvent, SimulationStatus};

pub fn subscribe(
    from_sim: RxFuture<SimulationEvent, SharedSenderBRecvF>,
) -> Subscription<SimulationEvent> {
    Subscription::from_recipe(SimulationSubscription { from_sim })
}

pub struct SimulationSubscription {
    from_sim: RxFuture<SimulationEvent, SharedSenderBRecvF>,
}

impl<H: Hasher, E> Recipe<H, E> for SimulationSubscription {
    type Output = SimulationEvent;

    fn hash(&self, state: &mut H) {
        TypeId::of::<Self>().hash(state);
    }

    fn stream(self: Box<Self>, _input: BoxStream<E>) -> BoxStream<Self::Output> {
        Box::pin(self.from_sim.into_stream())
    }
}

pub fn simulation_thread(
    to_ui: TxBlocking<SimulationEvent, SharedSenderBRecvF>,
    from_ui: RxBlocking<InterfaceEvent, SharedSenderFRecvB>,
) {
    simulation_thread_internal(to_ui, from_ui);

    warn!("Unable to communicate to the UI thread");
    info!("Shutting down simulation thread");
}

fn simulation_thread_internal(
    to_ui: TxBlocking<SimulationEvent, SharedSenderBRecvF>,
    from_ui: RxBlocking<InterfaceEvent, SharedSenderFRecvB>,
) -> Option<()> {
    let mut status = SimulationStatus::Idle;

    let update_status = |old_status: &mut _, new_status| {
        *old_status = new_status;

        to_ui.send(SimulationEvent::StatusUpdate(new_status)).ok()
    };

    update_status(&mut status, SimulationStatus::Idle);

    loop {
        let data = from_ui.recv().ok()?;

        trace!(%status, "Received InterfaceEvent: {:?}", data);

        match (data, status) {
            (InterfaceEvent::StartSimulation, SimulationStatus::Idle) => {
                update_status(&mut status, SimulationStatus::Running)?;
            }
            (InterfaceEvent::StopSimulation, SimulationStatus::Running) => {
                update_status(&mut status, SimulationStatus::Cancelled)?;
            }
            _ => unimplemented!(),
        }
    }
}
