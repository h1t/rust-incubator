use std::marker::PhantomData;

use crate::{Storage, User};

pub struct UserRepository<K> {
    storage: Box<dyn Storage<K, User>>,
    _data: PhantomData<K>,
}

impl<K> UserRepository<K> {
    pub fn new<S>(storage: S) -> Self
    where
        S: Storage<K, User> + 'static,
    {
        Self {
            storage: Box::new(storage),
            _data: PhantomData,
        }
    }

    pub fn get(&self, key: &K) -> Option<&User> {
        self.storage.get(key)
    }

    pub fn set(&mut self, key: K, val: User) {
        self.storage.set(key, val);
    }

    pub fn remove(&mut self, key: &K) -> Option<User> {
        self.storage.remove(key)
    }
}
