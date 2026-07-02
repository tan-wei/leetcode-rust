/**
 * [2527] Find Xor-Beauty of Array
 *
 * You are given a 0-indexed integer array nums.
 * The effective value of three indices i, j, and k is defined as ((nums[i] | nums[j]) &amp; nums[k]).
 * The xor-beauty of the array is the XORing of the effective values of all the possible triplets of indices (i, j, k) where 0 <= i, j, k < n.
 * Return the xor-beauty of nums.
 * Note that:
 *
 * 	val1 | val2 is bitwise OR of val1 and val2.
 * 	val1 &amp; val2 is bitwise AND of val1 and val2.
 *
 *  
 * Example 1:
 *
 * Input: nums = [1,4]
 * Output: 5
 * Explanation:
 * The triplets and their corresponding effective values are listed below:
 * - (0,0,0) with effective value ((1 | 1) &amp; 1) = 1
 * - (0,0,1) with effective value ((1 | 1) &amp; 4) = 0
 * - (0,1,0) with effective value ((1 | 4) &amp; 1) = 1
 * - (0,1,1) with effective value ((1 | 4) &amp; 4) = 4
 * - (1,0,0) with effective value ((4 | 1) &amp; 1) = 1
 * - (1,0,1) with effective value ((4 | 1) &amp; 4) = 4
 * - (1,1,0) with effective value ((4 | 4) &amp; 1) = 0
 * - (1,1,1) with effective value ((4 | 4) &amp; 4) = 4
 * Xor-beauty of array will be bitwise XOR of all beauties = 1 ^ 0 ^ 1 ^ 4 ^ 1 ^ 4 ^ 0 ^ 4 = 5.
 * Example 2:
 *
 * Input: nums = [15,45,20,2,34,35,5,44,32,30]
 * Output: 34
 * Explanation: The xor-beauty of the given array is 34.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-xor-beauty-of-array/
// discuss: https://leetcode.com/problems/find-xor-beauty-of-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn xor_beauty(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2527_example_1() {
        let nums = vec![1, 4];

        let result = 5;

        assert_eq!(Solution::xor_beauty(nums), result);
    }

    #[test]
    fn test_2527_example_2() {
        let nums = vec![15, 45, 20, 2, 34, 35, 5, 44, 32, 30];

        let result = 34;

        assert_eq!(Solution::xor_beauty(nums), result);
    }
}
