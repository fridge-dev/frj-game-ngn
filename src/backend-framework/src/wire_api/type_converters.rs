/// Wrappers of the `oneof` message type in protobuf.
mod oneof_wrappers {
    use crate::wire_api::proto_frj_ngn::proto_love_letter_data_out::ProtoLvLeOut;
    use crate::wire_api::proto_frj_ngn::proto_pre_game_message::{
        ProtoGameStartMsg, ProtoJoinGameAck, ProtoPlayerJoinMsg,
    };
    use crate::wire_api::proto_frj_ngn::ProtoPreGameMessage;
    use crate::wire_api::proto_frj_ngn::{
        proto_pre_game_message, ProtoLoveLetterDataOut, ProtoLvLeGameState,
    };

    impl From<ProtoJoinGameAck> for ProtoPreGameMessage {
        fn from(msg: ProtoJoinGameAck) -> Self {
            ProtoPreGameMessage {
                inner: Some(proto_pre_game_message::Inner::JoinGameAck(msg)),
            }
        }
    }

    impl From<ProtoPlayerJoinMsg> for ProtoPreGameMessage {
        fn from(msg: ProtoPlayerJoinMsg) -> Self {
            ProtoPreGameMessage {
                inner: Some(proto_pre_game_message::Inner::PlayerJoinMsg(msg)),
            }
        }
    }

    impl From<ProtoGameStartMsg> for ProtoPreGameMessage {
        fn from(msg: ProtoGameStartMsg) -> Self {
            ProtoPreGameMessage {
                inner: Some(proto_pre_game_message::Inner::GameStartMsg(msg)),
            }
        }
    }

    impl From<ProtoLvLeGameState> for ProtoLoveLetterDataOut {
        fn from(game_state: ProtoLvLeGameState) -> Self {
            ProtoLoveLetterDataOut {
                clock: 0,
                proto_lv_le_out: Some(ProtoLvLeOut::GameState(game_state)),
            }
        }
    }
}

/// All enums need a convert method like this because prost generated a `from_i32` method via macros
/// which doesn't actually exist in my IDE. Maybe I'm being too IDE dependent, but I hate stuff like
/// this. So I will create explicit methods and contain the "dark magic" within these small methods.
///
/// See https://github.com/danburkert/prost/issues/69
///
/// It also has the benefit of validating the i32 type is a known value and returning err via `TryFrom`.
mod enum_converters {
    use crate::wire_api::proto_frj_ngn::ProtoGameType;
    use std::convert::TryFrom;
    use tonic::{Code, Status};

    impl TryFrom<i32> for ProtoGameType {
        type Error = Status;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            ProtoGameType::from_i32(value).ok_or(Status::new(
                Code::InvalidArgument,
                format!("Illegal GameType i32 value '{}'", value),
            ))
        }
    }

    mod love_letter {
        use crate::wire_api::proto_frj_ngn::proto_lv_le_play_card_req::ProtoLvLeCardSource;
        use crate::wire_api::proto_frj_ngn::ProtoLvLeCard;
        use std::convert::TryFrom;
        use tonic::{Code, Status};

        impl TryFrom<i32> for ProtoLvLeCardSource {
            type Error = Status;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                ProtoLvLeCardSource::from_i32(value).ok_or(Status::new(
                    Code::InvalidArgument,
                    format!("Illegal CardSource i32 value '{}'", value),
                ))
            }
        }

        impl TryFrom<i32> for ProtoLvLeCard {
            type Error = Status;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                ProtoLvLeCard::from_i32(value).ok_or(Status::new(
                    Code::InvalidArgument,
                    format!("Illegal LvLeCard i32 value '{}'", value),
                ))
            }
        }
    }
}

/// Non-enum and non-oneof converters
mod normal_converters {
    use crate::common_types::ClientInfo;
    use crate::wire_api::proto_frj_ngn::proto_lv_le_card_outcome::{
        ProtoBaronOutcome, ProtoGuardOutcome, ProtoPrinceOutcome,
    };
    use crate::wire_api::proto_frj_ngn::proto_lv_le_card_selection::{
        ProtoBaronSelection, ProtoGuardSelection, ProtoKingSelection, ProtoPriestSelection,
        ProtoPrinceSelection,
    };
    use crate::wire_api::proto_frj_ngn::{
        proto_lv_le_card_outcome, proto_lv_le_card_selection, ProtoGameDataHandshake,
        ProtoGameType, ProtoLvLeCardOutcome, ProtoLvLeCardSelection, ProtoLvLeCommittedPlay,
    };

    impl From<ProtoGameDataHandshake> for ClientInfo {
        fn from(handshake: ProtoGameDataHandshake) -> Self {
            if handshake.game_type != ProtoGameType::LoveLetter as i32 {
                println!("INFO: Invalid game type in Handshake message. Panicking because this is a dead branch that will be deleted soon.");
                panic!("TODO:2 clean this up by removing game type");
            }

            ClientInfo {
                player_id: handshake.player_id,
                game_id: handshake.game_id,
            }
        }
    }

    impl From<(ProtoGuardSelection, ProtoGuardOutcome)> for ProtoLvLeCommittedPlay {
        fn from((selection, outcome): (ProtoGuardSelection, ProtoGuardOutcome)) -> Self {
            ProtoLvLeCommittedPlay {
                selection: Some(ProtoLvLeCardSelection {
                    inner: Some(proto_lv_le_card_selection::Inner::Guard(selection)),
                }),
                outcome: Some(ProtoLvLeCardOutcome {
                    inner: Some(proto_lv_le_card_outcome::Inner::Guard(outcome)),
                }),
            }
        }
    }

    impl From<ProtoPriestSelection> for ProtoLvLeCommittedPlay {
        fn from(selection: ProtoPriestSelection) -> Self {
            ProtoLvLeCommittedPlay {
                selection: Some(ProtoLvLeCardSelection {
                    inner: Some(proto_lv_le_card_selection::Inner::Priest(selection)),
                }),
                outcome: None,
            }
        }
    }

    impl From<(ProtoBaronSelection, ProtoBaronOutcome)> for ProtoLvLeCommittedPlay {
        fn from((selection, outcome): (ProtoBaronSelection, ProtoBaronOutcome)) -> Self {
            ProtoLvLeCommittedPlay {
                selection: Some(ProtoLvLeCardSelection {
                    inner: Some(proto_lv_le_card_selection::Inner::Baron(selection)),
                }),
                outcome: Some(ProtoLvLeCardOutcome {
                    inner: Some(proto_lv_le_card_outcome::Inner::Baron(outcome)),
                }),
            }
        }
    }

    impl From<(ProtoPrinceSelection, ProtoPrinceOutcome)> for ProtoLvLeCommittedPlay {
        fn from((selection, outcome): (ProtoPrinceSelection, ProtoPrinceOutcome)) -> Self {
            ProtoLvLeCommittedPlay {
                selection: Some(ProtoLvLeCardSelection {
                    inner: Some(proto_lv_le_card_selection::Inner::Prince(selection)),
                }),
                outcome: Some(ProtoLvLeCardOutcome {
                    inner: Some(proto_lv_le_card_outcome::Inner::Prince(outcome)),
                }),
            }
        }
    }

    impl From<ProtoKingSelection> for ProtoLvLeCommittedPlay {
        fn from(selection: ProtoKingSelection) -> Self {
            ProtoLvLeCommittedPlay {
                selection: Some(ProtoLvLeCardSelection {
                    inner: Some(proto_lv_le_card_selection::Inner::King(selection)),
                }),
                outcome: None,
            }
        }
    }

    impl ProtoLvLeCommittedPlay {
        pub fn empty() -> Self {
            ProtoLvLeCommittedPlay {
                selection: None,
                outcome: None,
            }
        }
    }
}
