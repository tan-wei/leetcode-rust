/**
 * [0930] Binary Subarrays With Sum
 *
 * Given a binary array nums and an integer goal, return the number of non-empty subarrays with a sum goal.
 *
 * A subarray is a contiguous part of the array.
 *
 *  
 * Example 1:
 *
 *
 * Input: nums = [1,0,1,0,1], goal = 2
 * Output: 4
 * Explanation: The 4 subarrays are bolded and underlined below:
 * [<u>1,0,1</u>,0,1]
 * [<u>1,0,1,0</u>,1]
 * [1,<u>0,1,0,1</u>]
 * [1,0,<u>1,0,1</u>]
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0,0,0,0,0], goal = 0
 * Output: 15
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= nums.length <= 3 * 10^4
 * 	nums[i] is either 0 or 1.
 * 	0 <= goal <= nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-subarrays-with-sum/
// discuss: https://leetcode.com/problems/binary-subarrays-with-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        let mut v = vec![0; n + 1];
        for i in 0..n {
            v[i + 1] = v[i] + nums[i];
        }

        let mut result = 0;
        let mut count = std::collections::HashMap::<i32, i32>::new();
        for x in v {
            result += count.get(&x).or(Some(&0)).unwrap();
            *count.entry(x + goal).or_default() += 1;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0930_example_1() {
        let nums = vec![1, 0, 1, 0, 1];
        let goal = 2;
        let result = 4;

        assert_eq!(Solution::num_subarrays_with_sum(nums, goal), result);
    }

    #[test]
    fn test_0930_example_2() {
        let nums = vec![0, 0, 0, 0, 0];
        let goal = 0;
        let result = 15;

        assert_eq!(Solution::num_subarrays_with_sum(nums, goal), result);
    }
}
