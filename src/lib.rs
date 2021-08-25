use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

struct CacheEntry<K, V> {
    prev: u32,
    next: u32,
    key: Rc<K>,
    val: V,
}

impl<K, V> CacheEntry<K, V> {
    pub fn set_previous(&mut self, prev: u32) {
        self.prev = prev;
    }

    pub fn set_next(&mut self, next: u32) {
        self.next = next;
    }

    pub fn set_key(&mut self, key: Rc<K>) {
        self.key = key;
    }

    pub fn set_value(&mut self, val: V) {
        self.val = val;
    }

    pub fn previous(&self) -> u32 {
        self.prev
    }

    pub fn next(&self) -> u32 {
        self.next
    }

    pub fn key(&self) -> &Rc<K> {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.val
    }
}

pub struct LruCache<K: Eq + Hash, V> {
    map: HashMap<Rc<K>, u32>,
    queue: Vec<CacheEntry<K, V>>,
    max_size: usize,
    head: u32,
    tail: u32,
}

impl<K: Eq + Hash, V> LruCache<K, V> {
    pub fn new(max_size: usize) -> Self {
        LruCache {
            map: HashMap::new(),
            queue: Vec::new(),
            max_size,
            head: 0,
            tail: 0,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        match self.get_index(key) {
            Some(idx) => {
                self.mark_access(idx);
                Some(self.queue[idx as usize].value())
            }
            None => None,
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        match self.get_index(&key) {
            Some(idx) => {
                let entry = &mut self.queue[idx as usize];
                entry.set_value(value);
            }
            None => {
                let key_rc = Rc::new(key);
                match self.queue.len() == self.max_size {
                    true => self.replace_tail(key_rc.clone(), value),
                    false => self.add_tail(key_rc.clone(), value),
                }

                self.map.insert(key_rc, self.tail);
                self.mark_access(self.tail);
            }
        }
    }

    fn replace_tail(&mut self, key: Rc<K>, value: V) {
        let entry = &mut self.queue[self.tail as usize];
        entry.set_value(value);
        self.map.remove(entry.key());
        entry.set_key(key);
    }

    fn add_tail(&mut self, key: Rc<K>, value: V) {
        self.queue.push(CacheEntry {
            prev: self.tail,
            next: 0,
            val: value,
            key,
        });
        self.tail = self.queue.len() as u32 - 1;

        if self.queue.len() == 1 {
            self.head = self.tail
        }
    }

    fn get_index(&self, key: &K) -> Option<u32> {
        self.map.get(key).copied()
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
        let mut cache = LruCache::new(16);
        cache.put(1, 2);
        assert_eq!(*cache.get(&1).unwrap(), 2);
    }

    #[test]
    fn test_update() {
        let mut cache = LruCache::new(16);
        cache.put(1, "hello");
        cache.put(2, "new");
        cache.put(2, "world");
        assert_eq!(*cache.get(&1).unwrap(), "hello");
        assert_eq!(*cache.get(&2).unwrap(), "world");
    }

    #[test]
    fn test_eviction() {
        let mut cache = LruCache::new(2);
        cache.put(1, "hello");
        cache.put(2, "world");
        cache.put(3, "hello");
        assert!(cache.get(&1).is_none());
        assert!(cache.get(&2).is_some());
        assert!(cache.get(&3).is_some());
    }

    #[test]
    fn test_put_get_strings() {
        let mut cache = LruCache::new(1);
        cache.put(format!("hola, {}", "s"), 101);
        let s = format!("hola, {}", "s");
        assert_eq!(*cache.get(&s).unwrap(), 101);
    }
}
