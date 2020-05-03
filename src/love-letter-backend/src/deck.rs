use crate::events::Card;
use backend_framework::shuffler;

pub fn new_shuffled_deck() -> Vec<Card> {
    let (deck, rng_seed) = shuffler::shuffle(new_unshuffled_deck());
    println!("INFO: Deck created with RNG seed '{}'", rng_seed);
    deck
}

fn new_unshuffled_deck() -> Vec<Card> {
    vec![
        // 5x Guard
        Card::Guard,
        Card::Guard,
        Card::Guard,
        Card::Guard,
        Card::Guard,

        // 2x of each
        Card::Priest,
        Card::Priest,
        Card::Baron,
        Card::Baron,
        Card::Handmaid,
        Card::Handmaid,
        Card::Prince,
        Card::Prince,

        // 1x of each
        Card::King,
        Card::Countess,
        Card::Princess,
    ]
}