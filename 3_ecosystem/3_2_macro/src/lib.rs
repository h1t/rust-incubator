pub mod declarative;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn get_etalon_map() -> BTreeMap<i32, &'static str> {
        BTreeMap::from([(1, "one"), (2, "two")])
    }

    #[test]
    fn test_declarative() {
        let map = declarative::btreemap! {
            1 => "one",
            2 => "two"
        };
        assert_eq!(get_etalon_map(), map);
    }

    #[test]
    fn test_procedural() {
        let map = procedural::btreemap! {
            1 => "one",
            2 => "two"
        };
        assert_eq!(get_etalon_map(), map);
    }
}
