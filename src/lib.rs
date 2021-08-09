use std::collections::HashMap;
use std::hash::Hash;

struct CacheEntry<T> {
    prev: u32,
    next: u32,
    val: T,
}

impl<T> CacheEntry<T> {
    pub fn new(val: T) -> Self {
        CacheEntry {
            prev: 0,
            next: 0,
            val,
        }
    }

    pub fn set_previous(&mut self, prev: u32) {
        self.prev = prev;
    }

    pub fn set_next(&mut self, next: u32) {
        self.next = next;
    }

    pub fn previous(&self) -> u32 {
        self.prev
    }

    pub fn next(&self) -> u32 {
        self.next
    }

    pub fn value(&self) -> &T {
        &self.val
    }
}

pub struct LRUCache<K: Eq + Hash, V> {
    map: HashMap<K, u32>,
    queue: Vec<CacheEntry<V>>,
    max_size: usize,
    head: u32,
    tail: u32,
}

impl<K: Eq + Hash, V> LRUCache<K, V> {

    pub fn new(max_size: usize) -> Self {
        LRUCache {
            map: HashMap::new(),
            queue: Vec::new(),
            max_size,
            head: 0,
            tail: 0,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        match self.map.get(key) {
            Some(idx) => {
                self.mark_access(*idx);
                Some(self.queue[*idx as usize].value())
            },
            None => None,
        }
    }

    pub fn put(&mut self, key: K, value: V) {

    }

    fn mark_access(&mut self, idx: u32) {
        if idx == self.head {
            return;
        }

        let prev = self.queue[idx as usize].previous() as usize;
        let next = self.queue[idx as usize].next() as usize;

        if idx == self.tail {
            self.tail = prev as u32;
        }

        self.queue[self.head as usize].set_previous(idx);
        self.queue[prev].set_next(next as u32);
        self.queue[idx as usize].set_next(self.head);
        self.head = idx;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_get() {
        let mut cache = LRUCache::new(16);
        cache.put(&1, 2);
        assert_eq!(*cache.get(&1).unwrap(), 2);
    }

    #[test]
    fn test_update() {
        let mut cache = LRUCache::new(16);
        cache.put(&1, "hello");
        cache.put(&2, "new");
        cache.put(&2, "world");
        assert_eq!(*cache.get(&1).unwrap(), "hello");
        assert_eq!(*cache.get(&2).unwrap(), "world");
    }

    #[test]
    fn test_eviction() {
        let mut cache = LRUCache::new(2);
        cache.put(&1, "hello");
        cache.put(&2, "world");
        cache.put(&3, "hello");
        assert!(cache.get(&1).is_none());
        assert!(cache.get(&2).is_some());
        assert!(cache.get(&3).is_some());
    }
}