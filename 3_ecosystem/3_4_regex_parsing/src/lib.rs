use std::{error::Error, fmt::Display, str::FromStr};

mod pest;
mod regex;

#[derive(Debug, PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}

impl FromStr for Sign {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Sign::Plus),
            "-" => Ok(Sign::Minus),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

impl FromStr for Precision {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Precision::Asterisk),
            str if str.ends_with('$') => str[..str.len() - 1]
                .parse()
                .map(Precision::Argument)
                .or(Err(ParseError)),
            str => str.parse().map(Precision::Integer).or(Err(ParseError)),
        }
    }
}

#[derive(Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error")
    }
}

impl Error for ParseError {}
