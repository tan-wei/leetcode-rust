/**
 * [0713] Subarray Product Less Than K
 *
 * Given an array of integers nums and an integer k, return the number of contiguous subarrays where the product of all the elements in the subarray is strictly less than k.
 *  
 * Example 1:
 *
 * Input: nums = [10,5,2,6], k = 100
 * Output: 8
 * Explanation: The 8 subarrays that have product less than 100 are:
 * [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
 * Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3], k = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 3 * 10^4
 * 	1 <= nums[i] <= 1000
 * 	0 <= k <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subarray-product-less-than-k/
// discuss: https://leetcode.com/problems/subarray-product-less-than-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/subarray-product-less-than-k/discuss/869008/Rust-sliding-window-solution
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }

        let mut result = 0;
        let mut p = 1;
        let mut j = 0;
        for (i, num) in nums.iter().enumerate() {
            p *= num;
            while p >= k {
                p /= nums[j];
                j += 1;
            }
            result += i + 1 - j;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0713_example_1() {
        let nums = vec![10, 5, 2, 6];
        let k = 100;
        let result = 8;

        assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), result);
    }

    #[test]
    fn test_0713_example_2() {
        let nums = vec![1, 2, 3];
        let k = 0;
        let result = 0;

        assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), result);
    }
}
