use crate::events::PlayCardSource;
use crate::state_machine::{LoveLetterStateMachine, LoveLetterState};
use crate::types::{StagedPlay, RoundData};
use tonic::Status;

impl LoveLetterStateMachine {

    pub fn play_card_staged(
        &self,
        from_state: LoveLetterState,
        client_player_id: String,
        card_source: PlayCardSource
    ) -> LoveLetterState {
        match from_state {
            LoveLetterState::PlayPending(round_data) => self.handle_staging(&client_player_id, card_source, round_data),
            LoveLetterState::PlayStaging(round_data, staged_play) => self.handle_staging_idempotent(&client_player_id, round_data, staged_play),
            _ => {
                self.streams.send_err(&client_player_id, Status::failed_precondition("Can't play card while in current state"));
                from_state
            },
        }
    }

    /// We do a lot of mutations and logic very similar to how you'd imagine this being done in the
    /// physical world.
    ///
    /// 1. Remove top card from deck.
    /// 2. Ensure player's hand either has same card or top deck card.
    /// 3. Ensure *other* card is marked as the played card AND added to play_history.
    /// 4. Remove Handmaid effect if we played it previous turn.
    /// 5. Move to next state based on if there's any action to do or not.
    fn handle_staging(&self, client_player_id: &String, card_source: PlayCardSource, mut round_data: RoundData) -> LoveLetterState {
        // Check: Is my turn
        if client_player_id != round_data.players.current_turn_player_id() {
            self.streams.send_err(&client_player_id, Status::failed_precondition("Can't play card, not your turn"));
            return LoveLetterState::PlayPending(round_data);
        }

        // Sanity check: Deck is not empty
        let top_deck = round_data.deck.pop()
            .expect("Illegal game state: We're in 'PlayPending' state with no cards in deck");

        // Discard current player's card and cycle new card
        let played_card = match card_source {
            PlayCardSource::Hand => round_data.players.replace_card(client_player_id.clone(), top_deck),
            PlayCardSource::TopDeck => top_deck,
        };

        // Append to play history
        round_data.play_history.push(played_card);

        // Clear self from Handmaids (played from previous turn) AFTER we validate it's our
        // turn and BEFORE we commit any self-effects (Handmaid only prevents others' effects).
        round_data.handmaid_immunity_player_ids.remove(client_player_id);

        // TODO:3 if selection not-needed, auto-commit.
        // Alternatively, client can be written to immediately send commit for certain cards.
        // This would keep the backend less modal.
        LoveLetterState::PlayStaging(round_data, StagedPlay::new(played_card))
    }

    /// We implement idempotency, to some extent. If a caller retried with a different request
    /// payload, we silently accept the request despite not honoring it.
    fn handle_staging_idempotent(&self, client_player_id: &String, round_data: RoundData, staged_play: StagedPlay) -> LoveLetterState {
        // Is my turn
        if client_player_id != round_data.players.current_turn_player_id() {
            self.streams.send_err(&client_player_id, Status::failed_precondition("Can't play card, not your turn"));
            return LoveLetterState::PlayStaging(round_data, staged_play)
        }

        // No state change
        let to_state = LoveLetterState::PlayStaging(round_data, staged_play);

        // Notify caller of latest game state.
        self.streams.send_err(client_player_id, Status::out_of_range("Your local state is stale."));
        self.send_game_state(&to_state, client_player_id);
        to_state
    }
}
