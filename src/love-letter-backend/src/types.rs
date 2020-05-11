use crate::deck;
use crate::events::{PlayCardSource, Card};
use std::collections::HashMap;

pub struct GameData {
    pub player_id_turn_order: Vec<String>,
    pub wins_per_player: HashMap<String, u8>,
    pub current_round: RoundData,
}

pub struct RoundData {
    pub deck: Vec<Card>,
    pub players: Players,
    pub play_history: Vec<Card>,
    pub most_recent_play_details: Option<CommittedPlay>,
}

/// Struct to track which players are still in game, which card they have, and turn order.
///
/// This is not efficient, but I've wasted too much time trying to get a nice interface
/// while eliminating invalid state space. My top priority is (1) to contain all of the
/// possible invalid states to this struct, and (2) to continue on with development.
pub struct Players {
    cards: HashMap<String, Card>,
    // This will be in the same cyclical order as GameData's order, but
    // won't necessarily start from the same index.
    turn_order: Vec<String>,
    turn_cursor: usize,
}

#[allow(dead_code)] // TODO use this
pub enum CommittedPlay {
    Guard {
        target_player_id: String,
        target_card: Card,
        correct: bool,
    },
    Priest {
        target_player_id: String,
    },
    Baron {
        target_player_id: String,
        eliminated_player_id: String,
        eliminated_card: Card,
    },
    Handmaid,
    Prince {
        target_player_id: String,
        discarded_card: Card,
    },
    King {
        target_player_id: String,
    },
    Countess,
}

pub struct StagedPlay {
    pub card: Card,
    pub target_player: Option<String>,
    pub target_card: Option<Card>,
}

impl GameData {
    pub fn new(player_ids: Vec<String>) -> Self {
        let current_round = RoundData::new(&player_ids);

        GameData {
            player_id_turn_order: player_ids,
            wins_per_player: HashMap::new(),
            current_round,
        }
    }
}

impl RoundData {
    pub fn new(player_ids: &Vec<String>) -> Self {
        let mut deck = deck::new_shuffled_deck();
        let mut turn_cursor = rand::random::<usize>() % player_ids.len();
        let mut players = Players::with_capacity(player_ids.len());

        // Discard the top card
        deck.pop();

        // Deal 1 card to each player
        for _ in 0..player_ids.len() {
            let player_id = player_ids.get(turn_cursor)
                .expect("player_ids iteration for dealing out cards")
                .to_owned();
            let card = deck.pop().expect("deck out of cards before game start");

            players.insert_at_tail(player_id, card);
            turn_cursor = (turn_cursor + 1) % player_ids.len();
        }

        let play_history = Vec::with_capacity(deck.len());

        RoundData {
            deck,
            players,
            play_history,
            most_recent_play_details: None
        }
    }

    pub fn get_card_to_stage(&self, player_id: &String, card_source: &PlayCardSource) -> Card {
        match card_source {
            PlayCardSource::Hand => self.players.get_card(player_id)
                .expect("Player attempted to stage card without being in round."),
            PlayCardSource::TopDeck => *self.deck
                .last()
                .expect("deck size"),
        }
    }
}

impl Players {
    pub fn with_capacity(capacity: usize) -> Self {
        Players {
            cards: HashMap::with_capacity(capacity),
            turn_order: Vec::with_capacity(capacity),
            turn_cursor: 0,
        }
    }

    pub fn insert_at_tail(&mut self, player_id: String, card: Card) {
        self.turn_order.push(player_id.clone());
        self.cards.insert(player_id, card);
    }

    pub fn get_card(&self, player_id: &String) -> Option<Card> {
        self.cards
            .get(player_id)
            .map(|c| *c)
    }

    pub fn increment_turn(&mut self) {
        self.turn_cursor = (self.turn_cursor + 1) % self.turn_order.len();
    }

    pub fn current_turn_player_id(&self) -> &String {
        self.turn_order
            .get(self.turn_cursor)
            .expect("Cursor should always ensure valid access")
    }

    /// Must be done as atomic operation
    pub fn eliminate_and_increment_turn(&mut self, player_id: &String) -> Card {
        let index_to_remove = {
            let mut index_to_remove = None;
            for (i, item) in self.turn_order.iter().enumerate() {
                if item == player_id {
                    index_to_remove = Some(i);
                    break;
                }
            }
            index_to_remove.expect("Players.eliminate_and_increment_turn() on player not in turn_order.")
        };

        let _ = self.turn_order.remove(index_to_remove);

        // If we removed an item before the current cursor position, then no need to increment
        // the cursor. The same cursor position will now refer to the next element.
        if index_to_remove < self.turn_cursor {
            // This must be called AFTER modifying `self.turn_order` above
            self.turn_cursor += 1;
        }
        // If we didn't increment turn, it's possible cursor is now out of bounds. So we
        // call this no matter what.
        self.turn_cursor %= self.turn_order.len();

        self.cards
            .remove(player_id)
            .expect("Players.eliminate_and_increment_turn() on player without card.")
    }

    // TODO use IntoIterator trait
    pub fn into_iter(self) -> impl Iterator<Item = (String, Card)> {
        self.cards.into_iter()
    }
}

impl StagedPlay {
    pub fn new(card: Card) -> Self {
        StagedPlay {
            card,
            target_player: None,
            target_card: None
        }
    }

    pub fn set_target_player(&mut self, player_id: String) {
        self.target_player.replace(player_id);
    }

    pub fn set_target_card(&mut self, card: Card) {
        self.target_card.replace(card);
    }
}
