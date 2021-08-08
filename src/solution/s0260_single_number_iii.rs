/**
 * [260] Single Number III
 *
 * Given an integer array nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once. You can return the answer in any order.
 * You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,1,3,2,5]
 * Output: [3,5]
 * Explanation:  [5, 3] is also a valid answer.
 *
 * Example 2:
 *
 * Input: nums = [-1,0]
 * Output: [-1,0]
 *
 * Example 3:
 *
 * Input: nums = [0,1]
 * Output: [1,0]
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 3 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	Each integer in nums will appear twice, only two integers will appear once.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/single-number-iii/
// discuss: https://leetcode.com/problems/single-number-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor_all = nums.iter().fold(0, |acc, n| acc ^ n);
        let diff = xor_all & (-xor_all);

        let mut result = vec![0, 0];

        for num in nums {
            if num & diff == 0 {
                result[0] ^= num;
            } else {
                result[1] ^= num;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0260_exmaple_1() {
        let nums = vec![1, 2, 1, 3, 2, 5];
        let result = vec![3, 5];

        assert_eq_sorted!(Solution::single_number(nums), result);
    }

    #[test]
    fn test_0260_exmaple_2() {
        let nums = vec![-1, 0];
        let result = vec![-1, 0];

        assert_eq_sorted!(Solution::single_number(nums), result);
    }

    #[test]
    fn test_0260_exmaple_3() {
        let nums = vec![0, 1];
        let result = vec![1, 0];

        assert_eq_sorted!(Solution::single_number(nums), result);
    }
}
