/**
 * [146] LRU Cache
 *
 * Design a data structure that follows the constraints of a <a href="https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU" target="_blank">Least Recently Used (LRU) cache</a>.
 * Implement the LRUCache class:
 *
 * 	LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
 * 	int get(int key) Return the value of the key if the key exists, otherwise return -1.
 * 	void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.
 *
 * Follow up:<br />
 * Could you do get and put in O(1) time complexity?
 *  
 * Example 1:
 *
 * Input
 * ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
 * [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
 * Output
 * [null, null, null, 1, null, -1, null, -1, 3, 4]
 * Explanation
 * LRUCache lRUCache = new LRUCache(2);
 * lRUCache.put(1, 1); // cache is {1=1}
 * lRUCache.put(2, 2); // cache is {1=1, 2=2}
 * lRUCache.get(1);    // return 1
 * lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
 * lRUCache.get(2);    // returns -1 (not found)
 * lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
 * lRUCache.get(1);    // return -1 (not found)
 * lRUCache.get(3);    // return 3
 * lRUCache.get(4);    // return 4
 *
 *  
 * Constraints:
 *
 * 	1 <= capacity <= 3000
 * 	0 <= key <= 3000
 * 	0 <= value <= 10^4
 * 	At most 3 * 10^4 calls will be made to get and put.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/lru-cache/
// discuss: https://leetcode.com/problems/lru-cache/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/lru-cache/discuss/594956/Rust-implementation-using-VecDeque-and-HashMap
struct LRUCache {
    dq: std::collections::VecDeque<i32>,
    map: std::collections::HashMap<i32, i32>,
    cap: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        use std::convert::TryInto;

        Self {
            dq: std::collections::VecDeque::new(),
            map: std::collections::HashMap::new(),
            cap: capacity.try_into().unwrap(),
        }
    }
    fn get(&mut self, key: i32) -> i32 {
        if self.map.contains_key(&key) {
            let i = self.dq.iter().position(|&x| x == key).unwrap();
            self.dq.remove(i);
            self.dq.push_front(key);
            return self.map.get(&key).cloned().unwrap();
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            let i = self.dq.iter().position(|&x| x == key).unwrap();
            self.dq.remove(i);
            self.dq.push_front(key);
            self.map.insert(key, value);
        } else {
            if self.map.len() == self.cap {
                let last = self.dq.pop_back().unwrap();
                self.map.remove(&last);
            }
            self.map.insert(key, value);
            self.dq.push_front(key);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0146_example_1() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1); // cache is {1=1}
        lru_cache.put(2, 2); // cache is {1=1, 2=2}
        assert_eq!(lru_cache.get(1), 1); // return 1
        lru_cache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        assert_eq!(lru_cache.get(2), -1); // returns -1 (not found)
        lru_cache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        assert_eq!(lru_cache.get(1), -1); // return -1 (not found)
        assert_eq!(lru_cache.get(3), 3); // return 3
        assert_eq!(lru_cache.get(4), 4); // return 4
    }
}
