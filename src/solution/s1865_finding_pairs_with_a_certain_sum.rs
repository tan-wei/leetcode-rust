/**
 * [1865] Finding Pairs With a Certain Sum
 *
 * You are given two integer arrays nums1 and nums2. You are tasked to implement a data structure that supports queries of two types:
 * <ol>
 * 	Add a positive integer to an element of a given index in the array nums2.
 * 	Count the number of pairs (i, j) such that nums1[i] + nums2[j] equals a given value (0 <= i < nums1.length and 0 <= j < nums2.length).
 * </ol>
 * Implement the FindSumPairs class:
 *
 * 	FindSumPairs(int[] nums1, int[] nums2) Initializes the FindSumPairs object with two integer arrays nums1 and nums2.
 * 	void add(int index, int val) Adds val to nums2[index], i.e., apply nums2[index] += val.
 * 	int count(int tot) Returns the number of pairs (i, j) such that nums1[i] + nums2[j] == tot.
 *
 *  
 * Example 1:
 *
 * Input
 * ["FindSumPairs", "count", "add", "count", "count", "add", "add", "count"]
 * [[[1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]], [7], [3, 2], [8], [4], [0, 1], [1, 1], [7]]
 * Output
 * [null, 8, null, 2, 1, null, null, 11]
 * Explanation
 * FindSumPairs findSumPairs = new FindSumPairs([1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]);
 * findSumPairs.count(7);  // return 8; pairs (2,2), (3,2), (4,2), (2,4), (3,4), (4,4) make 2 + 5 and pairs (5,1), (5,5) make 3 + 4
 * findSumPairs.add(3, 2); // now nums2 = [1,4,5,<u>4</u>,5,4]
 * findSumPairs.count(8);  // return 2; pairs (5,2), (5,4) make 3 + 5
 * findSumPairs.count(4);  // return 1; pair (5,0) makes 3 + 1
 * findSumPairs.add(0, 1); // now nums2 = [<u>2</u>,4,5,4,5,4]
 * findSumPairs.add(1, 1); // now nums2 = [2,<u>5</u>,5,4,5,4]
 * findSumPairs.count(7);  // return 11; pairs (2,1), (2,2), (2,4), (3,1), (3,2), (3,4), (4,1), (4,2), (4,4) make 2 + 5 and pairs (5,3), (5,5) make 3 + 4
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length <= 1000
 * 	1 <= nums2.length <= 10^5
 * 	1 <= nums1[i] <= 10^9
 * 	1 <= nums2[i] <= 10^5
 * 	0 <= index < nums2.length
 * 	1 <= val <= 10^5
 * 	1 <= tot <= 10^9
 * 	At most 1000 calls are made to add and count each.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/finding-pairs-with-a-certain-sum/
// discuss: https://leetcode.com/problems/finding-pairs-with-a-certain-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    map: std::collections::HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut map = std::collections::HashMap::new();
        for num in &nums2 {
            let cnt = map.entry(*num).or_insert(0);
            *cnt += 1;
        }

        Self { nums1, nums2, map }
    }

    fn add(&mut self, index: i32, val: i32) {
        if let Some(elem) = self.map.get_mut(&self.nums2[index as usize]) {
            *elem -= 1;
        }
        self.nums2[index as usize] += val;
        let cnt = self.map.entry(self.nums2[index as usize]).or_insert(0);
        *cnt += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        self.nums1.iter().fold(0, |prev, x| {
            if let Some(elem) = self.map.get(&(tot - x)) {
                return prev + *elem;
            }
            prev
        })
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1865_example_1() {
        let mut find_sum_pairs = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
        assert_eq!(find_sum_pairs.count(7), 8); // return 8; pairs (2,2), (3,2), (4,2), (2,4), (3,4), (4,4) make 2 + 5 and pairs (5,1), (5,5) make 3 + 4
        find_sum_pairs.add(3, 2); // now nums2 = [1,4,5,<u>4</u>,5,4]
        assert_eq!(find_sum_pairs.count(8), 2); // return 2; pairs (5,2), (5,4) make 3 + 5
        assert_eq!(find_sum_pairs.count(4), 1); // return 1; pair (5,0) makes 3 + 1
        find_sum_pairs.add(0, 1); // now nums2 = [<u>2</u>,4,5,4,5,4]
        find_sum_pairs.add(1, 1); // now nums2 = [2,<u>5</u>,5,4,5,4]
        assert_eq!(find_sum_pairs.count(7), 11); // return 11; pairs (2,1), (2,2), (2,4), (3,1), (3,2), (3,4), (4,1), (4,2), (4,4) make 2 + 5 and pairs (5,3), (5,5) make 3 + 4
    }
}
