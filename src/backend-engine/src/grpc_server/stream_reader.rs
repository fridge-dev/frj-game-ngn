use tonic::{Streaming, Code, Status};

/// Somewhere between hyper and tonic, they do not elegantly handle a destroyed stream
/// where sender goes away without explicitly closing stream/connection.
///
/// Message prefix: https://github.com/hyperium/hyper/blob/master/src/error.rs#L312
const HACK_MSG_CLIENT_DIED: &str = "error reading a body from connection: broken pipe";
const HACK_MSG_CLIENT_DC: &str = "error reading a body from connection: protocol error: stream no longer needed";
fn is_stream_done(status: &Status) -> bool {
    if status.code() == Code::Unknown {
        if status.message() == HACK_MSG_CLIENT_DIED || status.message() == HACK_MSG_CLIENT_DC {
            return true;
        }
    }

    false
}

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
                    self.log_err(&status);
                    if is_stream_done(&status) {
                        break;
                    }
                },
                Ok(None) => {
                    self.log_close();
                    break;
                },
                Ok(Some(message)) => {
                    self.log_msg();
                    self.message_handler.handle_message(message);
                },
            }
        }

        println!("INFO: StreamDriver [{}] exiting event loop", self.stream_id);
    }

    fn log_err(&self, status: &Status) {
        println!(
            "ERROR: ({}) StreamDriver [{}] received Status err: {:?}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
            self.stream_id,
            status
        );
    }

    fn log_close(&self) {
        println!(
            "INFO: ({}) StreamDriver [{}] received empty message. Stream closed by client.",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
            self.stream_id
        );
    }

    fn log_msg(&self) {
        println!(
            "DEBUG: ({}) StreamDriver [{}] received message.",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
            self.stream_id
        );
    }
}