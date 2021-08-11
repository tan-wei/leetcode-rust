/**
 * [268] Missing Number
 *
 * Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.
 * Follow up: Could you implement a solution using only O(1) extra space complexity and O(n) runtime complexity?
 *  
 * Example 1:
 *
 * Input: nums = [3,0,1]
 * Output: 2
 * Explanation: n = 3 since there are 3 numbers, so all numbers are in the range [0,3]. 2 is the missing number in the range since it does not appear in nums.
 *
 * Example 2:
 *
 * Input: nums = [0,1]
 * Output: 2
 * Explanation: n = 2 since there are 2 numbers, so all numbers are in the range [0,2]. 2 is the missing number in the range since it does not appear in nums.
 *
 * Example 3:
 *
 * Input: nums = [9,6,4,2,3,5,7,0,1]
 * Output: 8
 * Explanation: n = 9 since there are 9 numbers, so all numbers are in the range [0,9]. 8 is the missing number in the range since it does not appear in nums.
 *
 * Example 4:
 *
 * Input: nums = [0]
 * Output: 1
 * Explanation: n = 1 since there is 1 number, so all numbers are in the range [0,1]. 1 is the missing number in the range since it does not appear in nums.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 10^4
 * 	0 <= nums[i] <= n
 * 	All the numbers of nums are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/missing-number/
// discuss: https://leetcode.com/problems/missing-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .fold(nums.len() as i32, |acc, pairs| {
                acc ^ pairs.0 as i32 ^ pairs.1
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0268_example_1() {
        let nums = vec![3, 0, 1];
        let result = 2;

        assert_eq!(Solution::missing_number(nums), result);
    }

    #[test]
    fn test_0268_example_2() {
        let nums = vec![0, 1];
        let result = 2;

        assert_eq!(Solution::missing_number(nums), result);
    }

    #[test]
    fn test_0268_example_3() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let result = 8;

        assert_eq!(Solution::missing_number(nums), result);
    }

    #[test]
    fn test_0268_example_4() {
        let nums = vec![0];
        let result = 1;

        assert_eq!(Solution::missing_number(nums), result);
    }
}
