/**
 * [1991] Find the Middle Index in Array
 *
 * Given a 0-indexed integer array nums, find the leftmost middleIndex (i.e., the smallest amongst all the possible ones).
 * A middleIndex is an index where nums[0] + nums[1] + ... + nums[middleIndex-1] == nums[middleIndex+1] + nums[middleIndex+2] + ... + nums[nums.length-1].
 * If middleIndex == 0, the left side sum is considered to be 0. Similarly, if middleIndex == nums.length - 1, the right side sum is considered to be 0.
 * Return the leftmost middleIndex that satisfies the condition, or -1 if there is no such index.
 *  
 * Example 1:
 *
 * Input: nums = [2,3,-1,<u>8</u>,4]
 * Output: 3
 * Explanation: The sum of the numbers before index 3 is: 2 + 3 + -1 = 4
 * The sum of the numbers after index 3 is: 4 = 4
 *
 * Example 2:
 *
 * Input: nums = [1,-1,<u>4</u>]
 * Output: 2
 * Explanation: The sum of the numbers before index 2 is: 1 + -1 = 0
 * The sum of the numbers after index 2 is: 0
 *
 * Example 3:
 *
 * Input: nums = [2,5]
 * Output: -1
 * Explanation: There is no valid middleIndex.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	-1000 <= nums[i] <= 1000
 *
 *  
 * Note: This question is the same as 724: <a href="https://leetcode.com/problems/find-pivot-index/" target="_blank">https://leetcode.com/problems/find-pivot-index/</a>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-middle-index-in-array/
// discuss: https://leetcode.com/problems/find-the-middle-index-in-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();

        let mut sum_cur: i32 = 0;

        for (idx, num) in nums.into_iter().enumerate() {
            if sum_cur == sum - sum_cur - num {
                return idx as i32;
            }
            sum_cur += num;
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1991_example_1() {
        let nums = vec![2, 3, -1, 8, 4];

        let result = 3;

        assert_eq!(Solution::find_middle_index(nums), result);
    }

    #[test]
    fn test_1991_example_2() {
        let nums = vec![1, -1, 4];

        let result = 2;

        assert_eq!(Solution::find_middle_index(nums), result);
    }

    #[test]
    fn test_1991_example_3() {
        let nums = vec![2, 5];

        let result = -1;

        assert_eq!(Solution::find_middle_index(nums), result);
    }
}
