/**
 * [0974] Subarray Sums Divisible by K
 *
 * Given an integer array nums and an integer k, return the number of non-empty subarrays that have a sum divisible by k.
 * A subarray is a contiguous part of an array.
 *  
 * Example 1:
 *
 * Input: nums = [4,5,0,-2,-3,1], k = 5
 * Output: 7
 * Explanation: There are 7 subarrays with a sum divisible by k = 5:
 * [4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]
 *
 * Example 2:
 *
 * Input: nums = [5], k = 9
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 3 * 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 	2 <= k <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subarray-sums-divisible-by-k/
// discuss: https://leetcode.com/problems/subarray-sums-divisible-by-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = vec![0; k as usize];
        count[0] = 1;
        let mut prefix = 0;
        let mut result = 0;
        for num in nums {
            prefix = (prefix + num).rem_euclid(k);
            result += count[prefix as usize];
            count[prefix as usize] += 1;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0974_example_1() {
        let nums = vec![4, 5, 0, -2, -3, 1];
        let k = 5;
        let result = 7;
        assert_eq!(Solution::subarrays_div_by_k(nums, k), result);
    }

    #[test]
    fn test_0974_example_2() {
        let nums = vec![5];
        let k = 9;
        let result = 0;
        assert_eq!(Solution::subarrays_div_by_k(nums, k), result);
    }
}
