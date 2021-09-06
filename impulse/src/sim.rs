use crossfire::mpsc::{RecvError, RxBlocking, SharedSenderBRecvF, SharedSenderFRecvB, TxBlocking};

pub fn subscribe() {
    
}

pub fn simulation_thread(
    to_ui: TxBlocking<(), SharedSenderBRecvF>,
    from_ui: RxBlocking<(), SharedSenderFRecvB>,
) {
    loop {
        let data = match from_ui.recv() {
            Ok(d) => d,
            Err(RecvError) => {
                eprintln!("UI thread closed... Closing sim thread");

                break;
            }
        };

        to_ui.send(data).unwrap(); // FIXME: NO UNWRAP
    }
}
