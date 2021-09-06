use std::{
    any::TypeId,
    hash::{Hash, Hasher},
};

use crossfire::mpmc::{RxBlocking, RxFuture, SharedSenderBRecvF, SharedSenderFRecvB, TxBlocking};
use iced::Subscription;
use iced_futures::{subscription::Recipe, BoxStream};
use tracing::{info, trace, warn};

use crate::model::{InterfaceEvent, SimulationEvent};

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
    fn inner(
        to_ui: TxBlocking<SimulationEvent, SharedSenderBRecvF>,
        from_ui: RxBlocking<InterfaceEvent, SharedSenderFRecvB>,
    ) -> Option<()> {
        loop {
            let data = from_ui.recv().ok()?;

            trace!("Received InterfaceEvent: {:?}", data);

            // to_ui.send(SimulationEvent::StatusUpdate(SimulationStatus::Running)).unwrap(); // FIXME: NO UNWRAP
        }
    }

    inner(to_ui, from_ui);

    warn!("Unable to communicate to the UI thread");
    info!("Shutting down simulation thread");
}
