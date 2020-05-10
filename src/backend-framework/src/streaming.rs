use std::fmt;
use std::fmt::{Debug, Formatter};
use tokio::sync::mpsc;
use tonic::Status;

pub struct StreamSender<M: prost::Message> {
    sender: mpsc::UnboundedSender<Result<M, Status>>,
}

impl<M: prost::Message> StreamSender<M> {
    pub fn new(sender: mpsc::UnboundedSender<Result<M, Status>>) -> Self {
        StreamSender {
            sender
        }
    }

    pub fn send_message(&self, message: M) -> Result<(), ()> {
        self.sender.send(Ok(message))
            .map_err(|msg| println!("WARN: Client stream dropped. We failed to send message: {:?}", msg))
    }

    pub fn send_error_message(&self, status: Status) -> Result<(), ()> {
        self.sender.send(Err(status))
            .map_err(|msg| println!("WARN: Client stream dropped. We failed to send message: {:?}", msg))
    }

    pub(crate) fn disconnect_with_err(self, status: Status) {
        let _ = self.sender.send(Err(status));
        // Drop `self` closes the stream
    }
}

impl<M: prost::Message> Debug for StreamSender<M> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "StreamSender {{...}}")
    }
}
