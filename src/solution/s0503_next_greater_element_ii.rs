/**
 * [0503] Next Greater Element II
 *
 * Given a circular integer array nums (i.e., the next element of nums[nums.length - 1] is nums[0]), return the next greater number for every element in nums.
 * The next greater number of a number x is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, return -1 for this number.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,1]
 * Output: [2,-1,2]
 * Explanation: The first 1's next greater number is 2;
 * The number 2 can't find next greater number.
 * The second 1's next greater number needs to search circularly, which is also 2.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4,3]
 * Output: [2,3,4,-1,4]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/next-greater-element-ii/
// discuss: https://leetcode.com/problems/next-greater-element-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut stack = vec![];
        let mut result = vec![-1; n];

        for i in 0..n * 2 {
            while stack.len() > 0 && nums[*stack.last().unwrap()] < nums[i % n] {
                result[*stack.last().unwrap()] = nums[i % n];
                stack.pop();
            }

            stack.push(i % n);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0503_example_1() {
        let nums = vec![1, 2, 1];
        let result = vec![2, -1, 2];

        assert_eq!(Solution::next_greater_elements(nums), result);
    }

    #[test]
    fn test_0503_example_2() {
        let nums = vec![1, 2, 3, 4, 3];
        let result = vec![2, 3, 4, -1, 4];

        assert_eq!(Solution::next_greater_elements(nums), result);
    }
}
