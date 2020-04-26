use client_test::test_cases::pre_game_stream;
use std::error::Error;

/// I implemented my own (very simple) test execution framework because **I want to see stdout**
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pre_game_stream::run(game_id()).await?;
    success("pre_game_stream");

    Ok(())
}

fn game_id() -> String {
    format!("{:x}", rand::random::<u64>())
}

fn success(test_name: &'static str) {
    println!();
    println!("GREEN: {}", test_name);
    println!();
}