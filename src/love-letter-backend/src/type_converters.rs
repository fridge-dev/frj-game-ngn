use crate::events::{Card, PlayCardSource};
use crate::types::{StagedPlay, CommittedPlayOutcome};
use backend_framework::wire_api::proto_frj_ngn::{ProtoLvLeCard, ProtoLvLeCommittedPlay, ProtoLvLeCardSelection, proto_lv_le_card_selection};
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

impl From<CommittedPlayOutcome> for ProtoLvLeCommittedPlay {
    fn from(committed_play: CommittedPlayOutcome) -> Self {
        // TODO:1 fix conversion. Proto model wasn't updated after adding some fields to the app layer classes.
        match committed_play {
            CommittedPlayOutcome::Guard { target_player_id, guessed_card, correct } => {
                let selection = ProtoGuardSelection {
                    opt_player_id: target_player_id,
                    opt_card: ProtoLvLeCard::from(guessed_card) as i32,
                };
                let outcome = ProtoGuardOutcome {
                    correct
                };

                ProtoLvLeCommittedPlay::from((selection, outcome))
            },
            CommittedPlayOutcome::Priest { target_player_id, opponent_card } => {
                let selection = ProtoPriestSelection {
                    opt_player_id: target_player_id
                };

                ProtoLvLeCommittedPlay::from(selection)
            },
            CommittedPlayOutcome::Baron {
                target_player_id,
                eliminated_player_id,
                committer_card,
                opponent_card
            } => {
                let selection = ProtoBaronSelection {
                    opt_player_id: target_player_id
                };
                let outcome = ProtoBaronOutcome {
                    losing_player_id: eliminated_player_id.expect("TODO:3 don't panic in a From"),
                    losing_player_card: ProtoLvLeCard::from(Card::Guard) as i32
                };

                ProtoLvLeCommittedPlay::from((selection, outcome))
            },
            CommittedPlayOutcome::Handmaid => {
                ProtoLvLeCommittedPlay::empty()
            },
            CommittedPlayOutcome::Prince { target_player_id, discarded_card } => {
                let selection = ProtoPrinceSelection {
                    opt_player_id: target_player_id
                };
                let outcome = ProtoPrinceOutcome {
                    discarded_card: ProtoLvLeCard::from(discarded_card) as i32
                };

                ProtoLvLeCommittedPlay::from((selection, outcome))
            },
            CommittedPlayOutcome::King { target_player_id } => {
                let selection = ProtoKingSelection {
                    opt_player_id: target_player_id
                };

                ProtoLvLeCommittedPlay::from(selection)
            },
            CommittedPlayOutcome::Countess => {
                ProtoLvLeCommittedPlay::empty()
            },
            CommittedPlayOutcome::Princess => {
                ProtoLvLeCommittedPlay::empty()
            }
        }
    }
}

impl From<StagedPlay> for ProtoLvLeCardSelection {
    fn from(staged_play: StagedPlay) -> Self {
        let opt = |s: Option<String>| s.unwrap_or("".to_string());

        let proto_selection = match staged_play.played_card {
            Card::Guard => Some(proto_lv_le_card_selection::Inner::Guard(ProtoGuardSelection {
                opt_player_id: opt(staged_play.target_player),
                opt_card: staged_play.target_card.map(|c| ProtoLvLeCard::from(c) as i32).unwrap_or(0),
            })),
            Card::Priest => Some(proto_lv_le_card_selection::Inner::Priest(ProtoPriestSelection {
                opt_player_id: opt(staged_play.target_player),
            })),
            Card::Baron => Some(proto_lv_le_card_selection::Inner::Baron(ProtoBaronSelection {
                opt_player_id: opt(staged_play.target_player),
            })),
            Card::Prince => Some(proto_lv_le_card_selection::Inner::Prince(ProtoPrinceSelection {
                opt_player_id: opt(staged_play.target_player),
            })),
            Card::King => Some(proto_lv_le_card_selection::Inner::King(ProtoKingSelection {
                opt_player_id: opt(staged_play.target_player),
            })),
            _ => None
        };

        ProtoLvLeCardSelection {
            inner: proto_selection
        }
    }
}
