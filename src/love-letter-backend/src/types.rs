use crate::deck;
use crate::events::{PlayCardSource, Card};
use std::collections::HashMap;

pub struct GameData {
    pub player_id_turn_order: Vec<String>,
    pub wins_per_player: HashMap<String, u8>,
    pub current_round: RoundData,
}

pub struct RoundData {
    pub remaining_cards: Vec<Card>,
    pub player_cards: HashMap<String, Card>,
    pub turn_cursor: usize,
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

    pub fn current_player_turn(&self) -> &String {
        self.player_id_turn_order
            .get(self.current_round.turn_cursor)
            .expect("Cursor should always ensure valid access")
    }

    #[allow(dead_code)] // TODO implement
    pub fn commit_play(&mut self) {
        unimplemented!()
    }
}

impl RoundData {
    pub fn new(player_ids: &Vec<String>) -> Self {
        let mut deck = deck::new_shuffled_deck();
        let mut turn_cursor = rand::random::<usize>() % player_ids.len();
        let mut player_cards = HashMap::new();

        // Discard the top card
        deck.pop();

        // Deal 1 card to each player
        for _ in 0..player_ids.len() {
            let player: &String = player_ids.get(turn_cursor).expect("player vec");
            let player: String = player.to_owned();
            let card = deck.pop().unwrap();

            player_cards.insert(player, card);
            turn_cursor = (turn_cursor + 1) % player_ids.len();
        }

        RoundData {
            remaining_cards: deck,
            player_cards,
            turn_cursor,
        }
    }

    pub fn get_card_to_play(&self, player_id: &String, card_source: &PlayCardSource) -> Card {
        *match card_source {
            PlayCardSource::Hand => self.player_cards
                .get(player_id)
                .expect("player map"),
            PlayCardSource::TopDeck => self.remaining_cards
                .last()
                .expect("deck size"),
        }
    }
}

// TODO remove pub fields, consider turning into InstanceState variations
pub struct StagedPlay {
    pub card: Card,
    pub target_player: Option<String>,
    pub target_card: Option<Card>,
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
