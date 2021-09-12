use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    thread,
};

use crossfire::{
    mpmc::{RxBlocking, RxFuture, SharedSenderBRecvF, SharedSenderFRecvB, TxBlocking},
    mpsc::TryRecvError,
};
use iced::Subscription;
use iced_futures::{subscription::Recipe, BoxStream};
use tracing::{debug, info, trace, warn};

use crate::model::{SimulationControl, SimulationEvent, SimulationStatus};

/// Get a subscription to the events emitted from the simulation thread
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
    from_ui: RxBlocking<SimulationControl, SharedSenderFRecvB>,
) {
    simulation_thread_internal(to_ui, from_ui);

    warn!("Unable to communicate to the UI thread");
    info!("Shutting down simulation thread");
}

fn simulation_thread_internal(
    to_ui: TxBlocking<SimulationEvent, SharedSenderBRecvF>,
    from_ui: RxBlocking<SimulationControl, SharedSenderFRecvB>,
) -> Option<()> {
    let mut status = SimulationStatus::Idle;

    let update_status = |old_status: &mut _, new_status| {
        *old_status = new_status;

        to_ui.send(SimulationEvent::StatusUpdate(new_status)).ok()
    };

    loop {
        debug!("Parking simulation thread");
        update_status(&mut status, SimulationStatus::Idle);
        thread::park();

        loop {
            let control = match from_ui.try_recv() {
                Ok(e) => Some(e),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => todo!(),
            };

            trace!(?status, ?control);

            match (control, status) {
                (None, SimulationStatus::Idle) => {
                    break;
                }
                (None, _) => unimplemented!("Simulation not setup yet, but this should tick it"),
                (Some(SimulationControl::Start), SimulationStatus::Idle) => {
                    update_status(&mut status, SimulationStatus::Running)?;
                }
                (Some(SimulationControl::Stop), SimulationStatus::Running) => {
                    update_status(&mut status, SimulationStatus::Cancelled)?;
                    break;
                }
                _ => unimplemented!(),
            }
        }
    }
}
