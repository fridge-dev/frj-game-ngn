use crate::events::{PlayCardSource, Card};
use crate::state_machine::{LoveLetterStateMachineEventHandler, LoveLetterState};
use crate::types::{StagedPlay, RoundData};
use tonic::Status;

impl LoveLetterStateMachineEventHandler {

    pub fn play_card_staged(
        &self,
        from_state: LoveLetterState,
        player_id: String,
        card_source: PlayCardSource
    ) -> LoveLetterState {
        match from_state {
            LoveLetterState::PlayStaging(game_data, round_data, staged_play) => {
                // Is my turn
                if &player_id != round_data.players.current_turn_player_id() {
                    self.streams.send_err(&player_id, Status::failed_precondition("Can't play card, not your turn"));
                    return LoveLetterState::PlayStaging(game_data, round_data, staged_play)
                }

                // Idempotent check
                let card_to_stage = round_data.get_card_to_stage(&player_id, &card_source);
                if card_to_stage != staged_play.played_card {
                    self.streams.send_err(&player_id, Status::failed_precondition("Can't play card while pending commit"));
                }

                // No state change
                let to_state = LoveLetterState::PlayStaging(game_data, round_data, staged_play);

                // Notify caller of latest game state
                self.send_game_state(&to_state, &player_id);

                to_state
            },
            LoveLetterState::PlayPending(game_data, round_data) => {
                // Is my turn
                if &player_id != round_data.players.current_turn_player_id() {
                    self.streams.send_err(&player_id, Status::failed_precondition("Can't play card, not your turn"));
                    return LoveLetterState::PlayPending(game_data, round_data);
                }

                let card_to_stage = round_data.get_card_to_stage(&player_id, &card_source);

                // TODO if selection not-needed, auto-commit

                LoveLetterState::PlayStaging(game_data, round_data, StagedPlay::new(card_to_stage))
            },
            _ => {
                self.streams.send_err(&player_id, Status::failed_precondition("Can't play card while in current state"));
                from_state
            },
        }
    }

}

impl RoundData {
    fn get_card_to_stage(&self, player_id: &String, card_source: &PlayCardSource) -> Card {
        match card_source {
            PlayCardSource::Hand => self.players
                .get_card(player_id)
                .expect("Player attempted to stage card without being in round."),
            PlayCardSource::TopDeck => *self.deck
                .last()
                .expect("Player attempted to stage card with empty deck."),
        }
    }
}
