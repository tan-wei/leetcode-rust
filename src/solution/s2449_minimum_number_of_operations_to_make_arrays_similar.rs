/**
 * [2449] Minimum Number of Operations to Make Arrays Similar
 *
 * You are given two positive integer arrays nums and target, of the same length.
 * In one operation, you can choose any two distinct indices i and j where 0 <= i, j < nums.length and:
 *
 * 	set nums[i] = nums[i] + 2 and
 * 	set nums[j] = nums[j] - 2.
 *
 * Two arrays are considered to be similar if the frequency of each element is the same.
 * Return the minimum number of operations required to make nums similar to target. The test cases are generated such that nums can always be similar to target.
 *  
 * Example 1:
 *
 * Input: nums = [8,12,6], target = [2,14,10]
 * Output: 2
 * Explanation: It is possible to make nums similar to target in two operations:
 * - Choose i = 0 and j = 2, nums = [10,12,4].
 * - Choose i = 1 and j = 2, nums = [10,14,2].
 * It can be shown that 2 is the minimum number of operations needed.
 *
 * Example 2:
 *
 * Input: nums = [1,2,5], target = [4,1,3]
 * Output: 1
 * Explanation: We can make nums similar to target in one operation:
 * - Choose i = 1 and j = 2, nums = [1,4,3].
 *
 * Example 3:
 *
 * Input: nums = [1,1,1,1,1], target = [1,1,1,1,1]
 * Output: 0
 * Explanation: The array nums is already similiar to target.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length == target.length
 * 	1 <= n <= 10^5
 * 	1 <= nums[i], target[i] <= 10^6
 * 	It is possible to make nums similar to target.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-operations-to-make-arrays-similar/
// discuss: https://leetcode.com/problems/minimum-number-of-operations-to-make-arrays-similar/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn make_similar(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2449_example_1() {
        let nums = vec![8, 12, 6];
        let target = vec![2, 14, 10];

        let result = 2;

        assert_eq!(Solution::make_similar(nums, target), result);
    }

    #[test]
    #[ignore]
    fn test_2449_example_2() {
        let nums = vec![1, 2, 5];
        let target = vec![4, 1, 3];

        let result = 1;

        assert_eq!(Solution::make_similar(nums, target), result);
    }

    #[test]
    #[ignore]
    fn test_2449_example_3() {
        let nums = vec![1, 1, 1, 1, 1];
        let target = vec![1, 1, 1, 1, 1];

        let result = 0;

        assert_eq!(Solution::make_similar(nums, target), result);
    }
}
