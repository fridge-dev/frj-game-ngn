use tonic::Streaming;

/// Drives a stream until completion by polling the stream in an event loop. If the number of
/// concurrent stream grows to be extremely high (thousands), I can consider using tokio's
/// [StreamMap](https://docs.rs/tokio/0.2.20/tokio/stream/struct.StreamMap.html) to have fewer
/// tasks to schedule but each polled task does O(n) check to see if child streams are ready.
pub struct StreamDriver<M, H> where H: StreamMessageHandler<M> {
    stream: Streaming<M>,
    message_handler: H,
}

pub trait StreamMessageHandler<M> {
    fn handle_message(&self, message: M);
}

impl<M, H> StreamDriver<M, H> where H: StreamMessageHandler<M> {

    pub fn new(
        stream: Streaming<M>,
        message_handler: H,
    ) -> Self {
        StreamDriver {
            stream,
            message_handler,
        }
    }

    pub async fn run(mut self) {
        loop {
            match self.stream.message().await {
                Err(status) => {
                    println!("ERROR: StreamDriver received Status err from stream: {:?}", status);
                    // TODO consider "break"? When can this happen?
                },
                Ok(None) => {
                    println!("INFO: StreamDriver received empty message. Stream closed by client.");
                    break;
                },
                Ok(Some(message)) => {
                    println!("DEBUG: StreamDriver received message.");
                    self.message_handler.handle_message(message);
                },
            }
        }
    }
}