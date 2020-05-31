use crate::test_cases::love_letter_happy_path::pre_game::run_lvle_pregame;
use crate::test_cases::love_letter_happy_path::simple_ai::run_simple_game_ai;
use std::error::Error;
use tokio::sync::mpsc;

pub struct Config {
    pub game_id: String,
    // TODO:3 implement integ test with variable number of players
    pub players: [String; 3],
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // -- pregame --
    let (
        stream1,
        stream2,
        stream3,
    ) = run_lvle_pregame(config).await;

    // -- game --
    let (done_send, mut done_recv) = mpsc::channel(3);
    let mut done_send1 = done_send.clone();
    let mut done_send2 = done_send.clone();
    let mut done_send3 = done_send.clone();
    drop(done_send);

    tokio::task::spawn(async move {
        run_simple_game_ai(stream1).await;
        let _ = done_send1.send(()).await;
    });
    tokio::task::spawn(async move {
        run_simple_game_ai(stream2).await;
        let _ = done_send2.send(()).await;
    });
    tokio::task::spawn(async move {
        run_simple_game_ai(stream3).await;
        let _ = done_send3.send(()).await;
    });

    while let Some(()) = done_recv.recv().await {
        // There's probably a better way to do this :P
    }

    Ok(())
}
