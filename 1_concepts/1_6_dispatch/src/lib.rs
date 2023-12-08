use std::borrow::Cow;

mod dynamic_repo;
mod static_repo;

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, hash::Hash};

    use super::*;

    struct UserStorage<K>(HashMap<K, User>);

    impl<K> Storage<K, User> for UserStorage<K>
    where
        K: Eq + Hash,
    {
        fn set(&mut self, key: K, val: User) {
            self.0.insert(key, val);
        }

        fn get(&self, key: &K) -> Option<&User> {
            self.0.get(key)
        }

        fn remove(&mut self, key: &K) -> Option<User> {
            self.0.remove(key)
        }
    }

    #[test]
    fn test_static_repo() {
        let key = String::from("valid");
        let storage = UserStorage::<String>(HashMap::new());
        let user = User {
            id: 1,
            email: "test@test.com".into(),
            activated: false,
        };
        let mut repo = static_repo::UserRepository::new(storage);
        assert!(repo.get(&key).is_none());

        repo.set(key.clone(), user);
        assert!(repo.get(&key).is_some());

        repo.remove(&key);
        assert!(repo.get(&key).is_none());
    }

    #[test]
    fn test_dynamic_repo() {
        let key = String::from("valid");
        let storage = UserStorage::<String>(HashMap::new());
        let user = User {
            id: 1,
            email: "test@test.com".into(),
            activated: false,
        };
        let mut repo = dynamic_repo::UserRepository::new(storage);
        assert!(repo.get(&key).is_none());

        repo.set(key.clone(), user);
        assert!(repo.get(&key).is_some());

        repo.remove(&key);
        assert!(repo.get(&key).is_none());
    }
}
