/**
 * [0705] Design HashSet
 *
 * Design a HashSet without using any built-in hash table libraries.
 * Implement MyHashSet class:
 *
 * 	void add(key) Inserts the value key into the HashSet.
 * 	bool contains(key) Returns whether the value key exists in the HashSet or not.
 * 	void remove(key) Removes the value key in the HashSet. If key does not exist in the HashSet, do nothing.
 *
 *  
 * Example 1:
 *
 * Input
 * ["MyHashSet", "add", "add", "contains", "contains", "add", "contains", "remove", "contains"]
 * [[], [1], [2], [1], [3], [2], [2], [2], [2]]
 * Output
 * [null, null, null, true, false, null, true, null, false]
 * Explanation
 * MyHashSet myHashSet = new MyHashSet();
 * myHashSet.add(1);      // set = [1]
 * myHashSet.add(2);      // set = [1, 2]
 * myHashSet.contains(1); // return True
 * myHashSet.contains(3); // return False, (not found)
 * myHashSet.add(2);      // set = [1, 2]
 * myHashSet.contains(2); // return True
 * myHashSet.remove(2);   // set = [1]
 * myHashSet.contains(2); // return False, (already removed)
 *  
 * Constraints:
 *
 * 	0 <= key <= 10^6
 * 	At most 10^4 calls will be made to add, remove, and contains.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-hashset/
// discuss: https://leetcode.com/problems/design-hashset/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct MyHashSet {
    set: Vec<bool>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            set: vec![false; 1_000_001],
        }
    }

    fn add(&mut self, key: i32) {
        if let Some(elem) = self.set.get_mut(key as usize) {
            *elem = true;
        }
    }

    fn remove(&mut self, key: i32) {
        if let Some(elem) = self.set.get_mut(key as usize) {
            *elem = false;
        }
    }

    fn contains(&self, key: i32) -> bool {
        if let Some(elem) = self.set.get(key as usize) {
            *elem
        } else {
            false
        }
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0705_example_1() {
        let mut my_hast_set = MyHashSet::new();
        my_hast_set.add(1); // set = [1]
        my_hast_set.add(2); // set = [1, 2]
        assert_eq!(my_hast_set.contains(1), true); // return True
        assert_eq!(my_hast_set.contains(3), false); // return False, (not found)
        my_hast_set.add(2); // set = [1, 2]
        assert_eq!(my_hast_set.contains(2), true); // return True
        my_hast_set.remove(2); // set = [1]
        assert_eq!(my_hast_set.contains(2), false); // return False, (already removed)
    }
}
