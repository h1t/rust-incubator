use std::{
    fs,
    io::{self},
    path::Path,
};

use rand::{
    distributions::{Alphanumeric, Slice},
    seq::SliceRandom,
    thread_rng, Rng,
};
use sha3::{Digest, Sha3_256};

pub fn generate_password(chars: &[char], len: usize) -> Option<String> {
    Slice::new(chars)
        .ok()
        .map(|slice| thread_rng().sample_iter(&slice).take(len).collect())
}

pub fn select_rand_val<T>(v: &[T]) -> Option<&T> {
    v.choose(&mut thread_rng())
}

pub fn new_access_token() -> String {
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

pub fn get_file_hash(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    fs::read(path.as_ref()).map(|buf| Sha3_256::digest(buf).to_vec())
}

pub fn hash_password(password: &str) -> String {
    argonautica::Hasher::default()
        .with_password(password)
        .hash()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        let chars = ('a'..='z').collect::<Vec<_>>();
        assert_eq!(generate_password(&chars, 0), Some(String::new()));
        assert!(generate_password(&[], 1).is_none());

        let password = generate_password(&chars, 10);
        assert!(password.is_some());
        assert_eq!(password.as_ref().unwrap().len(), 10);
        assert!(password
            .unwrap()
            .chars()
            .all(|ch| ch.is_alphabetic() && ch.is_lowercase()));
    }

    #[test]
    fn test_new_access_token() {
        let token = new_access_token();
        assert_eq!(token.len(), 64);
        assert!(token.chars().all(char::is_alphanumeric));
    }
}
