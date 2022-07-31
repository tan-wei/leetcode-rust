/**
 * [380] Insert Delete GetRandom O(1)
 *
 * Implement the RandomizedSet class:
 *
 * 	RandomizedSet() Initializes the RandomizedSet object.
 * 	bool insert(int val) Inserts an item val into the set if not present. Returns true if the item was not present, false otherwise.
 * 	bool remove(int val) Removes an item val from the set if present. Returns true if the item was present, false otherwise.
 * 	int getRandom() Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the same probability of being returned.
 *
 * You must implement the functions of the class such that each function works in average O(1) time complexity.
 *  
 * Example 1:
 *
 * Input
 * ["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
 * [[], [1], [2], [2], [], [1], [2], []]
 * Output
 * [null, true, false, true, 2, true, false, 2]
 * Explanation
 * RandomizedSet randomizedSet = new RandomizedSet();
 * randomizedSet.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
 * randomizedSet.remove(2); // Returns false as 2 does not exist in the set.
 * randomizedSet.insert(2); // Inserts 2 to the set, returns true. Set now contains [1,2].
 * randomizedSet.getRandom(); // getRandom() should return either 1 or 2 randomly.
 * randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
 * randomizedSet.insert(2); // 2 was already in the set, so return false.
 * randomizedSet.getRandom(); // Since 2 is the only number in the set, getRandom() will always return 2.
 *
 *  
 * Constraints:
 *
 * 	-2^31 <= val <= 2^31 - 1
 * 	At most 2 * 10^5 calls will be made to insert, remove, and getRandom.
 * 	There will be at least one element in the data structure when getRandom is called.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/insert-delete-getrandom-o1/
// discuss: https://leetcode.com/problems/insert-delete-getrandom-o1/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use rand::{thread_rng, Rng};
use std::convert::TryInto;

struct RandomizedSet {
    um: std::collections::HashMap<i32, i32>,
    v: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl RandomizedSet {
    fn new() -> Self {
        Self {
            v: vec![],
            um: std::collections::HashMap::new(),
        }
    }
    fn insert(&mut self, val: i32) -> bool {
        match self.um.get(&val) {
            Some(_) => false,
            None => {
                self.v.push(val);
                self.um.insert(val, (self.v.len() - 1).try_into().unwrap());
                true
            }
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.um.get(&val) {
            //Get value of key
            None => false,
            Some(index) => {
                self.v[*index as usize] = self.v[self.v.len() - 1]; //Copy last
                self.um.insert(self.v[self.v.len() - 1], *index);
                self.v.pop();
                self.um.remove(&val);
                true
            }
        }
    }
    fn get_random(&self) -> i32 {
        self.v[thread_rng().gen_range(0, self.v.len())]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0380_example_1() {
        let mut randomized_set = RandomizedSet::new();
        assert_eq!(randomized_set.insert(1), true); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
        assert_eq!(randomized_set.remove(2), false); // Returns false as 2 does not exist in the set.
        assert_eq!(randomized_set.insert(2), true); // Inserts 2 to the set, returns true. Set now contains [1,2].
        randomized_set.get_random(); // getRandom() should return either 1 or 2 randomly.
        assert_eq!(randomized_set.remove(1), true); // Removes 1 from the set, returns true. Set now contains [2].
        assert_eq!(randomized_set.insert(2), false); // 2 was already in the set, so return false.
        assert_eq!(randomized_set.get_random(), 2); // Since 2 is the only number in the set, getRandom() will always return 2.
    }
}
