use crate::events::{Card, PlayCardSource};
use backend_framework::wire_api::proto_frj_ngn::ProtoLvLeCard;
use backend_framework::wire_api::proto_frj_ngn::proto_lv_le_play_card_req::ProtoLvLeCardSource;
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