/**
 * [2470] Number of Subarrays With LCM Equal to K
 *
 * Given an integer array nums and an integer k, return the number of subarrays of nums where the least common multiple of the subarray's elements is k.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 * The least common multiple of an array is the smallest positive integer that is divisible by all the array elements.
 *  
 * Example 1:
 *
 * Input: nums = [3,6,2,7,1], k = 6
 * Output: 4
 * Explanation: The subarrays of nums where 6 is the least common multiple of all the subarray's elements are:
 * - [<u>3</u>,<u>6</u>,2,7,1]
 * - [<u>3</u>,<u>6</u>,<u>2</u>,7,1]
 * - [3,<u>6</u>,2,7,1]
 * - [3,<u>6</u>,<u>2</u>,7,1]
 *
 * Example 2:
 *
 * Input: nums = [3], k = 2
 * Output: 0
 * Explanation: There are no subarrays of nums where 2 is the least common multiple of all the subarray's elements.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i], k <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-subarrays-with-lcm-equal-to-k/
// discuss: https://leetcode.com/problems/number-of-subarrays-with-lcm-equal-to-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2470_example_1() {
        let nums = vec![3, 6, 2, 7, 1];
        let k = 6;

        let result = 4;

        assert_eq!(Solution::subarray_lcm(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_2470_example_2() {
        let nums = vec![3];
        let k = 2;

        let result = 0;

        assert_eq!(Solution::subarray_lcm(nums, k), result);
    }
}
