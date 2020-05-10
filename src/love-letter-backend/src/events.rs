use backend_framework::common_types::ClientInfo;
use backend_framework::streaming::StreamSender;
use backend_framework::wire_api::proto_frj_ngn::ProtoLoveLetterDataOut;

#[derive(Debug)]
pub struct LoveLetterEvent {
    // TODO this unnecessarily leaks `game_id` into individual instance managers
    pub client_info: ClientInfo,
    pub payload: LoveLetterEventType,
}

#[derive(Debug)]
pub enum LoveLetterEventType {
    // Common
    RegisterDataStream(StreamSender<ProtoLoveLetterDataOut>),
    GetGameState,

    // Game-specific
    PlayCardStaged(PlayCardSource),
    SelectTargetPlayer(String),
    SelectTargetCard(Card),
    PlayCardCommit,
}

#[derive(Debug)]
pub enum PlayCardSource {
    Hand,
    TopDeck,
}

/// Input:
/// 1 - Guard    : `(String, Card)` - the player+card that is guessed
/// 2 - Priest   : `(String)` - player to view card
/// 3 - Baron    : `(String)` - player to compare with
/// 4 - Handmaid : `()`
/// 5 - Prince   : `(String)` - player to discard/replace their card
/// 6 - King     : `(String)` - player to swap with
/// 7 - Countess : `()`
/// 8 - Princess : `()`
///
/// Outcome:
/// 1 - Guard    : `(bool)` - was guess correct
/// 2 - Priest   : `()`
/// 3 - Baron    : `(String, Card)` - the player+card that was knocked out
/// 4 - Handmaid : `()`
/// 5 - Prince   : `(Card)` - the discarded card
/// 6 - King     : `(Card)` - new card received by each player
/// 7 - Countess : `()`
/// 8 - Princess : `()`
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum Card {
    /// 1 - Guesses another player's card, if correct, other player is out. Can't guess Guard(1).
    Guard,

    /// 2 - See another player's card.
    Priest,

    /// 3 - Privately compare card with another player. Lower card is out.
    Baron,

    /// 4 - Self cannot be targeted until the next turn.
    Handmaid,

    /// 5 - Choose any player (including self) to discard their card and draw a new one.
    Prince,

    /// 6 - Trade hands with any other player.
    King,

    /// 7 - Must be discarded if other card is King(6) or Prince(5).
    Countess,

    /// 8 - If this card is ever discarded, that player is out.
    Princess,
}
