use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

type Link<T> = Option<Arc<Mutex<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Node {
            data,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }
}

impl<T> List<T> {
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push_front(&mut self, data: T) {
        let node = Node::new(data);
        match self.head.take() {
            Some(head) => {
                head.lock().unwrap().prev = Some(node.clone());
                node.lock().unwrap().next = Some(head);
                self.head = Some(node);
            }
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node);
            }
        }
        self.size += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let node = Node::new(data);
        match self.tail.take() {
            Some(tail) => {
                tail.lock().unwrap().next = Some(node.clone());
                node.lock().unwrap().prev = Some(tail);
                self.tail = Some(node);
            }
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
        }
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().and_then(|head| {
            match head.lock().unwrap().next.take() {
                Some(new_head) => {
                    new_head.lock().unwrap().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            self.size -= 1;
            Arc::try_unwrap(head)
                .ok()
                .map(|value| value.into_inner().unwrap().data)
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().and_then(|tail| {
            match tail.lock().unwrap().prev.take() {
                Some(new_tail) => {
                    new_tail.lock().unwrap().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            self.size -= 1;
            Arc::try_unwrap(tail)
                .ok()
                .map(|value| value.into_inner().unwrap().data)
        })
    }

    pub fn front(&self) -> Option<MutexGuardInner<T>> {
        self.head
            .as_ref()
            .map(|head| MutexGuardInner(head.lock().unwrap()))
    }

    pub fn back(&self) -> Option<MutexGuardInner<T>> {
        self.tail
            .as_ref()
            .map(|tail| MutexGuardInner(tail.lock().unwrap()))
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct MutexGuardInner<'a, T>(MutexGuard<'a, Node<T>>);

impl<'a, T> Deref for MutexGuardInner<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.data
    }
}

impl<'a, T> DerefMut for MutexGuardInner<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.data
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.0.size;
        (size, Some(size))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_list_content(list: List<u8>, etalon: &[u8]) {
        assert_eq!(list.len(), etalon.len());
        assert_eq!(list.is_empty(), etalon.is_empty());

        for (list_item, etalon_item) in list.into_iter().zip(etalon.iter()) {
            assert_eq!(list_item, *etalon_item);
        }
    }

    fn check_list_bounds(list: &List<u8>, left: u8, right: u8) {
        assert_eq!(list.front().as_deref(), Some(&left));
        assert_eq!(list.back().as_deref(), Some(&right));
    }

    #[test]
    fn test_empty() {
        let list = List::new();
        assert!(list.front().is_none());
        assert!(list.back().is_none());
        check_list_content(list, &[]);
    }

    #[test]
    fn test_front_add_back_delete() {
        let mut list = List::<u8>::new();

        list.push_front(2);
        check_list_bounds(&list, 2, 2);

        list.push_front(1);
        check_list_bounds(&list, 1, 2);

        assert_eq!(list.pop_back(), Some(2));
        check_list_bounds(&list, 1, 1);

        assert_eq!(list.pop_back(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn test_front_add_front_delete() {
        let mut list = List::<u8>::new();

        list.push_front(2);
        check_list_bounds(&list, 2, 2);

        list.push_front(1);
        check_list_bounds(&list, 1, 2);

        assert_eq!(list.pop_front(), Some(1));
        check_list_bounds(&list, 2, 2);

        assert_eq!(list.pop_front(), Some(2));
        assert!(list.is_empty());
    }

    #[test]
    fn test_back_add_back_delete() {
        let mut list = List::<u8>::new();

        list.push_back(2);
        check_list_bounds(&list, 2, 2);

        list.push_back(1);
        check_list_bounds(&list, 2, 1);

        assert_eq!(list.pop_back(), Some(1));
        check_list_bounds(&list, 2, 2);

        assert_eq!(list.pop_back(), Some(2));
        assert!(list.is_empty());
    }

    #[test]
    fn test_back_add_front_delete() {
        let mut list = List::<u8>::new();

        list.push_back(2);
        check_list_bounds(&list, 2, 2);

        list.push_back(1);
        check_list_bounds(&list, 2, 1);

        assert_eq!(list.pop_front(), Some(2));
        check_list_bounds(&list, 1, 1);

        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn test_content() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        check_list_content(list, &[1, 2, 3]);
    }
}
