/// Wrappers of the `oneof` message type in protobuf.
mod oneof_wrappers {
    use crate::wire_api::proto_frj_ngn::proto_pre_game_message;
    use crate::wire_api::proto_frj_ngn::proto_pre_game_message::{
        ProtoGameStartMsg, ProtoJoinGameAck, ProtoPlayerJoinMsg,
    };
    use crate::wire_api::proto_frj_ngn::ProtoPreGameMessage;

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
}
