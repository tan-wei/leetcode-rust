/**
 * [1749] Maximum Absolute Sum of Any Subarray
 *
 * You are given an integer array nums. The absolute sum of a subarray [numsl, numsl+1, ..., numsr-1, numsr] is abs(numsl + numsl+1 + ... + numsr-1 + numsr).
 * Return the maximum absolute sum of any (possibly empty) subarray of nums.
 * Note that abs(x) is defined as follows:
 *
 * 	If x is a negative integer, then abs(x) = -x.
 * 	If x is a non-negative integer, then abs(x) = x.
 *
 *  
 * Example 1:
 *
 * Input: nums = [1,-3,2,3,-4]
 * Output: 5
 * Explanation: The subarray [2,3] has absolute sum = abs(2+3) = abs(5) = 5.
 *
 * Example 2:
 *
 * Input: nums = [2,-5,1,-4,3,-2]
 * Output: 8
 * Explanation: The subarray [-5,1,-4] has absolute sum = abs(-5+1-4) = abs(-8) = 8.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/
// discuss: https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut maximum = i32::MIN;
        let mut minimum = i32::MAX;

        let mut psum = 0; // positive sum
        let mut nsum = 0; // negative sum

        for &x in nums.iter() {
            psum += x;
            nsum += x;

            maximum = maximum.max(psum);
            minimum = minimum.min(nsum);

            if psum < 0 {
                psum = 0;
            }

            if nsum > 0 {
                nsum = 0;
            }
        }

        minimum.abs().max(maximum)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1749_example_1() {
        let nums = vec![1, -3, 2, 3, -4];

        let result = 5;

        assert_eq!(Solution::max_absolute_sum(nums), result);
    }

    #[test]
    fn test_1749_example_2() {
        let nums = vec![2, -5, 1, -4, 3, -2];

        let result = 8;

        assert_eq!(Solution::max_absolute_sum(nums), result);
    }
}
