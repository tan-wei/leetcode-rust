/**
 * [0460] LFU Cache
 *
 * Design and implement a data structure for a <a href="https://en.wikipedia.org/wiki/Least_frequently_used" target="_blank">Least Frequently Used (LFU)</a> cache.
 * Implement the LFUCache class:
 *
 * 	LFUCache(int capacity) Initializes the object with the capacity of the data structure.
 * 	int get(int key) Gets the value of the key if the key exists in the cache. Otherwise, returns -1.
 * 	void put(int key, int value) Update the value of the key if present, or inserts the key if not already present. When the cache reaches its capacity, it should invalidate and remove the least frequently used key before inserting a new item. For this problem, when there is a tie (i.e., two or more keys with the same frequency), the least recently used key would be invalidated.
 *
 * To determine the least frequently used key, a use counter is maintained for each key in the cache. The key with the smallest use counter is the least frequently used key.
 * When a key is first inserted into the cache, its use counter is set to 1 (due to the put operation). The use counter for a key in the cache is incremented either a get or put operation is called on it.
 * The functions <code data-stringify-type="code">get and <code data-stringify-type="code">put must each run in O(1) average time complexity.
 *  
 * Example 1:
 *
 * Input
 * ["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get"]
 * [[2], [1, 1], [2, 2], [1], [3, 3], [2], [3], [4, 4], [1], [3], [4]]
 * Output
 * [null, null, null, 1, null, -1, 3, null, -1, 3, 4]
 * Explanation
 * // cnt(x) = the use counter for key x
 * // cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
 * LFUCache lfu = new LFUCache(2);
 * lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
 * lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
 * lfu.get(1);      // return 1
 *                  // cache=[1,2], cnt(2)=1, cnt(1)=2
 * lfu.put(3, 3);   // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
 *                  // cache=[3,1], cnt(3)=1, cnt(1)=2
 * lfu.get(2);      // return -1 (not found)
 * lfu.get(3);      // return 3
 *                  // cache=[3,1], cnt(3)=2, cnt(1)=2
 * lfu.put(4, 4);   // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
 *                  // cache=[4,3], cnt(4)=1, cnt(3)=2
 * lfu.get(1);      // return -1 (not found)
 * lfu.get(3);      // return 3
 *                  // cache=[3,4], cnt(4)=1, cnt(3)=3
 * lfu.get(4);      // return 4
 *                  // cache=[3,4], cnt(4)=2, cnt(3)=3
 *
 *  
 * Constraints:
 *
 * 	0 <= capacity <= 10^4
 * 	0 <= key <= 10^5
 * 	0 <= value <= 10^9
 * 	At most 2 * 10^5 calls will be made to get and put.
 *
 *  
 * <span style="display: none;"> </span>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/lfu-cache/
// discuss: https://leetcode.com/problems/lfu-cache/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Credit: https://leetcode.com/problems/lfu-cache/discuss/762404/Dictionary-and-nested-linked-lists-in-Rust

struct ListNode {
    key: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(key: i32) -> Self {
        Self {
            key,
            prev: None,
            next: None,
        }
    }

    fn pluck(&mut self) {
        if let Some(node) = &self.prev {
            node.borrow_mut().next = self.next.as_ref().map(|r| r.clone());
        }

        if let Some(node) = &self.next {
            node.borrow_mut().prev = self.prev.as_ref().map(|r| r.clone());
        }

        self.next = None;
        self.prev = None;
    }
}

struct FrequencyBin {
    val: i32,
    count: i32,
    map: HashMap<i32, Rc<RefCell<ListNode>>>,
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl FrequencyBin {
    fn new(val: i32) -> Self {
        Self {
            val,
            count: 0,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn remove_key(&mut self, key: i32) {
        if let Some(node) = self.map.get_mut(&key) {
            if self.tail.as_ref().map_or(false, |n| Rc::ptr_eq(&n, node)) {
                self.tail = node.borrow().prev.as_ref().map(|r| r.clone());
            }

            if self.head.as_ref().map_or(false, |n| Rc::ptr_eq(&n, node)) {
                self.head = node.borrow().next.as_ref().map(|r| r.clone());
            }

            node.borrow_mut().pluck();
            self.map.remove(&key);
            self.count -= 1;
        }
    }

    fn push_back(&mut self, key: i32) {
        let new_node = Rc::new(RefCell::new(ListNode::new(key)));
        if let Some(node) = &self.tail {
            node.clone().borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(node.clone());
            self.tail = Some(new_node.clone())
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        }
        self.map.insert(key, new_node.clone());
        self.count += 1;
    }

    fn pop_front(&mut self) {
        let key = self.head.as_ref().unwrap().borrow().key;
        self.remove_key(key);
    }
}

struct CacheItem {
    value: i32,
    freq: i32,
}

struct LFUCache {
    capacity: i32,
    count: i32,
    lowest_freq: i32,
    item_map: HashMap<i32, CacheItem>,
    freq_map: HashMap<i32, FrequencyBin>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            count: 0,
            lowest_freq: 0,
            item_map: HashMap::new(),
            freq_map: HashMap::new(),
        }
    }

    fn evict(&mut self) {
        let lowest_freq = self.freq_map.get_mut(&self.lowest_freq).unwrap();
        self.item_map
            .remove(&lowest_freq.head.as_ref().unwrap().borrow().key);
        lowest_freq.pop_front();
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(item) = self.item_map.get_mut(&key) {
            let old_freq = self.freq_map.get_mut(&item.freq).unwrap();
            old_freq.remove_key(key);
            if self.lowest_freq == old_freq.val && old_freq.count == 0 {
                self.lowest_freq += 1;
            }
            item.freq += 1;
            self.freq_map
                .entry(item.freq)
                .or_insert(FrequencyBin::new(item.freq))
                .push_back(key);
            item.value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(item) = self.item_map.get_mut(&key) {
            let old_freq = self.freq_map.get_mut(&item.freq).unwrap();
            old_freq.remove_key(key);
            if self.lowest_freq == old_freq.val && old_freq.count == 0 {
                self.lowest_freq += 1;
            }
            item.freq += 1;
            item.value = value;

            self.freq_map
                .entry(item.freq)
                .or_insert(FrequencyBin::new(item.freq))
                .push_back(key);
        } else {
            if self.capacity > 0 {
                if self.count == self.capacity {
                    self.evict();
                    self.count -= 1;
                }

                self.item_map.insert(key, CacheItem { value, freq: 1 });
                self.freq_map
                    .entry(1)
                    .or_insert(FrequencyBin::new(1))
                    .push_back(key);

                self.count += 1;
                self.lowest_freq = 1;
            }
        }
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0460_example_1() {
        let mut lfu = LFUCache::new(2);
        lfu.put(1, 1); // cache=[1,_], cnt(1)=1
        lfu.put(2, 2); // cache=[2,1], cnt(2)=1, cnt(1)=1
        assert_eq!(lfu.get(1), 1); // return 1
                                   // cache=[1,2], cnt(2)=1, cnt(1)=2
        lfu.put(3, 3); // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
                       // cache=[3,1], cnt(3)=1, cnt(1)=2
        assert_eq!(lfu.get(2), -1); // return -1 (not found)
        assert_eq!(lfu.get(3), 3); // return 3
                                   // cache=[3,1], cnt(3)=2, cnt(1)=2
        lfu.put(4, 4); // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
                       // cache=[4,3], cnt(4)=1, cnt(3)=2
        assert_eq!(lfu.get(1), -1); // return -1 (not found)
        assert_eq!(lfu.get(3), 3); // return 3
                                   // cache=[3,4], cnt(4)=1, cnt(3)=3
        assert_eq!(lfu.get(4), 4); // return 4
                                   // cache=[3,4], cnt(4)=2, cnt(3)=3
    }
}
