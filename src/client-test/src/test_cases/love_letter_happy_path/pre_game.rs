use crate::client::{LoggingGameClient, LoggingBiStream};
use crate::test_cases::love_letter_happy_path::runner::Config;
use crate::test_cases::pre_game_stream;
use client_engine::wire_api::proto_frj_ngn::{ProtoGameType, ProtoGameDataHandshake, ProtoLoveLetterDataIn, ProtoLoveLetterDataOut};
use client_engine::wire_api::proto_frj_ngn::proto_love_letter_data_in::ProtoLvLeIn;
use std::collections::HashMap;

pub async fn run_lvle_pregame(config: Config) -> (
    LoggingBiStream<ProtoLoveLetterDataIn, ProtoLoveLetterDataOut>,
    LoggingBiStream<ProtoLoveLetterDataIn, ProtoLoveLetterDataOut>,
    LoggingBiStream<ProtoLoveLetterDataIn, ProtoLoveLetterDataOut>,
) {
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
    let mut client_conns = HashMap::new();
    client_conns.insert(p1.clone(), client1.clone());
    client_conns.insert(p2.clone(), client2.clone());
    client_conns.insert(p3.clone(), client3.clone());
    let pre_game_config = pre_game_stream::Config {
        game_type: ProtoGameType::LoveLetter,
        game_id: config.game_id.clone(),
        players: config.players.clone(),
        client_conns,
    };
    pre_game_stream::run(pre_game_config).await.expect("pre_game");

    // -- data stream connect --
    let bi_stream_1 = client1.open_love_letter_stream().await.expect("p1 data_stream");
    let bi_stream_2 = client2.open_love_letter_stream().await.expect("p2 data_stream");
    let bi_stream_3 = client3.open_love_letter_stream().await.expect("p3 data_stream");

    // -- handshakes --
    bi_stream_1.sender.send_lvle(ProtoLvLeIn::Handshake(ProtoGameDataHandshake {
        player_id: p1.clone(),
        game_id: game_id.clone(),
    }));
    bi_stream_2.sender.send_lvle(ProtoLvLeIn::Handshake(ProtoGameDataHandshake {
        player_id: p2.clone(),
        game_id: game_id.clone(),
    }));
    bi_stream_3.sender.send_lvle(ProtoLvLeIn::Handshake(ProtoGameDataHandshake {
        player_id: p3.clone(),
        game_id: game_id.clone(),
    }));

    return (
        bi_stream_1,
        bi_stream_2,
        bi_stream_3,
    );
}
