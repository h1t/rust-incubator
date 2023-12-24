use cucumber::Parameter;
use cucumber::{given, then, when, World as _};
use std::cmp::Ordering;
use std::str::FromStr;
use step_3_1::State;

#[derive(Debug, Parameter)]
#[param(name = "guess_state", regex = "less|great|equal")]
enum GuessState {
    Less,
    Great,
    Equal,
}

impl From<GuessState> for Ordering {
    fn from(value: GuessState) -> Self {
        match value {
            GuessState::Less => Self::Less,
            GuessState::Great => Self::Greater,
            GuessState::Equal => Self::Equal,
        }
    }
}

impl FromStr for GuessState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "less" => Self::Less,
            "great" => Self::Great,
            "equal" => Self::Equal,
            invalid => return Err(format!("Invalid `GuessState`: {invalid}")),
        })
    }
}

#[derive(cucumber::World, Debug, Default)]
struct World {
    state: Option<State>,
    number: u32,
}

#[given(expr = "{int} as secret number")] // Cucumber Expression
async fn someone_is_hungry(w: &mut World, secret: u32) {
    w.state = Some(State::new(secret));
}

#[when(expr = "guess {int}")]
async fn eat_cucumbers(w: &mut World, number: u32) {
    w.number = number;
}

#[then(expr = "guess number is {guess_state} then secret")]
async fn is_full(w: &mut World, guess_state: GuessState) {
    let state = w.state.take().unwrap();
    assert_eq!(state.guess(w.number), guess_state.into());
}

#[tokio::main]
async fn main() {
    World::run("tests/features/guess").await;
}
