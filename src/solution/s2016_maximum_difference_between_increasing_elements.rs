/**
 * [2016] Maximum Difference Between Increasing Elements
 *
 * Given a 0-indexed integer array nums of size n, find the maximum difference between nums[i] and nums[j] (i.e., nums[j] - nums[i]), such that 0 <= i < j < n and nums[i] < nums[j].
 * Return the maximum difference. If no such i and j exists, return -1.
 *  
 * Example 1:
 *
 * Input: nums = [7,<u>1</u>,<u>5</u>,4]
 * Output: 4
 * Explanation:
 * The maximum difference occurs with i = 1 and j = 2, nums[j] - nums[i] = 5 - 1 = 4.
 * Note that with i = 1 and j = 0, the difference nums[j] - nums[i] = 7 - 1 = 6, but i > j, so it is not valid.
 *
 * Example 2:
 *
 * Input: nums = [9,4,3,2]
 * Output: -1
 * Explanation:
 * There is no i and j such that i < j and nums[i] < nums[j].
 *
 * Example 3:
 *
 * Input: nums = [<u>1</u>,5,2,<u>10</u>]
 * Output: 9
 * Explanation:
 * The maximum difference occurs with i = 0 and j = 3, nums[j] - nums[i] = 10 - 1 = 9.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	2 <= n <= 1000
 * 	1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-difference-between-increasing-elements/
// discuss: https://leetcode.com/problems/maximum-difference-between-increasing-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let r = nums[1..]
            .into_iter()
            .fold((0, nums[0]), |(b, x), &y| (b.max(y - x), x.min(y)))
            .0;

        if r == 0 {
            -1
        } else {
            r
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2016_example_1() {
        let nums = vec![7, 1, 5, 4];

        let result = 4;

        assert_eq!(Solution::maximum_difference(nums), result);
    }

    #[test]
    fn test_2016_example_2() {
        let nums = vec![9, 4, 3, 2];

        let result = -1;
        assert_eq!(Solution::maximum_difference(nums), result);
    }

    #[test]
    fn test_2016_example_3() {
        let nums = vec![1, 5, 2, 10];

        let result = 9;
        assert_eq!(Solution::maximum_difference(nums), result);
    }
}
