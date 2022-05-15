/**
 * [0677] Map Sum Pairs
 *
 * Design a map that allows you to do the following:
 *
 * 	Maps a string key to a given value.
 * 	Returns the sum of the values that have a key with a prefix equal to a given string.
 *
 * Implement the MapSum class:
 *
 * 	MapSum() Initializes the MapSum object.
 * 	void insert(String key, int val) Inserts the key-val pair into the map. If the key already existed, the original key-value pair will be overridden to the new one.
 * 	int sum(string prefix) Returns the sum of all the pairs' value whose key starts with the prefix.
 *
 *  
 * Example 1:
 *
 * Input
 * ["MapSum", "insert", "sum", "insert", "sum"]
 * [[], ["apple", 3], ["ap"], ["app", 2], ["ap"]]
 * Output
 * [null, null, 3, null, 5]
 * Explanation
 * MapSum mapSum = new MapSum();
 * mapSum.insert("apple", 3);  
 * mapSum.sum("ap");           // return 3 (<u>ap</u>ple = 3)
 * mapSum.insert("app", 2);    
 * mapSum.sum("ap");           // return 5 (<u>ap</u>ple + <u>ap</u>p = 3 + 2 = 5)
 *
 *  
 * Constraints:
 *
 * 	1 <= key.length, prefix.length <= 50
 * 	key and prefix consist of only lowercase English letters.
 * 	1 <= val <= 1000
 * 	At most 50 calls will be made to insert and sum.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/map-sum-pairs/
// discuss: https://leetcode.com/problems/map-sum-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Default)]
struct Trie {
    sum: i32,
    children: [Option<Box<Trie>>; 26],
}

#[derive(Default)]
struct MapSum {
    trie: Trie,
    values: std::collections::HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        Default::default()
    }
    fn insert(&mut self, key: String, val: i32) {
        let delta = val - self.values.get(&key).unwrap_or(&0);
        let mut node = &mut self.trie;
        node.sum += delta;
        for &u in key.as_bytes() {
            node = node.children[(u - b'a') as usize].get_or_insert_with(Default::default);
            node.sum += delta;
        }
        self.values.insert(key, val);
    }
    fn sum(&self, prefix: String) -> i32 {
        let mut node = &self.trie;
        for &u in prefix.as_bytes() {
            if let Some(n) = &node.children[(u - b'a') as usize] {
                node = n;
            } else {
                return 0;
            }
        }
        node.sum
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0677_example_1() {
        let mut obj = MapSum::new();
        obj.insert("apple".to_string(), 3);
        assert_eq!(obj.sum("ap".to_string()), 3);
        obj.insert("app".to_string(), 2);
        assert_eq!(obj.sum("ap".to_string()), 5);
        assert_eq!(obj.sum("appl".to_string()), 3)
    }
}
