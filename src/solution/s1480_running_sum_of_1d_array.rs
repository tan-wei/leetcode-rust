/**
 * [1480] Running Sum of 1d Array
 *
 * Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]&hellip;nums[i]).
 *
 * Return the running sum of nums.
 *
 *  
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,4]
 * Output: [1,3,6,10]
 * Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
 *
 * Example 2:
 *
 *
 * Input: nums = [1,1,1,1,1]
 * Output: [1,2,3,4,5]
 * Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].
 *
 * Example 3:
 *
 *
 * Input: nums = [3,1,2,10,1]
 * Output: [3,4,6,16,17]
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= nums.length <= 1000
 * 	-10^6 <= nums[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/running-sum-of-1d-array/
// discuss: https://leetcode.com/problems/running-sum-of-1d-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .scan(0, |sum, x| {
                *sum += x;
                Some(*sum)
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1480_example_1() {
        let nums = vec![1, 2, 3, 4];

        let result = vec![1, 3, 6, 10];

        assert_eq!(Solution::running_sum(nums), result);
    }

    #[test]
    fn test_1480_example_2() {
        let nums = vec![1, 1, 1, 1, 1];

        let result = vec![1, 2, 3, 4, 5];

        assert_eq!(Solution::running_sum(nums), result);
    }

    #[test]
    fn test_1480_example_3() {
        let nums = vec![3, 1, 2, 10, 1];

        let result = vec![3, 4, 6, 16, 17];

        assert_eq!(Solution::running_sum(nums), result);
    }
}
