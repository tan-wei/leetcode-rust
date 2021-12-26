/**
 * [0462] Minimum Moves to Equal Array Elements II
 *
 * Given an integer array nums of size n, return the minimum number of moves required to make all array elements equal.
 * In one move, you can increment or decrement an element of the array by 1.
 * Test cases are designed so that the answer will fit in a 32-bit integer.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3]
 * Output: 2
 * Explanation:
 * Only two moves are needed (remember each move increments or decrements one element):
 * [<u>1</u>,2,3]  =>  [2,2,<u>3</u>]  =>  [2,2,2]
 *
 * Example 2:
 *
 * Input: nums = [1,10,2,9]
 * Output: 16
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/
// discuss: https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let median = nums[nums.len() / 2];
        nums.iter().map(|x| (x - median).abs()).sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0462_example_1() {
        let nums = vec![1, 2, 3];
        let result = 2;

        assert_eq!(Solution::min_moves2(nums), result);
    }

    #[test]
    fn test_0462_example_2() {
        let nums = vec![1, 10, 2, 9];
        let result = 16;

        assert_eq!(Solution::min_moves2(nums), result);
    }
}
