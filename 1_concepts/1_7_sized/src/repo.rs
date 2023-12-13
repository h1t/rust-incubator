use std::marker::PhantomData;

use crate::{Storage, User, UserRepository};

pub struct StaticUserRepository<K, S> {
    storage: S,
    _data: PhantomData<K>,
}

impl<K, S: Storage<K, User>> StaticUserRepository<K, S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            _data: PhantomData,
        }
    }
}

impl<K, S: Storage<K, User>> UserRepository<K> for StaticUserRepository<K, S> {
    fn get(&self, key: &K) -> Option<&User> {
        self.storage.get(key)
    }

    fn set(&mut self, key: K, val: User) {
        self.storage.set(key, val);
    }

    fn remove(&mut self, key: &K) -> Option<User> {
        self.storage.remove(key)
    }
}
