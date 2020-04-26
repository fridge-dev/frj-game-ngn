use client_engine::wire_api::proto_frj_ngn::{ProtoHostGameReq, ProtoGameType, ProtoPreGameMessage, ProtoJoinGameReq, ProtoStartGameReq};
use client_engine::wire_api::proto_frj_ngn::proto_pre_game_message::Inner;
use crate::client::{LoggingGameClient, LoggingStream};
use std::error::Error;

pub async fn run(game_id: String) -> Result<(), Box<dyn Error>> {
    // == setup ==
    let p1 = "p1".to_string();
    let p2 = "p2".to_string();
    let p3 = "p3".to_string();
    let mut client1 = LoggingGameClient::new(&p1).await.expect("connect1");
    let mut client2 = LoggingGameClient::new(&p2).await.expect("connect1");
    let mut client3 = LoggingGameClient::new(&p3).await.expect("connect1");
    let game_type = ProtoGameType::LoveLetter as i32;

    // -- p1 create new game --
    let mut p1_stream = client1.host_game(ProtoHostGameReq {
        player_id: p1.clone(),
        game_id: game_id.clone(),
        game_type,
    }).await.expect("host_game");

    if let Inner::JoinGameAck(msg) = get_next_message(&mut p1_stream, "p1_stream joinack").await {
        assert_eq!(msg.game_type, game_type);
        assert_eq!(msg.host_player_id, p1.clone());
        assert_eq!(msg.other_player_ids.len(), 0);
    } else {
        panic!("Received unexpected message.");
    }

    // -- p2 join game --
    let mut p2_stream = client2.join_game(ProtoJoinGameReq {
        player_id: p2.clone(),
        game_id: game_id.clone(),
        game_type
    }).await.expect("join_game p2");

    if let Inner::JoinGameAck(msg) = get_next_message(&mut p2_stream, "p2_stream joinack").await {
        assert_eq!(msg.game_type, game_type);
        assert_eq!(msg.host_player_id, p1.clone());
        assert_eq!(msg.other_player_ids, vec![p2.clone()]);
    } else {
        panic!("Received unexpected message.");
    }

    if let Inner::PlayerJoinMsg(msg) = get_next_message(&mut p1_stream, "p1_stream p2join").await {
        assert_eq!(msg.player_id, p2.clone());
    } else {
        panic!("Received unexpected message.");
    }

    // -- p3 join game --
    let mut p3_stream = client3.join_game(ProtoJoinGameReq {
        player_id: p3.clone(),
        game_id: game_id.clone(),
        game_type
    }).await.expect("join_game p3");

    if let Inner::JoinGameAck(msg) = get_next_message(&mut p3_stream, "p3_stream joinack").await {
        assert_eq!(msg.game_type, game_type);
        assert_eq!(msg.host_player_id, p1.clone());
        assert_eq!(msg.other_player_ids, vec![p2.clone(), p3.clone()]);
    } else {
        panic!("Received unexpected message.");
    }

    if let Inner::PlayerJoinMsg(msg) = get_next_message(&mut p1_stream, "p1_stream p3join").await {
        assert_eq!(msg.player_id, p3.clone());
    } else {
        panic!("Received unexpected message.");
    }

    if let Inner::PlayerJoinMsg(msg) = get_next_message(&mut p2_stream, "p2_stream p3join").await {
        assert_eq!(msg.player_id, p3.clone());
    } else {
        panic!("Received unexpected message.");
    }

    // -- p1 start game --
    let start_game_reply = client1.start_game(ProtoStartGameReq {
        player_id: p1.clone(),
        game_id: game_id.clone(),
        game_type
    }).await.expect("start_game p1");
    assert_eq!(start_game_reply.player_ids, vec![p1.clone(), p2.clone(), p3.clone()]);

    if let Inner::GameStartMsg(_) = get_next_message(&mut p1_stream, "p1_stream end").await {
        // it worked!
    } else {
        panic!("Received unexpected message.");
    }

    if let Inner::GameStartMsg(_) = get_next_message(&mut p2_stream, "p2_stream end").await {
        // it worked!
    } else {
        panic!("Received unexpected message.");
    }

    if let Inner::GameStartMsg(_) = get_next_message(&mut p3_stream, "p3_stream end").await {
        // it worked!
    } else {
        panic!("Received unexpected message.");
    }

    // -- verify stream closure --

    p1_stream.recv_closed("p1_stream").await;
    println!("Player 1 server closed stream.");
    p2_stream.recv_closed("p2_stream").await;
    println!("Player 2 server closed stream.");
    p3_stream.recv_closed("p3_stream").await;
    println!("Player 3 server closed stream.");

    Ok(())
}

async fn get_next_message(
    stream: &mut LoggingStream<ProtoPreGameMessage>,
    stream_name: &'static str
) -> Inner {
    stream.recv_data(stream_name)
        .await
        .inner
        .expect(&format!("Server sent invalid message to {}", stream_name))
}