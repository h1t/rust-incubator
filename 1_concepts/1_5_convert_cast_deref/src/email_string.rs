use std::{borrow::Borrow, error::Error, fmt::Display, sync::OnceLock};

use regex::Regex;

static EMAIL_REGEXP: OnceLock<Regex> = OnceLock::new();
const EMAIL_RAW_REGEXP: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailString(String);

impl EmailString {
    fn get_regex() -> &'static Regex {
        EMAIL_REGEXP.get_or_init(|| Regex::new(EMAIL_RAW_REGEXP).expect("email regexp is invalid"))
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for EmailString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Display for EmailString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<EmailString> for String {
    fn from(value: EmailString) -> Self {
        value.0
    }
}

impl TryFrom<String> for EmailString {
    type Error = InvalidEmailFormat;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if Self::get_regex().is_match(&value) {
            Ok(Self(value))
        } else {
            Err(InvalidEmailFormat)
        }
    }
}

impl<'a> TryFrom<&'a str> for EmailString {
    type Error = InvalidEmailFormat;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if Self::get_regex().is_match(value) {
            Ok(Self(value.into()))
        } else {
            Err(InvalidEmailFormat)
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvalidEmailFormat;

impl Display for InvalidEmailFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid email format")
    }
}

impl Error for InvalidEmailFormat {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email() {
        assert!(EmailString::try_from(String::from("")).is_err());

        assert!(EmailString::try_from("").is_err());
        assert!(EmailString::try_from("test").is_err());
        assert!(EmailString::try_from("test@").is_err());
        assert!(EmailString::try_from("@test").is_err());
        assert!(EmailString::try_from("test@test").is_err());
        assert!(EmailString::try_from("test@test.").is_err());

        let email = EmailString::try_from("test@test.com");
        assert!(email.is_ok());

        let email_str: String = email.unwrap().into();
        assert!(!email_str.is_empty());
    }
}
