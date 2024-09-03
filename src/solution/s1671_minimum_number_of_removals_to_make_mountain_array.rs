/**
 * [1671] Minimum Number of Removals to Make Mountain Array
 *
 * You may recall that an array arr is a mountain array if and only if:
 *
 * 	arr.length >= 3
 * 	There exists some index i (0-indexed) with 0 < i < arr.length - 1 such that:
 *
 * 		arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
 * 		arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
 *
 *
 *
 * Given an integer array nums​​​, return the minimum number of elements to remove to make nums​​​ a mountain array.
 *  
 * Example 1:
 *
 * Input: nums = [1,3,1]
 * Output: 0
 * Explanation: The array itself is a mountain array so we do not need to remove any elements.
 *
 * Example 2:
 *
 * Input: nums = [2,1,1,5,6,2,3,1]
 * Output: 3
 * Explanation: One solution is to remove the elements at indices 0, 1, and 5, making the array nums = [1,5,6,3,1].
 *
 *  
 * Constraints:
 *
 * 	3 <= nums.length <= 1000
 * 	1 <= nums[i] <= 10^9
 * 	It is guaranteed that you can make a mountain array out of nums.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/
// discuss: https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/solutions/3188288/just-a-runnable-solution/
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];

        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }

        let mut dp2 = vec![1; n];

        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                if nums[i] > nums[j] {
                    dp2[i] = dp2[i].max(dp2[j] + 1);
                }
            }
        }

        let mut result = 0;

        for i in 0..n {
            if dp[i] > 1 && dp2[i] > 1 {
                result = result.max(dp[i] + dp2[i] - 1);
            }
        }
        (n - result) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1671_example_1() {
        let nums = vec![1, 3, 1];

        let result = 0;

        assert_eq!(Solution::minimum_mountain_removals(nums), result);
    }

    #[test]
    fn test_1671_example_2() {
        let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];

        let result = 3;

        assert_eq!(Solution::minimum_mountain_removals(nums), result);
    }
}
