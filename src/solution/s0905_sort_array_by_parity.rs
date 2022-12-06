/**
 * [0905] Sort Array By Parity
 *
 * Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.
 * Return any array that satisfies this condition.
 *  
 * Example 1:
 *
 * Input: nums = [3,1,2,4]
 * Output: [2,4,3,1]
 * Explanation: The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
 *
 * Example 2:
 *
 * Input: nums = [0]
 * Output: [0]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5000
 * 	0 <= nums[i] <= 5000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-array-by-parity/
// discuss: https://leetcode.com/problems/sort-array-by-parity/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable_by_key(|num| num % 2);
        nums
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_0905_example_1() {
        let nums = vec![3, 1, 2, 4];
        let result = vec![2, 4, 3, 1];

        assert_eq!(Solution::sort_array_by_parity(nums), result);
    }

    #[ignore]
    #[test]
    fn test_0905_example_2() {
        let nums = vec![0];
        let result = vec![0];

        assert_eq!(Solution::sort_array_by_parity(nums), result);
    }
}
