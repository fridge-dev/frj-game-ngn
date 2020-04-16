use rand::RngCore;

/// Okay, so why would you ever implement your own RNG? Because surprisingly there's no fricking easy
/// existing rust **Pseudo** RNG! I need a seeded RNG (i.e. PRNG) to be able to reproduce deck shuffling.
/// Do I really need that? Maybe not. But for most (non-crypto related) use cases, I've found it's really
/// helpful to be able to reproduce the randomness.
///
/// Okay, so we're here. We're implementing a PRNG. How do we implement a PRNG? Idk. I just googled and
/// found [this](https://en.wikipedia.org/wiki/Xorshift#xorshift*) (make sure when you click you manually
/// type the `*` at the end of the URL if clicking doesn't pick up the whole link). This shows the following
/// C code:
///
/// ```c
/// #include <stdint.h>
///
/// struct xorshift64s_state {
///     uint64_t a;
/// };
///
/// uint64_t xorshift64s(struct xorshift64s_state *state)
/// {
///     uint64_t x = state->a; /* The state must be seeded with a nonzero value. */
///     x ^= x >> 12; // a
///     x ^= x << 25; // b
///     x ^= x >> 27; // c
///     state->a = x;
///     return x * UINT64_C(0x2545F4914F6CDD1D);
/// }
/// ```
///
/// That looks easy. But what is that `UINT64_C` macro? Welp, I don't really care, but here's what that
/// macro evaluates to:
///
/// ```c
/// #include <stdint.h>
/// #include <stdio.h>
/// int main() {
///   printf("UINT64_C(0x2545F4914F6CDD1D) => %d\n", UINT64_C(0x2545F4914F6CDD1D));
///   return 0;
/// }
/// ```
///
/// Paste the above into a file `test.c` and run `gcc test.c && ./a.out` and voila!
///
/// ```text
/// UINT64_C(0x2545F4914F6CDD1D) => 1332534557
/// ```
///
/// Alas. Here we are. I don't know what this does. But it works pretty decently.
pub struct PrngRand {
    n: u64,
}

const PRNG_MULTIPLIER: u64 = 1332534557;

impl PrngRand {

    /// Create a new PRNGenerator from the provided seed. Seed can't be 0.
    ///
    /// My usual strategy: Generate the seed randomly (from std lib), log the seed,
    /// then create and use the PRNG for all other randomness.
    pub fn new(seed: u64) -> Self {
        if seed == 0 {
            panic!("Don't fkn use 0 to seed random");
        }

        PrngRand {
            n: seed
        }
    }

    /// Generate next random number
    pub fn next(&mut self) -> u64 {
        let mut x = self.n;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;

        self.n = x;

        // prevents overflow
        (x as u128 * PRNG_MULTIPLIER as u128) as u64
    }
}

impl RngCore for PrngRand {
    fn next_u32(&mut self) -> u32 {
        self.next() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.next()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn prng_produces_different_values_per_seed() {

        let num_rngs_to_test = 1000;
        let num_values_to_generate_per_rng = 5;

        // Key-space is not full, so we can't use Vec<> as outer data structure.
        let mut prng_results: HashMap<u64, Vec<u64>> = HashMap::new();
        for i in 1..num_rngs_to_test {
            let mut results = Vec::new();
            let mut prng = PrngRand::new(i);
            for _ in 0..num_values_to_generate_per_rng {
                results.push(prng.next());
            }
            prng_results.insert(i, results);
        }

        for i in 1..num_rngs_to_test {
            let result: &Vec<u64> = prng_results.get(&i).expect(&format!("Outer: {}", i));
            for j in (i + 1)..num_rngs_to_test {
                let result2: &Vec<u64> = prng_results.get(&j).expect(&format!("Inner: {}, {}", i, j));
                assert_ne!(result, result2);
            }
        }
    }

    #[test]
    fn prng_produces_same_value_for_same_seed() {
        let num_rngs_to_test = 10000;
        let mut rng1 = PrngRand::new(1234);
        let mut rng2 = PrngRand::new(1234);

        for _ in 0..num_rngs_to_test {
            assert_eq!(
                rng1.next(),
                rng2.next()
            )
        }
    }
}
