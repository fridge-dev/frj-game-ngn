use crate::prng::PrngRand;
use rand::RngCore;
use rand::seq::SliceRandom;

/// Takes ownership and passes back same vector, but shuffled randomly. Also returns the
/// `u64` seed used to seed the RNG. Log this if you want to be able to deterministically
/// reproduce a random game.
pub fn shuffle<T>(collection: Vec<T>) -> (Vec<T>, u64) {
    let seed = rand::thread_rng().next_u64();
    (shuffle_impl(collection, seed), seed)
}

fn shuffle_impl<T>(mut collection: Vec<T>, seed_for_random: u64) -> Vec<T> {
    let prng = &mut PrngRand::new(seed_for_random);
    // Let's get wild
    collection.shuffle(prng);
    collection.reverse();
    collection.shuffle(prng);
    collection.reverse();
    collection.shuffle(prng);

    collection
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_unshuffled() -> Vec<u8> {
        vec![1, 2, 3, 4, 5]
    }

    #[test]
    fn different_seeds_produce_different_decks() {
        let unshuffled = new_unshuffled();
        let (deck2, seed2) = shuffle(unshuffled.clone());
        let (deck1, seed1) = shuffle(unshuffled.clone());

        assert_ne!(deck1, deck2);
        assert_ne!(seed1, seed2);
    }

    #[test]
    fn same_seed_produces_same_deck() {
        let unshuffled = new_unshuffled();
        let deck2 = shuffle_impl(unshuffled.clone(), 100);
        let deck1 = shuffle_impl(unshuffled.clone(), 100);

        assert_eq!(deck1, deck2);
    }
}
