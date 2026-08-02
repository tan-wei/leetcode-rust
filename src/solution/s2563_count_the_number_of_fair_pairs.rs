/**
 * [2563] Count the Number of Fair Pairs
 *
 * Given a 0-indexed integer array nums of size n and two integers lower and upper, return the number of fair pairs.
 * A pair (i, j) is fair if:
 *
 * 	0 <= i < j < n, and
 * 	lower <= nums[i] + nums[j] <= upper
 *
 *  
 * Example 1:
 *
 * Input: nums = [0,1,7,4,4,5], lower = 3, upper = 6
 * Output: 6
 * Explanation: There are 6 fair pairs: (0,3), (0,4), (0,5), (1,3), (1,4), and (1,5).
 *
 * Example 2:
 *
 * Input: nums = [1,7,9,2,5], lower = 11, upper = 11
 * Output: 1
 * Explanation: There is a single fair pair: (2,3).
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	nums.length == n
 * 	<font face="monospace">-10^9</font> <= nums[i] <= 10^9
 * 	<font face="monospace">-10^9 <= lower <= upper <= 10^9</font>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-fair-pairs/
// discuss: https://leetcode.com/problems/count-the-number-of-fair-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();

        (0..nums.len())
            .map(|i| {
                nums[..i].partition_point(|y| nums[i] + y <= upper)
                    - nums[..i].partition_point(|y| nums[i] + y < lower)
            })
            .sum::<usize>() as i64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_2563_example_1() {
        let nums = vec![0, 1, 7, 4, 4, 5];
        let lower = 3;
        let upper = 6;

        let result = 6;

        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), result);
    }

    #[test]
    fn test_2563_example_2() {
        let nums = vec![1, 7, 9, 2, 5];
        let lower = 11;
        let upper = 11;

        let result = 1;

        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), result);
    }
}
