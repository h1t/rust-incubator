use std::cmp::Ordering;

#[derive(Debug)]
pub struct State {
    secret: u32,
}

impl State {
    pub fn new(secret: u32) -> Self {
        Self { secret }
    }

    pub fn guess(&self, number: u32) -> Ordering {
        number.cmp(&self.secret)
    }
}

#[cfg(test)]
mod tests {
    use super::State;
    use proptest::prelude::*;
    use std::cmp::Ordering;

    fn check_state(secret: u32, number: u32) {
        let state = State::new(secret);
        assert_eq!(state.guess(number), number.cmp(&secret));
    }

    #[test]
    fn test_guess_number() {
        let state = State::new(5);

        assert_eq!(state.guess(6), Ordering::Greater);
        assert_eq!(state.guess(4), Ordering::Less);
        assert_eq!(state.guess(5), Ordering::Equal);

        check_state(2, 2);
        check_state(2, 1);
        check_state(1, 2);
    }

    proptest! {
        #[test]
        fn test_proptest(secret in 0..1000u32, guess in 0..1000u32) {
            check_state(secret, guess);
        }
    }
}
