/**
 * [0706] Design HashMap
 *
 * Design a HashMap without using any built-in hash table libraries.
 * Implement the MyHashMap class:
 *
 * 	MyHashMap() initializes the object with an empty map.
 * 	void put(int key, int value) inserts a (key, value) pair into the HashMap. If the key already exists in the map, update the corresponding value.
 * 	int get(int key) returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key.
 * 	void remove(key) removes the key and its corresponding value if the map contains the mapping for the key.
 *
 *  
 * Example 1:
 *
 * Input
 * ["MyHashMap", "put", "put", "get", "get", "put", "get", "remove", "get"]
 * [[], [1, 1], [2, 2], [1], [3], [2, 1], [2], [2], [2]]
 * Output
 * [null, null, null, 1, -1, null, 1, null, -1]
 * Explanation
 * MyHashMap myHashMap = new MyHashMap();
 * myHashMap.put(1, 1); // The map is now [[1,1]]
 * myHashMap.put(2, 2); // The map is now [[1,1], [2,2]]
 * myHashMap.get(1);    // return 1, The map is now [[1,1], [2,2]]
 * myHashMap.get(3);    // return -1 (i.e., not found), The map is now [[1,1], [2,2]]
 * myHashMap.put(2, 1); // The map is now [[1,1], [2,1]] (i.e., update the existing value)
 * myHashMap.get(2);    // return 1, The map is now [[1,1], [2,1]]
 * myHashMap.remove(2); // remove the mapping for 2, The map is now [[1,1]]
 * myHashMap.get(2);    // return -1 (i.e., not found), The map is now [[1,1]]
 *
 *  
 * Constraints:
 *
 * 	0 <= key, value <= 10^6
 * 	At most 10^4 calls will be made to put, get, and remove.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-hashmap/
// discuss: https://leetcode.com/problems/design-hashmap/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const TABLE_SIZE: usize = 32;

#[derive(Default)]
struct MyHashMap {
    table: [Vec<(i32, i32)>; TABLE_SIZE],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        Default::default()
    }

    fn put(&mut self, key: i32, value: i32) {
        let i = (key as usize) % TABLE_SIZE;
        if let Some(j) = self.table[i].iter().position(|&(k, _)| k == key) {
            self.table[i][j].1 = value;
        } else {
            self.table[i].push((key, value));
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let i = (key as usize) % TABLE_SIZE;
        self.table[i]
            .iter()
            .position(|&(k, _)| k == key)
            .map(|j| self.table[i][j].1)
            .unwrap_or(-1)
    }

    fn remove(&mut self, key: i32) {
        let i = (key as usize) % TABLE_SIZE;
        if let Some(j) = self.table[i].iter().position(|&(k, _)| k == key) {
            self.table[i].remove(j);
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0706_example_1() {
        let mut my_hash_map = MyHashMap::new();
        my_hash_map.put(1, 1); // The map is now [[1,1]]
        my_hash_map.put(2, 2); // The map is now [[1,1], [2,2]]
        assert_eq!(my_hash_map.get(1), 1); // return 1, The map is now [[1,1], [2,2]]
        assert_eq!(my_hash_map.get(3), -1); // return -1 (i.e., not found), The map is now [[1,1], [2,2]]
        my_hash_map.put(2, 1); // The map is now [[1,1], [2,1]] (i.e., update the existing value)
        assert_eq!(my_hash_map.get(2), 1); // return 1, The map is now [[1,1], [2,1]]
        my_hash_map.remove(2); // remove the mapping for 2, The map is now [[1,1]]
        assert_eq!(my_hash_map.get(2), -1); // return -1 (i.e., not found), The map is now [[1,1]]
    }
}
