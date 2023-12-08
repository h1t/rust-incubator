use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread,
};

pub struct GlobalStack<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T> GlobalStack<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&self, value: T) {
        self.get_data().push(value);
    }

    pub fn pop(&self) -> Option<T> {
        self.get_data().pop()
    }

    pub fn len(&self) -> usize {
        self.get_data().len()
    }

    pub fn is_empty(&self) -> bool {
        self.get_data().is_empty()
    }

    fn get_data(&self) -> MutexGuard<Vec<T>> {
        self.data.lock().expect("mutex is poisoned")
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

impl<T> Default for GlobalStack<T> {
    fn default() -> Self {
        Self {
            data: Arc::default(),
        }
    }
}

fn main() {
    let stack = GlobalStack::<u64>::new();

    assert!(stack.is_empty());
    stack.push(1);
    assert_eq!(stack.len(), 1);
    stack.push(2);
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
    assert!(stack.is_empty());

    {
        let stack = stack.clone();
        let _ = thread::spawn(move || {
            stack.push(5);
        })
        .join();
    }

    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(5));
}
