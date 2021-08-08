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
        let mut queue = &mut self.queue;
        let mut head_entry = queue[self.head as usize];
        let mut accessed_entry = &mut self.queue[idx as usize];
        let mut prev_accessed = &mut self.queue[accessed_entry.previous() as usize];
        head_entry.set_previous(idx);
        prev_accessed.set_next(accessed_entry.next());
        accessed_entry.set_next(self.head);
        self.head = idx;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_get() {
        let mut cache = LRUCache::new(16);
        cache.put(1, 2);
        assert_eq!(cache.get(&1).unwrap(), 2);
    }

    #[test]
    fn test_eviction() {

    }
}