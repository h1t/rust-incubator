use std::borrow::Cow;
mod command;
mod repo;

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

pub trait UserRepository<K> {
    fn set(&mut self, key: K, val: User);
    fn get(&self, key: &K) -> Option<&User>;
    fn remove(&mut self, key: &K) -> Option<User>;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::{CommandHandler, CreateUser};
    use std::{collections::HashMap, hash::Hash};

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
    fn test_create_user() {
        let storage = UserStorage::<u64>(HashMap::new());
        let mut repo = repo::StaticUserRepository::new(storage);
        let user_id = 1_u64;
        let user = User {
            id: user_id,
            email: "test@test.com".into(),
            activated: false,
        };
        let crate_user = CreateUser { id: user_id };

        assert!(repo.get(&user_id).is_none());
        let _ = user.handle_command(&crate_user, &mut repo);
        assert!(repo.get(&user_id).is_some());
    }
}
