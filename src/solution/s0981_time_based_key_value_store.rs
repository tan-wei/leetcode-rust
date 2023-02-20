/**
 * [0981] Time Based Key-Value Store
 *
 * Design a time-based key-value data structure that can store multiple values for the same key at different time stamps and retrieve the key's value at a certain timestamp.
 * Implement the TimeMap class:
 *
 * 	TimeMap() Initializes the object of the data structure.
 * 	void set(String key, String value, int timestamp) Stores the key key with the value value at the given time timestamp.
 * 	String get(String key, int timestamp) Returns a value such that set was called previously, with timestamp_prev <= timestamp. If there are multiple such values, it returns the value associated with the largest timestamp_prev. If there are no values, it returns "".
 *
 *  
 * Example 1:
 *
 * Input
 * ["TimeMap", "set", "get", "get", "set", "get", "get"]
 * [[], ["foo", "bar", 1], ["foo", 1], ["foo", 3], ["foo", "bar2", 4], ["foo", 4], ["foo", 5]]
 * Output
 * [null, null, "bar", "bar", null, "bar2", "bar2"]
 * Explanation
 * TimeMap timeMap = new TimeMap();
 * timeMap.set("foo", "bar", 1);  // store the key "foo" and value "bar" along with timestamp = 1.
 * timeMap.get("foo", 1);         // return "bar"
 * timeMap.get("foo", 3);         // return "bar", since there is no value corresponding to foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 is "bar".
 * timeMap.set("foo", "bar2", 4); // store the key "foo" and value "bar2" along with timestamp = 4.
 * timeMap.get("foo", 4);         // return "bar2"
 * timeMap.get("foo", 5);         // return "bar2"
 *
 *  
 * Constraints:
 *
 * 	1 <= key.length, value.length <= 100
 * 	key and value consist of lowercase English letters and digits.
 * 	1 <= timestamp <= 10^7
 * 	All the timestamps timestamp of set are strictly increasing.
 * 	At most 2 * 10^5 calls will be made to set and get.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/time-based-key-value-store/
// discuss: https://leetcode.com/problems/time-based-key-value-store/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Default)]
struct TimeMap {
    map: std::collections::HashMap<String, std::collections::BTreeMap<i32, String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Default::default()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.map
            .get(&key)
            .and_then(|m| m.range(..=timestamp).next_back())
            .map(|(_, v)| v.to_string())
            .unwrap_or("".to_string())
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0981_example_1() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1); // store the key "foo" and value "bar" along with timestamp = 1.
        assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string()); // return "bar"
        assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string()); // return "bar", since there is no value corresponding to foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 is "bar".
        time_map.set("foo".to_string(), "bar2".to_string(), 4); // store the key "foo" and value "bar2" along with timestamp = 4.
        assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string()); // return "bar2"
        assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string()); // return "bar2"
    }
}
