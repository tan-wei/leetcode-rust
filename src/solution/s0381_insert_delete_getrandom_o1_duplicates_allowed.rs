/**
 * [381] Insert Delete GetRandom O(1) - Duplicates allowed
 *
 * Implement the RandomizedCollection class:
 *
 * 	RandomizedCollection() Initializes the RandomizedCollection object.
 * 	bool insert(int val) Inserts an item val into the multiset if not present. Returns true if the item was not present, false otherwise.
 * 	bool remove(int val) Removes an item val from the multiset if present. Returns true if the item was present, false otherwise. Note that if val has multiple occurrences in the multiset, we only remove one of them.
 * 	int getRandom() Returns a random element from the current multiset of elements (it's guaranteed that at least one element exists when this method is called). The probability of each element being returned is linearly related to the number of same values the multiset contains.
 *
 * You must implement the functions of the class such that each function works in average O(1) time complexity.
 *  
 * Example 1:
 *
 * Input
 * ["RandomizedCollection", "insert", "insert", "insert", "getRandom", "remove", "getRandom"]
 * [[], [1], [1], [2], [], [1], []]
 * Output
 * [null, true, false, true, 2, true, 1]
 * Explanation
 * RandomizedCollection randomizedCollection = new RandomizedCollection();
 * randomizedCollection.insert(1);   // return True. Inserts 1 to the collection. Returns true as the collection did not contain 1.
 * randomizedCollection.insert(1);   // return False. Inserts another 1 to the collection. Returns false as the collection contained 1. Collection now contains [1,1].
 * randomizedCollection.insert(2);   // return True. Inserts 2 to the collection, returns true. Collection now contains [1,1,2].
 * randomizedCollection.getRandom(); // getRandom should return 1 with the probability 2/3, and returns 2 with the probability 1/3.
 * randomizedCollection.remove(1);   // return True. Removes 1 from the collection, returns true. Collection now contains [1,2].
 * randomizedCollection.getRandom(); // getRandom should return 1 and 2 both equally likely.
 *
 *  
 * Constraints:
 *
 * 	-2^31 <= val <= 2^31 - 1
 * 	At most 2 * 10^5  calls will be made to insert, remove, and getRandom.
 * 	There will be at least one element in the data structure when getRandom is called.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/
// discuss: https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

struct RandomizedCollection {
    rng: ThreadRng,
    numbers: Vec<i32>,
    map: HashMap<i32, HashSet<i32>>,
}

// Credit: https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/discuss/683547/Rust-Code
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    fn new() -> Self {
        Self {
            rng: Default::default(),
            numbers: vec![],
            map: Default::default(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let contained = self.map.contains_key(&val);
        if !contained {
            self.map.insert(val, Default::default());
        }
        self.map
            .get_mut(&val)
            .unwrap()
            .insert(self.numbers.len() as i32);
        self.numbers.push(val);

        !contained
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.map.get_mut(&val) {
            None => false,
            Some(x) => {
                let i = x.iter().next();
                match i {
                    None => {
                        return false;
                    }
                    Some(&z) => {
                        x.remove(&z);
                        let len = self.numbers.len();
                        if z < (len - 1) as i32 {
                            let last_one = self.numbers[len - 1];
                            self.numbers[z as usize] = last_one;
                            match self.map.get_mut(&last_one) {
                                Some(s) => {
                                    s.remove(&((len - 1) as i32));
                                    s.insert(z);
                                }
                                None => {
                                    return false;
                                }
                            }
                        }
                    }
                }
                self.numbers.pop();
                if let Some(x) = self.map.get(&val) {
                    if x.is_empty() {
                        self.map.remove(&val);
                    }
                };
                true
            }
        }
    }
    fn get_random(&mut self) -> i32 {
        self.numbers[self.rng.gen_range(0, self.numbers.len())]
    }
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0381_example_1() {
        let mut randomized_collection = RandomizedCollection::new();
        assert_eq!(randomized_collection.insert(1), true); // return True. Inserts 1 to the collection. Returns true as the collection did not contain 1.
        assert_eq!(randomized_collection.insert(1), false); // return False. Inserts another 1 to the collection. Returns false as the collection contained 1. Collection now contains [1,1].
        assert_eq!(randomized_collection.insert(2), true); // return True. Inserts 2 to the collection, returns true. Collection now contains [1,1,2].
        randomized_collection.get_random(); // get_random should return 1 with the probability 2/3, and returns 2 with the probability 1/3.
        assert_eq!(randomized_collection.remove(1), true); // return True. Removes 1 from the collection, returns true. Collection now contains [1,2].
        randomized_collection.get_random(); // get_random should return 1 and 2 both equally likely.
    }
}
