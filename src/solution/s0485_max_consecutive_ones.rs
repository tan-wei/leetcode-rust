/**
 * [0485] Max Consecutive Ones
 *
 * Given a binary array nums, return the maximum number of consecutive 1's in the array.
 *  
 * Example 1:
 *
 * Input: nums = [1,1,0,1,1,1]
 * Output: 3
 * Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.
 *
 * Example 2:
 *
 * Input: nums = [1,0,1,1,0,1]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	nums[i] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-consecutive-ones/
// discuss: https://leetcode.com/problems/max-consecutive-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(acc, n), num| {
                if *num == 1 {
                    (acc + 1, n.max(acc + 1))
                } else {
                    (0, n)
                }
            })
            .1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0485_example_1() {
        let nums = vec![1, 1, 0, 1, 1, 1];
        let result = 3;

        assert_eq!(Solution::find_max_consecutive_ones(nums), result);
    }

    #[test]
    fn test_0485_example_2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        let result = 2;

        assert_eq!(Solution::find_max_consecutive_ones(nums), result);
    }
}
