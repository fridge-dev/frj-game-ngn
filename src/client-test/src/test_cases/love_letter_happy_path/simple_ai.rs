use crate::client::{LoggingBiStream, LoggingStreamRecv, LoggingStreamSender};
use client_engine::wire_api::proto_frj_ngn::{ProtoLoveLetterDataIn, ProtoLoveLetterDataOut, ProtoLvLeGameState, ProtoGameDataReadyUpClick, ProtoLvLePlayCardReq, ProtoLvLeSelectTargetPlayer, ProtoLvLeSelectTargetCard, ProtoLvLeCommitSelectionReq, ProtoLvLeCard};
use client_engine::wire_api::proto_frj_ngn::proto_love_letter_data_out::ProtoLvLeOut;
use client_engine::wire_api::proto_frj_ngn::proto_lv_le_game_state::Stage;
use client_engine::wire_api::proto_frj_ngn::proto_love_letter_data_in::ProtoLvLeIn;
use client_engine::wire_api::proto_frj_ngn::proto_lv_le_game_state::proto_lv_le_round_state::Turn;
use client_engine::wire_api::proto_frj_ngn::proto_lv_le_play_card_req::ProtoLvLeCardSource;

/// This AI has simple rules:
/// 1. Always keep higher value card (if possible, i.e. Countess)
/// 2. Always select first (allowed) player
/// 3. When playing Guard, always select Princess
/// 4. Disconnect after 5 rounds
///
/// The state handling is messy and just enough to do the job. Don't judge me, I am planning to
/// finish this game implementation ASAP and move on to doing others more thoroughly.
pub async fn run_simple_game_ai(bi_stream: LoggingBiStream<ProtoLoveLetterDataIn, ProtoLoveLetterDataOut>) {
    let sender = bi_stream.sender;
    let mut game_state_receiver = GameStateReceiver(bi_stream.receiver);
    let my_player_id = bi_stream.my_player_id;

    let mut num_rounds_to_play = 10u8;
    let mut is_round_intermission = false;
    let mut skip_my_turn_actions = false;

    loop {
        let payload = game_state_receiver.recv().await;

        match payload.stage.unwrap() {
            Stage::RoundInProgress(round_state) => {
                is_round_intermission = false;
                match round_state.turn.unwrap() {
                    Turn::MyDrawnCard(top_deck) => {
                        if !skip_my_turn_actions {
                            assert!(round_state.staged_play.is_none());
                            let mut remaining_player_ids = round_state.remaining_player_ids;
                            let handmaid_player_ids = round_state.handmaid_player_ids;
                            remaining_player_ids.retain(|x| !handmaid_player_ids.contains(x));
                            take_my_turn(
                                &sender,
                                ProtoLvLeCard::from_i32(top_deck).unwrap(),
                                ProtoLvLeCard::from_i32(round_state.my_hand).unwrap(),
                                remaining_player_ids,
                                &my_player_id
                            );
                            skip_my_turn_actions = true;
                        }
                    },
                    Turn::CurrentTurnPlayerId(_) => {
                        // Do nothing (keep polling, just wait for our turn)
                        skip_my_turn_actions = false;
                    },
                    Turn::TurnIntermission(_) => {
                        // Send (and re-send (idempotent)) the ready up message
                        sender.send_lvle(ProtoLvLeIn::ReadyUp(ProtoGameDataReadyUpClick {}));
                        skip_my_turn_actions = false;
                    },
                }
            },
            Stage::RoundIntermission(round_result) => {
                skip_my_turn_actions = false;
                if !is_round_intermission {
                    is_round_intermission = true;
                    num_rounds_to_play -= 1;
                    if num_rounds_to_play == 0 {
                        println!("-- ({}) Game complete: {:#?}", &my_player_id, payload.players);
                        break;
                    } else {
                        println!("-- ({}) Round complete: {:#?}", &my_player_id, round_result.final_cards);
                        sender.send_lvle(ProtoLvLeIn::ReadyUp(ProtoGameDataReadyUpClick {}));
                    }
                }
            },
        }
    }
}

fn take_my_turn(
    sender: &LoggingStreamSender<ProtoLoveLetterDataIn>,
    top_deck_card: ProtoLvLeCard,
    my_hand: ProtoLvLeCard,
    mut remaining_non_handmaid_player_ids: Vec<String>,
    my_player_id: &String,
) {
    let mut card_source = if top_deck_card < my_hand {
        ProtoLvLeCardSource::TopDeck
    } else {
        ProtoLvLeCardSource::Hand
    };
    if ProtoLvLeCard::Countess == top_deck_card
        && (ProtoLvLeCard::Prince == my_hand || ProtoLvLeCard::King == my_hand) {
        card_source = ProtoLvLeCardSource::TopDeck;
    }
    if ProtoLvLeCard::Countess == my_hand
        && (ProtoLvLeCard::Prince == top_deck_card || ProtoLvLeCard::King == top_deck_card) {
        card_source = ProtoLvLeCardSource::Hand;
    }

    // For example, in a 1v1 situation, if someone plays handmaid, then we allow targeting self,
    // which results in a no-op (expect for Prince).
    remaining_non_handmaid_player_ids.retain(|x| x != my_player_id);
    let target_player_id = if !remaining_non_handmaid_player_ids.is_empty() {
        remaining_non_handmaid_player_ids.remove(0)
    } else {
        my_player_id.to_string()
    };
    let target_card = ProtoLvLeCard::Princess as i32;

    // Staging
    sender.send_lvle(ProtoLvLeIn::PlayCard(ProtoLvLePlayCardReq {
        card_source: card_source as i32
    }));

    // Selection
    let played_card = match card_source {
        ProtoLvLeCardSource::Hand => my_hand,
        ProtoLvLeCardSource::TopDeck => top_deck_card,
        ProtoLvLeCardSource::UnspecifiedCardSource => panic!("UnspecifiedCardSource"),
    };
    match played_card {
        ProtoLvLeCard::Guard => {
            sender.send_lvle(ProtoLvLeIn::SelectTargetPlayer(ProtoLvLeSelectTargetPlayer {
                target_player_id
            }));
            sender.send_lvle(ProtoLvLeIn::SelectTargetCard(ProtoLvLeSelectTargetCard {
                target_card
            }));
        },
        ProtoLvLeCard::Priest | ProtoLvLeCard::Baron | ProtoLvLeCard::Prince | ProtoLvLeCard::King => {
            sender.send_lvle(ProtoLvLeIn::SelectTargetPlayer(ProtoLvLeSelectTargetPlayer {
                target_player_id
            }));
        },
        ProtoLvLeCard::Handmaid | ProtoLvLeCard::Countess | ProtoLvLeCard::Princess => { /* no-op */ },
        ProtoLvLeCard::UnspecifiedLoveLetterCard => panic!("UnspecifiedLoveLetterCard"),
    }

    // Commit
    sender.send_lvle(ProtoLvLeIn::CommitSelection(ProtoLvLeCommitSelectionReq {}));
}

struct GameStateReceiver(LoggingStreamRecv<ProtoLoveLetterDataOut>);
impl GameStateReceiver {
    pub async fn recv(&mut self) -> ProtoLvLeGameState {
        let msg = self.0
            .recv_data("AI stream")
            .await
            .proto_lv_le_out
            .expect("payload missing");

        match msg {
            ProtoLvLeOut::GameState(game_state) => game_state,
            _ => panic!("Received unexpected message: {:?}", msg),
        }
    }
}