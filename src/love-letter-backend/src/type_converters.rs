use crate::events::{Card, PlayCardSource};
use crate::types::CommittedPlay;
use backend_framework::wire_api::proto_frj_ngn::{ProtoLvLeCard, ProtoLvLeCommittedPlay};
use backend_framework::wire_api::proto_frj_ngn::proto_lv_le_play_card_req::ProtoLvLeCardSource;
use backend_framework::wire_api::proto_frj_ngn::proto_lv_le_card_outcome::{ProtoGuardOutcome, ProtoBaronOutcome, ProtoPrinceOutcome};
use backend_framework::wire_api::proto_frj_ngn::proto_lv_le_card_selection::{ProtoGuardSelection, ProtoPriestSelection, ProtoBaronSelection, ProtoPrinceSelection, ProtoKingSelection};
use std::convert::TryFrom;

impl TryFrom<ProtoLvLeCardSource> for PlayCardSource {
    type Error = ();

    fn try_from(proto: ProtoLvLeCardSource) -> Result<Self, Self::Error> {
        match proto {
            ProtoLvLeCardSource::UnspecifiedCardSource => Err(()),
            ProtoLvLeCardSource::Hand => Ok(PlayCardSource::Hand),
            ProtoLvLeCardSource::TopDeck => Ok(PlayCardSource::TopDeck),
        }
    }
}

impl TryFrom<ProtoLvLeCard> for Card {
    type Error = ();

    fn try_from(proto: ProtoLvLeCard) -> Result<Self, Self::Error> {
        match proto {
            ProtoLvLeCard::UnspecifiedLoveLetterCard => Err(()),
            ProtoLvLeCard::Guard => Ok(Card::Guard),
            ProtoLvLeCard::Priest => Ok(Card::Priest),
            ProtoLvLeCard::Baron => Ok(Card::Baron),
            ProtoLvLeCard::Handmaid => Ok(Card::Handmaid),
            ProtoLvLeCard::Prince => Ok(Card::Prince),
            ProtoLvLeCard::King => Ok(Card::King),
            ProtoLvLeCard::Countess => Ok(Card::Countess),
            ProtoLvLeCard::Princess => Ok(Card::Princess),
        }
    }
}

impl From<Card> for ProtoLvLeCard {
    fn from(card: Card) -> Self {
        match card {
            Card::Guard => ProtoLvLeCard::Guard,
            Card::Priest => ProtoLvLeCard::Priest,
            Card::Baron => ProtoLvLeCard::Baron,
            Card::Handmaid => ProtoLvLeCard::Handmaid,
            Card::Prince => ProtoLvLeCard::Prince,
            Card::King => ProtoLvLeCard::King,
            Card::Countess => ProtoLvLeCard::Countess,
            Card::Princess => ProtoLvLeCard::Princess,
        }
    }
}

impl From<CommittedPlay> for ProtoLvLeCommittedPlay {
    fn from(committed_play: CommittedPlay) -> Self {
        match committed_play {
            CommittedPlay::Guard { target_player_id, target_card, correct } => {
                let selection = ProtoGuardSelection {
                    opt_player_id: target_player_id,
                    opt_card: ProtoLvLeCard::from(target_card) as i32,
                };
                let outcome = ProtoGuardOutcome {
                    correct
                };

                ProtoLvLeCommittedPlay::from((selection, outcome))
            },
            CommittedPlay::Priest { target_player_id } => {
                let selection = ProtoPriestSelection {
                    opt_player_id: target_player_id
                };

                ProtoLvLeCommittedPlay::from(selection)
            },
            CommittedPlay::Baron { target_player_id, eliminated_player_id, eliminated_card } => {
                let selection = ProtoBaronSelection {
                    opt_player_id: target_player_id
                };
                let outcome = ProtoBaronOutcome {
                    losing_player_id: eliminated_player_id,
                    losing_player_card: ProtoLvLeCard::from(eliminated_card) as i32
                };

                ProtoLvLeCommittedPlay::from((selection, outcome))
            },
            CommittedPlay::Handmaid => {
                ProtoLvLeCommittedPlay::empty()
            },
            CommittedPlay::Prince { target_player_id, discarded_card } => {
                let selection = ProtoPrinceSelection {
                    opt_player_id: target_player_id
                };
                let outcome = ProtoPrinceOutcome {
                    discarded_card: ProtoLvLeCard::from(discarded_card) as i32
                };

                ProtoLvLeCommittedPlay::from((selection, outcome))
            },
            CommittedPlay::King { target_player_id } => {
                let selection = ProtoKingSelection {
                    opt_player_id: target_player_id
                };

                ProtoLvLeCommittedPlay::from(selection)
            },
            CommittedPlay::Countess => {
                ProtoLvLeCommittedPlay::empty()
            },
        }
    }
}
