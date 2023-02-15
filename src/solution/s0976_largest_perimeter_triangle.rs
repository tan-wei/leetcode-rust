/**
 * [0976] Largest Perimeter Triangle
 *
 * Given an integer array nums, return the largest perimeter of a triangle with a non-zero area, formed from three of these lengths. If it is impossible to form any triangle of a non-zero area, return 0.
 *  
 * Example 1:
 *
 * Input: nums = [2,1,2]
 * Output: 5
 * Explanation: You can form a triangle with three side lengths: 1, 2, and 2.
 *
 * Example 2:
 *
 * Input: nums = [1,2,1,10]
 * Output: 0
 * Explanation:
 * You cannot use the side lengths 1, 1, and 2 to form a triangle.
 * You cannot use the side lengths 1, 1, and 10 to form a triangle.
 * You cannot use the side lengths 1, 2, and 10 to form a triangle.
 * As we cannot use any three side lengths to form a triangle of non-zero area, we return 0.
 *
 *  
 * Constraints:
 *
 * 	3 <= nums.length <= 10^4
 * 	1 <= nums[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-perimeter-triangle/
// discuss: https://leetcode.com/problems/largest-perimeter-triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return 0;
        }

        let mut nums = nums;
        nums.sort_unstable();
        for i in (2..=nums.len() - 1).rev() {
            if nums[i] < nums[i - 1] + nums[i - 2] {
                return nums[i] + nums[i - 1] + nums[i - 2];
            }
        }
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0976_example_1() {
        let nums = vec![2, 1, 2];
        let result = 5;

        assert_eq!(Solution::largest_perimeter(nums), result);
    }

    #[test]
    fn test_0976_example_2() {
        let nums = vec![1, 2, 1, 10];
        let result = 0;

        assert_eq!(Solution::largest_perimeter(nums), result);
    }
}
