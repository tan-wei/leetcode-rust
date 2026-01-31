/**
 * [2336] Smallest Number in Infinite Set
 *
 * You have a set which contains all positive integers [1, 2, 3, 4, 5, ...].
 * Implement the SmallestInfiniteSet class:
 *
 * 	SmallestInfiniteSet() Initializes the SmallestInfiniteSet object to contain all positive integers.
 * 	int popSmallest() Removes and returns the smallest integer contained in the infinite set.
 * 	void addBack(int num) Adds a positive integer num back into the infinite set, if it is not already in the infinite set.
 *
 *  
 * Example 1:
 *
 * Input
 * ["SmallestInfiniteSet", "addBack", "popSmallest", "popSmallest", "popSmallest", "addBack", "popSmallest", "popSmallest", "popSmallest"]
 * [[], [2], [], [], [], [1], [], [], []]
 * Output
 * [null, null, 1, 2, 3, null, 1, 4, 5]
 * Explanation
 * SmallestInfiniteSet smallestInfiniteSet = new SmallestInfiniteSet();
 * smallestInfiniteSet.addBack(2);    // 2 is already in the set, so no change is made.
 * smallestInfiniteSet.popSmallest(); // return 1, since 1 is the smallest number, and remove it from the set.
 * smallestInfiniteSet.popSmallest(); // return 2, and remove it from the set.
 * smallestInfiniteSet.popSmallest(); // return 3, and remove it from the set.
 * smallestInfiniteSet.addBack(1);    // 1 is added back to the set.
 * smallestInfiniteSet.popSmallest(); // return 1, since 1 was added back to the set and
 *                                    // is the smallest number, and remove it from the set.
 * smallestInfiniteSet.popSmallest(); // return 4, and remove it from the set.
 * smallestInfiniteSet.popSmallest(); // return 5, and remove it from the set.
 *
 *  
 * Constraints:
 *
 * 	1 <= num <= 1000
 * 	At most 1000 calls will be made in total to popSmallest and addBack.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-number-in-infinite-set/
// discuss: https://leetcode.com/problems/smallest-number-in-infinite-set/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/smallest-number-in-infinite-set/solutions/3452083/rust-binaryheap-hashset-by-icyrainz-r8i0/

struct SmallestInfiniteSet {
    heap: std::collections::BinaryHeap<i32>,
    set: std::collections::HashSet<i32>,
    smallest: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            heap: std::collections::BinaryHeap::new(),
            set: std::collections::HashSet::new(),
            smallest: 1,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        match self.heap.pop() {
            Some(n) => {
                self.set.remove(&n);
                -n
            }
            None => {
                self.smallest += 1;
                self.smallest - 1
            }
        }
    }

    fn add_back(&mut self, num: i32) {
        if num >= self.smallest {
            return;
        }
        if !self.set.contains(&(-num)) {
            self.heap.push(-num);
            self.set.insert(-num);
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2336_example_1() {
        let mut smallest_infinite_set = SmallestInfiniteSet::new();
        smallest_infinite_set.add_back(2); // 2 is already in the set, so no change is made.
        assert_eq!(smallest_infinite_set.pop_smallest(), 1); // return 1, since 1 is the smallest number, and remove it from the set.
        assert_eq!(smallest_infinite_set.pop_smallest(), 2); // return 2, and remove it from the set.
        assert_eq!(smallest_infinite_set.pop_smallest(), 3); // return 3, and remove it from the set.

        smallest_infinite_set.add_back(1); // 1 is added back to the set.

        assert_eq!(smallest_infinite_set.pop_smallest(), 1); // return 1, since 1 was added back to the set and is the smallest number, and remove it from the set.

        assert_eq!(smallest_infinite_set.pop_smallest(), 4); // return 4, and remove it from the set.
        assert_eq!(smallest_infinite_set.pop_smallest(), 5); // return 5, and remove it from the set.
    }
}
