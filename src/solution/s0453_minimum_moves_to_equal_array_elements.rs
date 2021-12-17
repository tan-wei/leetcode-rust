/**
 * [0453] Minimum Moves to Equal Array Elements
 *
 * Given an integer array nums of size n, return the minimum number of moves required to make all array elements equal.
 * In one move, you can increment n - 1 elements of the array by 1.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3]
 * Output: 3
 * Explanation: Only three moves are needed (remember each move increments two elements):
 * [1,2,3]  =>  [2,3,3]  =>  [3,4,3]  =>  [4,4,4]
 *
 * Example 2:
 *
 * Input: nums = [1,1,1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	The answer is guaranteed to fit in a 32-bit integer.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-moves-to-equal-array-elements/
// discuss: https://leetcode.com/problems/minimum-moves-to-equal-array-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut sum = 0;
        let len = nums.len();
        for v in nums.iter() {
            sum += *v;
            if *v < min {
                min = *v;
            }
        }
        sum - len as i32 * min
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0453_example_1() {
        let nums = vec![1, 2, 3];
        let result = 3;

        assert_eq!(Solution::min_moves(nums), result);
    }

    #[test]
    fn test_0453_example_2() {
        let nums = vec![1, 1, 1];
        let result = 0;

        assert_eq!(Solution::min_moves(nums), result);
    }
}
