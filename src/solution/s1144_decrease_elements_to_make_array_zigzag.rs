/**
 * [1144] Decrease Elements To Make Array Zigzag
 *
 * Given an array nums of integers, a move consists of choosing any element and decreasing it by 1.
 * An array A is a zigzag array if either:
 *
 * 	Every even-indexed element is greater than adjacent elements, ie. A[0] > A[1] < A[2] > A[3] < A[4] > ...
 * 	OR, every odd-indexed element is greater than adjacent elements, ie. A[0] < A[1] > A[2] < A[3] > A[4] < ...
 *
 * Return the minimum number of moves to transform the given array nums into a zigzag array.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3]
 * Output: 2
 * Explanation: We can decrease 2 to 0 or 3 to 1.
 *
 * Example 2:
 *
 * Input: nums = [9,6,1,6,2]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decrease-elements-to-make-array-zigzag/
// discuss: https://leetcode.com/problems/decrease-elements-to-make-array-zigzag/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let mut result = [0, 0];
        let n = nums.len();
        for i in 0..n {
            let left = if i > 0 { nums[i - 1] } else { 1001 };
            let right = if i + 1 < n { nums[i + 1] } else { 1001 };
            result[i % 2] += std::cmp::max(0, nums[i] - std::cmp::min(left, right) + 1);
        }
        std::cmp::min(result[0], result[1])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1144_example_1() {
        let nums = vec![1, 2, 3];
        let result = 2;

        assert_eq!(Solution::moves_to_make_zigzag(nums), result);
    }

    #[test]
    fn test_1144_example_2() {
        let nums = vec![9, 6, 1, 6, 2];
        let result = 4;

        assert_eq!(Solution::moves_to_make_zigzag(nums), result);
    }
}
