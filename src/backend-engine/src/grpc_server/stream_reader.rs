use tonic::{Streaming, Code};

/// Somewhere between hyper and tonic, they do not elegantly handle a destroyed stream
/// where sender goes away without explicitly closing stream/connection.
///
/// Message prefix: https://github.com/hyperium/hyper/blob/master/src/error.rs#L312
const HACK_MSG_WHEN_CLIENT_DIED: &str = "error reading a body from connection: broken pipe";

/// Drives a stream until completion by polling the stream in an event loop. If the number of
/// concurrent stream grows to be extremely high (thousands), I can consider using tokio's
/// [StreamMap](https://docs.rs/tokio/0.2.20/tokio/stream/struct.StreamMap.html) to have fewer
/// tasks to schedule but each polled task does O(n) check to see if child streams are ready.
pub struct StreamDriver<M, H> where H: StreamMessageHandler<M> {
    stream_id: String,
    stream: Streaming<M>,
    message_handler: H,
}

pub trait StreamMessageHandler<M> {
    fn handle_message(&self, message: M);
}

impl<M, H> StreamDriver<M, H> where H: StreamMessageHandler<M> {

    pub fn new(
        stream_id: String,
        stream: Streaming<M>,
        message_handler: H,
    ) -> Self {
        StreamDriver {
            stream_id,
            stream,
            message_handler,
        }
    }

    pub async fn run(mut self) {
        loop {
            match self.stream.message().await {
                Err(status) => {
                    println!(
                        "ERROR: ({}) StreamDriver [{}] received Status err: {:?}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
                        self.stream_id,
                        status
                    );
                    if status.code() == Code::Unknown && status.message() == HACK_MSG_WHEN_CLIENT_DIED {
                        break;
                    }
                },
                Ok(None) => {
                    println!(
                        "INFO: ({}) StreamDriver [{}] received empty message. Stream closed by client.",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
                        self.stream_id
                    );
                    break;
                },
                Ok(Some(message)) => {
                    println!(
                        "DEBUG: ({}) StreamDriver [{}] received message.",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
                        self.stream_id
                    );
                    self.message_handler.handle_message(message);
                },
            }
        }

        println!(
            "INFO: ({}) StreamDriver [{}] exiting event loop.",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
            self.stream_id
        );
    }
}