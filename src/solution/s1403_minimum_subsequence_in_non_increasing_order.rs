/**
 * [1403] Minimum Subsequence in Non-Increasing Order
 *
 * Given the array nums, obtain a subsequence of the array whose sum of elements is strictly greater than the sum of the non included elements in such subsequence.
 * If there are multiple solutions, return the subsequence with minimum size and if there still exist multiple solutions, return the subsequence with the maximum total sum of all its elements. A subsequence of an array can be obtained by erasing some (possibly zero) elements from the array.
 * Note that the solution with the given constraints is guaranteed to be unique. Also return the answer sorted in non-increasing order.
 *  
 * Example 1:
 *
 * Input: nums = [4,3,10,9,8]
 * Output: [10,9]
 * Explanation: The subsequences [10,9] and [10,8] are minimal such that the sum of their elements is strictly greater than the sum of elements not included. However, the subsequence [10,9] has the maximum total sum of its elements.
 *
 * Example 2:
 *
 * Input: nums = [4,4,7,6,7]
 * Output: [7,7,6]
 * Explanation: The subsequence [7,7] has the sum of its elements equal to 14 which is not strictly greater than the sum of elements not included (14 = 4 + 4 + 6). Therefore, the subsequence [7,6,7] is the minimal satisfying the conditions. Note the subsequence has to be returned in non-decreasing order.  
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 500
 * 	1 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-subsequence-in-non-increasing-order/
// discuss: https://leetcode.com/problems/minimum-subsequence-in-non-increasing-order/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut result = Vec::new();

        nums.sort();

        while nums.iter().sum::<i32>() >= result.iter().sum::<i32>() {
            result.push(nums.pop().unwrap());
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1403_example_1() {
        let nums = vec![4, 3, 10, 9, 8];

        let result = vec![10, 9];

        assert_eq!(Solution::min_subsequence(nums), result);
    }

    #[test]
    fn test_1403_example_2() {
        let nums = vec![4, 4, 7, 6, 7];

        let result = vec![7, 7, 6];

        assert_eq!(Solution::min_subsequence(nums), result);
    }
}
