use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};
use std::mem;

#[derive(Debug)]
pub struct HashMap<K,V> {
    buckets: Vec<Vec<(K,V)>>,
    items: usize,
}

pub struct HashMapIter<'a,K,V> {
    hashmap_holder: &'a HashMap<K,V>,
    bucket_num: usize,
    item_num: usize
}

impl<K,V> HashMap<K,V>
{
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0
        }
    }
}

impl<K,V> HashMap<K,V>  where
    K: Hash + Eq,
{
    fn bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket = (hasher.finish() % self.buckets.len() as u64) as usize;
        bucket
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if *ekey == key {
                return Some(mem::replace(evalue, value))
            }
        }

        bucket.push((key, value));
        self.items += 1;
        None
    }

    pub fn contains_key(&self, key: K) -> bool {
        if self.is_empty() {
            return false;
        }
        let bucket = self.bucket(&key);
        for &(ref ekey, _) in &self.buckets[bucket] {
           if *ekey == key {
               return true;
           }
        }
        false

    }

    pub fn get(&self, key: K) -> Option<&V> {
        if self.is_empty() {
            return None;
        }
        let bucket = self.bucket(&key);
        self.buckets[bucket]
            .iter()
            .find(|&(ref ekey, _)| {
                *ekey == key}).map(|&(_, ref v)| v)
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];
        let i = bucket.iter().position(|& (ref ekey, _)| *ekey == key)?;
        Some(bucket.swap_remove(i).1)
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    fn resize(&mut self) {
        let targets_size = match self.buckets.len() {
            0 => 1,
            n => 2 * n,
        };

        let mut new_buckets = Vec::with_capacity(targets_size);
        new_buckets.extend((0..targets_size).map(|_| Vec::new()));
        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }
        let _ = mem::replace(&mut self.buckets, new_buckets);
    }
}

impl<'a, K,V> Iterator for HashMapIter<'a, K,V> {
    type Item = &'a (K,V);
    fn next(&mut self) -> Option<Self::Item> {
        let buckets = &self.hashmap_holder.buckets;
        if self.item_num >= buckets[self.bucket_num].len() {
           self.item_num = 0;
           self.bucket_num += 1;
        }
        while buckets.get(self.bucket_num)?.is_empty() {
            self.bucket_num += 1;
        }

        let item = &buckets.get(self.bucket_num)?.get(self.item_num)?;
        self.item_num += 1;
        Some(item)
    }
}

impl<'a,K,V> IntoIterator for &'a HashMap<K,V> {
    type Item = &'a (K,V);
    type IntoIter = HashMapIter<'a,K,V>;
    fn into_iter(self) -> Self::IntoIter {
        HashMapIter {
            hashmap_holder: self,
            bucket_num: 0,
            item_num: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let map = HashMap::<u32, u32>::new();
        assert_eq!(map.is_empty(), true);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        map.insert("hello", 42);
        assert_eq!(map.get(&"hello"), Some(&42));
    }

    #[test]
    fn get() {
        let mut map = HashMap::new();
        assert_eq!(map.get("hello"), None);
        map.insert("hello", 42);
        assert_eq!(map.get(&"hello"), Some(&42));
    }

    #[test]
    fn contains_key() {
        let mut map = HashMap::new();
        assert_eq!(map.contains_key("hello".to_string()), false);
        map.insert("hello".to_string(), 42);
        assert_eq!(map.contains_key("hello".to_string()), true);
    }

    #[test]
    fn remove() {
        let mut map = HashMap::new();
        map.insert("hello", 42);
        assert_eq!(map.remove(&"hello"), Some(42));
        assert_eq!(map.get(&"hello"), None);
    }

    #[test]
    fn iterator() {
        let mut map = HashMap::<String, String>::new();
        let books = vec![
            ("Book1", "Review1"),
            ("Book2", "Review2")
        ];

        for (title, review) in &books {
           map.insert(title.to_string(), review.to_string());
        }

        let books2 : Vec<&(String, String)> = map.into_iter().collect();
        for i in 0..books.len() {
            assert_eq!(books[i].0, books2[i].0);
            assert_eq!(books[i].1, books2[i].1);
        }
    }
}
