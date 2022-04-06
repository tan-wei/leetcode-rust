/**
 * [0628] Maximum Product of Three Numbers
 *
 * Given an integer array nums, find three numbers whose product is maximum and return the maximum product.
 *  
 * Example 1:
 * Input: nums = [1,2,3]
 * Output: 6
 * Example 2:
 * Input: nums = [1,2,3,4]
 * Output: 24
 * Example 3:
 * Input: nums = [-1,-2,-3]
 * Output: -6
 *  
 * Constraints:
 *
 * 	3 <= nums.length <= 10^4
 * 	-1000 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-of-three-numbers/
// discuss: https://leetcode.com/problems/maximum-product-of-three-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let n = nums.len();

        let last = nums[(n - 3)..n].iter().fold(1, |mut product, &val| {
            product *= val;
            product
        });

        let first = nums[n - 1]
            * nums[0..2].iter().fold(1, |mut product, &val| {
                product *= val;
                product
            });

        first.max(last)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0628_example_1() {
        let nums = vec![1, 2, 3];
        let result = 6;

        assert_eq!(Solution::maximum_product(nums), result);
    }

    #[test]
    fn test_0628_example_2() {
        let nums = vec![1, 2, 3, 4];
        let result = 24;

        assert_eq!(Solution::maximum_product(nums), result);
    }

    #[test]
    fn test_0628_example_3() {
        let nums = vec![-1, -2, -3];
        let result = -6;

        assert_eq!(Solution::maximum_product(nums), result);
    }
}
