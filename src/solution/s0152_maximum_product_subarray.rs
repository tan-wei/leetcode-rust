/**
 * [152] Maximum Product Subarray
 *
 * Given an integer array nums, find a contiguous non-empty subarray within the array that has the largest product, and return the product.
 * It is guaranteed that the answer will fit in a 32-bit integer.
 * A subarray is a contiguous subsequence of the array.
 *  
 * Example 1:
 *
 * Input: nums = [2,3,-2,4]
 * Output: 6
 * Explanation: [2,3] has the largest product 6.
 *
 * Example 2:
 *
 * Input: nums = [-2,0,-1]
 * Output: 0
 * Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^4
 * 	-10 <= nums[i] <= 10
 * 	The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-subarray/
// discuss: https://leetcode.com/problems/maximum-product-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut dp = vec![(1, 1)];
        for i in 0..nums.len() {
            let products = [nums[i], nums[i] * dp[i].0, nums[i] * dp[i].1];
            dp.push((
                *products.iter().max().unwrap(),
                *products.iter().min().unwrap(),
            ));
        }

        dp.into_iter().skip(1).map(|(a, b)| a.max(b)).max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0152_example_1() {
        let nums = vec![2, 3, -2, 4];
        let result = 6;

        assert_eq!(Solution::max_product(nums), result);
    }

    #[test]
    fn test_0152_example_2() {
        let nums = vec![-2, 0, -1];
        let result = 0;

        assert_eq!(Solution::max_product(nums), result);
    }
}
