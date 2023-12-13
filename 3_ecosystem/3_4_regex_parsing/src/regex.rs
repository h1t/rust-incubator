use crate::{Precision, Sign};
use regex::{Captures, Regex};
use std::{str::FromStr, sync::OnceLock};

static REGEX: OnceLock<Regex> = OnceLock::new();

pub fn parse(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    if let Some(captures) = get_regex().captures(input) {
        (
            parse_capture(&captures, "sign"),
            parse_capture(&captures, "width"),
            parse_capture(&captures, "precision"),
        )
    } else {
        Default::default()
    }
}

fn parse_capture<T>(captures: &Captures, name: &str) -> Option<T>
where
    T: FromStr,
{
    captures
        .name(name)
        .map(|m| m.as_str())
        .and_then(|s| s.parse().ok())
}

fn get_regex() -> &'static Regex {
    REGEX.get_or_init(|| Regex::new(include_str!("regex.dat")).expect("regex is invalid"))
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
