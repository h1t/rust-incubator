use std::str::FromStr;

use crate::{Precision, Sign};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest.dat"]
struct FormatParser;

pub fn parse(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    let Ok(pairs) = FormatParser::parse(Rule::format_spec, input) else {
        return Default::default();
    };

    let (mut sign, mut width, mut precision) = Default::default();

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::sign => {
                    sign = parse_rule(inner_pair);
                }
                Rule::width => {
                    width = parse_rule(inner_pair);
                }
                Rule::precision => {
                    precision = parse_rule(inner_pair);
                }
                _ => unreachable!(),
            }
        }
    }

    (sign, width, precision)
}

fn parse_rule<T>(rule: Pair<'_, Rule>) -> Option<T>
where
    T: FromStr,
{
    rule.as_str().parse().ok()
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in [
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        println!("start");
        for (input, expected) in [
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse(input);
            assert_eq!(width, expected);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in [
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse(input);
            assert_eq!(precision, expected);
        }
    }
}
