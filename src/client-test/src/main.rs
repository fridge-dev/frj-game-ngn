use client_engine::wire_api::proto_frj_ngn::ProtoGameType;
use client_test::test_cases::{pre_game_stream, love_letter_happy_path};
use std::error::Error;
use std::collections::HashMap;

/// I implemented my own (very simple) test execution framework because **I want to see stdout**
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = pre_game_stream::Config {
        game_id: game_id(),
        game_type: ProtoGameType::LoveLetter,
        players: ["p1".to_string(), "p2".to_string(), "p3".to_string()],
        client_conns: HashMap::new(),
    };
    pass_fail("pre_game_stream", pre_game_stream::run(config).await);

    let config = love_letter_happy_path::runner::Config {
        game_id: game_id(),
        players: [player_id(), player_id(), player_id()],
    };
    pass_fail("love_letter_happy_path", love_letter_happy_path::runner::run(config).await);

    Ok(())
}

fn game_id() -> String {
    format!("g-{:x}", rand::random::<u32>())
}

fn player_id() -> String {
    format!("p-{:x}", rand::random::<u32>())
}

fn pass_fail(test_name: &'static str, result: Result<(), Box<dyn Error>>) {
    match result {
        Ok(_) => {
            println!();
            println!("PASSED: {}", test_name);
            println!();
        },
        Err(error) => {
            println!();
            println!("FAILED: {}", test_name);
            println!("Err Display='{}', Debug: {:?}", error, error);
            println!();
            panic!("Test failure");
        },
    }
}