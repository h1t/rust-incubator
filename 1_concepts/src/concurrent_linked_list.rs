use std::sync::{Arc, Mutex};

use crate::linked_list::List;

pub type ConcurrentList<T> = Arc<Mutex<List<T>>>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_concurrent_use() {
        let list = ConcurrentList::default();
        list.lock().unwrap().push_back(1);

        {
            let list = Arc::clone(&list);
            thread::scope(|s| {
                s.spawn(|| list.lock().unwrap().push_back(2));
            });
        }

        let list_content = Arc::try_unwrap(list)
            .unwrap()
            .into_inner()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(list_content, &[1, 2]);
    }
}
