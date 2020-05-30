use crate::client::LoggingGameClient;
use crate::test_cases::pre_game_stream;
use client_engine::wire_api::proto_frj_ngn::{ProtoGameType, ProtoGameDataHandshake};
use client_engine::wire_api::proto_frj_ngn::proto_love_letter_data_in::ProtoLvLeIn;
use std::collections::HashMap;
use std::error::Error;

pub struct Config {
    pub game_id: String,
    pub players: [String; 3],
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // -- setup --
    let game_id = config.game_id.clone();
    let p1 = config.players[0].to_owned();
    let p2 = config.players[1].to_owned();
    let p3 = config.players[2].to_owned();

    // -- connect --
    let mut client1 = LoggingGameClient::new(&p1).await.expect("connect1");
    let mut client2 = LoggingGameClient::new(&p2).await.expect("connect2");
    let mut client3 = LoggingGameClient::new(&p3).await.expect("connect3");

    // -- pre game --
    let mut conns = HashMap::new();
    conns.insert(p1.clone(), client1.clone());
    conns.insert(p2.clone(), client2.clone());
    conns.insert(p3.clone(), client3.clone());
    pre_game(&config, conns).await.expect("pre_game");

    // -- data stream connect --
    let (snd1, mut rcv1) = client1.open_love_letter_stream().await.expect("p1 data_stream");
    let (snd2, mut rcv2) = client2.open_love_letter_stream().await.expect("p2 data_stream");
    let (snd3, mut rcv3) = client3.open_love_letter_stream().await.expect("p3 data_stream");

    // -- handshakes --
    snd1.send_lvle(ProtoLvLeIn::Handshake(ProtoGameDataHandshake {
        player_id: p1.clone(),
        game_id: game_id.clone(),
        game_type: ProtoGameType::LoveLetter as i32,
    }));
    snd2.send_lvle(ProtoLvLeIn::Handshake(ProtoGameDataHandshake {
        player_id: p2.clone(),
        game_id: game_id.clone(),
        game_type: ProtoGameType::LoveLetter as i32,
    }));
    snd3.send_lvle(ProtoLvLeIn::Handshake(ProtoGameDataHandshake {
        player_id: p3.clone(),
        game_id: game_id.clone(),
        game_type: ProtoGameType::LoveLetter as i32,
    }));

    // TODO:2 implement full game
    let _ = rcv1.recv_data("p1 msg1").await;
    let _ = rcv2.recv_data("p2 msg1").await;
    let _ = rcv3.recv_data("p3 msg1").await;

    Ok(())
}

async fn pre_game(config: &Config, conns: HashMap<String, LoggingGameClient>) -> Result<(), Box<dyn Error>> {
    pre_game_stream::run(pre_game_stream::Config {
        game_type: ProtoGameType::LoveLetter,
        game_id: config.game_id.clone(),
        players: config.players.clone(),
        client_conns: conns,
    }).await
}
