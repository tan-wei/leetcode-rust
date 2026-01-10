/**
 * [2276] Count Integers in Intervals
 *
 * Given an empty set of intervals, implement a data structure that can:
 *
 * 	Add an interval to the set of intervals.
 * 	Count the number of integers that are present in at least one interval.
 *
 * Implement the CountIntervals class:
 *
 * 	CountIntervals() Initializes the object with an empty set of intervals.
 * 	void add(int left, int right) Adds the interval [left, right] to the set of intervals.
 * 	int count() Returns the number of integers that are present in at least one interval.
 *
 * Note that an interval [left, right] denotes all the integers x where left <= x <= right.
 *  
 * Example 1:
 *
 * Input
 * ["CountIntervals", "add", "add", "count", "add", "count"]
 * [[], [2, 3], [7, 10], [], [5, 8], []]
 * Output
 * [null, null, null, 6, null, 8]
 * Explanation
 * CountIntervals countIntervals = new CountIntervals(); // initialize the object with an empty set of intervals.
 * countIntervals.add(2, 3);  // add [2, 3] to the set of intervals.
 * countIntervals.add(7, 10); // add [7, 10] to the set of intervals.
 * countIntervals.count();    // return 6
 *                            // the integers 2 and 3 are present in the interval [2, 3].
 *                            // the integers 7, 8, 9, and 10 are present in the interval [7, 10].
 * countIntervals.add(5, 8);  // add [5, 8] to the set of intervals.
 * countIntervals.count();    // return 8
 *                            // the integers 2 and 3 are present in the interval [2, 3].
 *                            // the integers 5 and 6 are present in the interval [5, 8].
 *                            // the integers 7 and 8 are present in the intervals [5, 8] and [7, 10].
 *                            // the integers 9 and 10 are present in the interval [7, 10].
 *
 *  
 * Constraints:
 *
 * 	1 <= left <= right <= 10^9
 * 	At most 10^5 calls in total will be made to add and count.
 * 	At least one call will be made to count.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-integers-in-intervals/
// discuss: https://leetcode.com/problems/count-integers-in-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/count-integers-in-intervals/solutions/2375879/rust-solution-using-btreemap-by-xiaoping-pshe/

struct CountIntervals {
    count: i32,
    map: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CountIntervals {
    fn new() -> Self {
        let mp: std::collections::BTreeMap<i32, i32> = std::collections::BTreeMap::new();
        Self { count: 0, map: mp }
    }

    fn add(&mut self, left: i32, right: i32) {
        let (mut left, mut right) = (left, right);

        self.count += right - left + 1;
        if self.map.is_empty() {
            self.map.insert(left, right);
            return;
        }

        let (a, _) = self.map.iter().next().unwrap();
        let (_, b) = self.map.iter().rev().next().unwrap();
        if a - 1 > right || b + 1 < left {
            self.map.insert(left, right);
            return;
        }

        if let Some((&key, &value)) = self.map.range(..left + 1).rev().next() {
            if value + 1 >= left {
                if value > right {
                    self.count -= right - left + 1;
                } else {
                    self.count -= value + 1 - left;
                }
                left = key;
                right = right.max(value);
            }
        }

        let mut done = false;
        while done == false {
            done = true;
            if let Some((&key, &value)) = self.map.range(left + 1..).next() {
                if value > right {
                    break;
                }
                self.count -= value - key + 1;
                self.map.remove(&key);
                done = false;
            }
        }

        if let Some((&key, &value)) = self.map.range(left + 1..).next() {
            if right + 1 >= key {
                self.count -= right + 1 - key;
                right = value;
                self.map.remove(&key);
            }
        }

        self.map.insert(left, right);
    }

    fn count(&self) -> i32 {
        self.count
    }
}

/**
 * Your CountIntervals object will be instantiated and called as such:
 * let obj = CountIntervals::new();
 * obj.add(left, right);
 * let ret_2: i32 = obj.count();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2276_example_1() {
        let mut count_intervals = CountIntervals::new(); // initialize the object with an empty set of intervals.
        count_intervals.add(2, 3); // add [2, 3] to the set of intervals.
        count_intervals.add(7, 10); // add [7, 10] to the set of intervals.
        assert_eq!(count_intervals.count(), 6); // return 6
        // the integers 2 and 3 are present in the interval [2, 3].
        // the integers 7, 8, 9, and 10 are present in the interval [7, 10].
        count_intervals.add(5, 8); // add [5, 8] to the set of intervals.
        assert_eq!(count_intervals.count(), 8); // return 8
        // the integers 2 and 3 are present in the interval [2, 3].
        // the integers 5 and 6 are present in the interval [5, 8].
        // the integers 7 and 8 are present in the intervals [5, 8] and [7, 10].
        // the integers 9 and 10 are present in the interval [7, 10].
    }
}
