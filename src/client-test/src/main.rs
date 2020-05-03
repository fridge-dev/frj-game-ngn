use client_test::test_cases::pre_game_stream;
use std::error::Error;
use client_test::test_cases::pre_game_stream::PreGameStreamThreePlayersTestConfig;
use client_engine::wire_api::proto_frj_ngn::ProtoGameType;

/// I implemented my own (very simple) test execution framework because **I want to see stdout**
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = PreGameStreamThreePlayersTestConfig {
        game_id: game_id(),
        game_type: ProtoGameType::LoveLetter,
        players: ["p1".to_string(), "p2".to_string(), "p3".to_string()],
    };
    pre_game_stream::run(config).await?;
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