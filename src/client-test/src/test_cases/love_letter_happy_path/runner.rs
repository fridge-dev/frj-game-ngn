use crate::test_cases::love_letter_happy_path::pre_game::run_lvle_pregame;
use crate::test_cases::love_letter_happy_path::simple_ai::run_simple_game_ai;
use std::error::Error;
use tokio::sync::oneshot;

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
    let (done_send1, done_recv1) = oneshot::channel();
    let (done_send2, done_recv2) = oneshot::channel();
    let (done_send3, done_recv3) = oneshot::channel();

    tokio::task::spawn(async move {
        run_simple_game_ai(stream1).await;
        let _ = done_send1.send(());
    });
    tokio::task::spawn(async move {
        run_simple_game_ai(stream2).await;
        let _ = done_send2.send(());
    });
    tokio::task::spawn(async move {
        run_simple_game_ai(stream3).await;
        let _ = done_send3.send(());
    });

    let _ = done_recv1.await;
    println!("==== DONE1");
    let _ = done_recv2.await;
    println!("==== DONE2");
    let _ = done_recv3.await;
    println!("==== DONE3");

    Ok(())
}
