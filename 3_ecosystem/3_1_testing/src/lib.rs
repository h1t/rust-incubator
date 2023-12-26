use mockall::*;
use rand::prelude::*;
use std::cmp::Ordering;

#[automock]
pub trait NumberGenerator {
    fn gen(&self) -> u32;
}

pub struct RandomNumberGenerator {}

impl NumberGenerator for RandomNumberGenerator {
    fn gen(&self) -> u32 {
        rand::thread_rng().gen()
    }
}

pub struct FixedNumberGenerator {
    number: u32,
}

impl FixedNumberGenerator {
    pub fn new(number: u32) -> Self {
        Self { number }
    }
}

impl NumberGenerator for FixedNumberGenerator {
    fn gen(&self) -> u32 {
        self.number
    }
}

#[derive(Debug)]
pub struct State {
    secret: u32,
}

impl State {
    pub fn new(generator: impl NumberGenerator) -> Self {
        Self {
            secret: generator.gen(),
        }
    }

    pub fn guess(&self, number: u32) -> Ordering {
        number.cmp(&self.secret)
    }
}

#[cfg(test)]
mod tests {
    use crate::{FixedNumberGenerator, MockNumberGenerator, NumberGenerator};

    use super::State;
    use proptest::prelude::*;
    use std::cmp::Ordering;

    fn check_state(secret: u32, number: u32) {
        let state = State::new(FixedNumberGenerator::new(secret));
        assert_eq!(state.guess(number), number.cmp(&secret));
    }

    fn check_guess(
        generator: impl NumberGenerator,
        guess: impl IntoIterator<Item = (u32, Ordering)>,
    ) {
        let state = State::new(generator);

        for (number, op) in guess {
            assert_eq!(state.guess(number), op);
        }
    }

    fn check_with_secret(secret: u32) {
        check_guess(
            FixedNumberGenerator::new(secret),
            [(secret, Ordering::Equal)],
        );

        check_guess(
            FixedNumberGenerator::new(secret),
            [(secret + 1, Ordering::Greater), (secret, Ordering::Equal)],
        );

        check_guess(
            FixedNumberGenerator::new(secret),
            [(secret - 1, Ordering::Less), (secret, Ordering::Equal)],
        );

        check_guess(
            FixedNumberGenerator::new(secret),
            [
                (secret - 1, Ordering::Less),
                (secret + 1, Ordering::Greater),
                (secret, Ordering::Equal),
            ],
        );
    }

    #[test]
    fn test_check_state() {
        check_state(2, 2);
        check_state(2, 1);
        check_state(1, 2);
    }

    #[test]
    fn test_with_fix_number() {
        check_with_secret(5);
    }

    #[test]
    fn test_with_random_number() {
        check_with_secret(rand::random());
    }

    #[test]
    fn test_with_mock_number_generator() {
        let secret = 5;
        let mut mock = MockNumberGenerator::new();
        mock.expect_gen().return_const(secret);

        check_guess(
            mock,
            [
                (secret - 1, Ordering::Less),
                (secret + 1, Ordering::Greater),
                (secret, Ordering::Equal),
            ],
        );
    }

    proptest! {
        #[test]
        fn test_proptest(secret in 0..1000u32, guess in 0..1000u32) {
            check_state(secret, guess);
        }
    }
}
