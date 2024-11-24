/**
 * [1775] Equal Sum Arrays With Minimum Number of Operations
 *
 * You are given two arrays of integers nums1 and <font face="monospace">nums2</font>, possibly of different lengths. The values in the arrays are between 1 and 6, inclusive.
 * In one operation, you can change any integer's value in any of the arrays to any value between 1 and 6, inclusive.
 * Return the minimum number of operations required to make the sum of values in nums1 equal to the sum of values in nums2. Return -1​​​​​ if it is not possible to make the sum of the two arrays equal.
 *  
 * Example 1:
 *
 * Input: nums1 = [1,2,3,4,5,6], nums2 = [1,1,2,2,2,2]
 * Output: 3
 * Explanation: You can make the sums of nums1 and nums2 equal with 3 operations. All indices are 0-indexed.
 * - Change nums2[0] to 6. nums1 = [1,2,3,4,5,6], nums2 = [<u>6</u>,1,2,2,2,2].
 * - Change nums1[5] to 1. nums1 = [1,2,3,4,5,<u>1</u>], nums2 = [6,1,2,2,2,2].
 * - Change nums1[2] to 2. nums1 = [1,2,<u>2</u>,4,5,1], nums2 = [6,1,2,2,2,2].
 *
 * Example 2:
 *
 * Input: nums1 = [1,1,1,1,1,1,1], nums2 = [6]
 * Output: -1
 * Explanation: There is no way to decrease the sum of nums1 or to increase the sum of nums2 to make them equal.
 *
 * Example 3:
 *
 * Input: nums1 = [6,6], nums2 = [1]
 * Output: 3
 * Explanation: You can make the sums of nums1 and nums2 equal with 3 operations. All indices are 0-indexed.
 * - Change nums1[0] to 2. nums1 = [<u>2</u>,6], nums2 = [1].
 * - Change nums1[1] to 2. nums1 = [2,<u>2</u>], nums2 = [1].
 * - Change nums2[0] to 4. nums1 = [2,2], nums2 = [<u>4</u>].
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 10^5
 * 	1 <= nums1[i], nums2[i] <= 6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/equal-sum-arrays-with-minimum-number-of-operations/
// discuss: https://leetcode.com/problems/equal-sum-arrays-with-minimum-number-of-operations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let a: i32 = nums1.iter().map(|&x| x as i32).sum();
        let b = nums2.iter().map(|&x| x as i32).sum();

        if (a < b) {
            return Self::min_operations(nums2, nums1);
        }

        let mut pq = std::collections::BinaryHeap::new();
        let mut amount = a - b;

        for n in nums1 {
            if n > 1 {
                pq.push(n - 1);
            }
        }

        for n in nums2 {
            if n < 6 {
                pq.push(6 - n);
            }
        }

        let mut result = 0;

        while amount > 0 && pq.is_empty() == false {
            amount -= pq.pop().unwrap();
            result += 1;
        }

        if amount > 0 {
            return -1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1775_example_1() {
        let nums1 = vec![1, 2, 3, 4, 5, 6];
        let nums2 = vec![1, 1, 2, 2, 2, 2];

        let result = 3;

        assert_eq!(Solution::min_operations(nums1, nums2), result);
    }

    #[test]
    fn test_1775_example_2() {
        let nums1 = vec![1, 1, 1, 1, 1, 1, 1];
        let nums2 = vec![6];

        let result = -1;

        assert_eq!(Solution::min_operations(nums1, nums2), result);
    }

    #[test]
    fn test_1775_example_3() {
        let nums1 = vec![6, 6];
        let nums2 = vec![1];

        let result = 3;

        assert_eq!(Solution::min_operations(nums1, nums2), result);
    }
}
